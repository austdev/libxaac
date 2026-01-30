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
    fn atan(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn atan2(__y: core::ffi::c_double, __x: core::ffi::c_double) -> core::ffi::c_double;
    fn cos(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn sin(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn pow(__x: core::ffi::c_double, __y: core::ffi::c_double) -> core::ffi::c_double;
    fn sqrt(__x: core::ffi::c_double) -> core::ffi::c_double;
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type WORD16 = core::ffi::c_short;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
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
pub type REVERB_BUFFERS_CH_RI = [[[WORD16; 32]; 3]; 5];
pub type REVERB_BUFFERS_RI = *mut [[WORD16; 64]; 3];
pub const PI: core::ffi::c_double = 3.14159265358979323846264338327950288f64;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const MAX_NUM_COLUMNS: core::ffi::c_int = 32 as core::ffi::c_int;
pub const MAX_NUM_COLUMNS_960: core::ffi::c_int = 30 as core::ffi::c_int;
pub const NUM_OF_QUAD_MIRROR_FILTER_CHNLS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const NUM_OF_ALL_PASS_CHNLS: core::ffi::c_int = 23 as core::ffi::c_int;
pub const DEL_ALL_PASS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const HIGH_DEL: core::ffi::c_int = 14 as core::ffi::c_int;
pub const NRG_INT_COEFF: core::ffi::c_float = 0.75f32;
pub const INIT_FILT_COEFF: core::ffi::c_float = 1.0f32 - NRG_INT_COEFF;
pub const NUM_IPD_STEPS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const NUM_OPD_STEPS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const NUM_HI_RES_BINS: core::ffi::c_int = 34 as core::ffi::c_int;
pub const NUM_MID_RES_BINS: core::ffi::c_int = 20 as core::ffi::c_int;
pub const _M_PI_: core::ffi::c_double = 3.141592653589793238462643383279f64;
pub const PSC_SQRT2: core::ffi::c_double = 1.41421356237309504880f64;
pub const PSC_PIF: FLOAT32 = _M_PI_ as FLOAT32;
pub const PSC_SQRT2F: FLOAT32 = PSC_SQRT2 as FLOAT32;
pub const IPD_HALF_RANGE: FLOAT32 = _M_PI_ as FLOAT32;
pub const IPD_SCALE_FACTOR: FLOAT32 = IPD_HALF_RANGE / NUM_IPD_STEPS as FLOAT32;
pub const OPD_HALF_RANGE: FLOAT32 = _M_PI_ as FLOAT32;
pub const OPD_SCALE_FACTOR: FLOAT32 = OPD_HALF_RANGE / NUM_OPD_STEPS as FLOAT32;
pub const NEGATE_IPD_MASK: core::ffi::c_int = 0x1000 as core::ffi::c_int;
pub const DECAY_CUTOFF: core::ffi::c_int = 3 as core::ffi::c_int;
pub const DECAY_CUTOFF_HI_RES: core::ffi::c_int = 5 as core::ffi::c_int;
pub const DECAY_SLOPE: core::ffi::c_float = 0.05f32;
pub const PHASE_SMOOTH_HIST1: core::ffi::c_float = 0.5f32;
pub const PHASE_SMOOTH_HIST2: core::ffi::c_float = 0.25f32;
pub const NUM_QMF_BANDS_IN_HYBRID20: core::ffi::c_int = 3 as core::ffi::c_int;
pub const NUM_QMF_BANDS_IN_HYBRID34: core::ffi::c_int = 5 as core::ffi::c_int;
pub const PEAK_DECAY_FACTOR_FAST: core::ffi::c_float = 0.765928338364649f32;
pub const HYBRID_FILTER_LENGTH: core::ffi::c_int = 13 as core::ffi::c_int;
pub const HYBRID_FILTER_DELAY: core::ffi::c_int = 6 as core::ffi::c_int;
pub const NO_HYBRID_CHANNELS_2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const NO_HYBRID_CHANNELS_4: core::ffi::c_int = 4 as core::ffi::c_int;
pub const NO_HYBRID_CHANNELS_8: core::ffi::c_int = 8 as core::ffi::c_int;
pub const NO_HYBRID_CHANNELS_12: core::ffi::c_int = 12 as core::ffi::c_int;
pub const REAL: core::ffi::c_int = 0 as core::ffi::c_int;
pub const CPLX: core::ffi::c_int = 1 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_k_chan_filt(
    mut ptr_qmf_re: *const FLOAT32,
    mut ptr_qmf_im: *const FLOAT32,
    mut ptr_tmp_hyb_re: *mut [FLOAT32; 64],
    mut ptr_tmp_hyb_im: *mut [FLOAT32; 64],
    mut nSamples: WORD32,
    mut k: WORD32,
    mut bCplx: WORD32,
    mut p: *const FLOAT32,
    mut cos_sin_mod_tbl: *mut FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut q: WORD32 = 0;
    let mut real: FLOAT32 = 0.;
    let mut imag: FLOAT32 = 0.;
    let mut cos_val: FLOAT32 = 0.;
    let mut sin_val: FLOAT32 = 0.;
    let mut p_real_imag: *mut FLOAT32 = cos_sin_mod_tbl;
    if bCplx != 0 {
        i = 0 as core::ffi::c_int as WORD32;
        while i < nSamples {
            q = 0 as core::ffi::c_int as WORD32;
            while q < k {
                real = 0 as core::ffi::c_int as FLOAT32;
                imag = 0 as core::ffi::c_int as FLOAT32;
                n = 0 as core::ffi::c_int as WORD32;
                while n < 13 as core::ffi::c_int {
                    let fresh2 = p_real_imag;
                    p_real_imag = p_real_imag.offset(1);
                    cos_val = *fresh2;
                    let fresh3 = p_real_imag;
                    p_real_imag = p_real_imag.offset(1);
                    sin_val = *fresh3;
                    real
                        += *p.offset(n as isize)
                            * (*ptr_qmf_re.offset((n + i) as isize) * cos_val
                                - *ptr_qmf_im.offset((n + i) as isize) * sin_val);
                    imag
                        += *p.offset(n as isize)
                            * (*ptr_qmf_im.offset((n + i) as isize) * cos_val
                                + *ptr_qmf_re.offset((n + i) as isize) * sin_val);
                    n += 1;
                }
                (*ptr_tmp_hyb_re.offset(i as isize))[q as usize] = real;
                (*ptr_tmp_hyb_im.offset(i as isize))[q as usize] = imag;
                q += 1;
            }
            p_real_imag = p_real_imag
                .offset(
                    -((13 as core::ffi::c_int * k as core::ffi::c_int
                        * 2 as core::ffi::c_int) as isize),
                );
            i += 1;
        }
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < nSamples {
            q = 0 as core::ffi::c_int as WORD32;
            while q < k {
                real = 0 as core::ffi::c_int as FLOAT32;
                imag = 0 as core::ffi::c_int as FLOAT32;
                n = 0 as core::ffi::c_int as WORD32;
                while n < 13 as core::ffi::c_int {
                    let fresh4 = p_real_imag;
                    p_real_imag = p_real_imag.offset(1);
                    cos_val = *fresh4;
                    real
                        += *p.offset(n as isize)
                            * (*ptr_qmf_re.offset((n + i) as isize) * cos_val);
                    imag
                        += *p.offset(n as isize)
                            * (*ptr_qmf_im.offset((n + i) as isize) * cos_val);
                    n += 1;
                }
                (*ptr_tmp_hyb_re.offset(i as isize))[q as usize] = real;
                (*ptr_tmp_hyb_im.offset(i as isize))[q as usize] = imag;
                q += 1;
            }
            p_real_imag = p_real_imag.offset(-((13 as WORD32 * k) as isize));
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hyb_anal(
    mut ptr_qmf_re: *mut *const FLOAT32,
    mut ptr_qmf_im: *mut *const FLOAT32,
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
    mut ptr_ps_tables: *mut ia_ps_tables_struct,
    mut use_34_st_bands: WORD32,
) -> VOID {
    let mut k: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut band: WORD32 = 0;
    let mut band_res: WORD32 = 0;
    let mut frame_size: WORD32 = 0;
    let mut ch_offset: WORD32 = 0 as WORD32;
    let mut ptr_tmp_hyb_re: *mut [FLOAT32; 32] = 0 as *mut [FLOAT32; 32];
    let mut ptr_tmp_hyb_im: *mut [FLOAT32; 32] = 0 as *mut [FLOAT32; 32];
    let mut ptr_hybrid: *mut ia_hybrid_flt_struct = 0 as *mut ia_hybrid_flt_struct;
    if use_34_st_bands == 0 {
        ptr_tmp_hyb_re = ((*ptr_ps_dec).hyb_left_re).as_mut_ptr() as *mut [FLOAT32; 32];
        ptr_tmp_hyb_im = ((*ptr_ps_dec).hyb_left_im).as_mut_ptr() as *mut [FLOAT32; 32];
        ptr_hybrid = &mut (*ptr_ps_dec).str_flt_hybrid20;
    } else {
        ptr_tmp_hyb_re = 0 as *mut [FLOAT32; 32];
        ptr_tmp_hyb_im = ((*ptr_ps_dec).hyb_left_im).as_mut_ptr() as *mut [FLOAT32; 32];
        ptr_hybrid = &mut (*ptr_ps_dec).str_flt_hybrid34;
    }
    frame_size = (*ptr_hybrid).frame_size;
    band = 0 as core::ffi::c_int as WORD32;
    while band < (*ptr_hybrid).num_qmf_bands {
        band_res = *((*ptr_hybrid).ptr_resol).offset(band as isize) as WORD32;
        memcpy(
            (*ptr_hybrid).ptr_work_re as *mut core::ffi::c_void,
            (*((*ptr_hybrid).ptr_qmf_buf_re).offset(band as isize)).as_mut_ptr()
                as *const core::ffi::c_void,
            ((HYBRID_FILTER_LENGTH - 1 as core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        memcpy(
            (*ptr_hybrid).ptr_work_im as *mut core::ffi::c_void,
            (*((*ptr_hybrid).ptr_qmf_buf_im).offset(band as isize)).as_mut_ptr()
                as *const core::ffi::c_void,
            ((HYBRID_FILTER_LENGTH - 1 as core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        n = 0 as core::ffi::c_int as WORD32;
        while n < frame_size {
            *((*ptr_hybrid).ptr_work_re)
                .offset((HYBRID_FILTER_LENGTH - 1 as WORD32 + n) as isize) = *(*ptr_qmf_re
                .offset((n as core::ffi::c_int + HYBRID_FILTER_DELAY) as isize))
                .offset(band as isize);
            *((*ptr_hybrid).ptr_work_im)
                .offset((HYBRID_FILTER_LENGTH - 1 as WORD32 + n) as isize) = *(*ptr_qmf_im
                .offset((n as core::ffi::c_int + HYBRID_FILTER_DELAY) as isize))
                .offset(band as isize);
            n += 1;
        }
        memcpy(
            (*((*ptr_hybrid).ptr_qmf_buf_re).offset(band as isize)).as_mut_ptr()
                as *mut core::ffi::c_void,
            ((*ptr_hybrid).ptr_work_re).offset(frame_size as isize)
                as *const core::ffi::c_void,
            ((HYBRID_FILTER_LENGTH - 1 as core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        memcpy(
            (*((*ptr_hybrid).ptr_qmf_buf_im).offset(band as isize)).as_mut_ptr()
                as *mut core::ffi::c_void,
            ((*ptr_hybrid).ptr_work_im).offset(frame_size as isize)
                as *const core::ffi::c_void,
            ((HYBRID_FILTER_LENGTH - 1 as core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        if !ptr_tmp_hyb_re.is_null() {
            match band_res {
                NO_HYBRID_CHANNELS_2 => {
                    ixheaacd_k_chan_filt(
                        (*ptr_hybrid).ptr_work_re,
                        (*ptr_hybrid).ptr_work_im,
                        (*ptr_hybrid).ptr_temp_re,
                        (*ptr_hybrid).ptr_temp_im,
                        frame_size,
                        NO_HYBRID_CHANNELS_2,
                        REAL,
                        ((*ptr_ps_tables).p2_13_20).as_mut_ptr(),
                        &mut *(*((*ptr_ps_tables).cos_mod_2channel)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize),
                    );
                }
                NO_HYBRID_CHANNELS_4 => {
                    ixheaacd_k_chan_filt(
                        (*ptr_hybrid).ptr_work_re,
                        (*ptr_hybrid).ptr_work_im,
                        (*ptr_hybrid).ptr_temp_re,
                        (*ptr_hybrid).ptr_temp_im,
                        frame_size,
                        NO_HYBRID_CHANNELS_4,
                        CPLX,
                        ((*ptr_ps_tables).p4_13_34).as_mut_ptr(),
                        &mut *(*((*ptr_ps_tables).cos_sin_mod_4channel)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize),
                    );
                }
                NO_HYBRID_CHANNELS_8 => {
                    ixheaacd_k_chan_filt(
                        (*ptr_hybrid).ptr_work_re,
                        (*ptr_hybrid).ptr_work_im,
                        (*ptr_hybrid).ptr_temp_re,
                        (*ptr_hybrid).ptr_temp_im,
                        frame_size,
                        NO_HYBRID_CHANNELS_8,
                        CPLX,
                        if use_34_st_bands != 0 {
                            ((*ptr_ps_tables).p8_13_34).as_mut_ptr()
                        } else {
                            ((*ptr_ps_tables).p8_13_20).as_mut_ptr()
                        },
                        &mut *(*((*ptr_ps_tables).cos_sin_mod_8channel)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize),
                    );
                }
                NO_HYBRID_CHANNELS_12 => {
                    ixheaacd_k_chan_filt(
                        (*ptr_hybrid).ptr_work_re,
                        (*ptr_hybrid).ptr_work_im,
                        (*ptr_hybrid).ptr_temp_re,
                        (*ptr_hybrid).ptr_temp_im,
                        frame_size,
                        NO_HYBRID_CHANNELS_12,
                        CPLX,
                        ((*ptr_ps_tables).p12_13_34).as_mut_ptr(),
                        &mut *(*((*ptr_ps_tables).cos_sin_mod_12channel)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize),
                    );
                }
                _ => {}
            }
            n = 0 as core::ffi::c_int as WORD32;
            while n < frame_size {
                k = 0 as core::ffi::c_int as WORD32;
                while k < band_res {
                    (*ptr_tmp_hyb_re.offset(n as isize))[(ch_offset + k) as usize] = (*((*ptr_hybrid)
                        .ptr_temp_re)
                        .offset(n as isize))[k as usize];
                    (*ptr_tmp_hyb_im.offset(n as isize))[(ch_offset + k) as usize] = (*((*ptr_hybrid)
                        .ptr_temp_im)
                        .offset(n as isize))[k as usize];
                    k += 1;
                }
                n += 1;
            }
            ch_offset += band_res;
        }
        band += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hyb_synth(
    mut ptr_tmp_hyb_re: *mut [FLOAT32; 32],
    mut ptr_tmp_hyb_im: *mut [FLOAT32; 32],
    mut ptr_qmf_re: *mut *mut FLOAT32,
    mut ptr_qmf_im: *mut *mut FLOAT32,
    mut ptr_hybrid: *mut ia_hybrid_flt_struct,
) -> VOID {
    let mut k: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut band: WORD32 = 0;
    let mut band_res: WORD16 = 0;
    let mut frame_size: WORD32 = (*ptr_hybrid).frame_size;
    let mut ch_offset: WORD32 = 0 as WORD32;
    band = 0 as core::ffi::c_int as WORD32;
    while band < (*ptr_hybrid).num_qmf_bands {
        band_res = *((*ptr_hybrid).ptr_resol).offset(band as isize);
        n = 0 as core::ffi::c_int as WORD32;
        while n < frame_size {
            let ref mut fresh0 = *(*ptr_qmf_im.offset(n as isize)).offset(band as isize);
            *fresh0 = 0 as core::ffi::c_int as FLOAT32;
            *(*ptr_qmf_re.offset(n as isize)).offset(band as isize) = *fresh0;
            k = 0 as core::ffi::c_int as WORD32;
            while k < band_res as WORD32 {
                *(*ptr_qmf_re.offset(n as isize)).offset(band as isize)
                    += (*ptr_tmp_hyb_re.offset(n as isize))[(ch_offset + k) as usize];
                *(*ptr_qmf_im.offset(n as isize)).offset(band as isize)
                    += (*ptr_tmp_hyb_im.offset(n as isize))[(ch_offset + k) as usize];
                k += 1;
            }
            n += 1;
        }
        ch_offset += band_res as core::ffi::c_int;
        band += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_map_34_float_to_20(
    mut ptr_index: *mut FLOAT32,
) -> VOID {
    *ptr_index.offset(0 as core::ffi::c_int as isize) = ((2 as core::ffi::c_int
        as core::ffi::c_float * *ptr_index.offset(0 as core::ffi::c_int as isize)
        + *ptr_index.offset(1 as core::ffi::c_int as isize)) / 3.0f32) as FLOAT32;
    *ptr_index.offset(1 as core::ffi::c_int as isize) = ((*ptr_index
        .offset(1 as core::ffi::c_int as isize)
        + 2 as core::ffi::c_int as core::ffi::c_float
            * *ptr_index.offset(2 as core::ffi::c_int as isize)) / 3.0f32) as FLOAT32;
    *ptr_index.offset(2 as core::ffi::c_int as isize) = ((2 as core::ffi::c_int
        as core::ffi::c_float * *ptr_index.offset(3 as core::ffi::c_int as isize)
        + *ptr_index.offset(4 as core::ffi::c_int as isize)) / 3.0f32) as FLOAT32;
    *ptr_index.offset(3 as core::ffi::c_int as isize) = ((*ptr_index
        .offset(4 as core::ffi::c_int as isize)
        + 2 as core::ffi::c_int as core::ffi::c_float
            * *ptr_index.offset(5 as core::ffi::c_int as isize)) / 3.0f32) as FLOAT32;
    *ptr_index.offset(4 as core::ffi::c_int as isize) = ((*ptr_index
        .offset(6 as core::ffi::c_int as isize)
        + *ptr_index.offset(7 as core::ffi::c_int as isize)) / 2.0f32) as FLOAT32;
    *ptr_index.offset(5 as core::ffi::c_int as isize) = ((*ptr_index
        .offset(8 as core::ffi::c_int as isize)
        + *ptr_index.offset(9 as core::ffi::c_int as isize)) / 2.0f32) as FLOAT32;
    *ptr_index.offset(6 as core::ffi::c_int as isize) = *ptr_index
        .offset(10 as core::ffi::c_int as isize);
    *ptr_index.offset(7 as core::ffi::c_int as isize) = *ptr_index
        .offset(11 as core::ffi::c_int as isize);
    *ptr_index.offset(8 as core::ffi::c_int as isize) = ((*ptr_index
        .offset(12 as core::ffi::c_int as isize)
        + *ptr_index.offset(13 as core::ffi::c_int as isize)) / 2.0f32) as FLOAT32;
    *ptr_index.offset(9 as core::ffi::c_int as isize) = ((*ptr_index
        .offset(14 as core::ffi::c_int as isize)
        + *ptr_index.offset(15 as core::ffi::c_int as isize)) / 2.0f32) as FLOAT32;
    *ptr_index.offset(10 as core::ffi::c_int as isize) = *ptr_index
        .offset(16 as core::ffi::c_int as isize);
    *ptr_index.offset(11 as core::ffi::c_int as isize) = *ptr_index
        .offset(17 as core::ffi::c_int as isize);
    *ptr_index.offset(12 as core::ffi::c_int as isize) = *ptr_index
        .offset(18 as core::ffi::c_int as isize);
    *ptr_index.offset(13 as core::ffi::c_int as isize) = *ptr_index
        .offset(19 as core::ffi::c_int as isize);
    *ptr_index.offset(14 as core::ffi::c_int as isize) = ((*ptr_index
        .offset(20 as core::ffi::c_int as isize)
        + *ptr_index.offset(21 as core::ffi::c_int as isize)) / 2.0f32) as FLOAT32;
    *ptr_index.offset(15 as core::ffi::c_int as isize) = ((*ptr_index
        .offset(22 as core::ffi::c_int as isize)
        + *ptr_index.offset(23 as core::ffi::c_int as isize)) / 2.0f32) as FLOAT32;
    *ptr_index.offset(16 as core::ffi::c_int as isize) = ((*ptr_index
        .offset(24 as core::ffi::c_int as isize)
        + *ptr_index.offset(25 as core::ffi::c_int as isize)) / 2.0f32) as FLOAT32;
    *ptr_index.offset(17 as core::ffi::c_int as isize) = ((*ptr_index
        .offset(26 as core::ffi::c_int as isize)
        + *ptr_index.offset(27 as core::ffi::c_int as isize)) / 2.0f32) as FLOAT32;
    *ptr_index.offset(18 as core::ffi::c_int as isize) = ((*ptr_index
        .offset(28 as core::ffi::c_int as isize)
        + *ptr_index.offset(29 as core::ffi::c_int as isize)
        + *ptr_index.offset(30 as core::ffi::c_int as isize)
        + *ptr_index.offset(31 as core::ffi::c_int as isize)) / 4.0f32) as FLOAT32;
    *ptr_index.offset(19 as core::ffi::c_int as isize) = ((*ptr_index
        .offset(32 as core::ffi::c_int as isize)
        + *ptr_index.offset(33 as core::ffi::c_int as isize)) / 2.0f32) as FLOAT32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_map_20_float_to_34(
    mut ptr_index: *mut FLOAT32,
) -> VOID {
    let mut arr_temp: [FLOAT32; 34] = [0.; 34];
    let mut i: WORD32 = 0;
    arr_temp[0 as core::ffi::c_int as usize] = *ptr_index
        .offset(0 as core::ffi::c_int as isize);
    arr_temp[1 as core::ffi::c_int as usize] = ((*ptr_index
        .offset(0 as core::ffi::c_int as isize)
        + *ptr_index.offset(1 as core::ffi::c_int as isize)) / 2.0f32) as FLOAT32;
    arr_temp[2 as core::ffi::c_int as usize] = *ptr_index
        .offset(1 as core::ffi::c_int as isize);
    arr_temp[3 as core::ffi::c_int as usize] = *ptr_index
        .offset(2 as core::ffi::c_int as isize);
    arr_temp[4 as core::ffi::c_int as usize] = ((*ptr_index
        .offset(2 as core::ffi::c_int as isize)
        + *ptr_index.offset(3 as core::ffi::c_int as isize)) / 2.0f32) as FLOAT32;
    arr_temp[5 as core::ffi::c_int as usize] = *ptr_index
        .offset(3 as core::ffi::c_int as isize);
    arr_temp[6 as core::ffi::c_int as usize] = *ptr_index
        .offset(4 as core::ffi::c_int as isize);
    arr_temp[7 as core::ffi::c_int as usize] = *ptr_index
        .offset(4 as core::ffi::c_int as isize);
    arr_temp[8 as core::ffi::c_int as usize] = *ptr_index
        .offset(5 as core::ffi::c_int as isize);
    arr_temp[9 as core::ffi::c_int as usize] = *ptr_index
        .offset(5 as core::ffi::c_int as isize);
    arr_temp[10 as core::ffi::c_int as usize] = *ptr_index
        .offset(6 as core::ffi::c_int as isize);
    arr_temp[11 as core::ffi::c_int as usize] = *ptr_index
        .offset(7 as core::ffi::c_int as isize);
    arr_temp[12 as core::ffi::c_int as usize] = *ptr_index
        .offset(8 as core::ffi::c_int as isize);
    arr_temp[13 as core::ffi::c_int as usize] = *ptr_index
        .offset(8 as core::ffi::c_int as isize);
    arr_temp[14 as core::ffi::c_int as usize] = *ptr_index
        .offset(9 as core::ffi::c_int as isize);
    arr_temp[15 as core::ffi::c_int as usize] = *ptr_index
        .offset(9 as core::ffi::c_int as isize);
    arr_temp[16 as core::ffi::c_int as usize] = *ptr_index
        .offset(10 as core::ffi::c_int as isize);
    arr_temp[17 as core::ffi::c_int as usize] = *ptr_index
        .offset(11 as core::ffi::c_int as isize);
    arr_temp[18 as core::ffi::c_int as usize] = *ptr_index
        .offset(12 as core::ffi::c_int as isize);
    arr_temp[19 as core::ffi::c_int as usize] = *ptr_index
        .offset(13 as core::ffi::c_int as isize);
    arr_temp[20 as core::ffi::c_int as usize] = *ptr_index
        .offset(14 as core::ffi::c_int as isize);
    arr_temp[21 as core::ffi::c_int as usize] = *ptr_index
        .offset(14 as core::ffi::c_int as isize);
    arr_temp[22 as core::ffi::c_int as usize] = *ptr_index
        .offset(15 as core::ffi::c_int as isize);
    arr_temp[23 as core::ffi::c_int as usize] = *ptr_index
        .offset(15 as core::ffi::c_int as isize);
    arr_temp[24 as core::ffi::c_int as usize] = *ptr_index
        .offset(16 as core::ffi::c_int as isize);
    arr_temp[25 as core::ffi::c_int as usize] = *ptr_index
        .offset(16 as core::ffi::c_int as isize);
    arr_temp[26 as core::ffi::c_int as usize] = *ptr_index
        .offset(17 as core::ffi::c_int as isize);
    arr_temp[27 as core::ffi::c_int as usize] = *ptr_index
        .offset(17 as core::ffi::c_int as isize);
    arr_temp[28 as core::ffi::c_int as usize] = *ptr_index
        .offset(18 as core::ffi::c_int as isize);
    arr_temp[29 as core::ffi::c_int as usize] = *ptr_index
        .offset(18 as core::ffi::c_int as isize);
    arr_temp[30 as core::ffi::c_int as usize] = *ptr_index
        .offset(18 as core::ffi::c_int as isize);
    arr_temp[31 as core::ffi::c_int as usize] = *ptr_index
        .offset(18 as core::ffi::c_int as isize);
    arr_temp[32 as core::ffi::c_int as usize] = *ptr_index
        .offset(19 as core::ffi::c_int as isize);
    arr_temp[33 as core::ffi::c_int as usize] = *ptr_index
        .offset(19 as core::ffi::c_int as isize);
    i = 0 as core::ffi::c_int as WORD32;
    while i < 34 as core::ffi::c_int {
        *ptr_index.offset(i as isize) = arr_temp[i as usize];
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_create_ps_esbr_dec(
    mut ptr_ps_dec_struct: *mut ia_ps_dec_struct,
    mut ptr_ps_tables: *mut ia_ps_tables_struct,
    mut noQmfChans: UWORD32,
    mut num_sub_samples: UWORD32,
    mut ps_mode: WORD32,
) -> WORD32 {
    let mut i: UWORD32 = 0;
    let mut ptr_ps_dec: *mut ia_ps_dec_struct = ptr_ps_dec_struct;
    ptr_ps_dec = ptr_ps_dec_struct;
    (*ptr_ps_dec).num_sub_samples = num_sub_samples as WORD32;
    (*ptr_ps_dec).num_chans = noQmfChans as WORD32;
    (*ptr_ps_dec).ps_mode = ps_mode;
    (*ptr_ps_dec).ps_data_present = 0 as core::ffi::c_int as FLAG;
    (*ptr_ps_dec).enable_iid = 0 as core::ffi::c_int as FLAG;
    (*ptr_ps_dec).iid_mode = 0 as WORD16;
    (*ptr_ps_dec).enable_icc = 0 as core::ffi::c_int as FLAG;
    (*ptr_ps_dec).icc_mode = 0 as WORD16;
    (*ptr_ps_dec).enable_ext = 0 as core::ffi::c_int as FLAG;
    (*ptr_ps_dec).use_pca_rot_flg = 0 as core::ffi::c_int as WORD32;
    (*ptr_ps_dec).freq_res_ipd = 0 as core::ffi::c_int as WORD32;
    (*ptr_ps_dec).use_34_st_bands = 0 as core::ffi::c_int as WORD32;
    (*ptr_ps_dec).use_34_st_bands_prev = 0 as core::ffi::c_int as WORD32;
    (*ptr_ps_dec).str_flt_hybrid20.frame_size = (*ptr_ps_dec).num_sub_samples;
    (*ptr_ps_dec).str_flt_hybrid20.num_qmf_bands = NUM_QMF_BANDS_IN_HYBRID20 as WORD32;
    (*ptr_ps_dec).str_flt_hybrid20.ptr_resol = &mut *((*ptr_ps_tables).band_res_hyb20)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
    (*ptr_ps_dec).str_flt_hybrid20.ptr_work_re = &mut *((*ptr_ps_dec).hyb_work_re_20)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
    (*ptr_ps_dec).str_flt_hybrid20.ptr_work_im = &mut *((*ptr_ps_dec).hyb_work_im_20)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
    (*ptr_ps_dec).str_flt_hybrid20.ptr_qmf_buf_re = ((*ptr_ps_dec).hyb_qmf_buf_re_20)
        .as_mut_ptr() as *mut [FLOAT32; 12];
    (*ptr_ps_dec).str_flt_hybrid20.ptr_qmf_buf_im = ((*ptr_ps_dec).hyb_qmf_buf_im_20)
        .as_mut_ptr() as *mut [FLOAT32; 12];
    (*ptr_ps_dec).str_flt_hybrid20.ptr_temp_re = ((*ptr_ps_dec).hyb_temp_re_20)
        .as_mut_ptr() as *mut [FLOAT32; 64];
    (*ptr_ps_dec).str_flt_hybrid20.ptr_temp_im = ((*ptr_ps_dec).hyb_temp_im_20)
        .as_mut_ptr() as *mut [FLOAT32; 64];
    (*ptr_ps_dec).str_flt_hybrid34.frame_size = (*ptr_ps_dec).num_sub_samples;
    (*ptr_ps_dec).str_flt_hybrid34.num_qmf_bands = NUM_QMF_BANDS_IN_HYBRID34 as WORD32;
    (*ptr_ps_dec).str_flt_hybrid34.ptr_resol = &mut *((*ptr_ps_tables).band_res_hyb34)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut WORD16;
    (*ptr_ps_dec).str_flt_hybrid34.ptr_work_re = &mut *((*ptr_ps_dec).hyb_work_re_34)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
    (*ptr_ps_dec).str_flt_hybrid34.ptr_work_im = &mut *((*ptr_ps_dec).hyb_work_im_34)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
    (*ptr_ps_dec).str_flt_hybrid34.ptr_qmf_buf_re = ((*ptr_ps_dec).hyb_qmf_buf_re_34)
        .as_mut_ptr() as *mut [FLOAT32; 12];
    (*ptr_ps_dec).str_flt_hybrid34.ptr_qmf_buf_im = ((*ptr_ps_dec).hyb_qmf_buf_im_34)
        .as_mut_ptr() as *mut [FLOAT32; 12];
    (*ptr_ps_dec).str_flt_hybrid34.ptr_temp_re = ((*ptr_ps_dec).hyb_temp_re_34)
        .as_mut_ptr() as *mut [FLOAT32; 64];
    (*ptr_ps_dec).str_flt_hybrid34.ptr_temp_im = ((*ptr_ps_dec).hyb_temp_im_34)
        .as_mut_ptr() as *mut [FLOAT32; 64];
    (*ptr_ps_dec).delay_buf_idx = 0 as WORD16;
    i = 0 as UWORD32;
    while i < NUM_OF_QUAD_MIRROR_FILTER_CHNLS as UWORD32 {
        (*ptr_ps_dec).delay_qmf_delay_buf_idx[i as usize] = 0 as core::ffi::c_int
            as WORD32;
        (*ptr_ps_dec).delay_qmf_delay_num_samp[i as usize] = (*ptr_ps_tables)
            .qmf_delay_idx_tbl[i as usize];
        i = i.wrapping_add(1);
    }
    i = 0 as UWORD32;
    while i < NUM_HI_RES_BINS as UWORD32 {
        (*ptr_ps_dec).h11_re_prev[i as usize] = 1.0f32 as FLOAT32;
        (*ptr_ps_dec).h12_re_prev[i as usize] = 1.0f32 as FLOAT32;
        i = i.wrapping_add(1);
    }
    memset(
        ((*ptr_ps_dec).h11_im_prev).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[FLOAT32; 34]>() as size_t,
    );
    memset(
        ((*ptr_ps_dec).h12_im_prev).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[FLOAT32; 34]>() as size_t,
    );
    memset(
        ((*ptr_ps_dec).h21_re_prev).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[FLOAT32; 34]>() as size_t,
    );
    memset(
        ((*ptr_ps_dec).h22_re_prev).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[FLOAT32; 34]>() as size_t,
    );
    memset(
        ((*ptr_ps_dec).h21_im_prev).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[FLOAT32; 34]>() as size_t,
    );
    memset(
        ((*ptr_ps_dec).h22_im_prev).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[FLOAT32; 34]>() as size_t,
    );
    memset(
        ((*ptr_ps_dec).ipd_idx_map_1).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[WORD32; 17]>() as size_t,
    );
    memset(
        ((*ptr_ps_dec).opd_idx_map_1).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[WORD32; 17]>() as size_t,
    );
    memset(
        ((*ptr_ps_dec).ipd_idx_map_2).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[WORD32; 17]>() as size_t,
    );
    memset(
        ((*ptr_ps_dec).opd_idx_map_2).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[WORD32; 17]>() as size_t,
    );
    i = 0 as UWORD32;
    while i < NUM_HI_RES_BINS as UWORD32 {
        (*ptr_ps_dec).peak_decay_fast_bin[i as usize] = 0.0f32 as FLOAT32;
        (*ptr_ps_dec).prev_nrg_bin[i as usize] = 0.0f32 as FLOAT32;
        (*ptr_ps_dec).prev_peak_diff_bin[i as usize] = 0.0f32 as FLOAT32;
        i = i.wrapping_add(1);
    }
    memset(
        ((*ptr_ps_dec).qmf_delay_buf_re).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[[FLOAT32; 64]; 14]>() as size_t,
    );
    memset(
        ((*ptr_ps_dec).qmf_delay_buf_im).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[[FLOAT32; 64]; 14]>() as size_t,
    );
    memset(
        ((*ptr_ps_dec).sub_qmf_delay_buf_re).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[[FLOAT32; 64]; 14]>() as size_t,
    );
    memset(
        ((*ptr_ps_dec).sub_qmf_delay_buf_im).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[[FLOAT32; 64]; 14]>() as size_t,
    );
    i = 0 as UWORD32;
    while i < NUM_SER_AP_LINKS as UWORD32 {
        memset(
            &mut *(*(*((*ptr_ps_dec).ser_qmf_delay_buf_re)
                .as_mut_ptr()
                .offset(i as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<[[FLOAT32; 64]; 5]>() as size_t,
        );
        memset(
            &mut *(*(*((*ptr_ps_dec).ser_qmf_delay_buf_im)
                .as_mut_ptr()
                .offset(i as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<[[FLOAT32; 64]; 5]>() as size_t,
        );
        memset(
            &mut *(*(*((*ptr_ps_dec).ser_sub_qmf_dealy_buf_re)
                .as_mut_ptr()
                .offset(i as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<[[FLOAT32; 64]; 5]>() as size_t,
        );
        memset(
            &mut *(*(*((*ptr_ps_dec).ser_sub_qmf_dealy_buf_im)
                .as_mut_ptr()
                .offset(i as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<[[FLOAT32; 64]; 5]>() as size_t,
        );
        i = i.wrapping_add(1);
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_esbr_apply_ps(
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
    mut pp_qmf_buf_re_left: *mut *mut FLOAT32,
    mut pp_qmf_buf_im_left: *mut *mut FLOAT32,
    mut pp_qmf_buf_re_right: *mut *mut FLOAT32,
    mut pp_qmf_buf_im_right: *mut *mut FLOAT32,
    mut usb: WORD32,
    mut ptr_ps_tables: *mut ia_ps_tables_struct,
    mut num_time_slot: WORD32,
) -> VOID {
    let mut sb: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut max_num_column: WORD32 = 0;
    if num_time_slot == 15 as core::ffi::c_int {
        max_num_column = MAX_NUM_COLUMNS_960 as WORD32;
    } else {
        max_num_column = MAX_NUM_COLUMNS as WORD32;
    }
    if (*ptr_ps_dec).use_34_st_bands != 0 {
        (*ptr_ps_dec).ptr_group_borders = &mut *((*ptr_ps_tables).group_borders_34_tbl)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
        (*ptr_ps_dec).ptr_bins_group_map = &mut *((*ptr_ps_tables).bin_group_map_34)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
        (*ptr_ps_dec).ptr_hybrid = &mut (*ptr_ps_dec).str_flt_hybrid34;
        (*ptr_ps_dec).num_groups = NUM_IID_GROUPS_HI_RES as WORD32;
        (*ptr_ps_dec).num_sub_qmf_groups = SUBQMF_GROUPS_HI_RES as WORD32;
        (*ptr_ps_dec).num_bins = NUM_HI_RES_BINS as WORD32;
        (*ptr_ps_dec).first_delay_gr = SUBQMF_GROUPS_HI_RES as WORD32;
    } else {
        (*ptr_ps_dec).ptr_group_borders = &mut *((*ptr_ps_tables).group_borders_20_tbl)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
        (*ptr_ps_dec).ptr_bins_group_map = &mut *((*ptr_ps_tables).bin_group_map_20)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
        (*ptr_ps_dec).ptr_hybrid = &mut (*ptr_ps_dec).str_flt_hybrid20;
        (*ptr_ps_dec).num_groups = NUM_IID_GROUPS as WORD32;
        (*ptr_ps_dec).num_sub_qmf_groups = SUBQMF_GROUPS as WORD32;
        (*ptr_ps_dec).num_bins = NUM_MID_RES_BINS as WORD32;
        (*ptr_ps_dec).first_delay_gr = SUBQMF_GROUPS as WORD32;
    }
    sb = usb;
    while sb < (*ptr_ps_dec).num_chans {
        i = 0 as core::ffi::c_int as WORD32;
        while i < NUM_SER_AP_LINKS {
            k = 0 as core::ffi::c_int as WORD32;
            while k < (*ptr_ps_dec).delay_sample_ser[i as usize] as core::ffi::c_int {
                (*ptr_ps_dec)
                    .ser_qmf_delay_buf_re[i as usize][k as usize][sb as usize] = 0
                    as core::ffi::c_int as FLOAT32;
                (*ptr_ps_dec)
                    .ser_qmf_delay_buf_im[i as usize][k as usize][sb as usize] = 0
                    as core::ffi::c_int as FLOAT32;
                k += 1;
            }
            i += 1;
        }
        k = 0 as core::ffi::c_int as WORD32;
        while k < HIGH_DEL {
            (*ptr_ps_dec).qmf_delay_buf_re[k as usize][sb as usize] = 0
                as core::ffi::c_int as FLOAT32;
            (*ptr_ps_dec).qmf_delay_buf_im[k as usize][sb as usize] = 0
                as core::ffi::c_int as FLOAT32;
            k += 1;
        }
        sb += 1;
    }
    ixheaacd_hyb_anal(
        pp_qmf_buf_re_left as *mut *const FLOAT32,
        pp_qmf_buf_im_left as *mut *const FLOAT32,
        ptr_ps_dec,
        ptr_ps_tables,
        0 as WORD32,
    );
    ixheaacd_hyb_anal(
        pp_qmf_buf_re_left as *mut *const FLOAT32,
        pp_qmf_buf_im_left as *mut *const FLOAT32,
        ptr_ps_dec,
        ptr_ps_tables,
        1 as WORD32,
    );
    if (*ptr_ps_dec).use_34_st_bands == 0 {
        let mut k_0: WORD32 = 0;
        k_0 = 0 as core::ffi::c_int as WORD32;
        while k_0 < (*ptr_ps_dec).num_sub_samples {
            (*ptr_ps_dec).hyb_left_re[k_0 as usize][3 as core::ffi::c_int as usize]
                += (*ptr_ps_dec)
                    .hyb_left_re[k_0 as usize][4 as core::ffi::c_int as usize];
            (*ptr_ps_dec).hyb_left_im[k_0 as usize][3 as core::ffi::c_int as usize]
                += (*ptr_ps_dec)
                    .hyb_left_im[k_0 as usize][4 as core::ffi::c_int as usize];
            (*ptr_ps_dec).hyb_left_re[k_0 as usize][4 as core::ffi::c_int as usize] = 0.0f32;
            (*ptr_ps_dec).hyb_left_im[k_0 as usize][4 as core::ffi::c_int as usize] = 0.0f32;
            (*ptr_ps_dec).hyb_left_re[k_0 as usize][2 as core::ffi::c_int as usize]
                += (*ptr_ps_dec)
                    .hyb_left_re[k_0 as usize][5 as core::ffi::c_int as usize];
            (*ptr_ps_dec).hyb_left_im[k_0 as usize][2 as core::ffi::c_int as usize]
                += (*ptr_ps_dec)
                    .hyb_left_im[k_0 as usize][5 as core::ffi::c_int as usize];
            (*ptr_ps_dec).hyb_left_re[k_0 as usize][5 as core::ffi::c_int as usize] = 0.0f32;
            (*ptr_ps_dec).hyb_left_im[k_0 as usize][5 as core::ffi::c_int as usize] = 0.0f32;
            k_0 += 1;
        }
    }
    if (*ptr_ps_dec).ps_mode as core::ffi::c_int & 0x80 as core::ffi::c_int != 0 {
        let mut i_0: WORD32 = 0;
        let mut j: WORD32 = 0;
        i_0 = 0 as core::ffi::c_int as WORD32;
        while i_0 < max_num_column {
            j = 0 as core::ffi::c_int as WORD32;
            while j < NUM_OF_QUAD_MIRROR_FILTER_CHNLS {
                *(*pp_qmf_buf_im_right.offset(i_0 as isize)).offset(j as isize) = *(*pp_qmf_buf_im_left
                    .offset(i_0 as isize))
                    .offset(j as isize);
                *(*pp_qmf_buf_re_right.offset(i_0 as isize)).offset(j as isize) = *(*pp_qmf_buf_re_left
                    .offset(i_0 as isize))
                    .offset(j as isize);
                j += 1;
            }
            i_0 += 1;
        }
        i_0 = 0 as core::ffi::c_int as WORD32;
        while i_0 < max_num_column {
            j = 0 as core::ffi::c_int as WORD32;
            while j < NUM_SUB_QMF_CHANNELS_HI_RES {
                (*ptr_ps_dec).hyb_right_re[i_0 as usize][j as usize] = (*ptr_ps_dec)
                    .hyb_left_re[i_0 as usize][j as usize];
                (*ptr_ps_dec).hyb_right_im[i_0 as usize][j as usize] = (*ptr_ps_dec)
                    .hyb_left_im[i_0 as usize][j as usize];
                j += 1;
            }
            i_0 += 1;
        }
    } else {
        if (*ptr_ps_dec).ps_mode as core::ffi::c_int & 0x2 as core::ffi::c_int != 0 {
            let mut i_1: WORD32 = 0;
            let mut j_0: WORD32 = 0;
            i_1 = 0 as core::ffi::c_int as WORD32;
            while i_1 < max_num_column {
                j_0 = 0 as core::ffi::c_int as WORD32;
                while j_0 < NUM_OF_QUAD_MIRROR_FILTER_CHNLS {
                    *(*pp_qmf_buf_im_right.offset(i_1 as isize)).offset(j_0 as isize) = 0.0f32;
                    *(*pp_qmf_buf_re_right.offset(i_1 as isize)).offset(j_0 as isize) = 0.0f32;
                    j_0 += 1;
                }
                i_1 += 1;
            }
            i_1 = 0 as core::ffi::c_int as WORD32;
            while i_1 < max_num_column {
                j_0 = 0 as core::ffi::c_int as WORD32;
                while j_0 < NUM_SUB_QMF_CHANNELS_HI_RES {
                    (*ptr_ps_dec).hyb_right_re[i_1 as usize][j_0 as usize] = 0.0f32;
                    (*ptr_ps_dec).hyb_right_im[i_1 as usize][j_0 as usize] = 0.0f32;
                    j_0 += 1;
                }
                i_1 += 1;
            }
        } else {
            ixheaacd_esbr_ps_de_correlate(
                ptr_ps_dec,
                pp_qmf_buf_re_left,
                pp_qmf_buf_im_left,
                pp_qmf_buf_re_right,
                pp_qmf_buf_im_right,
                ptr_ps_tables,
            );
        }
        if (*ptr_ps_dec).ps_mode as core::ffi::c_int & 0x40 as core::ffi::c_int == 0 {
            ixheaacd_esbr_ps_apply_rotation(
                ptr_ps_dec,
                pp_qmf_buf_re_left,
                pp_qmf_buf_im_left,
                pp_qmf_buf_re_right,
                pp_qmf_buf_im_right,
                ptr_ps_tables,
            );
        }
    }
    ixheaacd_hyb_synth(
        ((*ptr_ps_dec).hyb_left_re).as_mut_ptr() as *mut [FLOAT32; 32],
        ((*ptr_ps_dec).hyb_left_im).as_mut_ptr() as *mut [FLOAT32; 32],
        pp_qmf_buf_re_left,
        pp_qmf_buf_im_left,
        (*ptr_ps_dec).ptr_hybrid,
    );
    ixheaacd_hyb_synth(
        ((*ptr_ps_dec).hyb_right_re).as_mut_ptr() as *mut [FLOAT32; 32],
        ((*ptr_ps_dec).hyb_right_im).as_mut_ptr() as *mut [FLOAT32; 32],
        pp_qmf_buf_re_right,
        pp_qmf_buf_im_right,
        (*ptr_ps_dec).ptr_hybrid,
    );
    (*ptr_ps_dec).use_34_st_bands_prev = (*ptr_ps_dec).use_34_st_bands;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_esbr_ps_de_correlate(
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
    mut pp_qmf_buf_re_left: *mut *mut FLOAT32,
    mut pp_qmf_buf_im_left: *mut *mut FLOAT32,
    mut pp_qmf_buf_re_right: *mut *mut FLOAT32,
    mut pp_qmf_buf_im_right: *mut *mut FLOAT32,
    mut ptr_ps_tables: *mut ia_ps_tables_struct,
) -> VOID {
    let mut sb: WORD32 = 0;
    let mut maxsb: WORD32 = 0;
    let mut gr: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut m: WORD32 = 0;
    let mut l_delay: WORD32 = 0 as WORD32;
    let mut l_ser_delay_arr: [WORD32; 3] = [0 as core::ffi::c_int; 3];
    let mut re_left: FLOAT32 = 0.;
    let mut im_left: FLOAT32 = 0.;
    let mut peak_diff: FLOAT32 = 0.;
    let mut nrg: FLOAT32 = 0.;
    let mut trans_ratio: FLOAT32 = 0.;
    let mut pp_hyb_left_re: *mut [FLOAT32; 32] = 0 as *mut [FLOAT32; 32];
    let mut pp_hyb_left_im: *mut [FLOAT32; 32] = 0 as *mut [FLOAT32; 32];
    let mut pp_hyb_right_re: *mut [FLOAT32; 32] = 0 as *mut [FLOAT32; 32];
    let mut pp_hyb_right_im: *mut [FLOAT32; 32] = 0 as *mut [FLOAT32; 32];
    let mut ppp_ser_sub_qmf_dealy_buf_re: *mut [[FLOAT32; 64]; 5] = 0
        as *mut [[FLOAT32; 64]; 5];
    let mut ppp_ser_sub_qmf_dealy_buf_im: *mut [[FLOAT32; 64]; 5] = 0
        as *mut [[FLOAT32; 64]; 5];
    let mut pp_sub_qmf_delay_buf_re: *mut [FLOAT32; 64] = 0 as *mut [FLOAT32; 64];
    let mut pp_sub_qmf_delay_buf_im: *mut [FLOAT32; 64] = 0 as *mut [FLOAT32; 64];
    let mut pp_frac_delay_phase_fac_re: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut pp_frac_delay_phase_fac_im: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut pp_frac_delay_phase_fac_ser_re: *mut [FLOAT32; 3] = 0 as *mut [FLOAT32; 3];
    let mut pp_frac_delay_phase_fac_ser_im: *mut [FLOAT32; 3] = 0 as *mut [FLOAT32; 3];
    let mut p_delay_qmf_delay_num_samp: *mut WORD32 = 0 as *mut WORD32;
    let mut p_delay_qmf_delay_buf_idx: *mut WORD32 = 0 as *mut WORD32;
    let mut pow_arr: [[FLOAT32; 34]; 32] = [[0.; 34]; 32];
    let mut trans_ratio_arr: [[FLOAT32; 34]; 32] = [[0.; 34]; 32];
    let mut bin: WORD32 = 0;
    let mut decay_cutoff: FLOAT32 = 0.;
    pp_hyb_left_re = ((*ptr_ps_dec).hyb_left_re).as_mut_ptr() as *mut [FLOAT32; 32];
    pp_hyb_left_im = ((*ptr_ps_dec).hyb_left_im).as_mut_ptr() as *mut [FLOAT32; 32];
    pp_hyb_right_re = ((*ptr_ps_dec).hyb_right_re).as_mut_ptr() as *mut [FLOAT32; 32];
    pp_hyb_right_im = ((*ptr_ps_dec).hyb_right_im).as_mut_ptr() as *mut [FLOAT32; 32];
    ppp_ser_sub_qmf_dealy_buf_re = ((*ptr_ps_dec).ser_sub_qmf_dealy_buf_re).as_mut_ptr()
        as *mut [[FLOAT32; 64]; 5];
    ppp_ser_sub_qmf_dealy_buf_im = ((*ptr_ps_dec).ser_sub_qmf_dealy_buf_im).as_mut_ptr()
        as *mut [[FLOAT32; 64]; 5];
    pp_sub_qmf_delay_buf_re = ((*ptr_ps_dec).sub_qmf_delay_buf_re).as_mut_ptr()
        as *mut [FLOAT32; 64];
    pp_sub_qmf_delay_buf_im = ((*ptr_ps_dec).sub_qmf_delay_buf_im).as_mut_ptr()
        as *mut [FLOAT32; 64];
    if (*ptr_ps_dec).use_34_st_bands != (*ptr_ps_dec).use_34_st_bands_prev {
        if (*ptr_ps_dec).use_34_st_bands != 0 {
            let mut i: WORD32 = 0;
            i = 0 as core::ffi::c_int as WORD32;
            while i < NUM_SER_AP_LINKS {
                memset(
                    &mut *(*(*((*ptr_ps_dec).ser_qmf_delay_buf_re)
                        .as_mut_ptr()
                        .offset(i as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                        as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    ::core::mem::size_of::<[[FLOAT32; 64]; 5]>() as size_t,
                );
                memset(
                    &mut *(*(*((*ptr_ps_dec).ser_qmf_delay_buf_im)
                        .as_mut_ptr()
                        .offset(i as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                        as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    ::core::mem::size_of::<[[FLOAT32; 64]; 5]>() as size_t,
                );
                memset(
                    &mut *(*(*((*ptr_ps_dec).ser_sub_qmf_dealy_buf_re)
                        .as_mut_ptr()
                        .offset(i as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                        as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    ::core::mem::size_of::<[[FLOAT32; 64]; 5]>() as size_t,
                );
                memset(
                    &mut *(*(*((*ptr_ps_dec).ser_sub_qmf_dealy_buf_im)
                        .as_mut_ptr()
                        .offset(i as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                        as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    ::core::mem::size_of::<[[FLOAT32; 64]; 5]>() as size_t,
                );
                i += 1;
            }
            return;
        } else {
            let mut i_0: WORD32 = 0;
            i_0 = 0 as core::ffi::c_int as WORD32;
            while i_0 < NUM_SER_AP_LINKS {
                memset(
                    &mut *(*(*((*ptr_ps_dec).ser_qmf_delay_buf_re)
                        .as_mut_ptr()
                        .offset(i_0 as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                        as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    ::core::mem::size_of::<[[FLOAT32; 64]; 5]>() as size_t,
                );
                memset(
                    &mut *(*(*((*ptr_ps_dec).ser_qmf_delay_buf_im)
                        .as_mut_ptr()
                        .offset(i_0 as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                        as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    ::core::mem::size_of::<[[FLOAT32; 64]; 5]>() as size_t,
                );
                memset(
                    &mut *(*(*((*ptr_ps_dec).ser_sub_qmf_dealy_buf_re)
                        .as_mut_ptr()
                        .offset(i_0 as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                        as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    ::core::mem::size_of::<[[FLOAT32; 64]; 5]>() as size_t,
                );
                memset(
                    &mut *(*(*((*ptr_ps_dec).ser_sub_qmf_dealy_buf_im)
                        .as_mut_ptr()
                        .offset(i_0 as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32
                        as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    ::core::mem::size_of::<[[FLOAT32; 64]; 5]>() as size_t,
                );
                i_0 += 1;
            }
            return;
        }
    }
    if (*ptr_ps_dec).use_34_st_bands != 0 {
        pp_frac_delay_phase_fac_re = ((*ptr_ps_tables)
            .frac_delay_phase_fac_qmf_sub_re_34)
            .as_mut_ptr();
        pp_frac_delay_phase_fac_im = ((*ptr_ps_tables)
            .frac_delay_phase_fac_qmf_sub_im_34)
            .as_mut_ptr();
        pp_frac_delay_phase_fac_ser_re = ((*ptr_ps_tables)
            .frac_delay_phase_fac_ser_qmf_sub_re_34)
            .as_mut_ptr() as *mut [FLOAT32; 3];
        pp_frac_delay_phase_fac_ser_im = ((*ptr_ps_tables)
            .frac_delay_phase_fac_ser_qmf_sub_im_34)
            .as_mut_ptr() as *mut [FLOAT32; 3];
    } else {
        pp_frac_delay_phase_fac_re = ((*ptr_ps_tables)
            .frac_delay_phase_fac_qmf_sub_re_20)
            .as_mut_ptr();
        pp_frac_delay_phase_fac_im = ((*ptr_ps_tables)
            .frac_delay_phase_fac_qmf_sub_im_20)
            .as_mut_ptr();
        pp_frac_delay_phase_fac_ser_re = ((*ptr_ps_tables)
            .frac_delay_phase_fac_ser_qmf_sub_re_20)
            .as_mut_ptr() as *mut [FLOAT32; 3];
        pp_frac_delay_phase_fac_ser_im = ((*ptr_ps_tables)
            .frac_delay_phase_fac_ser_qmf_sub_im_20)
            .as_mut_ptr() as *mut [FLOAT32; 3];
    }
    k = 0 as core::ffi::c_int as WORD32;
    while k < 32 as core::ffi::c_int {
        bin = 0 as core::ffi::c_int as WORD32;
        while bin < NUM_HI_RES_BINS {
            pow_arr[k as usize][bin as usize] = 0 as core::ffi::c_int as FLOAT32;
            bin += 1;
        }
        k += 1;
    }
    gr = 0 as core::ffi::c_int as WORD32;
    while gr < (*ptr_ps_dec).num_sub_qmf_groups {
        bin = !NEGATE_IPD_MASK & *((*ptr_ps_dec).ptr_bins_group_map).offset(gr as isize);
        maxsb = (*((*ptr_ps_dec).ptr_group_borders).offset(gr as isize)
            + 1 as core::ffi::c_int) as WORD32;
        sb = *((*ptr_ps_dec).ptr_group_borders).offset(gr as isize);
        while sb < maxsb {
            k = (*ptr_ps_dec).border_position[0 as core::ffi::c_int as usize] as WORD32;
            while k
                < (*ptr_ps_dec).border_position[(*ptr_ps_dec).num_env as usize]
                    as core::ffi::c_int
            {
                im_left = (*pp_hyb_left_re.offset(k as isize))[sb as usize];
                re_left = (*pp_hyb_left_im.offset(k as isize))[sb as usize];
                pow_arr[k as usize][bin as usize]
                    += im_left * im_left + re_left * re_left;
                k += 1;
            }
            sb += 1;
        }
        gr += 1;
    }
    while gr < (*ptr_ps_dec).num_groups {
        bin = !NEGATE_IPD_MASK & *((*ptr_ps_dec).ptr_bins_group_map).offset(gr as isize);
        maxsb = *((*ptr_ps_dec).ptr_group_borders)
            .offset((gr as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
        sb = *((*ptr_ps_dec).ptr_group_borders).offset(gr as isize);
        while sb < maxsb {
            k = (*ptr_ps_dec).border_position[0 as core::ffi::c_int as usize] as WORD32;
            while k
                < (*ptr_ps_dec).border_position[(*ptr_ps_dec).num_env as usize]
                    as core::ffi::c_int
            {
                im_left = *(*pp_qmf_buf_re_left.offset(k as isize)).offset(sb as isize);
                re_left = *(*pp_qmf_buf_im_left.offset(k as isize)).offset(sb as isize);
                pow_arr[k as usize][bin as usize]
                    += im_left * im_left + re_left * re_left;
                k += 1;
            }
            sb += 1;
        }
        gr += 1;
    }
    bin = 0 as core::ffi::c_int as WORD32;
    while bin < (*ptr_ps_dec).num_bins {
        k = (*ptr_ps_dec).border_position[0 as core::ffi::c_int as usize] as WORD32;
        while k
            < (*ptr_ps_dec).border_position[(*ptr_ps_dec).num_env as usize]
                as core::ffi::c_int
        {
            let mut q: FLOAT32 = 1.5f32;
            (*ptr_ps_dec).peak_decay_fast_bin[bin as usize] *= PEAK_DECAY_FACTOR_FAST;
            if (*ptr_ps_dec).peak_decay_fast_bin[bin as usize]
                < pow_arr[k as usize][bin as usize]
            {
                (*ptr_ps_dec).peak_decay_fast_bin[bin as usize] = pow_arr[k
                    as usize][bin as usize];
            }
            peak_diff = (*ptr_ps_dec).prev_peak_diff_bin[bin as usize];
            peak_diff
                += (INIT_FILT_COEFF
                    * ((*ptr_ps_dec).peak_decay_fast_bin[bin as usize]
                        - pow_arr[k as usize][bin as usize]
                        - (*ptr_ps_dec).prev_peak_diff_bin[bin as usize]))
                    as core::ffi::c_float;
            (*ptr_ps_dec).prev_peak_diff_bin[bin as usize] = peak_diff;
            nrg = (*ptr_ps_dec).prev_nrg_bin[bin as usize];
            nrg
                += (INIT_FILT_COEFF
                    * (pow_arr[k as usize][bin as usize]
                        - (*ptr_ps_dec).prev_nrg_bin[bin as usize]))
                    as core::ffi::c_float;
            (*ptr_ps_dec).prev_nrg_bin[bin as usize] = nrg;
            if q * peak_diff <= nrg {
                trans_ratio_arr[k as usize][bin as usize] = 1.0f32 as FLOAT32;
            } else {
                trans_ratio_arr[k as usize][bin as usize] = nrg / (q * peak_diff);
            }
            k += 1;
        }
        bin += 1;
    }
    if (*ptr_ps_dec).use_34_st_bands != 0 {
        decay_cutoff = DECAY_CUTOFF_HI_RES as FLOAT32;
    } else {
        decay_cutoff = DECAY_CUTOFF as FLOAT32;
    }
    gr = 0 as core::ffi::c_int as WORD32;
    while gr < (*ptr_ps_dec).num_sub_qmf_groups {
        maxsb = (*((*ptr_ps_dec).ptr_group_borders).offset(gr as isize)
            + 1 as core::ffi::c_int) as WORD32;
        sb = *((*ptr_ps_dec).ptr_group_borders).offset(gr as isize);
        while sb < maxsb {
            let mut decay_scale_factor: FLOAT32 = 0.;
            decay_scale_factor = 1.0f32 as FLOAT32;
            decay_scale_factor = (if decay_scale_factor > 0.0f32 {
                decay_scale_factor as core::ffi::c_float
            } else {
                0.0f32
            }) as FLOAT32;
            l_delay = (*ptr_ps_dec).delay_buf_idx as WORD32;
            k = 0 as core::ffi::c_int as WORD32;
            while k < NUM_SER_AP_LINKS {
                l_ser_delay_arr[k as usize] = (*ptr_ps_dec).delay_buf_idx_ser[k as usize]
                    as WORD32;
                k += 1;
            }
            k = (*ptr_ps_dec).border_position[0 as core::ffi::c_int as usize] as WORD32;
            while k
                < (*ptr_ps_dec).border_position[(*ptr_ps_dec).num_env as usize]
                    as core::ffi::c_int
            {
                let mut real: FLOAT32 = 0.;
                let mut imag: FLOAT32 = 0.;
                let mut real0: FLOAT32 = 0.;
                let mut imag0: FLOAT32 = 0.;
                let mut r_r0: FLOAT32 = 0.;
                let mut i_r0: FLOAT32 = 0.;
                im_left = (*pp_hyb_left_re.offset(k as isize))[sb as usize];
                re_left = (*pp_hyb_left_im.offset(k as isize))[sb as usize];
                real0 = (*pp_sub_qmf_delay_buf_re.offset(l_delay as isize))[sb as usize];
                imag0 = (*pp_sub_qmf_delay_buf_im.offset(l_delay as isize))[sb as usize];
                (*pp_sub_qmf_delay_buf_re.offset(l_delay as isize))[sb as usize] = im_left;
                (*pp_sub_qmf_delay_buf_im.offset(l_delay as isize))[sb as usize] = re_left;
                real = real0 * *pp_frac_delay_phase_fac_re.offset(sb as isize)
                    - imag0 * *pp_frac_delay_phase_fac_im.offset(sb as isize);
                imag = real0 * *pp_frac_delay_phase_fac_im.offset(sb as isize)
                    + imag0 * *pp_frac_delay_phase_fac_re.offset(sb as isize);
                r_r0 = real;
                i_r0 = imag;
                m = 0 as core::ffi::c_int as WORD32;
                while m < NUM_SER_AP_LINKS {
                    real0 = (*ppp_ser_sub_qmf_dealy_buf_re
                        .offset(
                            m as isize,
                        ))[l_ser_delay_arr[m as usize] as usize][sb as usize];
                    imag0 = (*ppp_ser_sub_qmf_dealy_buf_im
                        .offset(
                            m as isize,
                        ))[l_ser_delay_arr[m as usize] as usize][sb as usize];
                    real = real0
                        * (*pp_frac_delay_phase_fac_ser_re
                            .offset(sb as isize))[m as usize]
                        - imag0
                            * (*pp_frac_delay_phase_fac_ser_im
                                .offset(sb as isize))[m as usize];
                    imag = real0
                        * (*pp_frac_delay_phase_fac_ser_im
                            .offset(sb as isize))[m as usize]
                        + imag0
                            * (*pp_frac_delay_phase_fac_ser_re
                                .offset(sb as isize))[m as usize];
                    real
                        += -decay_scale_factor
                            * (*ptr_ps_tables).all_pass_link_decay_ser[m as usize]
                            * r_r0;
                    imag
                        += -decay_scale_factor
                            * (*ptr_ps_tables).all_pass_link_decay_ser[m as usize]
                            * i_r0;
                    (*ppp_ser_sub_qmf_dealy_buf_re
                        .offset(
                            m as isize,
                        ))[l_ser_delay_arr[m as usize] as usize][sb as usize] = r_r0
                        + decay_scale_factor
                            * (*ptr_ps_tables).all_pass_link_decay_ser[m as usize]
                            * real;
                    (*ppp_ser_sub_qmf_dealy_buf_im
                        .offset(
                            m as isize,
                        ))[l_ser_delay_arr[m as usize] as usize][sb as usize] = i_r0
                        + decay_scale_factor
                            * (*ptr_ps_tables).all_pass_link_decay_ser[m as usize]
                            * imag;
                    r_r0 = real;
                    i_r0 = imag;
                    m += 1;
                }
                bin = !NEGATE_IPD_MASK
                    & *((*ptr_ps_dec).ptr_bins_group_map).offset(gr as isize);
                trans_ratio = trans_ratio_arr[k as usize][bin as usize];
                (*pp_hyb_right_re.offset(k as isize))[sb as usize] = trans_ratio * r_r0;
                (*pp_hyb_right_im.offset(k as isize))[sb as usize] = trans_ratio * i_r0;
                l_delay += 1;
                if l_delay >= DEL_ALL_PASS {
                    l_delay = 0 as core::ffi::c_int as WORD32;
                }
                m = 0 as core::ffi::c_int as WORD32;
                while m < NUM_SER_AP_LINKS {
                    l_ser_delay_arr[m as usize] += 1;
                    if l_ser_delay_arr[m as usize]
                        >= (*ptr_ps_dec).delay_sample_ser[m as usize] as core::ffi::c_int
                    {
                        l_ser_delay_arr[m as usize] = 0 as core::ffi::c_int as WORD32;
                    }
                    m += 1;
                }
                k += 1;
            }
            sb += 1;
        }
        gr += 1;
    }
    ppp_ser_sub_qmf_dealy_buf_re = ((*ptr_ps_dec).ser_qmf_delay_buf_re).as_mut_ptr()
        as *mut [[FLOAT32; 64]; 5];
    ppp_ser_sub_qmf_dealy_buf_im = ((*ptr_ps_dec).ser_qmf_delay_buf_im).as_mut_ptr()
        as *mut [[FLOAT32; 64]; 5];
    pp_sub_qmf_delay_buf_re = ((*ptr_ps_dec).qmf_delay_buf_re).as_mut_ptr()
        as *mut [FLOAT32; 64];
    pp_sub_qmf_delay_buf_im = ((*ptr_ps_dec).qmf_delay_buf_im).as_mut_ptr()
        as *mut [FLOAT32; 64];
    pp_frac_delay_phase_fac_re = ((*ptr_ps_tables).qmf_fract_delay_phase_factor_re)
        .as_mut_ptr();
    pp_frac_delay_phase_fac_im = ((*ptr_ps_tables).qmf_fract_delay_phase_factor_im)
        .as_mut_ptr();
    pp_frac_delay_phase_fac_ser_re = ((*ptr_ps_tables)
        .qmf_ser_fract_delay_phase_factor_re)
        .as_mut_ptr() as *mut [FLOAT32; 3];
    pp_frac_delay_phase_fac_ser_im = ((*ptr_ps_tables)
        .qmf_ser_fract_delay_phase_factor_im)
        .as_mut_ptr() as *mut [FLOAT32; 3];
    p_delay_qmf_delay_buf_idx = ((*ptr_ps_dec).delay_qmf_delay_buf_idx).as_mut_ptr();
    p_delay_qmf_delay_num_samp = ((*ptr_ps_dec).delay_qmf_delay_num_samp).as_mut_ptr();
    while gr < (*ptr_ps_dec).num_groups {
        maxsb = *((*ptr_ps_dec).ptr_group_borders)
            .offset((gr as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
        sb = *((*ptr_ps_dec).ptr_group_borders).offset(gr as isize);
        while sb < maxsb {
            let mut decay_scale_factor_0: FLOAT32 = 0.;
            if sb as FLOAT32 <= decay_cutoff {
                decay_scale_factor_0 = 1.0f32 as FLOAT32;
            } else {
                decay_scale_factor_0 = (1.0f32
                    + decay_cutoff as core::ffi::c_float * DECAY_SLOPE
                    - DECAY_SLOPE * sb as core::ffi::c_float) as FLOAT32;
            }
            decay_scale_factor_0 = (if decay_scale_factor_0 > 0.0f32 {
                decay_scale_factor_0 as core::ffi::c_float
            } else {
                0.0f32
            }) as FLOAT32;
            l_delay = (*ptr_ps_dec).delay_buf_idx as WORD32;
            k = 0 as core::ffi::c_int as WORD32;
            while k < NUM_SER_AP_LINKS {
                l_ser_delay_arr[k as usize] = (*ptr_ps_dec).delay_buf_idx_ser[k as usize]
                    as WORD32;
                k += 1;
            }
            k = (*ptr_ps_dec).border_position[0 as core::ffi::c_int as usize] as WORD32;
            while k
                < (*ptr_ps_dec).border_position[(*ptr_ps_dec).num_env as usize]
                    as core::ffi::c_int
            {
                let mut real_0: FLOAT32 = 0.;
                let mut imag_0: FLOAT32 = 0.;
                let mut real0_0: FLOAT32 = 0.;
                let mut imag0_0: FLOAT32 = 0.;
                let mut r_r0_0: FLOAT32 = 0.;
                let mut i_r0_0: FLOAT32 = 0.;
                im_left = *(*pp_qmf_buf_re_left.offset(k as isize)).offset(sb as isize);
                re_left = *(*pp_qmf_buf_im_left.offset(k as isize)).offset(sb as isize);
                if gr >= (*ptr_ps_dec).first_delay_gr && sb >= NUM_OF_ALL_PASS_CHNLS {
                    real_0 = (*pp_sub_qmf_delay_buf_re
                        .offset(
                            *p_delay_qmf_delay_buf_idx.offset(sb as isize) as isize,
                        ))[sb as usize];
                    imag_0 = (*pp_sub_qmf_delay_buf_im
                        .offset(
                            *p_delay_qmf_delay_buf_idx.offset(sb as isize) as isize,
                        ))[sb as usize];
                    r_r0_0 = real_0;
                    i_r0_0 = imag_0;
                    (*pp_sub_qmf_delay_buf_re
                        .offset(
                            *p_delay_qmf_delay_buf_idx.offset(sb as isize) as isize,
                        ))[sb as usize] = im_left;
                    (*pp_sub_qmf_delay_buf_im
                        .offset(
                            *p_delay_qmf_delay_buf_idx.offset(sb as isize) as isize,
                        ))[sb as usize] = re_left;
                } else {
                    real0_0 = (*pp_sub_qmf_delay_buf_re
                        .offset(l_delay as isize))[sb as usize];
                    imag0_0 = (*pp_sub_qmf_delay_buf_im
                        .offset(l_delay as isize))[sb as usize];
                    (*pp_sub_qmf_delay_buf_re.offset(l_delay as isize))[sb as usize] = im_left;
                    (*pp_sub_qmf_delay_buf_im.offset(l_delay as isize))[sb as usize] = re_left;
                    real_0 = real0_0 * *pp_frac_delay_phase_fac_re.offset(sb as isize)
                        - imag0_0 * *pp_frac_delay_phase_fac_im.offset(sb as isize);
                    imag_0 = real0_0 * *pp_frac_delay_phase_fac_im.offset(sb as isize)
                        + imag0_0 * *pp_frac_delay_phase_fac_re.offset(sb as isize);
                    r_r0_0 = real_0;
                    i_r0_0 = imag_0;
                    m = 0 as core::ffi::c_int as WORD32;
                    while m < NUM_SER_AP_LINKS {
                        real0_0 = (*ppp_ser_sub_qmf_dealy_buf_re
                            .offset(
                                m as isize,
                            ))[l_ser_delay_arr[m as usize] as usize][sb as usize];
                        imag0_0 = (*ppp_ser_sub_qmf_dealy_buf_im
                            .offset(
                                m as isize,
                            ))[l_ser_delay_arr[m as usize] as usize][sb as usize];
                        real_0 = real0_0
                            * (*pp_frac_delay_phase_fac_ser_re
                                .offset(sb as isize))[m as usize]
                            - imag0_0
                                * (*pp_frac_delay_phase_fac_ser_im
                                    .offset(sb as isize))[m as usize];
                        imag_0 = real0_0
                            * (*pp_frac_delay_phase_fac_ser_im
                                .offset(sb as isize))[m as usize]
                            + imag0_0
                                * (*pp_frac_delay_phase_fac_ser_re
                                    .offset(sb as isize))[m as usize];
                        real_0
                            += -decay_scale_factor_0
                                * (*ptr_ps_tables).all_pass_link_decay_ser[m as usize]
                                * r_r0_0;
                        imag_0
                            += -decay_scale_factor_0
                                * (*ptr_ps_tables).all_pass_link_decay_ser[m as usize]
                                * i_r0_0;
                        (*ppp_ser_sub_qmf_dealy_buf_re
                            .offset(
                                m as isize,
                            ))[l_ser_delay_arr[m as usize] as usize][sb as usize] = r_r0_0
                            + decay_scale_factor_0
                                * (*ptr_ps_tables).all_pass_link_decay_ser[m as usize]
                                * real_0;
                        (*ppp_ser_sub_qmf_dealy_buf_im
                            .offset(
                                m as isize,
                            ))[l_ser_delay_arr[m as usize] as usize][sb as usize] = i_r0_0
                            + decay_scale_factor_0
                                * (*ptr_ps_tables).all_pass_link_decay_ser[m as usize]
                                * imag_0;
                        r_r0_0 = real_0;
                        i_r0_0 = imag_0;
                        m += 1;
                    }
                }
                bin = !NEGATE_IPD_MASK
                    & *((*ptr_ps_dec).ptr_bins_group_map).offset(gr as isize);
                trans_ratio = trans_ratio_arr[k as usize][bin as usize];
                *(*pp_qmf_buf_re_right.offset(k as isize)).offset(sb as isize) = trans_ratio
                    * r_r0_0;
                *(*pp_qmf_buf_im_right.offset(k as isize)).offset(sb as isize) = trans_ratio
                    * i_r0_0;
                l_delay += 1;
                if l_delay >= DEL_ALL_PASS {
                    l_delay = 0 as core::ffi::c_int as WORD32;
                }
                if gr >= (*ptr_ps_dec).first_delay_gr && sb >= NUM_OF_ALL_PASS_CHNLS {
                    let ref mut fresh1 = *p_delay_qmf_delay_buf_idx.offset(sb as isize);
                    *fresh1 += 1;
                    if *fresh1 >= *p_delay_qmf_delay_num_samp.offset(sb as isize) {
                        *p_delay_qmf_delay_buf_idx.offset(sb as isize) = 0
                            as core::ffi::c_int as WORD32;
                    }
                }
                m = 0 as core::ffi::c_int as WORD32;
                while m < NUM_SER_AP_LINKS {
                    l_ser_delay_arr[m as usize] += 1;
                    if l_ser_delay_arr[m as usize]
                        >= (*ptr_ps_dec).delay_sample_ser[m as usize] as core::ffi::c_int
                    {
                        l_ser_delay_arr[m as usize] = 0 as core::ffi::c_int as WORD32;
                    }
                    m += 1;
                }
                k += 1;
            }
            sb += 1;
        }
        gr += 1;
    }
    (*ptr_ps_dec).delay_buf_idx = l_delay as WORD16;
    m = 0 as core::ffi::c_int as WORD32;
    while m < NUM_SER_AP_LINKS {
        (*ptr_ps_dec).delay_buf_idx_ser[m as usize] = l_ser_delay_arr[m as usize]
            as WORD16;
        m += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_esbr_ps_apply_rotation(
    mut ptr_ps_dec: *mut ia_ps_dec_struct,
    mut pp_qmf_buf_re_left: *mut *mut FLOAT32,
    mut pp_qmf_buf_im_left: *mut *mut FLOAT32,
    mut pp_qmf_buf_re_right: *mut *mut FLOAT32,
    mut pp_qmf_buf_im_right: *mut *mut FLOAT32,
    mut ptr_ps_tables: *mut ia_ps_tables_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut group: WORD32 = 0;
    let mut bin: WORD32 = 0 as WORD32;
    let mut subband: WORD32 = 0;
    let mut max_subband: WORD32 = 0;
    let mut env: WORD32 = 0;
    let mut p_hyb_left_re: *mut [FLOAT32; 32] = 0 as *mut [FLOAT32; 32];
    let mut p_hyb_left_im: *mut [FLOAT32; 32] = 0 as *mut [FLOAT32; 32];
    let mut p_hyb_rigth_re: *mut [FLOAT32; 32] = 0 as *mut [FLOAT32; 32];
    let mut p_hyb_rigth_im: *mut [FLOAT32; 32] = 0 as *mut [FLOAT32; 32];
    let mut scale_fac_l: FLOAT32 = 0.;
    let mut scale_fac_r: FLOAT32 = 0.;
    let mut alpha: FLOAT32 = 0.;
    let mut beta: FLOAT32 = 0.;
    let mut ipd: FLOAT32 = 0.;
    let mut opd: FLOAT32 = 0.;
    let mut ipd1: FLOAT32 = 0.;
    let mut opd1: FLOAT32 = 0.;
    let mut ipd2: FLOAT32 = 0.;
    let mut opd2: FLOAT32 = 0.;
    let mut h11r: FLOAT32 = 0.;
    let mut h12r: FLOAT32 = 0.;
    let mut h21r: FLOAT32 = 0.;
    let mut h22r: FLOAT32 = 0.;
    let mut h11i: FLOAT32 = 0.;
    let mut h12i: FLOAT32 = 0.;
    let mut h21i: FLOAT32 = 0.;
    let mut h22i: FLOAT32 = 0.;
    let mut H11r: FLOAT32 = 0.;
    let mut H12r: FLOAT32 = 0.;
    let mut H21r: FLOAT32 = 0.;
    let mut H22r: FLOAT32 = 0.;
    let mut H11i: FLOAT32 = 0.;
    let mut H12i: FLOAT32 = 0.;
    let mut H21i: FLOAT32 = 0.;
    let mut H22i: FLOAT32 = 0.;
    let mut deltaH11r: FLOAT32 = 0.;
    let mut deltaH12r: FLOAT32 = 0.;
    let mut deltaH21r: FLOAT32 = 0.;
    let mut deltaH22r: FLOAT32 = 0.;
    let mut deltaH11i: FLOAT32 = 0.;
    let mut deltaH12i: FLOAT32 = 0.;
    let mut deltaH21i: FLOAT32 = 0.;
    let mut deltaH22i: FLOAT32 = 0.;
    let mut l_left_re: FLOAT32 = 0.;
    let mut l_left_im: FLOAT32 = 0.;
    let mut l_right_re: FLOAT32 = 0.;
    let mut l_right_im: FLOAT32 = 0.;
    let mut L: WORD32 = 0;
    let mut ptr_scale_factors: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut ptr_quantized_iids: *const WORD32 = 0 as *const WORD32;
    let mut num_iid_steps: WORD32 = 0;
    if (*ptr_ps_dec).iid_quant != 0 {
        num_iid_steps = NUM_IID_STEPS_FINE as WORD32;
        ptr_scale_factors = ((*ptr_ps_tables).scale_factors_fine_flt).as_mut_ptr();
        ptr_quantized_iids = ((*ptr_ps_tables).quantized_iids_fine).as_mut_ptr();
    } else {
        num_iid_steps = NUM_IID_STEPS as WORD32;
        ptr_scale_factors = ((*ptr_ps_tables).scale_factors_flt).as_mut_ptr();
        ptr_quantized_iids = ((*ptr_ps_tables).quantized_iids).as_mut_ptr();
    }
    if (*ptr_ps_dec).use_34_st_bands != (*ptr_ps_dec).use_34_st_bands_prev {
        if (*ptr_ps_dec).use_34_st_bands != 0 {
            ixheaacd_map_20_float_to_34(((*ptr_ps_dec).h11_re_prev).as_mut_ptr());
            ixheaacd_map_20_float_to_34(((*ptr_ps_dec).h12_re_prev).as_mut_ptr());
            ixheaacd_map_20_float_to_34(((*ptr_ps_dec).h21_re_prev).as_mut_ptr());
            ixheaacd_map_20_float_to_34(((*ptr_ps_dec).h22_re_prev).as_mut_ptr());
            ixheaacd_map_20_float_to_34(((*ptr_ps_dec).h11_im_prev).as_mut_ptr());
            ixheaacd_map_20_float_to_34(((*ptr_ps_dec).h12_im_prev).as_mut_ptr());
            ixheaacd_map_20_float_to_34(((*ptr_ps_dec).h21_im_prev).as_mut_ptr());
            ixheaacd_map_20_float_to_34(((*ptr_ps_dec).h22_im_prev).as_mut_ptr());
            memset(
                ((*ptr_ps_dec).ipd_idx_map_1).as_mut_ptr() as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<[WORD32; 17]>() as size_t,
            );
            memset(
                ((*ptr_ps_dec).opd_idx_map_1).as_mut_ptr() as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<[WORD32; 17]>() as size_t,
            );
            memset(
                ((*ptr_ps_dec).ipd_idx_map_2).as_mut_ptr() as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<[WORD32; 17]>() as size_t,
            );
            memset(
                ((*ptr_ps_dec).opd_idx_map_2).as_mut_ptr() as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<[WORD32; 17]>() as size_t,
            );
        } else {
            ixheaacd_map_34_float_to_20(((*ptr_ps_dec).h11_re_prev).as_mut_ptr());
            ixheaacd_map_34_float_to_20(((*ptr_ps_dec).h12_re_prev).as_mut_ptr());
            ixheaacd_map_34_float_to_20(((*ptr_ps_dec).h21_re_prev).as_mut_ptr());
            ixheaacd_map_34_float_to_20(((*ptr_ps_dec).h22_re_prev).as_mut_ptr());
            ixheaacd_map_34_float_to_20(((*ptr_ps_dec).h11_im_prev).as_mut_ptr());
            ixheaacd_map_34_float_to_20(((*ptr_ps_dec).h12_im_prev).as_mut_ptr());
            ixheaacd_map_34_float_to_20(((*ptr_ps_dec).h21_im_prev).as_mut_ptr());
            ixheaacd_map_34_float_to_20(((*ptr_ps_dec).h22_im_prev).as_mut_ptr());
            memset(
                ((*ptr_ps_dec).ipd_idx_map_1).as_mut_ptr() as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<[WORD32; 17]>() as size_t,
            );
            memset(
                ((*ptr_ps_dec).opd_idx_map_1).as_mut_ptr() as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<[WORD32; 17]>() as size_t,
            );
            memset(
                ((*ptr_ps_dec).ipd_idx_map_2).as_mut_ptr() as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<[WORD32; 17]>() as size_t,
            );
            memset(
                ((*ptr_ps_dec).opd_idx_map_2).as_mut_ptr() as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<[WORD32; 17]>() as size_t,
            );
        }
    }
    env = 0 as core::ffi::c_int as WORD32;
    while env < (*ptr_ps_dec).num_env as core::ffi::c_int {
        bin = 0 as core::ffi::c_int as WORD32;
        while bin < (*ptr_ps_dec).num_bins {
            if (*ptr_ps_dec).use_pca_rot_flg == 0 {
                scale_fac_r = *ptr_scale_factors
                    .offset(
                        (num_iid_steps as core::ffi::c_int
                            + (*ptr_ps_dec).iid_par_table[env as usize][bin as usize]
                                as core::ffi::c_int) as isize,
                    );
                scale_fac_l = *ptr_scale_factors
                    .offset(
                        (num_iid_steps as core::ffi::c_int
                            - (*ptr_ps_dec).iid_par_table[env as usize][bin as usize]
                                as core::ffi::c_int) as isize,
                    );
                alpha = (*ptr_ps_tables)
                    .alphas[(*ptr_ps_dec).icc_par_table[env as usize][bin as usize]
                    as usize];
                beta = alpha * (scale_fac_r - scale_fac_l) / PSC_SQRT2F;
                h11r = (scale_fac_l as core::ffi::c_double
                    * cos((beta + alpha) as core::ffi::c_double)) as FLOAT32;
                h12r = (scale_fac_r as core::ffi::c_double
                    * cos((beta - alpha) as core::ffi::c_double)) as FLOAT32;
                h21r = (scale_fac_l as core::ffi::c_double
                    * sin((beta + alpha) as core::ffi::c_double)) as FLOAT32;
                h22r = (scale_fac_r as core::ffi::c_double
                    * sin((beta - alpha) as core::ffi::c_double)) as FLOAT32;
            } else {
                let mut c: FLOAT32 = 0.;
                let mut rho: FLOAT32 = 0.;
                let mut mu: FLOAT32 = 0.;
                let mut alpha_0: FLOAT32 = 0.;
                let mut gamma: FLOAT32 = 0.;
                let mut i_0: WORD32 = 0;
                i_0 = (*ptr_ps_dec).iid_par_table[env as usize][bin as usize] as WORD32;
                c = pow(
                    10.0f64,
                    (if i_0 != 0 {
                        ((if i_0 > 0 as core::ffi::c_int {
                            1 as WORD32
                        } else {
                            -(1 as WORD32)
                        })
                            * *ptr_quantized_iids
                                .offset(
                                    ((if i_0 > 0 as core::ffi::c_int {
                                        i_0 as core::ffi::c_int
                                    } else {
                                        -(i_0 as core::ffi::c_int)
                                    }) - 1 as core::ffi::c_int) as isize,
                                )) as core::ffi::c_double
                    } else {
                        0.0f64
                    }) / 20.0f64,
                ) as FLOAT32;
                rho = (*ptr_ps_tables)
                    .quantized_rhos[(*ptr_ps_dec)
                    .icc_par_table[env as usize][bin as usize] as usize];
                rho = (if rho > 0.05f32 { rho as core::ffi::c_float } else { 0.05f32 })
                    as FLOAT32;
                if rho == 0.0f32 && c as core::ffi::c_double == 1.0f64 {
                    alpha_0 = (PI as core::ffi::c_float / 4.0f32) as FLOAT32;
                } else {
                    if rho <= 0.05f32 {
                        rho = 0.05f32 as FLOAT32;
                    }
                    alpha_0 = 0.5f32
                        * atan(
                            (2.0f32 * c as core::ffi::c_float * rho as core::ffi::c_float
                                / (c as core::ffi::c_float * c as core::ffi::c_float
                                    - 1.0f32)) as core::ffi::c_double,
                        ) as FLOAT32;
                    if (alpha_0 as core::ffi::c_double) < 0.0f64 {
                        alpha_0 += PI as core::ffi::c_float / 2.0f32;
                    }
                }
                mu = c + 1.0f32 / c;
                mu = 1 as core::ffi::c_int as FLOAT32
                    + (4.0f32 * rho * rho - 4.0f32) / (mu * mu);
                gamma = atan(
                    sqrt(
                        (1.0f64 - sqrt(mu as core::ffi::c_double))
                            / (1.0f64 + sqrt(mu as core::ffi::c_double)),
                    ),
                ) as FLOAT32;
                h11r = (sqrt(2.0f64) * cos(alpha_0 as core::ffi::c_double)
                    * cos(gamma as core::ffi::c_double)) as FLOAT32;
                h12r = (sqrt(2.0f64) * sin(alpha_0 as core::ffi::c_double)
                    * cos(gamma as core::ffi::c_double)) as FLOAT32;
                h21r = (sqrt(2.0f64) * -sin(alpha_0 as core::ffi::c_double)
                    * sin(gamma as core::ffi::c_double)) as FLOAT32;
                h22r = (sqrt(2.0f64) * cos(alpha_0 as core::ffi::c_double)
                    * sin(gamma as core::ffi::c_double)) as FLOAT32;
            }
            if bin >= (*ptr_ps_tables).ipd_bins_tbl[(*ptr_ps_dec).freq_res_ipd as usize]
            {
                h22i = 0.0f32 as FLOAT32;
                h21i = h22i;
                h12i = h21i;
                h11i = h12i;
            } else {
                ipd = (IPD_SCALE_FACTOR * 2.0f32
                    * (*ptr_ps_dec).ipd_idx_map[env as usize][bin as usize]
                        as core::ffi::c_float) as FLOAT32;
                opd = (OPD_SCALE_FACTOR * 2.0f32
                    * (*ptr_ps_dec).opd_idx_map[env as usize][bin as usize]
                        as core::ffi::c_float) as FLOAT32;
                ipd1 = (IPD_SCALE_FACTOR * 2.0f32
                    * (*ptr_ps_dec).ipd_idx_map_1[bin as usize] as core::ffi::c_float)
                    as FLOAT32;
                opd1 = (OPD_SCALE_FACTOR * 2.0f32
                    * (*ptr_ps_dec).opd_idx_map_1[bin as usize] as core::ffi::c_float)
                    as FLOAT32;
                ipd2 = (IPD_SCALE_FACTOR * 2.0f32
                    * (*ptr_ps_dec).ipd_idx_map_2[bin as usize] as core::ffi::c_float)
                    as FLOAT32;
                opd2 = (OPD_SCALE_FACTOR * 2.0f32
                    * (*ptr_ps_dec).opd_idx_map_2[bin as usize] as core::ffi::c_float)
                    as FLOAT32;
                l_left_re = cos(ipd as core::ffi::c_double) as FLOAT32;
                l_left_im = sin(ipd as core::ffi::c_double) as FLOAT32;
                l_right_re = cos(opd as core::ffi::c_double) as FLOAT32;
                l_right_im = sin(opd as core::ffi::c_double) as FLOAT32;
                l_left_re
                    += (PHASE_SMOOTH_HIST1 * cos(ipd1 as core::ffi::c_double) as FLOAT32)
                        as core::ffi::c_float;
                l_left_im
                    += (PHASE_SMOOTH_HIST1 * sin(ipd1 as core::ffi::c_double) as FLOAT32)
                        as core::ffi::c_float;
                l_right_re
                    += (PHASE_SMOOTH_HIST1 * cos(opd1 as core::ffi::c_double) as FLOAT32)
                        as core::ffi::c_float;
                l_right_im
                    += (PHASE_SMOOTH_HIST1 * sin(opd1 as core::ffi::c_double) as FLOAT32)
                        as core::ffi::c_float;
                l_left_re
                    += (PHASE_SMOOTH_HIST2 * cos(ipd2 as core::ffi::c_double) as FLOAT32)
                        as core::ffi::c_float;
                l_left_im
                    += (PHASE_SMOOTH_HIST2 * sin(ipd2 as core::ffi::c_double) as FLOAT32)
                        as core::ffi::c_float;
                l_right_re
                    += (PHASE_SMOOTH_HIST2 * cos(opd2 as core::ffi::c_double) as FLOAT32)
                        as core::ffi::c_float;
                l_right_im
                    += (PHASE_SMOOTH_HIST2 * sin(opd2 as core::ffi::c_double) as FLOAT32)
                        as core::ffi::c_float;
                ipd = atan2(
                    l_left_im as core::ffi::c_double,
                    l_left_re as core::ffi::c_double,
                ) as FLOAT32;
                opd = atan2(
                    l_right_im as core::ffi::c_double,
                    l_right_re as core::ffi::c_double,
                ) as FLOAT32;
                l_left_re = cos(opd as core::ffi::c_double) as FLOAT32;
                l_left_im = sin(opd as core::ffi::c_double) as FLOAT32;
                opd -= ipd;
                l_right_re = cos(opd as core::ffi::c_double) as FLOAT32;
                l_right_im = sin(opd as core::ffi::c_double) as FLOAT32;
                h11i = h11r * l_left_im;
                h12i = h12r * l_right_im;
                h21i = h21r * l_left_im;
                h22i = h22r * l_right_im;
                h11r *= l_left_re;
                h12r *= l_right_re;
                h21r *= l_left_re;
                h22r *= l_right_re;
            }
            (*ptr_ps_dec).h11_re_vec[bin as usize] = h11r;
            (*ptr_ps_dec).h12_re_vec[bin as usize] = h12r;
            (*ptr_ps_dec).h21_re_vec[bin as usize] = h21r;
            (*ptr_ps_dec).h22_re_vec[bin as usize] = h22r;
            (*ptr_ps_dec).h11_im_vec[bin as usize] = h11i;
            (*ptr_ps_dec).h12_im_vec[bin as usize] = h12i;
            (*ptr_ps_dec).h21_im_vec[bin as usize] = h21i;
            (*ptr_ps_dec).h22_im_vec[bin as usize] = h22i;
            bin += 1;
        }
        p_hyb_left_re = ((*ptr_ps_dec).hyb_left_re).as_mut_ptr() as *mut [FLOAT32; 32];
        p_hyb_left_im = ((*ptr_ps_dec).hyb_left_im).as_mut_ptr() as *mut [FLOAT32; 32];
        p_hyb_rigth_re = ((*ptr_ps_dec).hyb_right_re).as_mut_ptr() as *mut [FLOAT32; 32];
        p_hyb_rigth_im = ((*ptr_ps_dec).hyb_right_im).as_mut_ptr() as *mut [FLOAT32; 32];
        group = 0 as core::ffi::c_int as WORD32;
        while group < (*ptr_ps_dec).num_sub_qmf_groups {
            bin = !NEGATE_IPD_MASK
                & *((*ptr_ps_dec).ptr_bins_group_map).offset(group as isize);
            max_subband = (*((*ptr_ps_dec).ptr_group_borders).offset(group as isize)
                + 1 as core::ffi::c_int) as WORD32;
            L = ((*ptr_ps_dec)
                .border_position[(env as core::ffi::c_int + 1 as core::ffi::c_int)
                as usize] as core::ffi::c_int
                - (*ptr_ps_dec).border_position[env as usize] as core::ffi::c_int)
                as WORD32;
            H11r = (*ptr_ps_dec).h11_re_prev[bin as usize];
            H12r = (*ptr_ps_dec).h12_re_prev[bin as usize];
            H21r = (*ptr_ps_dec).h21_re_prev[bin as usize];
            H22r = (*ptr_ps_dec).h22_re_prev[bin as usize];
            if NEGATE_IPD_MASK
                & *((*ptr_ps_dec).ptr_bins_group_map).offset(group as isize)
                != 0 as core::ffi::c_int
            {
                H11i = -(*ptr_ps_dec).h11_im_prev[bin as usize];
                H12i = -(*ptr_ps_dec).h12_im_prev[bin as usize];
                H21i = -(*ptr_ps_dec).h21_im_prev[bin as usize];
                H22i = -(*ptr_ps_dec).h22_im_prev[bin as usize];
            } else {
                H11i = (*ptr_ps_dec).h11_im_prev[bin as usize];
                H12i = (*ptr_ps_dec).h12_im_prev[bin as usize];
                H21i = (*ptr_ps_dec).h21_im_prev[bin as usize];
                H22i = (*ptr_ps_dec).h22_im_prev[bin as usize];
            }
            h11r = (*ptr_ps_dec).h11_re_vec[bin as usize];
            h12r = (*ptr_ps_dec).h12_re_vec[bin as usize];
            h21r = (*ptr_ps_dec).h21_re_vec[bin as usize];
            h22r = (*ptr_ps_dec).h22_re_vec[bin as usize];
            if NEGATE_IPD_MASK
                & *((*ptr_ps_dec).ptr_bins_group_map).offset(group as isize)
                != 0 as core::ffi::c_int
            {
                h11i = -(*ptr_ps_dec).h11_im_vec[bin as usize];
                h12i = -(*ptr_ps_dec).h12_im_vec[bin as usize];
                h21i = -(*ptr_ps_dec).h21_im_vec[bin as usize];
                h22i = -(*ptr_ps_dec).h22_im_vec[bin as usize];
            } else {
                h11i = (*ptr_ps_dec).h11_im_vec[bin as usize];
                h12i = (*ptr_ps_dec).h12_im_vec[bin as usize];
                h21i = (*ptr_ps_dec).h21_im_vec[bin as usize];
                h22i = (*ptr_ps_dec).h22_im_vec[bin as usize];
            }
            deltaH11r = (h11r - H11r) / L as FLOAT32;
            deltaH12r = (h12r - H12r) / L as FLOAT32;
            deltaH21r = (h21r - H21r) / L as FLOAT32;
            deltaH22r = (h22r - H22r) / L as FLOAT32;
            deltaH11i = (h11i - H11i) / L as FLOAT32;
            deltaH12i = (h12i - H12i) / L as FLOAT32;
            deltaH21i = (h21i - H21i) / L as FLOAT32;
            deltaH22i = (h22i - H22i) / L as FLOAT32;
            i = (*ptr_ps_dec).border_position[env as usize] as WORD32;
            while i
                < (*ptr_ps_dec)
                    .border_position[(env as core::ffi::c_int + 1 as core::ffi::c_int)
                    as usize] as core::ffi::c_int
            {
                H11r += deltaH11r;
                H12r += deltaH12r;
                H21r += deltaH21r;
                H22r += deltaH22r;
                H11i += deltaH11i;
                H12i += deltaH12i;
                H21i += deltaH21i;
                H22i += deltaH22i;
                subband = *((*ptr_ps_dec).ptr_group_borders).offset(group as isize);
                while subband < max_subband {
                    l_left_re = H11r
                        * (*p_hyb_left_re.offset(i as isize))[subband as usize]
                        - H11i * (*p_hyb_left_im.offset(i as isize))[subband as usize]
                        + H21r * (*p_hyb_rigth_re.offset(i as isize))[subband as usize]
                        - H21i * (*p_hyb_rigth_im.offset(i as isize))[subband as usize];
                    l_left_im = H11i
                        * (*p_hyb_left_re.offset(i as isize))[subband as usize]
                        + H11r * (*p_hyb_left_im.offset(i as isize))[subband as usize]
                        + H21i * (*p_hyb_rigth_re.offset(i as isize))[subband as usize]
                        + H21r * (*p_hyb_rigth_im.offset(i as isize))[subband as usize];
                    l_right_re = H12r
                        * (*p_hyb_left_re.offset(i as isize))[subband as usize]
                        - H12i * (*p_hyb_left_im.offset(i as isize))[subband as usize]
                        + H22r * (*p_hyb_rigth_re.offset(i as isize))[subband as usize]
                        - H22i * (*p_hyb_rigth_im.offset(i as isize))[subband as usize];
                    l_right_im = H12i
                        * (*p_hyb_left_re.offset(i as isize))[subband as usize]
                        + H12r * (*p_hyb_left_im.offset(i as isize))[subband as usize]
                        + H22i * (*p_hyb_rigth_re.offset(i as isize))[subband as usize]
                        + H22r * (*p_hyb_rigth_im.offset(i as isize))[subband as usize];
                    (*p_hyb_left_re.offset(i as isize))[subband as usize] = l_left_re;
                    (*p_hyb_left_im.offset(i as isize))[subband as usize] = l_left_im;
                    (*p_hyb_rigth_re.offset(i as isize))[subband as usize] = l_right_re;
                    (*p_hyb_rigth_im.offset(i as isize))[subband as usize] = l_right_im;
                    subband += 1;
                }
                i += 1;
            }
            group += 1;
        }
        while group < (*ptr_ps_dec).num_groups {
            bin = !NEGATE_IPD_MASK
                & *((*ptr_ps_dec).ptr_bins_group_map).offset(group as isize);
            max_subband = *((*ptr_ps_dec).ptr_group_borders)
                .offset((group as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            L = ((*ptr_ps_dec)
                .border_position[(env as core::ffi::c_int + 1 as core::ffi::c_int)
                as usize] as core::ffi::c_int
                - (*ptr_ps_dec).border_position[env as usize] as core::ffi::c_int)
                as WORD32;
            H11r = (*ptr_ps_dec).h11_re_prev[bin as usize];
            H12r = (*ptr_ps_dec).h12_re_prev[bin as usize];
            H21r = (*ptr_ps_dec).h21_re_prev[bin as usize];
            H22r = (*ptr_ps_dec).h22_re_prev[bin as usize];
            if NEGATE_IPD_MASK
                & *((*ptr_ps_dec).ptr_bins_group_map).offset(group as isize)
                != 0 as core::ffi::c_int
            {
                H11i = -(*ptr_ps_dec).h11_im_prev[bin as usize];
                H12i = -(*ptr_ps_dec).h12_im_prev[bin as usize];
                H21i = -(*ptr_ps_dec).h21_im_prev[bin as usize];
                H22i = -(*ptr_ps_dec).h22_im_prev[bin as usize];
            } else {
                H11i = (*ptr_ps_dec).h11_im_prev[bin as usize];
                H12i = (*ptr_ps_dec).h12_im_prev[bin as usize];
                H21i = (*ptr_ps_dec).h21_im_prev[bin as usize];
                H22i = (*ptr_ps_dec).h22_im_prev[bin as usize];
            }
            h11r = (*ptr_ps_dec).h11_re_vec[bin as usize];
            h12r = (*ptr_ps_dec).h12_re_vec[bin as usize];
            h21r = (*ptr_ps_dec).h21_re_vec[bin as usize];
            h22r = (*ptr_ps_dec).h22_re_vec[bin as usize];
            if NEGATE_IPD_MASK
                & *((*ptr_ps_dec).ptr_bins_group_map).offset(group as isize)
                != 0 as core::ffi::c_int
            {
                h11i = -(*ptr_ps_dec).h11_im_vec[bin as usize];
                h12i = -(*ptr_ps_dec).h12_im_vec[bin as usize];
                h21i = -(*ptr_ps_dec).h21_im_vec[bin as usize];
                h22i = -(*ptr_ps_dec).h22_im_vec[bin as usize];
            } else {
                h11i = (*ptr_ps_dec).h11_im_vec[bin as usize];
                h12i = (*ptr_ps_dec).h12_im_vec[bin as usize];
                h21i = (*ptr_ps_dec).h21_im_vec[bin as usize];
                h22i = (*ptr_ps_dec).h22_im_vec[bin as usize];
            }
            deltaH11r = (h11r - H11r) / L as FLOAT32;
            deltaH12r = (h12r - H12r) / L as FLOAT32;
            deltaH21r = (h21r - H21r) / L as FLOAT32;
            deltaH22r = (h22r - H22r) / L as FLOAT32;
            deltaH11i = (h11i - H11i) / L as FLOAT32;
            deltaH12i = (h12i - H12i) / L as FLOAT32;
            deltaH21i = (h21i - H21i) / L as FLOAT32;
            deltaH22i = (h22i - H22i) / L as FLOAT32;
            i = (*ptr_ps_dec).border_position[env as usize] as WORD32;
            while i
                < (*ptr_ps_dec)
                    .border_position[(env as core::ffi::c_int + 1 as core::ffi::c_int)
                    as usize] as core::ffi::c_int
            {
                H11r += deltaH11r;
                H12r += deltaH12r;
                H21r += deltaH21r;
                H22r += deltaH22r;
                H11i += deltaH11i;
                H12i += deltaH12i;
                H21i += deltaH21i;
                H22i += deltaH22i;
                subband = *((*ptr_ps_dec).ptr_group_borders).offset(group as isize);
                while subband < max_subband {
                    l_left_re = H11r
                        * *(*pp_qmf_buf_re_left.offset(i as isize))
                            .offset(subband as isize)
                        - H11i
                            * *(*pp_qmf_buf_im_left.offset(i as isize))
                                .offset(subband as isize)
                        + H21r
                            * *(*pp_qmf_buf_re_right.offset(i as isize))
                                .offset(subband as isize)
                        - H21i
                            * *(*pp_qmf_buf_im_right.offset(i as isize))
                                .offset(subband as isize);
                    l_left_im = H11i
                        * *(*pp_qmf_buf_re_left.offset(i as isize))
                            .offset(subband as isize)
                        + H11r
                            * *(*pp_qmf_buf_im_left.offset(i as isize))
                                .offset(subband as isize)
                        + H21i
                            * *(*pp_qmf_buf_re_right.offset(i as isize))
                                .offset(subband as isize)
                        + H21r
                            * *(*pp_qmf_buf_im_right.offset(i as isize))
                                .offset(subband as isize);
                    l_right_re = H12r
                        * *(*pp_qmf_buf_re_left.offset(i as isize))
                            .offset(subband as isize)
                        - H12i
                            * *(*pp_qmf_buf_im_left.offset(i as isize))
                                .offset(subband as isize)
                        + H22r
                            * *(*pp_qmf_buf_re_right.offset(i as isize))
                                .offset(subband as isize)
                        - H22i
                            * *(*pp_qmf_buf_im_right.offset(i as isize))
                                .offset(subband as isize);
                    l_right_im = H12i
                        * *(*pp_qmf_buf_re_left.offset(i as isize))
                            .offset(subband as isize)
                        + H12r
                            * *(*pp_qmf_buf_im_left.offset(i as isize))
                                .offset(subband as isize)
                        + H22i
                            * *(*pp_qmf_buf_re_right.offset(i as isize))
                                .offset(subband as isize)
                        + H22r
                            * *(*pp_qmf_buf_im_right.offset(i as isize))
                                .offset(subband as isize);
                    *(*pp_qmf_buf_re_left.offset(i as isize)).offset(subband as isize) = l_left_re;
                    *(*pp_qmf_buf_im_left.offset(i as isize)).offset(subband as isize) = l_left_im;
                    *(*pp_qmf_buf_re_right.offset(i as isize))
                        .offset(subband as isize) = l_right_re;
                    *(*pp_qmf_buf_im_right.offset(i as isize))
                        .offset(subband as isize) = l_right_im;
                    subband += 1;
                }
                i += 1;
            }
            group += 1;
        }
        bin = 0 as core::ffi::c_int as WORD32;
        while bin < (*ptr_ps_dec).num_bins {
            (*ptr_ps_dec).h11_re_prev[bin as usize] = (*ptr_ps_dec)
                .h11_re_vec[bin as usize];
            (*ptr_ps_dec).h12_re_prev[bin as usize] = (*ptr_ps_dec)
                .h12_re_vec[bin as usize];
            (*ptr_ps_dec).h21_re_prev[bin as usize] = (*ptr_ps_dec)
                .h21_re_vec[bin as usize];
            (*ptr_ps_dec).h22_re_prev[bin as usize] = (*ptr_ps_dec)
                .h22_re_vec[bin as usize];
            (*ptr_ps_dec).h11_im_prev[bin as usize] = (*ptr_ps_dec)
                .h11_im_vec[bin as usize];
            (*ptr_ps_dec).h12_im_prev[bin as usize] = (*ptr_ps_dec)
                .h12_im_vec[bin as usize];
            (*ptr_ps_dec).h21_im_prev[bin as usize] = (*ptr_ps_dec)
                .h21_im_vec[bin as usize];
            (*ptr_ps_dec).h22_im_prev[bin as usize] = (*ptr_ps_dec)
                .h22_im_vec[bin as usize];
            bin += 1;
        }
        bin = 0 as core::ffi::c_int as WORD32;
        while bin < (*ptr_ps_tables).ipd_bins_tbl[(*ptr_ps_dec).freq_res_ipd as usize] {
            (*ptr_ps_dec).ipd_idx_map_2[bin as usize] = (*ptr_ps_dec)
                .ipd_idx_map_1[bin as usize];
            (*ptr_ps_dec).opd_idx_map_2[bin as usize] = (*ptr_ps_dec)
                .opd_idx_map_1[bin as usize];
            (*ptr_ps_dec).ipd_idx_map_1[bin as usize] = (*ptr_ps_dec)
                .ipd_idx_map[env as usize][bin as usize];
            (*ptr_ps_dec).opd_idx_map_1[bin as usize] = (*ptr_ps_dec)
                .opd_idx_map[env as usize][bin as usize];
            bin += 1;
        }
        env += 1;
    }
}
pub const NUM_SER_AP_LINKS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const SUBQMF_GROUPS: core::ffi::c_int = 10 as core::ffi::c_int;
pub const QMF_GROUPS: core::ffi::c_int = 12 as core::ffi::c_int;
pub const SUBQMF_GROUPS_HI_RES: core::ffi::c_int = 32 as core::ffi::c_int;
pub const QMF_GROUPS_HI_RES: core::ffi::c_int = 18 as core::ffi::c_int;
pub const NUM_IID_GROUPS: core::ffi::c_int = SUBQMF_GROUPS + QMF_GROUPS;
pub const NUM_IID_GROUPS_HI_RES: core::ffi::c_int = SUBQMF_GROUPS_HI_RES
    + QMF_GROUPS_HI_RES;
pub const NUM_SUB_QMF_CHANNELS_HI_RES: core::ffi::c_int = 32 as core::ffi::c_int;
pub const NUM_IID_STEPS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const NUM_IID_STEPS_FINE: core::ffi::c_int = 15 as core::ffi::c_int;
