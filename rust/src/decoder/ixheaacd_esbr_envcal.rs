extern "C" {
    fn log(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn sqrt(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn fabs(__x: core::ffi::c_double) -> core::ffi::c_double;
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
    static ixheaac_random_phase: [[FLOAT32; 2]; 512];
    static ixheaac_hphase_tbl: [[FLOAT32; 8]; 2];
    static ixheaac_g_lim_gains: [FLOAT32; 4];
    static mut ixheaac_fir_table: [*const ia_fir_table_struct; 5];
    static ixheaac_q_gamma_table: [FLOAT32; 4];
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type WORD32 = core::ffi::c_int;
pub type FLOAT32 = core::ffi::c_float;
pub type FLOAT64 = core::ffi::c_double;
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
pub type IA_ERRORCODE = WORD32;
pub type C2RustUnnamed = core::ffi::c_uint;
pub const PVC_SBR: C2RustUnnamed = 2;
pub const ORIG_SBR: C2RustUnnamed = 1;
pub const UNKNOWN_SBR: C2RustUnnamed = 0;
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
pub struct ixheaacd_lpp_trans_patch {
    pub num_patches: WORD32,
    pub start_subband: [WORD32; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_frame_info_data_struct {
    pub num_env_sfac: WORD16,
    pub str_frame_info_details: ia_frame_info_struct,
    pub del_cod_dir_arr: [WORD16; 8],
    pub del_cod_dir_noise_arr: [WORD16; 2],
    pub sbr_invf_mode: [WORD32; 10],
    pub coupling_mode: WORD32,
    pub amp_res: WORD16,
    pub max_qmf_subband_aac: WORD32,
    pub add_harmonics: [FLAG; 56],
    pub int_env_sf_arr: [WORD16; 448],
    pub int_noise_floor: [WORD16; 10],
    pub num_noise_sfac: WORD32,
    pub str_pvc_frame_info: ia_frame_info_struct,
    pub env_short_flag_prev: WORD32,
    pub pstr_sbr_header: *mut ia_sbr_header_data_struct,
    pub num_time_slots: WORD32,
    pub rate: WORD32,
    pub sbr_patching_mode: WORD32,
    pub prev_sbr_patching_mode: WORD32,
    pub over_sampling_flag: WORD32,
    pub pitch_in_bins: WORD32,
    pub pvc_mode: WORD32,
    pub cov_count: WORD32,
    pub sbr_invf_mode_prev: [WORD32; 10],
    pub flt_env_sf_arr: [FLOAT32; 448],
    pub flt_noise_floor: [FLOAT32; 10],
    pub sfb_nrg_prev: [FLOAT32; 56],
    pub prev_noise_level: [FLOAT32; 10],
    pub inter_temp_shape_mode: [WORD32; 8],
    pub var_len: WORD32,
    pub bs_sin_pos_present: WORD32,
    pub sine_position: WORD32,
    pub sin_start_for_next_top: WORD32,
    pub sin_len_for_next_top: WORD32,
    pub sin_start_for_cur_top: WORD32,
    pub sin_len_for_cur_top: WORD32,
    pub var_len_id_prev: WORD32,
    pub str_frame_info_prev: ia_frame_info_struct,
    pub bw_array_prev: [FLOAT32; 6],
    pub patch_param: ixheaacd_lpp_trans_patch,
    pub harm_index: WORD32,
    pub phase_index: WORD32,
    pub harm_flag_prev: [WORD8; 64],
    pub e_gain: [[FLOAT32; 64]; 5],
    pub noise_buf: [[FLOAT32; 64]; 5],
    pub lim_table: [[WORD32; 13]; 4],
    pub gate_mode: [WORD32; 4],
    pub harm_flag_varlen_prev: [WORD8; 64],
    pub harm_flag_varlen: [WORD8; 64],
    pub qmapped_pvc: [[FLOAT32; 48]; 64],
    pub env_tmp: [[FLOAT32; 48]; 64],
    pub noise_level_pvc: [[FLOAT32; 48]; 64],
    pub nrg_est_pvc: [[FLOAT32; 48]; 64],
    pub nrg_ref_pvc: [[FLOAT32; 48]; 64],
    pub nrg_gain_pvc: [[FLOAT32; 48]; 64],
    pub nrg_tone_pvc: [[FLOAT32; 48]; 64],
    pub stereo_config_idx: WORD32,
    pub reset_flag: FLAG,
    pub mps_sbr_flag: FLAG,
    pub usac_independency_flag: FLAG,
    pub inter_tes_flag: FLAG,
    pub sbr_mode: FLAG,
    pub prev_sbr_mode: FLAG,
    pub eld_sbr_flag: WORD32,
}
pub type ia_fir_table_struct = [FLOAT32; 5];
pub const LOW: core::ffi::c_int = 0 as core::ffi::c_int;
pub const HIGH: core::ffi::c_int = 1 as core::ffi::c_int;
pub const TIMESLOT_BUFFER_SIZE: core::ffi::c_int = 78 as core::ffi::c_int;
pub const EPS: core::ffi::c_float = 1e-12f32;
pub const MAX_NOISE_ENVELOPES: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_NOISE_COEFFS: core::ffi::c_int = 5 as core::ffi::c_int;
pub const MAX_FREQ_COEFFS_SBR: core::ffi::c_int = 48 as core::ffi::c_int;
pub const MAX_NUM_PATCHES: core::ffi::c_int = 6 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_shellsort(
    mut in_0: *mut WORD32,
    mut n: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut v: WORD32 = 0;
    let mut inc: WORD32 = 1 as WORD32;
    loop {
        inc = (3 as core::ffi::c_int * inc as core::ffi::c_int + 1 as core::ffi::c_int)
            as WORD32;
        if !(inc <= n) {
            break;
        }
    }
    loop {
        inc = (inc as core::ffi::c_int / 3 as core::ffi::c_int) as WORD32;
        i = (inc as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
        while i <= n {
            v = *in_0.offset((i as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
            j = i;
            while *in_0
                .offset(
                    (j as core::ffi::c_int - inc as core::ffi::c_int
                        - 1 as core::ffi::c_int) as isize,
                ) > v
            {
                *in_0.offset((j as core::ffi::c_int - 1 as core::ffi::c_int) as isize) = *in_0
                    .offset(
                        (j as core::ffi::c_int - inc as core::ffi::c_int
                            - 1 as core::ffi::c_int) as isize,
                    );
                j -= inc;
                if j <= inc {
                    break;
                }
            }
            *in_0.offset((j as core::ffi::c_int - 1 as core::ffi::c_int) as isize) = v;
            i += 1;
        }
        if !(inc > 1 as core::ffi::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_sbr_env_calc(
    mut frame_data: *mut ia_sbr_frame_info_data_struct,
    mut input_real: *mut [FLOAT32; 64],
    mut input_imag: *mut [FLOAT32; 64],
    mut input_real1: *mut [FLOAT32; 64],
    mut input_imag1: *mut [FLOAT32; 64],
    mut x_over_qmf: *mut WORD32,
    mut scratch_buff: *mut FLOAT32,
    mut env_out: *mut FLOAT32,
    mut ldmps_present: WORD32,
    mut ec_flag: WORD32,
) -> WORD32 {
    let mut err_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut harmonics: [WORD8; 64] = [0; 64];
    let mut env_tmp: *mut [FLOAT32; 48] = 0 as *mut [FLOAT32; 48];
    let mut noise_level_pvc: *mut [FLOAT32; 48] = 0 as *mut [FLOAT32; 48];
    let mut nrg_est_pvc: *mut [FLOAT32; 48] = 0 as *mut [FLOAT32; 48];
    let mut nrg_ref_pvc: *mut [FLOAT32; 48] = 0 as *mut [FLOAT32; 48];
    let mut nrg_gain_pvc: *mut [FLOAT32; 48] = 0 as *mut [FLOAT32; 48];
    let mut nrg_tone_pvc: *mut [FLOAT32; 48] = 0 as *mut [FLOAT32; 48];
    let mut n: WORD32 = 0;
    let mut c: WORD32 = 0;
    let mut li: WORD32 = 0;
    let mut ui: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0 as WORD32;
    let mut l: WORD32 = 0;
    let mut m: WORD32 = 0 as WORD32;
    let mut kk: WORD32 = 0 as WORD32;
    let mut o: WORD32 = 0;
    let mut next: WORD32 = -(1 as WORD32);
    let mut ui2: WORD32 = 0;
    let mut flag: WORD32 = 0;
    let mut tmp: WORD32 = 0;
    let mut noise_absc_flag: WORD32 = 0;
    let mut smooth_length: WORD32 = 0;
    let mut upsamp_4_flag: WORD32 = (*(*frame_data).pstr_sbr_header).is_usf_4;
    let mut ptr_real_buf: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut ptr_imag_buf: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut nrg: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
    let mut p_ref: FLOAT32 = 0.;
    let mut p_est: FLOAT32 = 0.;
    let mut avg_gain: FLOAT32 = 0.;
    let mut g_max: FLOAT32 = 0.;
    let mut p_adj: FLOAT32 = 0.;
    let mut boost_gain: FLOAT32 = 0.;
    let mut sb_gain: FLOAT32 = 0.;
    let mut sb_noise: FLOAT32 = 0.;
    let mut temp: [FLOAT32; 64] = [0.; 64];
    let mut t: WORD32 = 0;
    let mut start_pos: WORD32 = 0 as WORD32;
    let mut end_pos: WORD32 = 0 as WORD32;
    let mut slot_idx: WORD32 = 0;
    let mut prev_env_noise_level: *mut FLOAT32 = ((*frame_data).prev_noise_level)
        .as_mut_ptr();
    let mut nrg_tone: *mut FLOAT32 = scratch_buff;
    let mut noise_level: *mut FLOAT32 = scratch_buff
        .offset(64 as core::ffi::c_int as isize);
    let mut nrg_est: *mut FLOAT32 = scratch_buff
        .offset(128 as core::ffi::c_int as isize);
    let mut nrg_ref: *mut FLOAT32 = scratch_buff
        .offset(192 as core::ffi::c_int as isize);
    let mut nrg_gain: *mut FLOAT32 = scratch_buff
        .offset(256 as core::ffi::c_int as isize);
    let mut smooth_filt: *const FLOAT32 = 0 as *const FLOAT32;
    let mut sfb_nrg: *mut FLOAT32 = ((*frame_data).flt_env_sf_arr).as_mut_ptr();
    let mut noise_floor: *mut FLOAT32 = ((*frame_data).flt_noise_floor).as_mut_ptr();
    let mut p_frame_info: *mut ia_frame_info_struct = &mut (*frame_data)
        .str_frame_info_details;
    let mut pvc_frame_info: *mut ia_frame_info_struct = &mut (*frame_data)
        .str_pvc_frame_info;
    let mut smoothing_length: WORD32 = if (*(*frame_data).pstr_sbr_header).smoothing_mode
        as core::ffi::c_int != 0
    {
        0 as WORD32
    } else {
        4 as WORD32
    };
    let mut int_mode: WORD32 = (*(*frame_data).pstr_sbr_header).interpol_freq as WORD32;
    let mut limiter_band: WORD32 = (*(*frame_data).pstr_sbr_header).limiter_bands
        as WORD32;
    let mut limiter_gains: WORD32 = (*(*frame_data).pstr_sbr_header).limiter_gains
        as WORD32;
    let mut add_harmonics: *mut WORD32 = ((*frame_data).add_harmonics).as_mut_ptr()
        as *mut WORD32;
    let mut sub_band_start: WORD32 = (*(*(*frame_data).pstr_sbr_header)
        .pstr_freq_band_data)
        .sub_band_start as WORD32;
    let mut sub_band_end: WORD32 = (*(*(*frame_data).pstr_sbr_header)
        .pstr_freq_band_data)
        .sub_band_end as WORD32;
    let mut reset: WORD32 = (*frame_data).reset_flag as WORD32;
    let mut num_subbands: WORD32 = sub_band_end - sub_band_start;
    let mut bs_num_env: WORD32 = (*p_frame_info).num_env as WORD32;
    let mut trans_env: WORD32 = (*p_frame_info).transient_env as WORD32;
    let mut sbr_mode: WORD32 = (*frame_data).sbr_mode as WORD32;
    let mut prev_sbr_mode: WORD32 = (*frame_data).prev_sbr_mode as WORD32;
    let mut freq_band_table: [*mut WORD16; 2] = [0 as *mut WORD16; 2];
    let mut num_sf_bands: *const WORD16 = ((*(*(*frame_data).pstr_sbr_header)
        .pstr_freq_band_data)
        .num_sf_bands)
        .as_mut_ptr();
    let mut freq_band_table_noise: *mut WORD16 = ((*(*(*frame_data).pstr_sbr_header)
        .pstr_freq_band_data)
        .freq_band_tbl_noise)
        .as_mut_ptr();
    let mut num_nf_bands: WORD32 = (*(*(*frame_data).pstr_sbr_header)
        .pstr_freq_band_data)
        .num_nf_bands as WORD32;
    let mut harm_index: WORD32 = (*frame_data).harm_index;
    let mut phase_index: WORD32 = (*frame_data).phase_index;
    let mut esbr_start_up: WORD32 = (*(*frame_data).pstr_sbr_header).esbr_start_up;
    let mut esbr_start_up_pvc: WORD32 = (*(*frame_data).pstr_sbr_header)
        .esbr_start_up_pvc;
    let mut harm_flag_prev: *mut [WORD8; 64] = &mut (*frame_data).harm_flag_prev;
    let mut e_gain: *mut [[FLOAT32; 64]; 5] = &mut (*frame_data).e_gain;
    let mut noise_buf: *mut [[FLOAT32; 64]; 5] = &mut (*frame_data).noise_buf;
    let mut lim_table: *mut [[WORD32; 13]; 4] = &mut (*frame_data).lim_table;
    let mut gate_mode: *mut [WORD32; 4] = &mut (*frame_data).gate_mode;
    let mut freq_inv: WORD32 = 1 as WORD32;
    let mut harm_flag_varlen_prev: *mut [WORD8; 64] = &mut (*frame_data)
        .harm_flag_varlen_prev;
    let mut harm_flag_varlen: *mut [WORD8; 64] = &mut (*frame_data).harm_flag_varlen;
    let mut band_loop_end: WORD32 = 0;
    let mut rate: WORD32 = if upsamp_4_flag != 0 { 4 as WORD32 } else { 2 as WORD32 };
    let mut guard: FLOAT64 = 1e-17f64;
    if ldmps_present == 1 as core::ffi::c_int {
        rate = 1 as core::ffi::c_int as WORD32;
    }
    env_tmp = ((*frame_data).env_tmp).as_mut_ptr() as *mut [FLOAT32; 48];
    noise_level_pvc = ((*frame_data).noise_level_pvc).as_mut_ptr() as *mut [FLOAT32; 48];
    nrg_est_pvc = ((*frame_data).nrg_est_pvc).as_mut_ptr() as *mut [FLOAT32; 48];
    nrg_ref_pvc = ((*frame_data).nrg_ref_pvc).as_mut_ptr() as *mut [FLOAT32; 48];
    nrg_gain_pvc = ((*frame_data).nrg_gain_pvc).as_mut_ptr() as *mut [FLOAT32; 48];
    nrg_tone_pvc = ((*frame_data).nrg_tone_pvc).as_mut_ptr() as *mut [FLOAT32; 48];
    freq_band_table[0 as core::ffi::c_int as usize] = (*(*(*frame_data).pstr_sbr_header)
        .pstr_freq_band_data)
        .freq_band_table[0 as core::ffi::c_int as usize];
    freq_band_table[1 as core::ffi::c_int as usize] = (*(*(*frame_data).pstr_sbr_header)
        .pstr_freq_band_data)
        .freq_band_table[1 as core::ffi::c_int as usize];
    if reset != 0 || ldmps_present == 1 as core::ffi::c_int {
        esbr_start_up = 1 as core::ffi::c_int as WORD32;
        esbr_start_up_pvc = 1 as core::ffi::c_int as WORD32;
        if reset != 0 {
            phase_index = 0 as core::ffi::c_int as WORD32;
        }
        if ixheaacd_createlimiterbands(
            (*lim_table).as_mut_ptr(),
            (*gate_mode).as_mut_ptr(),
            ((*(*(*frame_data).pstr_sbr_header).pstr_freq_band_data).freq_band_tbl_lo)
                .as_mut_ptr(),
            *num_sf_bands.offset(LOW as isize) as WORD32,
            x_over_qmf,
            (*frame_data).sbr_patching_mode,
            upsamp_4_flag,
            &mut (*frame_data).patch_param,
            ec_flag,
        ) != 0
        {
            return IA_FATAL_ERROR as WORD32;
        }
    }
    if (*frame_data).sbr_patching_mode != (*frame_data).prev_sbr_patching_mode {
        if ixheaacd_createlimiterbands(
            (*lim_table).as_mut_ptr(),
            (*gate_mode).as_mut_ptr(),
            ((*(*(*frame_data).pstr_sbr_header).pstr_freq_band_data).freq_band_tbl_lo)
                .as_mut_ptr(),
            *num_sf_bands.offset(LOW as isize) as WORD32,
            x_over_qmf,
            (*frame_data).sbr_patching_mode,
            upsamp_4_flag,
            &mut (*frame_data).patch_param,
            ec_flag,
        ) != 0
        {
            return IA_FATAL_ERROR as WORD32;
        }
        (*frame_data).prev_sbr_patching_mode = (*frame_data).sbr_patching_mode;
    }
    memset(
        harmonics.as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (64 as size_t).wrapping_mul(::core::mem::size_of::<WORD8>() as size_t),
    );
    if sbr_mode == PVC_SBR as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < *num_sf_bands.offset(HIGH as isize) as core::ffi::c_int {
            li = (*(*(*frame_data).pstr_sbr_header).pstr_freq_band_data)
                .freq_band_tbl_hi[i as usize] as WORD32;
            ui = (*(*(*frame_data).pstr_sbr_header).pstr_freq_band_data)
                .freq_band_tbl_hi[(i as core::ffi::c_int + 1 as core::ffi::c_int)
                as usize] as WORD32;
            tmp = ui + li - (sub_band_start << 1 as core::ffi::c_int)
                >> 1 as core::ffi::c_int;
            if tmp >= 64 as core::ffi::c_int || tmp < 0 as core::ffi::c_int {
                if ec_flag != 0 {
                    tmp = 0 as core::ffi::c_int as WORD32;
                } else {
                    return -(1 as WORD32)
                }
            }
            harmonics[tmp as usize] = *add_harmonics.offset(i as isize) as WORD8;
            i += 1;
        }
        t = 0 as core::ffi::c_int as WORD32;
        while t
            < (*p_frame_info).border_vec[0 as core::ffi::c_int as usize]
                as core::ffi::c_int
        {
            c = 0 as core::ffi::c_int as WORD32;
            while c < 64 as core::ffi::c_int {
                (*frame_data).qmapped_pvc[c as usize][t as usize] = (*frame_data)
                    .qmapped_pvc[c
                    as usize][(t as core::ffi::c_int + 16 as core::ffi::c_int) as usize];
                c += 1;
            }
            t += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < bs_num_env {
            if kk > MAX_NOISE_ENVELOPES {
                if ec_flag != 0 {
                    kk = MAX_NOISE_ENVELOPES as WORD32;
                } else {
                    return IA_FATAL_ERROR as WORD32
                }
            }
            if (*p_frame_info).border_vec[i as usize] as core::ffi::c_int
                == (*p_frame_info).noise_border_vec[kk as usize] as core::ffi::c_int
            {
                kk += 1;
                next += 1;
            }
            start_pos = (*p_frame_info).border_vec[i as usize] as WORD32;
            end_pos = (*p_frame_info)
                .border_vec[(i as core::ffi::c_int + 1 as core::ffi::c_int) as usize]
                as WORD32;
            if start_pos < 0 as core::ffi::c_int || end_pos > MAX_FREQ_COEFFS_SBR {
                if ec_flag != 0 {
                    start_pos = 0 as core::ffi::c_int as WORD32;
                    end_pos = MAX_FREQ_COEFFS_SBR as WORD32;
                } else {
                    return IA_FATAL_ERROR as WORD32
                }
            }
            t = start_pos;
            while t < end_pos {
                band_loop_end = *num_sf_bands
                    .offset((*p_frame_info).freq_res[i as usize] as isize) as WORD32;
                c = 0 as core::ffi::c_int as WORD32;
                o = 0 as core::ffi::c_int as WORD32;
                j = 0 as core::ffi::c_int as WORD32;
                while j < band_loop_end {
                    li = *(freq_band_table[(*p_frame_info).freq_res[i as usize]
                        as usize])
                        .offset(j as isize) as WORD32;
                    ui = *(freq_band_table[(*p_frame_info).freq_res[i as usize]
                        as usize])
                        .offset((j as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                        as WORD32;
                    ui2 = (*(*(*frame_data).pstr_sbr_header).pstr_freq_band_data)
                        .freq_band_tbl_noise[(o as core::ffi::c_int
                        + 1 as core::ffi::c_int) as usize] as WORD32;
                    k = 0 as core::ffi::c_int as WORD32;
                    while k < ui - li {
                        o = (if k + li >= ui2 {
                            o as core::ffi::c_int + 1 as core::ffi::c_int
                        } else {
                            o as core::ffi::c_int
                        }) as WORD32;
                        if o >= MAX_NOISE_COEFFS {
                            if ec_flag != 0 {
                                o = (MAX_NOISE_COEFFS - 1 as core::ffi::c_int) as WORD32;
                            } else {
                                return IA_FATAL_ERROR as WORD32
                            }
                        }
                        ui2 = *freq_band_table_noise
                            .offset(
                                (o as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                            ) as WORD32;
                        (*frame_data).qmapped_pvc[c as usize][t as usize] = *noise_floor
                            .offset((next * num_nf_bands + o) as isize);
                        c += 1;
                        k += 1;
                    }
                    j += 1;
                }
                t += 1;
            }
            i += 1;
        }
        kk = 0 as core::ffi::c_int as WORD32;
        next = -(1 as core::ffi::c_int) as WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < bs_num_env {
            if kk > MAX_NOISE_ENVELOPES {
                if ec_flag != 0 {
                    kk = MAX_NOISE_ENVELOPES as WORD32;
                } else {
                    return IA_FATAL_ERROR as WORD32
                }
            }
            if (*p_frame_info).border_vec[i as usize] as core::ffi::c_int
                == (*p_frame_info).noise_border_vec[kk as usize] as core::ffi::c_int
            {
                kk += 1;
                next += 1;
            }
            start_pos = (*pvc_frame_info).border_vec[i as usize] as WORD32;
            end_pos = (*pvc_frame_info)
                .border_vec[(i as core::ffi::c_int + 1 as core::ffi::c_int) as usize]
                as WORD32;
            if start_pos < 0 as core::ffi::c_int || end_pos > MAX_FREQ_COEFFS_SBR {
                if ec_flag != 0 {
                    start_pos = 0 as core::ffi::c_int as WORD32;
                    end_pos = MAX_FREQ_COEFFS_SBR as WORD32;
                } else {
                    return IA_FATAL_ERROR as WORD32
                }
            }
            t = start_pos;
            while t < end_pos {
                c = 0 as core::ffi::c_int as WORD32;
                while c < 64 as core::ffi::c_int {
                    (*env_tmp.offset(c as isize))[t as usize] = *env_out
                        .offset((64 as WORD32 * t + c) as isize);
                    c += 1;
                }
                t += 1;
            }
            noise_absc_flag = (if i == trans_env
                || i == (*frame_data).env_short_flag_prev
            {
                1 as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            }) as WORD32;
            if prev_sbr_mode == ORIG_SBR as core::ffi::c_int {
                noise_absc_flag = 0 as core::ffi::c_int as WORD32;
            }
            smooth_length = (if noise_absc_flag != 0 {
                0 as core::ffi::c_int
            } else {
                smoothing_length as core::ffi::c_int
            }) as WORD32;
            smooth_filt = (*ixheaac_fir_table[smooth_length as usize]).as_ptr();
            t = start_pos;
            while t < (*frame_data).sin_len_for_cur_top {
                band_loop_end = *num_sf_bands
                    .offset(
                        (*frame_data)
                            .str_frame_info_prev
                            .freq_res[(*frame_data).var_len_id_prev as usize] as isize,
                    ) as WORD32;
                c = 0 as core::ffi::c_int as WORD32;
                o = 0 as core::ffi::c_int as WORD32;
                j = 0 as core::ffi::c_int as WORD32;
                while j < band_loop_end {
                    let mut tmp_0: core::ffi::c_double = 0.;
                    li = *(freq_band_table[(*frame_data)
                        .str_frame_info_prev
                        .freq_res[(*frame_data).var_len_id_prev as usize] as usize])
                        .offset(j as isize) as WORD32;
                    ui = *(freq_band_table[(*frame_data)
                        .str_frame_info_prev
                        .freq_res[(*frame_data).var_len_id_prev as usize] as usize])
                        .offset((j as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                        as WORD32;
                    ui2 = (*(*(*frame_data).pstr_sbr_header).pstr_freq_band_data)
                        .freq_band_tbl_noise[(o as core::ffi::c_int
                        + 1 as core::ffi::c_int) as usize] as WORD32;
                    flag = 0 as core::ffi::c_int as WORD32;
                    k = li;
                    while k < ui {
                        flag = (if (*harm_flag_varlen)[c as usize] as core::ffi::c_int
                            != 0
                            && (t >= (*frame_data).sin_start_for_cur_top
                                || (*harm_flag_varlen_prev)[(c + sub_band_start) as usize]
                                    as core::ffi::c_int != 0)
                        {
                            1 as core::ffi::c_int
                        } else {
                            flag as core::ffi::c_int
                        }) as WORD32;
                        (*nrg_ref_pvc.offset(c as isize))[t as usize] = (*env_tmp
                            .offset(k as isize))[t as usize];
                        nrg = 0 as core::ffi::c_int as FLOAT32;
                        l = 0 as core::ffi::c_int as WORD32;
                        while l < rate {
                            nrg
                                += (*input_real.offset((rate * t + l) as isize))[k as usize]
                                    * (*input_real.offset((rate * t + l) as isize))[k as usize]
                                    + (*input_imag.offset((rate * t + l) as isize))[k as usize]
                                        * (*input_imag.offset((rate * t + l) as isize))[k as usize];
                            l += 1;
                        }
                        (*nrg_est_pvc.offset(c as isize))[t as usize] = nrg
                            / rate as FLOAT32;
                        c += 1;
                        k += 1;
                    }
                    if int_mode == 0 && ui != li {
                        nrg = 0 as core::ffi::c_int as FLOAT32;
                        k = c - (ui - li);
                        while k < c {
                            nrg += (*nrg_est_pvc.offset(k as isize))[t as usize];
                            k += 1;
                        }
                        nrg /= (ui - li) as FLOAT32;
                    } else {
                        nrg = 0 as core::ffi::c_int as FLOAT32;
                    }
                    c -= ui - li;
                    k = 0 as core::ffi::c_int as WORD32;
                    while k < ui - li {
                        o = (if k + li >= ui2 {
                            o as core::ffi::c_int + 1 as core::ffi::c_int
                        } else {
                            o as core::ffi::c_int
                        }) as WORD32;
                        if o >= MAX_NOISE_COEFFS {
                            if ec_flag != 0 {
                                o = (MAX_NOISE_COEFFS - 1 as core::ffi::c_int) as WORD32;
                            } else {
                                return IA_FATAL_ERROR as WORD32
                            }
                        }
                        ui2 = *freq_band_table_noise
                            .offset(
                                (o as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                            ) as WORD32;
                        (*nrg_est_pvc.offset(c as isize))[t as usize] = if int_mode == 0
                        {
                            nrg
                        } else {
                            (*nrg_est_pvc.offset(c as isize))[t as usize]
                        };
                        (*nrg_tone_pvc.offset(c as isize))[t as usize] = 0.0f32
                            as FLOAT32;
                        tmp_0 = ((*frame_data).qmapped_pvc[c as usize][t as usize]
                            as FLOAT64
                            / ((1 as core::ffi::c_int as FLOAT32
                                + (*frame_data).qmapped_pvc[c as usize][t as usize])
                                as FLOAT64 + guard)) as core::ffi::c_double;
                        if flag != 0 {
                            (*nrg_gain_pvc.offset(c as isize))[t as usize] = sqrt(
                                (*nrg_ref_pvc.offset(c as isize))[t as usize]
                                    as core::ffi::c_double * tmp_0
                                    / ((*nrg_est_pvc.offset(c as isize))[t as usize]
                                        + 1 as core::ffi::c_int as FLOAT32) as core::ffi::c_double,
                            ) as FLOAT32;
                            (*nrg_tone_pvc.offset(c as isize))[t as usize] = (if harmonics[c
                                as usize] as core::ffi::c_int != 0
                                && (t >= (*frame_data).sine_position
                                    || (*harm_flag_prev)[(c + sub_band_start) as usize]
                                        as core::ffi::c_int != 0)
                            {
                                sqrt(
                                    (*nrg_ref_pvc.offset(c as isize))[t as usize]
                                        as core::ffi::c_double * tmp_0
                                        / ((*frame_data).qmapped_pvc[c as usize][t as usize]
                                            as core::ffi::c_double + guard as core::ffi::c_double),
                                )
                            } else {
                                (*nrg_tone_pvc.offset(c as isize))[t as usize]
                                    as core::ffi::c_double
                            }) as FLOAT32;
                            (*nrg_tone_pvc.offset(c as isize))[t as usize] = (if (*harm_flag_varlen)[c
                                as usize] as core::ffi::c_int != 0
                                && (t >= (*frame_data).sin_start_for_cur_top
                                    || (*harm_flag_varlen_prev)[(c + sub_band_start) as usize]
                                        as core::ffi::c_int != 0)
                            {
                                sqrt(
                                    (*nrg_ref_pvc.offset(c as isize))[t as usize]
                                        as core::ffi::c_double * tmp_0
                                        / (*prev_env_noise_level.offset(o as isize)
                                            as core::ffi::c_double + guard as core::ffi::c_double),
                                )
                            } else {
                                (*nrg_tone_pvc.offset(c as isize))[t as usize]
                                    as core::ffi::c_double
                            }) as FLOAT32;
                        } else if noise_absc_flag != 0 {
                            (*nrg_gain_pvc.offset(c as isize))[t as usize] = sqrt(
                                ((*nrg_ref_pvc.offset(c as isize))[t as usize]
                                    / ((*nrg_est_pvc.offset(c as isize))[t as usize]
                                        + 1 as core::ffi::c_int as FLOAT32)) as core::ffi::c_double,
                            ) as FLOAT32;
                        } else {
                            (*nrg_gain_pvc.offset(c as isize))[t as usize] = sqrt(
                                (*nrg_ref_pvc.offset(c as isize))[t as usize]
                                    as core::ffi::c_double * tmp_0
                                    / (((*nrg_est_pvc.offset(c as isize))[t as usize]
                                        + 1 as core::ffi::c_int as FLOAT32) as core::ffi::c_double
                                        * ((*frame_data).qmapped_pvc[c as usize][t as usize]
                                            as core::ffi::c_double + guard as core::ffi::c_double)),
                            ) as FLOAT32;
                        }
                        (*noise_level_pvc.offset(c as isize))[t as usize] = sqrt(
                            (*nrg_ref_pvc.offset(c as isize))[t as usize]
                                as core::ffi::c_double * tmp_0,
                        ) as FLOAT32;
                        c += 1;
                        k += 1;
                    }
                    j += 1;
                }
                c = 0 as core::ffi::c_int as WORD32;
                while c < (*gate_mode)[limiter_band as usize] {
                    p_est = 0.0f32 as FLOAT32;
                    p_ref = p_est;
                    p_adj = 0 as core::ffi::c_int as FLOAT32;
                    k = (*lim_table)[limiter_band as usize][c as usize];
                    while k
                        < (*lim_table)[limiter_band
                            as usize][(c as core::ffi::c_int + 1 as core::ffi::c_int)
                            as usize]
                    {
                        p_ref += (*nrg_ref_pvc.offset(k as isize))[t as usize];
                        p_est += (*nrg_est_pvc.offset(k as isize))[t as usize];
                        k += 1;
                    }
                    avg_gain = sqrt(
                        ((p_ref as core::ffi::c_float + EPS)
                            / (p_est as core::ffi::c_float + EPS)) as core::ffi::c_double,
                    ) as FLOAT32;
                    g_max = avg_gain * ixheaac_g_lim_gains[limiter_gains as usize];
                    if g_max > 1.0e5f32 {
                        g_max = 1.0e5f32 as FLOAT32;
                    } else {};
                    k = (*lim_table)[limiter_band as usize][c as usize];
                    while k
                        < (*lim_table)[limiter_band
                            as usize][(c as core::ffi::c_int + 1 as core::ffi::c_int)
                            as usize]
                    {
                        if g_max <= (*nrg_gain_pvc.offset(k as isize))[t as usize] {
                            (*noise_level_pvc.offset(k as isize))[t as usize] = ((*noise_level_pvc
                                .offset(k as isize))[t as usize] as FLOAT64
                                * (g_max as FLOAT64
                                    / ((*nrg_gain_pvc.offset(k as isize))[t as usize] as FLOAT64
                                        + guard))) as FLOAT32;
                            (*nrg_gain_pvc.offset(k as isize))[t as usize] = g_max;
                        }
                        p_adj
                            += (*nrg_gain_pvc.offset(k as isize))[t as usize]
                                * (*nrg_gain_pvc.offset(k as isize))[t as usize]
                                * (*nrg_est_pvc.offset(k as isize))[t as usize];
                        if (*nrg_tone_pvc.offset(k as isize))[t as usize] != 0. {
                            p_adj
                                += (*nrg_tone_pvc.offset(k as isize))[t as usize]
                                    * (*nrg_tone_pvc.offset(k as isize))[t as usize];
                        } else if noise_absc_flag == 0 {
                            p_adj
                                += (*noise_level_pvc.offset(k as isize))[t as usize]
                                    * (*noise_level_pvc.offset(k as isize))[t as usize];
                        }
                        k += 1;
                    }
                    boost_gain = sqrt(
                        ((p_ref as core::ffi::c_float + EPS)
                            / (p_adj as core::ffi::c_float + EPS)) as core::ffi::c_double,
                    ) as FLOAT32;
                    boost_gain = (if boost_gain > 1.584893192f32 {
                        1.584893192f32
                    } else {
                        boost_gain as core::ffi::c_float
                    }) as FLOAT32;
                    k = (*lim_table)[limiter_band as usize][c as usize];
                    while k
                        < (*lim_table)[limiter_band
                            as usize][(c as core::ffi::c_int + 1 as core::ffi::c_int)
                            as usize]
                    {
                        (*nrg_gain_pvc.offset(k as isize))[t as usize] *= boost_gain;
                        (*noise_level_pvc.offset(k as isize))[t as usize] *= boost_gain;
                        (*nrg_tone_pvc.offset(k as isize))[t as usize] *= boost_gain;
                        k += 1;
                    }
                    c += 1;
                }
                t += 1;
            }
            while t < end_pos {
                band_loop_end = *num_sf_bands
                    .offset((*pvc_frame_info).freq_res[i as usize] as isize) as WORD32;
                c = 0 as core::ffi::c_int as WORD32;
                o = 0 as core::ffi::c_int as WORD32;
                j = 0 as core::ffi::c_int as WORD32;
                while j < band_loop_end {
                    let mut tmp_1: core::ffi::c_double = 0.;
                    li = *(freq_band_table[(*pvc_frame_info).freq_res[i as usize]
                        as usize])
                        .offset(j as isize) as WORD32;
                    ui = *(freq_band_table[(*pvc_frame_info).freq_res[i as usize]
                        as usize])
                        .offset((j as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                        as WORD32;
                    ui2 = (*(*(*frame_data).pstr_sbr_header).pstr_freq_band_data)
                        .freq_band_tbl_noise[(o as core::ffi::c_int
                        + 1 as core::ffi::c_int) as usize] as WORD32;
                    flag = 0 as core::ffi::c_int as WORD32;
                    k = li;
                    while k < ui {
                        flag = (if harmonics[c as usize] as core::ffi::c_int != 0
                            && (t >= (*frame_data).sine_position
                                || (*harm_flag_prev)[(c + sub_band_start) as usize]
                                    as core::ffi::c_int != 0)
                        {
                            1 as core::ffi::c_int
                        } else {
                            flag as core::ffi::c_int
                        }) as WORD32;
                        (*nrg_ref_pvc.offset(c as isize))[t as usize] = (*env_tmp
                            .offset(k as isize))[t as usize];
                        nrg = 0 as core::ffi::c_int as FLOAT32;
                        l = 0 as core::ffi::c_int as WORD32;
                        while l < rate {
                            nrg
                                += (*input_real.offset((rate * t + l) as isize))[k as usize]
                                    * (*input_real.offset((rate * t + l) as isize))[k as usize]
                                    + (*input_imag.offset((rate * t + l) as isize))[k as usize]
                                        * (*input_imag.offset((rate * t + l) as isize))[k as usize];
                            l += 1;
                        }
                        (*nrg_est_pvc.offset(c as isize))[t as usize] = nrg
                            / rate as FLOAT32;
                        c += 1;
                        k += 1;
                    }
                    if int_mode == 0 && ui != li {
                        nrg = 0 as core::ffi::c_int as FLOAT32;
                        k = c - (ui - li);
                        while k < c {
                            nrg += (*nrg_est_pvc.offset(k as isize))[t as usize];
                            k += 1;
                        }
                        nrg /= (ui - li) as FLOAT32;
                    } else {
                        nrg = 0 as core::ffi::c_int as FLOAT32;
                    }
                    c -= ui - li;
                    k = 0 as core::ffi::c_int as WORD32;
                    while k < ui - li {
                        o = (if k + li >= ui2 {
                            o as core::ffi::c_int + 1 as core::ffi::c_int
                        } else {
                            o as core::ffi::c_int
                        }) as WORD32;
                        if o >= MAX_NOISE_COEFFS {
                            if ec_flag != 0 {
                                o = (MAX_NOISE_COEFFS - 1 as core::ffi::c_int) as WORD32;
                            } else {
                                return IA_FATAL_ERROR as WORD32
                            }
                        }
                        ui2 = *freq_band_table_noise
                            .offset(
                                (o as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                            ) as WORD32;
                        (*nrg_est_pvc.offset(c as isize))[t as usize] = if int_mode == 0
                        {
                            nrg
                        } else {
                            (*nrg_est_pvc.offset(c as isize))[t as usize]
                        };
                        (*nrg_tone_pvc.offset(c as isize))[t as usize] = 0.0f32
                            as FLOAT32;
                        tmp_1 = ((*frame_data).qmapped_pvc[c as usize][t as usize]
                            as FLOAT64
                            / ((1 as core::ffi::c_int as FLOAT32
                                + (*frame_data).qmapped_pvc[c as usize][t as usize])
                                as FLOAT64 + guard)) as core::ffi::c_double;
                        if flag != 0 {
                            (*nrg_gain_pvc.offset(c as isize))[t as usize] = sqrt(
                                (*nrg_ref_pvc.offset(c as isize))[t as usize]
                                    as core::ffi::c_double * tmp_1
                                    / ((*nrg_est_pvc.offset(c as isize))[t as usize]
                                        + 1 as core::ffi::c_int as FLOAT32) as core::ffi::c_double,
                            ) as FLOAT32;
                            (*nrg_tone_pvc.offset(c as isize))[t as usize] = (if harmonics[c
                                as usize] as core::ffi::c_int != 0
                                && (t >= (*frame_data).sine_position
                                    || (*harm_flag_prev)[(c + sub_band_start) as usize]
                                        as core::ffi::c_int != 0)
                            {
                                sqrt(
                                    (*nrg_ref_pvc.offset(c as isize))[t as usize]
                                        as core::ffi::c_double * tmp_1
                                        / ((*frame_data).qmapped_pvc[c as usize][t as usize]
                                            as core::ffi::c_double + guard as core::ffi::c_double),
                                )
                            } else {
                                (*nrg_tone_pvc.offset(c as isize))[t as usize]
                                    as core::ffi::c_double
                            }) as FLOAT32;
                        } else if noise_absc_flag != 0 {
                            (*nrg_gain_pvc.offset(c as isize))[t as usize] = sqrt(
                                ((*nrg_ref_pvc.offset(c as isize))[t as usize]
                                    / ((*nrg_est_pvc.offset(c as isize))[t as usize]
                                        + 1 as core::ffi::c_int as FLOAT32)) as core::ffi::c_double,
                            ) as FLOAT32;
                        } else {
                            (*nrg_gain_pvc.offset(c as isize))[t as usize] = sqrt(
                                (*nrg_ref_pvc.offset(c as isize))[t as usize]
                                    as core::ffi::c_double * tmp_1
                                    / (((*nrg_est_pvc.offset(c as isize))[t as usize]
                                        + 1 as core::ffi::c_int as FLOAT32) as core::ffi::c_double
                                        * ((*frame_data).qmapped_pvc[c as usize][t as usize]
                                            as core::ffi::c_double + guard as core::ffi::c_double)),
                            ) as FLOAT32;
                        }
                        (*noise_level_pvc.offset(c as isize))[t as usize] = sqrt(
                            (*nrg_ref_pvc.offset(c as isize))[t as usize]
                                as core::ffi::c_double * tmp_1,
                        ) as FLOAT32;
                        c += 1;
                        k += 1;
                    }
                    j += 1;
                }
                c = 0 as core::ffi::c_int as WORD32;
                while c < (*gate_mode)[limiter_band as usize] {
                    p_est = 0.0f32 as FLOAT32;
                    p_ref = p_est;
                    p_adj = 0 as core::ffi::c_int as FLOAT32;
                    k = (*lim_table)[limiter_band as usize][c as usize];
                    while k
                        < (*lim_table)[limiter_band
                            as usize][(c as core::ffi::c_int + 1 as core::ffi::c_int)
                            as usize]
                    {
                        p_ref += (*nrg_ref_pvc.offset(k as isize))[t as usize];
                        p_est += (*nrg_est_pvc.offset(k as isize))[t as usize];
                        k += 1;
                    }
                    avg_gain = sqrt(
                        ((p_ref as core::ffi::c_float + EPS)
                            / (p_est as core::ffi::c_float + EPS)) as core::ffi::c_double,
                    ) as FLOAT32;
                    g_max = avg_gain * ixheaac_g_lim_gains[limiter_gains as usize];
                    if g_max > 1.0e5f32 {
                        g_max = 1.0e5f32 as FLOAT32;
                    } else {};
                    k = (*lim_table)[limiter_band as usize][c as usize];
                    while k
                        < (*lim_table)[limiter_band
                            as usize][(c as core::ffi::c_int + 1 as core::ffi::c_int)
                            as usize]
                    {
                        if g_max <= (*nrg_gain_pvc.offset(k as isize))[t as usize] {
                            (*noise_level_pvc.offset(k as isize))[t as usize] = ((*noise_level_pvc
                                .offset(k as isize))[t as usize] as FLOAT64
                                * (g_max as FLOAT64
                                    / ((*nrg_gain_pvc.offset(k as isize))[t as usize] as FLOAT64
                                        + guard))) as FLOAT32;
                            (*nrg_gain_pvc.offset(k as isize))[t as usize] = g_max;
                        }
                        p_adj
                            += (*nrg_gain_pvc.offset(k as isize))[t as usize]
                                * (*nrg_gain_pvc.offset(k as isize))[t as usize]
                                * (*nrg_est_pvc.offset(k as isize))[t as usize];
                        if (*nrg_tone_pvc.offset(k as isize))[t as usize] != 0. {
                            p_adj
                                += (*nrg_tone_pvc.offset(k as isize))[t as usize]
                                    * (*nrg_tone_pvc.offset(k as isize))[t as usize];
                        } else if noise_absc_flag == 0 {
                            p_adj
                                += (*noise_level_pvc.offset(k as isize))[t as usize]
                                    * (*noise_level_pvc.offset(k as isize))[t as usize];
                        }
                        k += 1;
                    }
                    boost_gain = sqrt(
                        ((p_ref as core::ffi::c_float + EPS)
                            / (p_adj as core::ffi::c_float + EPS)) as core::ffi::c_double,
                    ) as FLOAT32;
                    boost_gain = (if boost_gain > 1.584893192f32 {
                        1.584893192f32
                    } else {
                        boost_gain as core::ffi::c_float
                    }) as FLOAT32;
                    k = (*lim_table)[limiter_band as usize][c as usize];
                    while k
                        < (*lim_table)[limiter_band
                            as usize][(c as core::ffi::c_int + 1 as core::ffi::c_int)
                            as usize]
                    {
                        (*nrg_gain_pvc.offset(k as isize))[t as usize] *= boost_gain;
                        (*noise_level_pvc.offset(k as isize))[t as usize] *= boost_gain;
                        (*nrg_tone_pvc.offset(k as isize))[t as usize] *= boost_gain;
                        k += 1;
                    }
                    c += 1;
                }
                t += 1;
            }
            if esbr_start_up_pvc != 0 {
                n = 0 as core::ffi::c_int as WORD32;
                while n < 4 as core::ffi::c_int {
                    c = 0 as core::ffi::c_int as WORD32;
                    while c < num_subbands {
                        (*e_gain)[n as usize][c as usize] = (*nrg_gain_pvc
                            .offset(c as isize))[start_pos as usize];
                        (*noise_buf)[n as usize][c as usize] = (*noise_level_pvc
                            .offset(c as isize))[start_pos as usize];
                        c += 1;
                    }
                    n += 1;
                }
                esbr_start_up_pvc = 0 as core::ffi::c_int as WORD32;
                esbr_start_up = 0 as core::ffi::c_int as WORD32;
            }
            l = (rate as core::ffi::c_int
                * (*pvc_frame_info).border_vec[i as usize] as core::ffi::c_int)
                as WORD32;
            while l
                < rate as core::ffi::c_int
                    * (*pvc_frame_info).border_vec[(1 as WORD32 + i) as usize]
                        as core::ffi::c_int
            {
                ptr_real_buf = (*input_real.offset(l as isize))
                    .as_mut_ptr()
                    .offset(sub_band_start as isize);
                ptr_imag_buf = (*input_imag.offset(l as isize))
                    .as_mut_ptr()
                    .offset(sub_band_start as isize);
                slot_idx = l / rate;
                if sub_band_start as core::ffi::c_int & 1 as core::ffi::c_int != 0 {
                    freq_inv = -(1 as core::ffi::c_int) as WORD32;
                } else {
                    freq_inv = 1 as core::ffi::c_int as WORD32;
                }
                k = 0 as core::ffi::c_int as WORD32;
                while k < num_subbands {
                    (*e_gain)[4 as core::ffi::c_int as usize][k as usize] = (*nrg_gain_pvc
                        .offset(k as isize))[slot_idx as usize];
                    (*noise_buf)[4 as core::ffi::c_int as usize][k as usize] = (*noise_level_pvc
                        .offset(k as isize))[slot_idx as usize];
                    c = 0 as core::ffi::c_int as WORD32;
                    sb_gain = 0 as core::ffi::c_int as FLOAT32;
                    sb_noise = 0 as core::ffi::c_int as FLOAT32;
                    n = 4 as WORD32 - smooth_length;
                    while n <= 4 as core::ffi::c_int {
                        sb_gain
                            += (*e_gain)[n as usize][k as usize]
                                * *smooth_filt.offset(c as isize);
                        let fresh3 = c;
                        c = c + 1;
                        sb_noise
                            += (*noise_buf)[n as usize][k as usize]
                                * *smooth_filt.offset(fresh3 as isize);
                        n += 1;
                    }
                    phase_index = (phase_index as core::ffi::c_int
                        + 1 as core::ffi::c_int & 511 as core::ffi::c_int) as WORD32;
                    sb_noise = if (*nrg_tone_pvc.offset(k as isize))[slot_idx as usize]
                        != 0 as core::ffi::c_int as FLOAT32 || noise_absc_flag != 0
                    {
                        0 as core::ffi::c_int as FLOAT32
                    } else {
                        sb_noise
                    };
                    *ptr_real_buf = *ptr_real_buf * sb_gain
                        + sb_noise
                            * ixheaac_random_phase[phase_index
                                as usize][0 as core::ffi::c_int as usize]
                        + (*nrg_tone_pvc.offset(k as isize))[slot_idx as usize]
                            * ixheaac_hphase_tbl[0 as core::ffi::c_int
                                as usize][harm_index as usize];
                    *ptr_imag_buf = *ptr_imag_buf * sb_gain
                        + sb_noise
                            * ixheaac_random_phase[phase_index
                                as usize][1 as core::ffi::c_int as usize]
                        + (*nrg_tone_pvc.offset(k as isize))[slot_idx as usize]
                            * freq_inv as FLOAT32
                            * ixheaac_hphase_tbl[1 as core::ffi::c_int
                                as usize][harm_index as usize];
                    ptr_real_buf = ptr_real_buf.offset(1);
                    ptr_imag_buf = ptr_imag_buf.offset(1);
                    freq_inv = -freq_inv;
                    k += 1;
                }
                harm_index = (harm_index as core::ffi::c_int + 1 as core::ffi::c_int
                    & 3 as core::ffi::c_int) as WORD32;
                memcpy(
                    temp.as_mut_ptr() as *mut core::ffi::c_void,
                    ((*e_gain)[0 as core::ffi::c_int as usize]).as_mut_ptr()
                        as *const core::ffi::c_void,
                    (64 as size_t)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                );
                n = 0 as core::ffi::c_int as WORD32;
                while n < 4 as core::ffi::c_int {
                    memcpy(
                        ((*e_gain)[n as usize]).as_mut_ptr() as *mut core::ffi::c_void,
                        ((*e_gain)[(n as core::ffi::c_int + 1 as core::ffi::c_int)
                            as usize])
                            .as_mut_ptr() as *const core::ffi::c_void,
                        (64 as size_t)
                            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                    );
                    n += 1;
                }
                memcpy(
                    ((*e_gain)[4 as core::ffi::c_int as usize]).as_mut_ptr()
                        as *mut core::ffi::c_void,
                    temp.as_mut_ptr() as *const core::ffi::c_void,
                    (64 as size_t)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                );
                memcpy(
                    temp.as_mut_ptr() as *mut core::ffi::c_void,
                    ((*noise_buf)[0 as core::ffi::c_int as usize]).as_mut_ptr()
                        as *const core::ffi::c_void,
                    (64 as size_t)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                );
                n = 0 as core::ffi::c_int as WORD32;
                while n < 4 as core::ffi::c_int {
                    memcpy(
                        ((*noise_buf)[n as usize]).as_mut_ptr()
                            as *mut core::ffi::c_void,
                        ((*noise_buf)[(n as core::ffi::c_int + 1 as core::ffi::c_int)
                            as usize])
                            .as_mut_ptr() as *const core::ffi::c_void,
                        (64 as size_t)
                            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                    );
                    n += 1;
                }
                memcpy(
                    ((*noise_buf)[4 as core::ffi::c_int as usize]).as_mut_ptr()
                        as *mut core::ffi::c_void,
                    temp.as_mut_ptr() as *const core::ffi::c_void,
                    (64 as size_t)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                );
                l += 1;
            }
            i += 1;
        }
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < *num_sf_bands.offset(HIGH as isize) as core::ffi::c_int {
            li = (*(*(*frame_data).pstr_sbr_header).pstr_freq_band_data)
                .freq_band_tbl_hi[i as usize] as WORD32;
            ui = (*(*(*frame_data).pstr_sbr_header).pstr_freq_band_data)
                .freq_band_tbl_hi[(i as core::ffi::c_int + 1 as core::ffi::c_int)
                as usize] as WORD32;
            tmp = ui + li - (sub_band_start << 1 as core::ffi::c_int)
                >> 1 as core::ffi::c_int;
            if tmp >= 64 as core::ffi::c_int || tmp < 0 as core::ffi::c_int {
                return -(1 as WORD32);
            }
            harmonics[tmp as usize] = *add_harmonics.offset(i as isize) as WORD8;
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < bs_num_env {
            if kk > MAX_NOISE_ENVELOPES {
                if ec_flag != 0 {
                    kk = MAX_NOISE_ENVELOPES as WORD32;
                } else {
                    return IA_FATAL_ERROR as WORD32
                }
            }
            if (*p_frame_info).border_vec[i as usize] as core::ffi::c_int
                == (*p_frame_info).noise_border_vec[kk as usize] as core::ffi::c_int
            {
                kk += 1;
                next += 1;
            }
            noise_absc_flag = (if i == trans_env
                || i == (*frame_data).env_short_flag_prev
            {
                1 as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            }) as WORD32;
            smooth_length = (if noise_absc_flag != 0 {
                0 as core::ffi::c_int
            } else {
                smoothing_length as core::ffi::c_int
            }) as WORD32;
            smooth_filt = (*ixheaac_fir_table[smooth_length as usize]).as_ptr();
            if sbr_mode == ORIG_SBR as core::ffi::c_int {
                c = 0 as core::ffi::c_int as WORD32;
                o = 0 as core::ffi::c_int as WORD32;
                j = 0 as core::ffi::c_int as WORD32;
                while j
                    < *num_sf_bands.offset((*p_frame_info).freq_res[i as usize] as isize)
                        as core::ffi::c_int
                {
                    let mut tmp_2: core::ffi::c_double = 0.;
                    li = *(freq_band_table[(*p_frame_info).freq_res[i as usize]
                        as usize])
                        .offset(j as isize) as WORD32;
                    ui = *(freq_band_table[(*p_frame_info).freq_res[i as usize]
                        as usize])
                        .offset((j as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                        as WORD32;
                    ui2 = (*(*(*frame_data).pstr_sbr_header).pstr_freq_band_data)
                        .freq_band_tbl_noise[(o as core::ffi::c_int
                        + 1 as core::ffi::c_int) as usize] as WORD32;
                    if (*p_frame_info).border_vec[i as usize] as core::ffi::c_int
                        >= (*p_frame_info)
                            .border_vec[(i as core::ffi::c_int + 1 as core::ffi::c_int)
                            as usize] as core::ffi::c_int
                    {
                        flag = 0 as core::ffi::c_int as WORD32;
                        k = li;
                        while k < ui {
                            flag = (if harmonics[c as usize] as core::ffi::c_int != 0
                                && (i >= trans_env
                                    || (*harm_flag_prev)[(c + sub_band_start) as usize]
                                        as core::ffi::c_int != 0)
                            {
                                1 as core::ffi::c_int
                            } else {
                                flag as core::ffi::c_int
                            }) as WORD32;
                            let fresh4 = c;
                            c = c + 1;
                            *nrg_est.offset(fresh4 as isize) = 0 as core::ffi::c_int
                                as FLOAT32;
                            k += 1;
                        }
                    } else {
                        flag = 0 as core::ffi::c_int as WORD32;
                        k = li;
                        while k < ui {
                            nrg = 0 as core::ffi::c_int as FLOAT32;
                            l = (rate as core::ffi::c_int
                                * (*p_frame_info).border_vec[i as usize]
                                    as core::ffi::c_int) as WORD32;
                            while l
                                < rate as core::ffi::c_int
                                    * (*p_frame_info)
                                        .border_vec[(i as core::ffi::c_int + 1 as core::ffi::c_int)
                                        as usize] as core::ffi::c_int
                            {
                                nrg
                                    += (*input_real.offset(l as isize))[k as usize]
                                        * (*input_real.offset(l as isize))[k as usize]
                                        + (*input_imag.offset(l as isize))[k as usize]
                                            * (*input_imag.offset(l as isize))[k as usize];
                                l += 1;
                            }
                            flag = (if harmonics[c as usize] as core::ffi::c_int != 0
                                && (i >= trans_env
                                    || (*harm_flag_prev)[(c + sub_band_start) as usize]
                                        as core::ffi::c_int != 0)
                            {
                                1 as core::ffi::c_int
                            } else {
                                flag as core::ffi::c_int
                            }) as WORD32;
                            let fresh5 = c;
                            c = c + 1;
                            *nrg_est.offset(fresh5 as isize) = nrg
                                / (rate as core::ffi::c_int
                                    * (*p_frame_info)
                                        .border_vec[(i as core::ffi::c_int + 1 as core::ffi::c_int)
                                        as usize] as core::ffi::c_int
                                    - rate as core::ffi::c_int
                                        * (*p_frame_info).border_vec[i as usize]
                                            as core::ffi::c_int) as FLOAT32;
                            k += 1;
                        }
                    }
                    if int_mode == 0 && ui != li {
                        nrg = 0 as core::ffi::c_int as FLOAT32;
                        k = c - (ui - li);
                        while k < c {
                            nrg += *nrg_est.offset(k as isize);
                            k += 1;
                        }
                        nrg /= (ui - li) as FLOAT32;
                    } else {
                        nrg = 0 as core::ffi::c_int as FLOAT32;
                    }
                    c -= ui - li;
                    k = 0 as core::ffi::c_int as WORD32;
                    while k < ui - li {
                        o = (if k + li >= ui2 {
                            o as core::ffi::c_int + 1 as core::ffi::c_int
                        } else {
                            o as core::ffi::c_int
                        }) as WORD32;
                        if o >= MAX_NOISE_COEFFS {
                            if ec_flag != 0 {
                                o = (MAX_NOISE_COEFFS - 1 as core::ffi::c_int) as WORD32;
                            } else {
                                return IA_FATAL_ERROR as WORD32
                            }
                        }
                        ui2 = (*(*(*frame_data).pstr_sbr_header).pstr_freq_band_data)
                            .freq_band_tbl_noise[(o as core::ffi::c_int
                            + 1 as core::ffi::c_int) as usize] as WORD32;
                        *nrg_ref.offset(c as isize) = *sfb_nrg.offset(m as isize);
                        *nrg_est.offset(c as isize) = if int_mode == 0 {
                            nrg
                        } else {
                            *nrg_est.offset(c as isize)
                        };
                        *nrg_tone.offset(c as isize) = 0 as core::ffi::c_int as FLOAT32;
                        tmp_2 = (*noise_floor.offset((next * num_nf_bands + o) as isize)
                            as FLOAT64
                            / ((1 as core::ffi::c_int as FLOAT32
                                + *noise_floor.offset((next * num_nf_bands + o) as isize))
                                as FLOAT64 + guard)) as core::ffi::c_double;
                        if flag != 0 {
                            *nrg_gain.offset(c as isize) = sqrt(
                                *nrg_ref.offset(c as isize) as core::ffi::c_double * tmp_2
                                    / (*nrg_est.offset(c as isize)
                                        + 1 as core::ffi::c_int as FLOAT32) as core::ffi::c_double,
                            ) as FLOAT32;
                            *nrg_tone.offset(c as isize) = (if harmonics[c as usize]
                                as core::ffi::c_int != 0
                                && (i >= trans_env
                                    || (*harm_flag_prev)[(c + sub_band_start) as usize]
                                        as core::ffi::c_int != 0)
                            {
                                sqrt(
                                    *nrg_ref.offset(c as isize) as core::ffi::c_double * tmp_2
                                        / fabs(
                                            *noise_floor.offset((next * num_nf_bands + o) as isize)
                                                as core::ffi::c_double + guard as core::ffi::c_double,
                                        ),
                                )
                            } else {
                                *nrg_tone.offset(c as isize) as core::ffi::c_double
                            }) as FLOAT32;
                        } else if noise_absc_flag != 0 {
                            *nrg_gain.offset(c as isize) = sqrt(
                                (*nrg_ref.offset(c as isize)
                                    / (*nrg_est.offset(c as isize)
                                        + 1 as core::ffi::c_int as FLOAT32)) as core::ffi::c_double,
                            ) as FLOAT32;
                        } else {
                            *nrg_gain.offset(c as isize) = sqrt(
                                *nrg_ref.offset(c as isize) as core::ffi::c_double * tmp_2
                                    / ((*nrg_est.offset(c as isize)
                                        + 1 as core::ffi::c_int as FLOAT32) as core::ffi::c_double
                                        * fabs(
                                            *noise_floor.offset((next * num_nf_bands + o) as isize)
                                                as core::ffi::c_double + guard as core::ffi::c_double,
                                        )),
                            ) as FLOAT32;
                        }
                        *noise_level.offset(c as isize) = sqrt(
                            *nrg_ref.offset(c as isize) as core::ffi::c_double * tmp_2,
                        ) as FLOAT32;
                        c += 1;
                        k += 1;
                    }
                    m += 1;
                    j += 1;
                }
                c = 0 as core::ffi::c_int as WORD32;
                while c < (*gate_mode)[limiter_band as usize] {
                    p_est = 0 as core::ffi::c_int as FLOAT32;
                    p_ref = p_est;
                    k = (*lim_table)[limiter_band as usize][c as usize];
                    while k
                        < (*lim_table)[limiter_band
                            as usize][(c as core::ffi::c_int + 1 as core::ffi::c_int)
                            as usize]
                    {
                        p_ref += *nrg_ref.offset(k as isize);
                        p_est += *nrg_est.offset(k as isize);
                        k += 1;
                    }
                    avg_gain = sqrt(
                        ((p_ref as core::ffi::c_float + EPS)
                            / (p_est as core::ffi::c_float + EPS)) as core::ffi::c_double,
                    ) as FLOAT32;
                    g_max = avg_gain * ixheaac_g_lim_gains[limiter_gains as usize];
                    if g_max > 1.0e5f32 {
                        g_max = 1.0e5f32 as FLOAT32;
                    } else {};
                    k = (*lim_table)[limiter_band as usize][c as usize];
                    while k
                        < (*lim_table)[limiter_band
                            as usize][(c as core::ffi::c_int + 1 as core::ffi::c_int)
                            as usize]
                    {
                        if g_max <= *nrg_gain.offset(k as isize) {
                            *noise_level.offset(k as isize) = (*noise_level
                                .offset(k as isize) as FLOAT64
                                * (g_max as FLOAT64
                                    / (*nrg_gain.offset(k as isize) as FLOAT64 + guard)))
                                as FLOAT32;
                            *nrg_gain.offset(k as isize) = g_max;
                        }
                        k += 1;
                    }
                    p_adj = 0 as core::ffi::c_int as FLOAT32;
                    k = (*lim_table)[limiter_band as usize][c as usize];
                    while k
                        < (*lim_table)[limiter_band
                            as usize][(c as core::ffi::c_int + 1 as core::ffi::c_int)
                            as usize]
                    {
                        p_adj
                            += *nrg_gain.offset(k as isize)
                                * *nrg_gain.offset(k as isize)
                                * *nrg_est.offset(k as isize);
                        if *nrg_tone.offset(k as isize) != 0. {
                            p_adj
                                += *nrg_tone.offset(k as isize)
                                    * *nrg_tone.offset(k as isize);
                        } else if noise_absc_flag == 0 {
                            p_adj
                                += *noise_level.offset(k as isize)
                                    * *noise_level.offset(k as isize);
                        }
                        k += 1;
                    }
                    boost_gain = sqrt(
                        ((p_ref as core::ffi::c_float + EPS)
                            / (p_adj as core::ffi::c_float + EPS)) as core::ffi::c_double,
                    ) as FLOAT32;
                    boost_gain = (if boost_gain > 1.584893192f32 {
                        1.584893192f32
                    } else {
                        boost_gain as core::ffi::c_float
                    }) as FLOAT32;
                    k = (*lim_table)[limiter_band as usize][c as usize];
                    while k
                        < (*lim_table)[limiter_band
                            as usize][(c as core::ffi::c_int + 1 as core::ffi::c_int)
                            as usize]
                    {
                        *nrg_gain.offset(k as isize) *= boost_gain;
                        *noise_level.offset(k as isize) *= boost_gain;
                        *nrg_tone.offset(k as isize) *= boost_gain;
                        k += 1;
                    }
                    c += 1;
                }
                if esbr_start_up != 0 && ldmps_present != 1 as core::ffi::c_int {
                    n = 0 as core::ffi::c_int as WORD32;
                    while n < 4 as core::ffi::c_int {
                        memcpy(
                            ((*e_gain)[n as usize]).as_mut_ptr()
                                as *mut core::ffi::c_void,
                            nrg_gain as *const core::ffi::c_void,
                            (num_subbands as size_t)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                        );
                        memcpy(
                            ((*noise_buf)[n as usize]).as_mut_ptr()
                                as *mut core::ffi::c_void,
                            noise_level as *const core::ffi::c_void,
                            (num_subbands as size_t)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                        );
                        n += 1;
                    }
                    esbr_start_up = 0 as core::ffi::c_int as WORD32;
                    esbr_start_up_pvc = 0 as core::ffi::c_int as WORD32;
                }
                l = (rate as core::ffi::c_int
                    * (*p_frame_info).border_vec[i as usize] as core::ffi::c_int)
                    as WORD32;
                while l
                    < rate as core::ffi::c_int
                        * (*p_frame_info)
                            .border_vec[(i as core::ffi::c_int + 1 as core::ffi::c_int)
                            as usize] as core::ffi::c_int
                {
                    ptr_real_buf = (*input_real.offset(l as isize))
                        .as_mut_ptr()
                        .offset(sub_band_start as isize);
                    ptr_imag_buf = (*input_imag.offset(l as isize))
                        .as_mut_ptr()
                        .offset(sub_band_start as isize);
                    freq_inv = 1 as core::ffi::c_int as WORD32;
                    if ldmps_present == 1 as core::ffi::c_int
                        && sub_band_start as core::ffi::c_int & 1 as core::ffi::c_int
                            != 0
                    {
                        freq_inv = -(1 as core::ffi::c_int) as WORD32;
                    }
                    k = 0 as core::ffi::c_int as WORD32;
                    while k < num_subbands {
                        (*e_gain)[4 as core::ffi::c_int as usize][k as usize] = *nrg_gain
                            .offset(k as isize);
                        (*noise_buf)[4 as core::ffi::c_int as usize][k as usize] = *noise_level
                            .offset(k as isize);
                        c = 0 as core::ffi::c_int as WORD32;
                        sb_gain = 0 as core::ffi::c_int as FLOAT32;
                        sb_noise = 0 as core::ffi::c_int as FLOAT32;
                        n = 4 as WORD32 - smooth_length;
                        while n <= 4 as core::ffi::c_int {
                            sb_gain
                                += (*e_gain)[n as usize][k as usize]
                                    * *smooth_filt.offset(c as isize);
                            let fresh6 = c;
                            c = c + 1;
                            sb_noise
                                += (*noise_buf)[n as usize][k as usize]
                                    * *smooth_filt.offset(fresh6 as isize);
                            n += 1;
                        }
                        phase_index = (phase_index as core::ffi::c_int
                            + 1 as core::ffi::c_int & 511 as core::ffi::c_int) as WORD32;
                        sb_noise = if *nrg_tone.offset(k as isize)
                            != 0 as core::ffi::c_int as FLOAT32 || noise_absc_flag != 0
                        {
                            0 as core::ffi::c_int as FLOAT32
                        } else {
                            sb_noise
                        };
                        if ldmps_present == 1 as core::ffi::c_int {
                            *ptr_real_buf = *ptr_real_buf * sb_gain
                                + sb_noise
                                    * ixheaac_random_phase[phase_index
                                        as usize][0 as core::ffi::c_int as usize]
                                + *nrg_tone.offset(k as isize)
                                    * ixheaac_hphase_tbl[0 as core::ffi::c_int
                                        as usize][harm_index as usize];
                            *ptr_imag_buf = *ptr_imag_buf * sb_gain
                                + sb_noise
                                    * ixheaac_random_phase[phase_index
                                        as usize][1 as core::ffi::c_int as usize]
                                + *nrg_tone.offset(k as isize) * freq_inv as FLOAT32
                                    * ixheaac_hphase_tbl[1 as core::ffi::c_int
                                        as usize][harm_index as usize];
                            freq_inv = -freq_inv;
                        } else {
                            *ptr_real_buf = *ptr_real_buf * sb_gain
                                + sb_noise
                                    * ixheaac_random_phase[phase_index
                                        as usize][0 as core::ffi::c_int as usize];
                            *ptr_imag_buf = *ptr_imag_buf * sb_gain
                                + sb_noise
                                    * ixheaac_random_phase[phase_index
                                        as usize][1 as core::ffi::c_int as usize];
                        }
                        ptr_real_buf = ptr_real_buf.offset(1);
                        ptr_imag_buf = ptr_imag_buf.offset(1);
                        k += 1;
                    }
                    if ldmps_present == 1 as core::ffi::c_int {
                        harm_index = (harm_index as core::ffi::c_int
                            + 1 as core::ffi::c_int & 3 as core::ffi::c_int) as WORD32;
                    }
                    memcpy(
                        temp.as_mut_ptr() as *mut core::ffi::c_void,
                        ((*e_gain)[0 as core::ffi::c_int as usize]).as_mut_ptr()
                            as *const core::ffi::c_void,
                        (64 as size_t)
                            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                    );
                    n = 0 as core::ffi::c_int as WORD32;
                    while n < 4 as core::ffi::c_int {
                        memcpy(
                            ((*e_gain)[n as usize]).as_mut_ptr()
                                as *mut core::ffi::c_void,
                            ((*e_gain)[(n as core::ffi::c_int + 1 as core::ffi::c_int)
                                as usize])
                                .as_mut_ptr() as *const core::ffi::c_void,
                            (64 as size_t)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                        );
                        n += 1;
                    }
                    memcpy(
                        ((*e_gain)[4 as core::ffi::c_int as usize]).as_mut_ptr()
                            as *mut core::ffi::c_void,
                        temp.as_mut_ptr() as *const core::ffi::c_void,
                        (64 as size_t)
                            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                    );
                    memcpy(
                        temp.as_mut_ptr() as *mut core::ffi::c_void,
                        ((*noise_buf)[0 as core::ffi::c_int as usize]).as_mut_ptr()
                            as *const core::ffi::c_void,
                        (64 as size_t)
                            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                    );
                    n = 0 as core::ffi::c_int as WORD32;
                    while n < 4 as core::ffi::c_int {
                        memcpy(
                            ((*noise_buf)[n as usize]).as_mut_ptr()
                                as *mut core::ffi::c_void,
                            ((*noise_buf)[(n as core::ffi::c_int + 1 as core::ffi::c_int)
                                as usize])
                                .as_mut_ptr() as *const core::ffi::c_void,
                            (64 as size_t)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                        );
                        n += 1;
                    }
                    memcpy(
                        ((*noise_buf)[4 as core::ffi::c_int as usize]).as_mut_ptr()
                            as *mut core::ffi::c_void,
                        temp.as_mut_ptr() as *const core::ffi::c_void,
                        (64 as size_t)
                            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
                    );
                    l += 1;
                }
                if ldmps_present != 1 as core::ffi::c_int {
                    err_code = ixheaacd_apply_inter_tes(
                        (*input_real1
                            .offset(
                                (rate as core::ffi::c_int
                                    * (*p_frame_info).border_vec[i as usize]
                                        as core::ffi::c_int) as isize,
                            ))
                            .as_mut_ptr(),
                        (*input_imag1
                            .offset(
                                (rate as core::ffi::c_int
                                    * (*p_frame_info).border_vec[i as usize]
                                        as core::ffi::c_int) as isize,
                            ))
                            .as_mut_ptr(),
                        (*input_real
                            .offset(
                                (rate as core::ffi::c_int
                                    * (*p_frame_info).border_vec[i as usize]
                                        as core::ffi::c_int) as isize,
                            ))
                            .as_mut_ptr(),
                        (*input_imag
                            .offset(
                                (rate as core::ffi::c_int
                                    * (*p_frame_info).border_vec[i as usize]
                                        as core::ffi::c_int) as isize,
                            ))
                            .as_mut_ptr(),
                        rate
                            * (*p_frame_info)
                                .border_vec[(i as core::ffi::c_int + 1 as core::ffi::c_int)
                                as usize] as WORD32
                            - rate * (*p_frame_info).border_vec[i as usize] as WORD32,
                        sub_band_start,
                        num_subbands,
                        (*frame_data).inter_temp_shape_mode[i as usize],
                    ) as IA_ERRORCODE;
                    if err_code != 0 as core::ffi::c_int {
                        return err_code as WORD32;
                    }
                    l = (rate as core::ffi::c_int
                        * (*p_frame_info).border_vec[i as usize] as core::ffi::c_int)
                        as WORD32;
                    while l
                        < rate as core::ffi::c_int
                            * (*p_frame_info)
                                .border_vec[(i as core::ffi::c_int + 1 as core::ffi::c_int)
                                as usize] as core::ffi::c_int
                    {
                        ptr_real_buf = (*input_real.offset(l as isize))
                            .as_mut_ptr()
                            .offset(sub_band_start as isize);
                        ptr_imag_buf = (*input_imag.offset(l as isize))
                            .as_mut_ptr()
                            .offset(sub_band_start as isize);
                        if sub_band_start as core::ffi::c_int & 1 as core::ffi::c_int
                            != 0
                        {
                            freq_inv = -(1 as core::ffi::c_int) as WORD32;
                        } else {
                            freq_inv = 1 as core::ffi::c_int as WORD32;
                        }
                        k = 0 as core::ffi::c_int as WORD32;
                        while k < num_subbands {
                            *ptr_real_buf
                                += *nrg_tone.offset(k as isize)
                                    * ixheaac_hphase_tbl[0 as core::ffi::c_int
                                        as usize][harm_index as usize];
                            *ptr_imag_buf
                                += *nrg_tone.offset(k as isize) * freq_inv as FLOAT32
                                    * ixheaac_hphase_tbl[1 as core::ffi::c_int
                                        as usize][harm_index as usize];
                            ptr_real_buf = ptr_real_buf.offset(1);
                            ptr_imag_buf = ptr_imag_buf.offset(1);
                            freq_inv = -freq_inv;
                            k += 1;
                        }
                        harm_index = (harm_index as core::ffi::c_int
                            + 1 as core::ffi::c_int & 3 as core::ffi::c_int) as WORD32;
                        l += 1;
                    }
                }
            }
            i += 1;
        }
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < 64 as core::ffi::c_int {
        (*harm_flag_varlen_prev)[i as usize] = (*harm_flag_prev)[i as usize];
        (*harm_flag_varlen)[i as usize] = harmonics[i as usize];
        i += 1;
    }
    memcpy(
        (&mut *(*harm_flag_prev).as_mut_ptr().offset(0 as core::ffi::c_int as isize)
            as *mut WORD8)
            .offset(sub_band_start as isize) as *mut core::ffi::c_void,
        harmonics.as_mut_ptr() as *const core::ffi::c_void,
        ((64 as WORD32 - sub_band_start) as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD8>() as size_t),
    );
    if trans_env == bs_num_env {
        (*frame_data).env_short_flag_prev = 0 as core::ffi::c_int as WORD32;
    } else {
        (*frame_data).env_short_flag_prev = -(1 as core::ffi::c_int) as WORD32;
    }
    if ldmps_present != 1 as core::ffi::c_int {
        memcpy(
            &mut (*frame_data).str_frame_info_prev as *mut ia_frame_info_struct
                as *mut core::ffi::c_void,
            &mut (*frame_data).str_frame_info_details as *mut ia_frame_info_struct
                as *mut core::ffi::c_void,
            ::core::mem::size_of::<ia_frame_info_struct>() as size_t,
        );
        if (*frame_data).str_frame_info_details.num_env as core::ffi::c_int
            == 1 as core::ffi::c_int
        {
            (*frame_data).var_len_id_prev = 0 as core::ffi::c_int as WORD32;
        } else if (*frame_data).str_frame_info_details.num_env as core::ffi::c_int
            == 2 as core::ffi::c_int
        {
            (*frame_data).var_len_id_prev = 1 as core::ffi::c_int as WORD32;
        }
        if ((*frame_data).str_frame_info_details.num_noise_env as core::ffi::c_int)
            < 1 as core::ffi::c_int
            || (*frame_data).str_frame_info_details.num_noise_env as core::ffi::c_int
                > 2 as core::ffi::c_int
        {
            if ec_flag != 0 {
                (*frame_data).str_frame_info_details.num_noise_env = 1 as WORD16;
            } else {
                return IA_FATAL_ERROR as WORD32
            }
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < num_nf_bands {
            *prev_env_noise_level.offset(i as isize) = (*frame_data)
                .flt_noise_floor[(((*frame_data).str_frame_info_details.num_noise_env
                as WORD32 - 1 as WORD32) * num_nf_bands + i) as usize];
            i += 1;
        }
    }
    (*frame_data).harm_index = harm_index;
    (*frame_data).phase_index = phase_index;
    (*(*frame_data).pstr_sbr_header).esbr_start_up = esbr_start_up;
    (*(*frame_data).pstr_sbr_header).esbr_start_up_pvc = esbr_start_up_pvc;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_createlimiterbands(
    mut lim_table: *mut [WORD32; 13],
    mut gate_mode: *mut WORD32,
    mut freq_band_tbl: *mut WORD16,
    mut ixheaacd_num_bands: WORD32,
    mut x_over_qmf: *mut WORD32,
    mut b_patching_mode: WORD32,
    mut upsamp_4_flag: WORD32,
    mut patch_param: *mut ixheaacd_lpp_trans_patch,
    mut ec_flag: WORD32,
) -> IA_ERRORCODE {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut is_patch_border: [WORD32; 2] = [0; 2];
    let mut patch_borders: [WORD32; 7] = [0; 7];
    let mut temp_limiter_band_calc: [WORD32; 39] = [0; 39];
    let mut num_octave: core::ffi::c_double = 0.;
    let mut num_patches: WORD32 = 0;
    let mut sub_band_start: WORD32 = *freq_band_tbl
        .offset(0 as core::ffi::c_int as isize) as WORD32;
    let mut sub_band_end: WORD32 = *freq_band_tbl.offset(ixheaacd_num_bands as isize)
        as WORD32;
    let log2: core::ffi::c_double = log(2.0f64) as core::ffi::c_double;
    let limbnd_per_oct: [core::ffi::c_double; 4] = [
        0 as core::ffi::c_int as core::ffi::c_double,
        1.2f64,
        2.0f64,
        3.0f64,
    ];
    if b_patching_mode == 0 && !x_over_qmf.is_null() {
        num_patches = 0 as core::ffi::c_int as WORD32;
        if upsamp_4_flag != 0 {
            i = 1 as core::ffi::c_int as WORD32;
            while i < MAX_NUM_PATCHES {
                if *x_over_qmf.offset(i as isize) != 0 as core::ffi::c_int {
                    num_patches += 1;
                }
                i += 1;
            }
        } else {
            i = 1 as core::ffi::c_int as WORD32;
            while i < 4 as core::ffi::c_int {
                if *x_over_qmf.offset(i as isize) != 0 as core::ffi::c_int {
                    num_patches += 1;
                }
                i += 1;
            }
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < num_patches {
            patch_borders[i as usize] = *x_over_qmf.offset(i as isize) - sub_band_start;
            i += 1;
        }
    } else {
        num_patches = (*patch_param).num_patches;
        i = 0 as core::ffi::c_int as WORD32;
        while i < num_patches {
            patch_borders[i as usize] = (*patch_param).start_subband[i as usize]
                - sub_band_start;
            i += 1;
        }
    }
    patch_borders[i as usize] = sub_band_end - sub_band_start;
    (*lim_table
        .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = *freq_band_tbl
        .offset(0 as core::ffi::c_int as isize) as WORD32 - sub_band_start;
    (*lim_table
        .offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = *freq_band_tbl
        .offset(ixheaacd_num_bands as isize) as WORD32 - sub_band_start;
    *gate_mode.offset(0 as core::ffi::c_int as isize) = 1 as core::ffi::c_int as WORD32;
    i = 1 as core::ffi::c_int as WORD32;
    while i < 4 as core::ffi::c_int {
        k = 0 as core::ffi::c_int as WORD32;
        while k <= ixheaacd_num_bands {
            temp_limiter_band_calc[k as usize] = *freq_band_tbl.offset(k as isize)
                as WORD32 - sub_band_start;
            k += 1;
        }
        k = 1 as core::ffi::c_int as WORD32;
        while k < num_patches {
            temp_limiter_band_calc[(ixheaacd_num_bands + k) as usize] = patch_borders[k
                as usize];
            k += 1;
        }
        *gate_mode.offset(i as isize) = (ixheaacd_num_bands as core::ffi::c_int
            + num_patches as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        ixheaacd_shellsort(
            temp_limiter_band_calc.as_mut_ptr(),
            *gate_mode.offset(i as isize) + 1 as WORD32,
        );
        j = 1 as core::ffi::c_int as WORD32;
        while j <= *gate_mode.offset(i as isize) {
            num_octave = log(
                (temp_limiter_band_calc[j as usize] + sub_band_start)
                    as core::ffi::c_double
                    / (temp_limiter_band_calc[(j as core::ffi::c_int
                        - 1 as core::ffi::c_int) as usize] + sub_band_start)
                        as core::ffi::c_double,
            ) / log2;
            if num_octave * limbnd_per_oct[i as usize] < 0.49f64 {
                if temp_limiter_band_calc[j as usize]
                    == temp_limiter_band_calc[(j as core::ffi::c_int
                        - 1 as core::ffi::c_int) as usize]
                {
                    temp_limiter_band_calc[j as usize] = sub_band_end;
                    ixheaacd_shellsort(
                        temp_limiter_band_calc.as_mut_ptr(),
                        *gate_mode.offset(i as isize) + 1 as WORD32,
                    );
                    let ref mut fresh0 = *gate_mode.offset(i as isize);
                    *fresh0 -= 1;
                    j -= 1;
                } else {
                    is_patch_border[1 as core::ffi::c_int as usize] = 0
                        as core::ffi::c_int as WORD32;
                    is_patch_border[0 as core::ffi::c_int as usize] = is_patch_border[1
                        as core::ffi::c_int as usize];
                    k = 0 as core::ffi::c_int as WORD32;
                    while k <= num_patches {
                        if temp_limiter_band_calc[(j as core::ffi::c_int
                            - 1 as core::ffi::c_int) as usize]
                            == patch_borders[k as usize]
                        {
                            is_patch_border[0 as core::ffi::c_int as usize] = 1
                                as core::ffi::c_int as WORD32;
                            break;
                        } else {
                            k += 1;
                        }
                    }
                    k = 0 as core::ffi::c_int as WORD32;
                    while k <= num_patches {
                        if temp_limiter_band_calc[j as usize]
                            == patch_borders[k as usize]
                        {
                            is_patch_border[1 as core::ffi::c_int as usize] = 1
                                as core::ffi::c_int as WORD32;
                            break;
                        } else {
                            k += 1;
                        }
                    }
                    if is_patch_border[1 as core::ffi::c_int as usize] == 0 {
                        temp_limiter_band_calc[j as usize] = sub_band_end;
                        ixheaacd_shellsort(
                            temp_limiter_band_calc.as_mut_ptr(),
                            *gate_mode.offset(i as isize) + 1 as WORD32,
                        );
                        let ref mut fresh1 = *gate_mode.offset(i as isize);
                        *fresh1 -= 1;
                        j -= 1;
                    } else if is_patch_border[0 as core::ffi::c_int as usize] == 0 {
                        temp_limiter_band_calc[(j as core::ffi::c_int
                            - 1 as core::ffi::c_int) as usize] = sub_band_end;
                        ixheaacd_shellsort(
                            temp_limiter_band_calc.as_mut_ptr(),
                            *gate_mode.offset(i as isize) + 1 as WORD32,
                        );
                        let ref mut fresh2 = *gate_mode.offset(i as isize);
                        *fresh2 -= 1;
                        j -= 1;
                    }
                }
            }
            j += 1;
        }
        if *gate_mode.offset(i as isize) > 12 as core::ffi::c_int {
            if ec_flag != 0 {
                *gate_mode.offset(i as isize) = 12 as core::ffi::c_int as WORD32;
            } else {
                return IA_FATAL_ERROR as IA_ERRORCODE
            }
        }
        k = 0 as core::ffi::c_int as WORD32;
        while k <= *gate_mode.offset(i as isize) {
            (*lim_table.offset(i as isize))[k as usize] = temp_limiter_band_calc[k
                as usize];
            k += 1;
        }
        i += 1;
    }
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_apply_inter_tes(
    mut qmf_real1: *mut FLOAT32,
    mut qmf_imag1: *mut FLOAT32,
    mut qmf_real: *mut FLOAT32,
    mut qmf_imag: *mut FLOAT32,
    mut num_sample: WORD32,
    mut sub_band_start: WORD32,
    mut num_subband: WORD32,
    mut gamma_idx: WORD32,
) -> WORD32 {
    let mut sub_band_end: WORD32 = sub_band_start + num_subband;
    let mut subsample_power_high: [FLOAT32; 78] = [0.; 78];
    let mut subsample_power_low: [FLOAT32; 78] = [0.; 78];
    let mut total_power_high: FLOAT32 = 0.0f32;
    let mut total_power_low: FLOAT32 = 0.0f32;
    let mut total_power_high_after: FLOAT32 = 1.0e-6f32;
    let mut gain: [FLOAT32; 78] = [0.; 78];
    let mut gain_adj: FLOAT32 = 0.;
    let mut gain_adj_2: FLOAT32 = 0.;
    let mut gamma: FLOAT32 = ixheaac_q_gamma_table[gamma_idx as usize];
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    if num_sample > TIMESLOT_BUFFER_SIZE {
        return IA_FATAL_ERROR as WORD32;
    }
    if gamma > 0 as core::ffi::c_int as FLOAT32 {
        i = 0 as core::ffi::c_int as WORD32;
        while i < num_sample {
            memcpy(
                &mut *qmf_real.offset((64 as WORD32 * i) as isize) as *mut FLOAT32
                    as *mut core::ffi::c_void,
                &mut *qmf_real1.offset((64 as WORD32 * i) as isize) as *mut FLOAT32
                    as *const core::ffi::c_void,
                (sub_band_start as size_t)
                    .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
            );
            memcpy(
                &mut *qmf_imag.offset((64 as WORD32 * i) as isize) as *mut FLOAT32
                    as *mut core::ffi::c_void,
                &mut *qmf_imag1.offset((64 as WORD32 * i) as isize) as *mut FLOAT32
                    as *const core::ffi::c_void,
                (sub_band_start as size_t)
                    .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
            );
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < num_sample {
            subsample_power_low[i as usize] = 0.0f32 as FLOAT32;
            j = 0 as core::ffi::c_int as WORD32;
            while j < sub_band_start {
                subsample_power_low[i as usize]
                    += *qmf_real.offset((64 as WORD32 * i + j) as isize)
                        * *qmf_real.offset((64 as WORD32 * i + j) as isize);
                subsample_power_low[i as usize]
                    += *qmf_imag.offset((64 as WORD32 * i + j) as isize)
                        * *qmf_imag.offset((64 as WORD32 * i + j) as isize);
                j += 1;
            }
            subsample_power_high[i as usize] = 0.0f32 as FLOAT32;
            j = sub_band_start;
            while j < sub_band_end {
                subsample_power_high[i as usize]
                    += *qmf_real.offset((64 as WORD32 * i + j) as isize)
                        * *qmf_real.offset((64 as WORD32 * i + j) as isize);
                subsample_power_high[i as usize]
                    += *qmf_imag.offset((64 as WORD32 * i + j) as isize)
                        * *qmf_imag.offset((64 as WORD32 * i + j) as isize);
                j += 1;
            }
            total_power_low += subsample_power_low[i as usize];
            total_power_high += subsample_power_high[i as usize];
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < num_sample {
            gain[i as usize] = sqrt(
                (subsample_power_low[i as usize] * num_sample as core::ffi::c_float
                    / (total_power_low as core::ffi::c_float + 1.0e-6f32))
                    as core::ffi::c_double,
            ) as FLOAT32;
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < num_sample {
            gain[i as usize] = 1.0f32
                + gamma as core::ffi::c_float * (gain[i as usize] - 1.0f32);
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < num_sample {
            if gain[i as usize] < 0.2f32 {
                gain[i as usize] = 0.2f32 as FLOAT32;
            }
            subsample_power_high[i as usize] *= gain[i as usize] * gain[i as usize];
            total_power_high_after += subsample_power_high[i as usize];
            i += 1;
        }
        gain_adj_2 = total_power_high / total_power_high_after;
        gain_adj = sqrt(gain_adj_2 as core::ffi::c_double) as FLOAT32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < num_sample {
            gain[i as usize] *= gain_adj;
            j = sub_band_start;
            while j < sub_band_end {
                *qmf_real.offset((64 as WORD32 * i + j) as isize) *= gain[i as usize];
                *qmf_imag.offset((64 as WORD32 * i + j) as isize) *= gain[i as usize];
                j += 1;
            }
            i += 1;
        }
    }
    return 0 as WORD32;
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_FATAL_ERROR: core::ffi::c_uint = 0x80000000 as core::ffi::c_uint;
