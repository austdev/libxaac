extern "C" {
    fn sqrt(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn cbrt(__x: core::ffi::c_double) -> core::ffi::c_double;
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
    static ixheaac_sub_samp_qmf_window_coeff: [FLOAT32; 2000];
    static ixheaac_start_subband2kL_tbl: [WORD32; 33];
    fn ixheaacd_complex_anal_filt(
        ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    ) -> WORD32;
    fn ixheaacd_real_synth_filt(
        ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
        num_columns: WORD32,
        qmf_buf_real: *mut [FLOAT32; 64],
        qmf_buf_imag: *mut [FLOAT32; 64],
    ) -> WORD32;
    fn ixheaac_cmplx_anal_fft_p2(
        inp: *mut FLOAT32,
        out: *mut FLOAT32,
        n_points: WORD32,
    ) -> VOID;
    fn ixheaac_cmplx_anal_fft_p3(
        inp: *mut FLOAT32,
        out: *mut FLOAT32,
        n_points: WORD32,
    ) -> VOID;
    fn ixheaac_real_synth_fft_p2(
        inp: *mut FLOAT32,
        out: *mut FLOAT32,
        n_points: WORD32,
    ) -> VOID;
    fn ixheaac_real_synth_fft_p3(
        inp: *mut FLOAT32,
        out: *mut FLOAT32,
        n_points: WORD32,
    ) -> VOID;
    static ixheaac_phase_vocoder_cos_table: [FLOAT32; 64];
    static ixheaac_phase_vocoder_sin_table: [FLOAT32; 64];
    static ixheaac_hbe_post_anal_proc_interp_coeff: [[FLOAT32; 2]; 4];
    static ixheaac_hbe_x_prod_cos_table_trans_2: [FLOAT32; 512];
    static ixheaac_hbe_x_prod_cos_table_trans_3: [FLOAT32; 512];
    static ixheaac_hbe_x_prod_cos_table_trans_4: [FLOAT32; 512];
    static ixheaac_hbe_x_prod_cos_table_trans_4_1: [FLOAT32; 512];
    static ixheaac_synth_cos_table_kl_4: [FLOAT32; 16];
    static ixheaac_synth_cos_table_kl_8: [FLOAT32; 32];
    static ixheaac_synth_cos_table_kl_12: [FLOAT32; 48];
    static ixheaac_synth_cos_table_kl_16: [FLOAT32; 64];
    static ixheaac_synth_cos_table_kl_20: [FLOAT32; 800];
    static ixheaac_analy_cos_sin_table_kl_8: [FLOAT32; 32];
    static ixheaac_analy_cos_sin_table_kl_16: [FLOAT32; 64];
    static ixheaac_analy_cos_sin_table_kl_24: [FLOAT32; 96];
    static ixheaac_analy_cos_sin_table_kl_32: [FLOAT32; 128];
    static ixheaac_analy_cos_sin_table_kl_40: [FLOAT32; 3200];
    static ixheaac_sel_case: [[FLOAT32; 8]; 5];
}
pub type size_t = usize;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type WORD32 = core::ffi::c_int;
pub type FLOAT32 = core::ffi::c_float;
pub type FLOAT64 = core::ffi::c_double;
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
pub type IA_ERRORCODE = WORD32;
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
pub struct ia_esbr_hbe_txposer_struct {
    pub x_over_qmf: [WORD32; 6],
    pub max_stretch: WORD32,
    pub core_frame_length: WORD32,
    pub hbe_qmf_in_len: WORD32,
    pub hbe_qmf_out_len: WORD32,
    pub no_bins: WORD32,
    pub start_band: WORD32,
    pub end_band: WORD32,
    pub upsamp_4_flag: WORD32,
    pub synth_buf_offset: WORD32,
    pub ptr_input_buf: *mut FLOAT32,
    pub qmf_in_buf: *mut *mut FLOAT32,
    pub qmf_out_buf: *mut *mut FLOAT32,
    pub k_start: WORD32,
    pub synth_size: WORD32,
    pub synth_buf: [FLOAT32; 1280],
    pub analy_buf: [FLOAT32; 640],
    pub synth_wind_coeff: *mut FLOAT32,
    pub analy_wind_coeff: *mut FLOAT32,
    pub synth_cos_tab: *mut FLOAT32,
    pub analy_cos_sin_tab: *mut FLOAT32,
    pub norm_qmf_in_buf: [[FLOAT32; 128]; 46],
    pub ixheaacd_real_synth_fft: Option<
        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
    >,
    pub ixheaacd_cmplx_anal_fft: Option<
        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
    >,
    pub esbr_hq: WORD32,
    pub in_hop_size: WORD32,
    pub fft_size: [WORD32; 2],
    pub anal_window: *mut FLOAT32,
    pub synth_window: *mut FLOAT32,
    pub ptr_spectrum: *mut FLOAT32,
    pub ptr_spectrum_tx: *mut FLOAT32,
    pub mag: *mut FLOAT32,
    pub phase: *mut FLOAT32,
    pub ptr_output_buf: *mut FLOAT32,
    pub ana_fft_size: [WORD32; 2],
    pub syn_fft_size: [WORD32; 2],
    pub out_hop_size: WORD32,
    pub analy_size: WORD32,
    pub x_over_bin: [[WORD32; 2]; 4],
    pub a_start: WORD32,
    pub spectrum_buf: [FLOAT32; 1536],
    pub spectrum_transposed_buf: [FLOAT32; 1536],
    pub mag_buf: [FLOAT32; 1536],
    pub phase_buf: [FLOAT32; 1536],
    pub output_buf: [FLOAT32; 4096],
    pub fd_win_buf: [[[FLOAT32; 1536]; 3]; 3],
    pub analysis_window_buf: [FLOAT32; 1024],
    pub synthesis_window_buf: [FLOAT32; 1024],
    pub oversampling_flag: WORD32,
    pub str_dft_hbe_anal_coeff: ia_dft_hbe_anal_coeff,
    pub ixheaacd_hbe_anal_fft: Option<
        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32, WORD32) -> VOID,
    >,
    pub ixheaacd_hbe_synth_ifft: Option<
        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32, WORD32) -> VOID,
    >,
    pub syn_cos_sin_tab: *mut FLOAT32,
    pub ana_cos_sin_tab: *mut FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_dft_hbe_anal_coeff {
    pub real: [[FLOAT32; 128]; 64],
    pub imag: [[FLOAT32; 128]; 64],
}
pub const LOW: core::ffi::c_int = 0 as core::ffi::c_int;
pub const HIGH: core::ffi::c_int = 1 as core::ffi::c_int;
pub const HBE_OPER_WIN_LEN: core::ffi::c_int = 13 as core::ffi::c_int;
pub const NO_QMF_SYNTH_CHANNELS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const TWICE_QMF_SYNTH_CHANNELS_NUM: core::ffi::c_int = 128 as core::ffi::c_int;
pub const HBE_OPER_BLK_LEN_2: core::ffi::c_int = 10 as core::ffi::c_int;
pub const HBE_OPER_BLK_LEN_3: core::ffi::c_int = 8 as core::ffi::c_int;
pub const HBE_OPER_BLK_LEN_4: core::ffi::c_int = 6 as core::ffi::c_int;
pub const MAX_STRETCH: core::ffi::c_int = 4 as core::ffi::c_int;
pub const HBE_ZERO_BAND_IDX: core::ffi::c_int = 6 as core::ffi::c_int;
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_FATAL_ERROR: core::ffi::c_uint = 0x80000000 as core::ffi::c_uint;
pub const SBR_CONST_PMIN: core::ffi::c_float = 1.0f32;
unsafe extern "C" fn ixheaacd_map_prot_filter(mut filt_length: WORD32) -> *mut FLOAT32 {
    match filt_length {
        4 => {
            return &*ixheaac_sub_samp_qmf_window_coeff
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const FLOAT32 as *mut FLOAT32;
        }
        8 => {
            return &*ixheaac_sub_samp_qmf_window_coeff
                .as_ptr()
                .offset(40 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
        }
        12 => {
            return &*ixheaac_sub_samp_qmf_window_coeff
                .as_ptr()
                .offset(120 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
        }
        16 => {
            return &*ixheaac_sub_samp_qmf_window_coeff
                .as_ptr()
                .offset(240 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
        }
        20 => {
            return &*ixheaac_sub_samp_qmf_window_coeff
                .as_ptr()
                .offset(400 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
        }
        24 => {
            return &*ixheaac_sub_samp_qmf_window_coeff
                .as_ptr()
                .offset(600 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
        }
        32 => {
            return &*ixheaac_sub_samp_qmf_window_coeff
                .as_ptr()
                .offset(840 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
        }
        40 => {
            return &*ixheaac_sub_samp_qmf_window_coeff
                .as_ptr()
                .offset(1160 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
        }
        _ => {
            return &*ixheaac_sub_samp_qmf_window_coeff
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const FLOAT32 as *mut FLOAT32;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_qmf_hbe_data_reinit(
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    mut p_freq_band_tab: *mut *mut WORD16,
    mut p_num_sfb: *mut WORD16,
    mut upsamp_4_flag: WORD32,
) -> WORD32 {
    let mut synth_size: WORD32 = 0;
    let mut sfb: WORD32 = 0;
    let mut patch: WORD32 = 0;
    let mut stop_patch: WORD32 = 0;
    if !ptr_hbe_txposer.is_null() {
        (*ptr_hbe_txposer).start_band = *(*p_freq_band_tab.offset(LOW as isize))
            .offset(0 as core::ffi::c_int as isize) as WORD32;
        (*ptr_hbe_txposer).end_band = *(*p_freq_band_tab.offset(LOW as isize))
            .offset(*p_num_sfb.offset(LOW as isize) as isize) as WORD32;
        (*ptr_hbe_txposer).synth_size = (4 as core::ffi::c_int
            * (((*ptr_hbe_txposer).start_band as core::ffi::c_int
                + 4 as core::ffi::c_int) / 8 as core::ffi::c_int
                + 1 as core::ffi::c_int)) as WORD32;
        (*ptr_hbe_txposer).k_start = ixheaac_start_subband2kL_tbl[(*ptr_hbe_txposer)
            .start_band as usize];
        (*ptr_hbe_txposer).upsamp_4_flag = upsamp_4_flag;
        (*ptr_hbe_txposer).esbr_hq = 0 as core::ffi::c_int as WORD32;
        if upsamp_4_flag != 0 {
            if (*ptr_hbe_txposer).k_start + (*ptr_hbe_txposer).synth_size
                > 16 as core::ffi::c_int
            {
                (*ptr_hbe_txposer).k_start = 16 as WORD32
                    - (*ptr_hbe_txposer).synth_size;
            }
        } else if (*ptr_hbe_txposer).core_frame_length == 768 as core::ffi::c_int {
            if (*ptr_hbe_txposer).k_start + (*ptr_hbe_txposer).synth_size
                > 24 as core::ffi::c_int
            {
                (*ptr_hbe_txposer).k_start = 24 as WORD32
                    - (*ptr_hbe_txposer).synth_size;
            }
        }
        memset(
            ((*ptr_hbe_txposer).synth_buf).as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (1280 as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        synth_size = (*ptr_hbe_txposer).synth_size;
        (*ptr_hbe_txposer).synth_buf_offset = 18 as WORD32 * synth_size;
        match synth_size {
            4 => {
                (*ptr_hbe_txposer).synth_cos_tab = ixheaac_synth_cos_table_kl_4.as_ptr()
                    as *mut FLOAT32;
                (*ptr_hbe_txposer).analy_cos_sin_tab = ixheaac_analy_cos_sin_table_kl_8
                    .as_ptr() as *mut FLOAT32;
                (*ptr_hbe_txposer).ixheaacd_real_synth_fft = Some(
                    ixheaac_real_synth_fft_p2
                        as unsafe extern "C" fn(
                            *mut FLOAT32,
                            *mut FLOAT32,
                            WORD32,
                        ) -> VOID,
                )
                    as Option<
                        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
                    >;
                (*ptr_hbe_txposer).ixheaacd_cmplx_anal_fft = Some(
                    ixheaac_cmplx_anal_fft_p2
                        as unsafe extern "C" fn(
                            *mut FLOAT32,
                            *mut FLOAT32,
                            WORD32,
                        ) -> VOID,
                )
                    as Option<
                        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
                    >;
            }
            8 => {
                (*ptr_hbe_txposer).synth_cos_tab = ixheaac_synth_cos_table_kl_8.as_ptr()
                    as *mut FLOAT32;
                (*ptr_hbe_txposer).analy_cos_sin_tab = ixheaac_analy_cos_sin_table_kl_16
                    .as_ptr() as *mut FLOAT32;
                (*ptr_hbe_txposer).ixheaacd_real_synth_fft = Some(
                    ixheaac_real_synth_fft_p2
                        as unsafe extern "C" fn(
                            *mut FLOAT32,
                            *mut FLOAT32,
                            WORD32,
                        ) -> VOID,
                )
                    as Option<
                        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
                    >;
                (*ptr_hbe_txposer).ixheaacd_cmplx_anal_fft = Some(
                    ixheaac_cmplx_anal_fft_p2
                        as unsafe extern "C" fn(
                            *mut FLOAT32,
                            *mut FLOAT32,
                            WORD32,
                        ) -> VOID,
                )
                    as Option<
                        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
                    >;
            }
            12 => {
                (*ptr_hbe_txposer).synth_cos_tab = ixheaac_synth_cos_table_kl_12.as_ptr()
                    as *mut FLOAT32;
                (*ptr_hbe_txposer).analy_cos_sin_tab = ixheaac_analy_cos_sin_table_kl_24
                    .as_ptr() as *mut FLOAT32;
                (*ptr_hbe_txposer).ixheaacd_real_synth_fft = Some(
                    ixheaac_real_synth_fft_p3
                        as unsafe extern "C" fn(
                            *mut FLOAT32,
                            *mut FLOAT32,
                            WORD32,
                        ) -> VOID,
                )
                    as Option<
                        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
                    >;
                (*ptr_hbe_txposer).ixheaacd_cmplx_anal_fft = Some(
                    ixheaac_cmplx_anal_fft_p3
                        as unsafe extern "C" fn(
                            *mut FLOAT32,
                            *mut FLOAT32,
                            WORD32,
                        ) -> VOID,
                )
                    as Option<
                        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
                    >;
            }
            16 => {
                (*ptr_hbe_txposer).synth_cos_tab = ixheaac_synth_cos_table_kl_16.as_ptr()
                    as *mut FLOAT32;
                (*ptr_hbe_txposer).analy_cos_sin_tab = ixheaac_analy_cos_sin_table_kl_32
                    .as_ptr() as *mut FLOAT32;
                (*ptr_hbe_txposer).ixheaacd_real_synth_fft = Some(
                    ixheaac_real_synth_fft_p2
                        as unsafe extern "C" fn(
                            *mut FLOAT32,
                            *mut FLOAT32,
                            WORD32,
                        ) -> VOID,
                )
                    as Option<
                        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
                    >;
                (*ptr_hbe_txposer).ixheaacd_cmplx_anal_fft = Some(
                    ixheaac_cmplx_anal_fft_p2
                        as unsafe extern "C" fn(
                            *mut FLOAT32,
                            *mut FLOAT32,
                            WORD32,
                        ) -> VOID,
                )
                    as Option<
                        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
                    >;
            }
            20 => {
                (*ptr_hbe_txposer).synth_cos_tab = ixheaac_synth_cos_table_kl_20.as_ptr()
                    as *mut FLOAT32;
                (*ptr_hbe_txposer).analy_cos_sin_tab = ixheaac_analy_cos_sin_table_kl_40
                    .as_ptr() as *mut FLOAT32;
            }
            _ => {
                (*ptr_hbe_txposer).synth_cos_tab = ixheaac_synth_cos_table_kl_4.as_ptr()
                    as *mut FLOAT32;
                (*ptr_hbe_txposer).analy_cos_sin_tab = ixheaac_analy_cos_sin_table_kl_8
                    .as_ptr() as *mut FLOAT32;
                (*ptr_hbe_txposer).ixheaacd_real_synth_fft = Some(
                    ixheaac_real_synth_fft_p2
                        as unsafe extern "C" fn(
                            *mut FLOAT32,
                            *mut FLOAT32,
                            WORD32,
                        ) -> VOID,
                )
                    as Option<
                        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
                    >;
                (*ptr_hbe_txposer).ixheaacd_cmplx_anal_fft = Some(
                    ixheaac_cmplx_anal_fft_p2
                        as unsafe extern "C" fn(
                            *mut FLOAT32,
                            *mut FLOAT32,
                            WORD32,
                        ) -> VOID,
                )
                    as Option<
                        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
                    >;
            }
        }
        (*ptr_hbe_txposer).synth_wind_coeff = ixheaacd_map_prot_filter(synth_size);
        memset(
            ((*ptr_hbe_txposer).analy_buf).as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (640 as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        synth_size = 2 as WORD32 * (*ptr_hbe_txposer).synth_size;
        (*ptr_hbe_txposer).analy_wind_coeff = ixheaacd_map_prot_filter(synth_size);
        memset(
            ((*ptr_hbe_txposer).x_over_qmf).as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (MAX_NUM_PATCHES as size_t)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
        );
        sfb = 0 as core::ffi::c_int as WORD32;
        if upsamp_4_flag != 0 {
            stop_patch = MAX_NUM_PATCHES as WORD32;
            (*ptr_hbe_txposer).max_stretch = MAX_STRETCH as WORD32;
        } else {
            stop_patch = MAX_STRETCH as WORD32;
        }
        patch = 1 as core::ffi::c_int as WORD32;
        while patch <= stop_patch {
            while sfb <= *p_num_sfb.offset(LOW as isize) as core::ffi::c_int
                && *(*p_freq_band_tab.offset(LOW as isize)).offset(sfb as isize)
                    as core::ffi::c_int <= patch * (*ptr_hbe_txposer).start_band
            {
                sfb += 1;
            }
            if sfb <= *p_num_sfb.offset(LOW as isize) as core::ffi::c_int {
                if patch as core::ffi::c_int
                    * (*ptr_hbe_txposer).start_band as core::ffi::c_int
                    - *(*p_freq_band_tab.offset(LOW as isize))
                        .offset(
                            (sfb as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_int <= 3 as core::ffi::c_int
                {
                    (*ptr_hbe_txposer)
                        .x_over_qmf[(patch as core::ffi::c_int - 1 as core::ffi::c_int)
                        as usize] = *(*p_freq_band_tab.offset(LOW as isize))
                        .offset(
                            (sfb as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        ) as WORD32;
                } else {
                    let mut sfb_0: WORD32 = 0 as WORD32;
                    while sfb_0 <= *p_num_sfb.offset(HIGH as isize) as core::ffi::c_int
                        && *(*p_freq_band_tab.offset(HIGH as isize))
                            .offset(sfb_0 as isize) as core::ffi::c_int
                            <= patch * (*ptr_hbe_txposer).start_band
                    {
                        sfb_0 += 1;
                    }
                    (*ptr_hbe_txposer)
                        .x_over_qmf[(patch as core::ffi::c_int - 1 as core::ffi::c_int)
                        as usize] = *(*p_freq_band_tab.offset(HIGH as isize))
                        .offset(
                            (sfb_0 as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        ) as WORD32;
                }
                patch += 1;
            } else {
                (*ptr_hbe_txposer)
                    .x_over_qmf[(patch as core::ffi::c_int - 1 as core::ffi::c_int)
                    as usize] = (*ptr_hbe_txposer).end_band;
                (*ptr_hbe_txposer).max_stretch = (if patch < 4 as core::ffi::c_int {
                    patch as core::ffi::c_int
                } else {
                    4 as core::ffi::c_int
                }) as WORD32;
                break;
            }
        }
        if (*ptr_hbe_txposer).k_start < 0 as core::ffi::c_int {
            return -(1 as WORD32);
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_qmf_hbe_apply(
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    mut qmf_buf_real: *mut [FLOAT32; 64],
    mut qmf_buf_imag: *mut [FLOAT32; 64],
    mut num_columns: WORD32,
    mut pv_qmf_buf_real: *mut [FLOAT32; 64],
    mut pv_qmf_buf_imag: *mut [FLOAT32; 64],
    mut pitch_in_bins: WORD32,
    mut ptr_header_data: *mut ia_sbr_header_data_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut qmf_band_idx: WORD32 = 0;
    let mut qmf_voc_columns: WORD32 = (*ptr_hbe_txposer).no_bins / 2 as WORD32;
    let mut err_code: WORD32 = 0 as WORD32;
    memcpy(
        (*ptr_hbe_txposer).ptr_input_buf as *mut core::ffi::c_void,
        ((*ptr_hbe_txposer).ptr_input_buf)
            .offset(
                ((*ptr_hbe_txposer).no_bins * (*ptr_hbe_txposer).synth_size) as isize,
            ) as *const core::ffi::c_void,
        ((*ptr_hbe_txposer).synth_size as size_t)
            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
    if ((*ptr_hbe_txposer).ixheaacd_cmplx_anal_fft).is_none() {
        let mut err: WORD32 = ixheaacd_qmf_hbe_data_reinit(
            ptr_hbe_txposer,
            ((*(*ptr_header_data).pstr_freq_band_data).freq_band_table).as_mut_ptr(),
            ((*(*ptr_header_data).pstr_freq_band_data).num_sf_bands).as_mut_ptr(),
            (*ptr_header_data).is_usf_4,
        );
        if err != 0 {
            return err;
        }
    }
    err_code = ixheaacd_real_synth_filt(
        ptr_hbe_txposer,
        num_columns,
        qmf_buf_real,
        qmf_buf_imag,
    );
    if err_code != 0 {
        return err_code;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < HBE_OPER_WIN_LEN - 1 as core::ffi::c_int {
        memcpy(
            *((*ptr_hbe_txposer).qmf_in_buf).offset(i as isize)
                as *mut core::ffi::c_void,
            *((*ptr_hbe_txposer).qmf_in_buf).offset((i + qmf_voc_columns) as isize)
                as *const core::ffi::c_void,
            (TWICE_QMF_SYNTH_CHANNELS_NUM as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        i += 1;
    }
    err_code = ixheaacd_complex_anal_filt(ptr_hbe_txposer);
    if err_code != 0 {
        return err_code;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*ptr_hbe_txposer).hbe_qmf_out_len - (*ptr_hbe_txposer).no_bins {
        memcpy(
            *((*ptr_hbe_txposer).qmf_out_buf).offset(i as isize)
                as *mut core::ffi::c_void,
            *((*ptr_hbe_txposer).qmf_out_buf)
                .offset((i + (*ptr_hbe_txposer).no_bins) as isize)
                as *const core::ffi::c_void,
            (TWICE_QMF_SYNTH_CHANNELS_NUM as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        i += 1;
    }
    while i < (*ptr_hbe_txposer).hbe_qmf_out_len {
        memset(
            *((*ptr_hbe_txposer).qmf_out_buf).offset(i as isize)
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (TWICE_QMF_SYNTH_CHANNELS_NUM as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        i += 1;
    }
    err_code = ixheaacd_hbe_post_anal_process(
        ptr_hbe_txposer,
        pitch_in_bins,
        (*ptr_hbe_txposer).upsamp_4_flag,
    ) as WORD32;
    if err_code != 0 {
        return err_code;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*ptr_hbe_txposer).no_bins {
        qmf_band_idx = (*ptr_hbe_txposer).start_band;
        while qmf_band_idx < (*ptr_hbe_txposer).end_band {
            (*pv_qmf_buf_real.offset(i as isize))[qmf_band_idx as usize] = *(*((*ptr_hbe_txposer)
                .qmf_out_buf)
                .offset(i as isize))
                .offset((2 as WORD32 * qmf_band_idx) as isize)
                * ixheaac_phase_vocoder_cos_table[qmf_band_idx as usize]
                - *(*((*ptr_hbe_txposer).qmf_out_buf).offset(i as isize))
                    .offset(
                        (2 as core::ffi::c_int * qmf_band_idx as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    ) * ixheaac_phase_vocoder_sin_table[qmf_band_idx as usize];
            (*pv_qmf_buf_imag.offset(i as isize))[qmf_band_idx as usize] = *(*((*ptr_hbe_txposer)
                .qmf_out_buf)
                .offset(i as isize))
                .offset((2 as WORD32 * qmf_band_idx) as isize)
                * ixheaac_phase_vocoder_sin_table[qmf_band_idx as usize]
                + *(*((*ptr_hbe_txposer).qmf_out_buf).offset(i as isize))
                    .offset(
                        (2 as core::ffi::c_int * qmf_band_idx as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    ) * ixheaac_phase_vocoder_cos_table[qmf_band_idx as usize];
            qmf_band_idx += 1;
        }
        i += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_norm_qmf_in_buf_4(
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    mut qmf_band_idx: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut in_buf: *mut FLOAT32 = &mut *(*((*ptr_hbe_txposer).qmf_in_buf)
        .offset(0 as core::ffi::c_int as isize))
        .offset((2 as WORD32 * qmf_band_idx) as isize) as *mut FLOAT32;
    let mut norm_buf: *mut FLOAT32 = &mut *(*((*ptr_hbe_txposer).norm_qmf_in_buf)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset((2 as WORD32 * qmf_band_idx) as isize) as *mut FLOAT32;
    while qmf_band_idx <= (*ptr_hbe_txposer).x_over_qmf[3 as core::ffi::c_int as usize] {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*ptr_hbe_txposer).hbe_qmf_in_len {
            let mut mag_scaling_fac: FLOAT32 = 0.0f32;
            let mut x_r: FLOAT32 = 0.;
            let mut x_i: FLOAT32 = 0.;
            let mut temp: FLOAT32 = 0.;
            let mut base: FLOAT64 = 1e-17f64;
            x_r = *in_buf.offset(0 as core::ffi::c_int as isize);
            x_i = *in_buf.offset(1 as core::ffi::c_int as isize);
            temp = x_r * x_r;
            base = base + temp as FLOAT64;
            temp = x_i * x_i;
            base = base + temp as FLOAT64;
            temp = sqrt(sqrt(base as core::ffi::c_double)) as FLOAT32;
            mag_scaling_fac = temp * sqrt(temp as core::ffi::c_double) as FLOAT32;
            mag_scaling_fac = 1 as core::ffi::c_int as FLOAT32 / mag_scaling_fac;
            x_r *= mag_scaling_fac;
            x_i *= mag_scaling_fac;
            *norm_buf.offset(0 as core::ffi::c_int as isize) = x_r;
            *norm_buf.offset(1 as core::ffi::c_int as isize) = x_i;
            in_buf = in_buf.offset(128 as core::ffi::c_int as isize);
            norm_buf = norm_buf.offset(128 as core::ffi::c_int as isize);
            i += 1;
        }
        in_buf = in_buf
            .offset(
                -((128 as core::ffi::c_int
                    * (*ptr_hbe_txposer).hbe_qmf_in_len as core::ffi::c_int
                    - 2 as core::ffi::c_int) as isize),
            );
        norm_buf = norm_buf
            .offset(
                -((128 as core::ffi::c_int
                    * (*ptr_hbe_txposer).hbe_qmf_in_len as core::ffi::c_int
                    - 2 as core::ffi::c_int) as isize),
            );
        qmf_band_idx += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_norm_qmf_in_buf_2(
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    mut qmf_band_idx: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut in_buf: *mut FLOAT32 = &mut *(*((*ptr_hbe_txposer).qmf_in_buf)
        .offset(0 as core::ffi::c_int as isize))
        .offset((2 as WORD32 * qmf_band_idx) as isize) as *mut FLOAT32;
    let mut norm_buf: *mut FLOAT32 = &mut *(*((*ptr_hbe_txposer).norm_qmf_in_buf)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset((2 as WORD32 * qmf_band_idx) as isize) as *mut FLOAT32;
    while qmf_band_idx <= (*ptr_hbe_txposer).x_over_qmf[1 as core::ffi::c_int as usize] {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*ptr_hbe_txposer).hbe_qmf_in_len {
            let mut mag_scaling_fac: FLOAT32 = 0.0f32;
            let mut x_r: FLOAT32 = 0.;
            let mut x_i: FLOAT32 = 0.;
            let mut temp: FLOAT32 = 0.;
            let mut base: FLOAT64 = 1e-17f64;
            x_r = *in_buf.offset(0 as core::ffi::c_int as isize);
            x_i = *in_buf.offset(1 as core::ffi::c_int as isize);
            temp = x_r * x_r;
            base = base + temp as FLOAT64;
            temp = x_i * x_i;
            base = base + (x_i * x_i) as FLOAT64;
            mag_scaling_fac = (1.0f64 / base) as FLOAT32;
            mag_scaling_fac = sqrt(sqrt(mag_scaling_fac as core::ffi::c_double))
                as FLOAT32;
            x_r *= mag_scaling_fac;
            x_i *= mag_scaling_fac;
            *norm_buf.offset(0 as core::ffi::c_int as isize) = x_r;
            *norm_buf.offset(1 as core::ffi::c_int as isize) = x_i;
            in_buf = in_buf.offset(128 as core::ffi::c_int as isize);
            norm_buf = norm_buf.offset(128 as core::ffi::c_int as isize);
            i += 1;
        }
        in_buf = in_buf
            .offset(
                -((128 as core::ffi::c_int
                    * (*ptr_hbe_txposer).hbe_qmf_in_len as core::ffi::c_int
                    - 2 as core::ffi::c_int) as isize),
            );
        norm_buf = norm_buf
            .offset(
                -((128 as core::ffi::c_int
                    * (*ptr_hbe_txposer).hbe_qmf_in_len as core::ffi::c_int
                    - 2 as core::ffi::c_int) as isize),
            );
        qmf_band_idx += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_xprod_proc_3(
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    mut qmf_band_idx: WORD32,
    mut qmf_col_idx: WORD32,
    mut p: FLOAT32,
    mut pitch_in_bins_idx: WORD32,
) -> VOID {
    let mut tr: WORD32 = 0;
    let mut n1: WORD32 = 0;
    let mut n2: WORD32 = 0;
    let mut max_trans_fac: WORD32 = 0;
    let mut max_n1: WORD32 = 0;
    let mut max_n2: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut addrshift: WORD32 = 0;
    let mut inp_band_idx: WORD32 = 2 as WORD32 * qmf_band_idx / 3 as WORD32;
    let mut temp_fac: FLOAT64 = 0.;
    let mut max_mag_value: FLOAT32 = 0.;
    let mut mag_zero_band: FLOAT32 = 0.;
    let mut mag_n1_band: FLOAT32 = 0.;
    let mut mag_n2_band: FLOAT32 = 0.;
    let mut temp: FLOAT32 = 0.;
    let mut temp_r: FLOAT32 = 0.;
    let mut temp_i: FLOAT32 = 0.;
    let mut mag_cmplx_gain: FLOAT32 = 1.8856f32;
    let mut qmf_in_buf_ri: *mut FLOAT32 = *((*ptr_hbe_txposer).qmf_in_buf)
        .offset((qmf_col_idx as core::ffi::c_int + HBE_ZERO_BAND_IDX) as isize);
    mag_zero_band = *qmf_in_buf_ri.offset((2 as WORD32 * inp_band_idx) as isize)
        * *qmf_in_buf_ri.offset((2 as WORD32 * inp_band_idx) as isize)
        + *qmf_in_buf_ri
            .offset(
                (2 as core::ffi::c_int * inp_band_idx as core::ffi::c_int
                    + 1 as core::ffi::c_int) as isize,
            )
            * *qmf_in_buf_ri
                .offset(
                    (2 as core::ffi::c_int * inp_band_idx as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                );
    max_mag_value = 0 as core::ffi::c_int as FLOAT32;
    max_trans_fac = 0 as core::ffi::c_int as WORD32;
    max_n2 = max_trans_fac;
    max_n1 = max_n2;
    tr = 1 as core::ffi::c_int as WORD32;
    while tr < 3 as core::ffi::c_int {
        temp_fac = ((2.0f32 * qmf_band_idx as FLOAT32 + 1 as core::ffi::c_int as FLOAT32
            - tr as FLOAT32 * p) as core::ffi::c_double * 0.3333334f64) as FLOAT64;
        n1 = temp_fac as WORD32;
        n2 = (temp_fac + p as FLOAT64) as WORD32;
        mag_n1_band = *qmf_in_buf_ri.offset((2 as WORD32 * n1) as isize)
            * *qmf_in_buf_ri.offset((2 as WORD32 * n1) as isize)
            + *qmf_in_buf_ri
                .offset(
                    (2 as core::ffi::c_int * n1 as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                )
                * *qmf_in_buf_ri
                    .offset(
                        (2 as core::ffi::c_int * n1 as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    );
        mag_n2_band = *qmf_in_buf_ri.offset((2 as WORD32 * n2) as isize)
            * *qmf_in_buf_ri.offset((2 as WORD32 * n2) as isize)
            + *qmf_in_buf_ri
                .offset(
                    (2 as core::ffi::c_int * n2 as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                )
                * *qmf_in_buf_ri
                    .offset(
                        (2 as core::ffi::c_int * n2 as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    );
        temp = if mag_n1_band < mag_n2_band { mag_n1_band } else { mag_n2_band };
        if temp > max_mag_value {
            max_mag_value = temp;
            max_trans_fac = tr;
            max_n1 = n1;
            max_n2 = n2;
        }
        tr += 1;
    }
    if max_mag_value > mag_zero_band && max_n1 >= 0 as core::ffi::c_int
        && max_n2 < NO_QMF_SYNTH_CHANNELS
    {
        let mut vec_y_r: [FLOAT32; 2] = [0.; 2];
        let mut vec_y_i: [FLOAT32; 2] = [0.; 2];
        let mut vec_o_r: [FLOAT32; 2] = [0.; 2];
        let mut vec_o_i: [FLOAT32; 2] = [0.; 2];
        let mut coeff_real: [FLOAT32; 2] = [0.; 2];
        let mut coeff_imag: [FLOAT32; 2] = [0.; 2];
        let mut d1: FLOAT32 = 0.;
        let mut d2: FLOAT32 = 0.;
        let mut mid_trans_fac: WORD32 = 0;
        let mut idx: WORD32 = 0;
        let mut base: FLOAT64 = 1e-17f64;
        let mut mag_scaling_fac: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
        let mut x_zero_band_r: FLOAT32 = 0.;
        let mut x_zero_band_i: FLOAT32 = 0.;
        x_zero_band_r = 0 as core::ffi::c_int as FLOAT32;
        x_zero_band_i = 0 as core::ffi::c_int as FLOAT32;
        mid_trans_fac = 3 as WORD32 - max_trans_fac;
        if max_trans_fac == 1 as core::ffi::c_int {
            let mut idx_0: WORD32 = 0;
            d1 = 0 as core::ffi::c_int as FLOAT32;
            d2 = 1.5f32;
            x_zero_band_r = *qmf_in_buf_ri.offset((2 as WORD32 * max_n1) as isize);
            x_zero_band_i = *qmf_in_buf_ri
                .offset(
                    (2 as core::ffi::c_int * max_n1 as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                );
            idx_0 = (max_n2 as core::ffi::c_int & 3 as core::ffi::c_int) as WORD32;
            idx_0 = (idx_0 as core::ffi::c_int + 1 as core::ffi::c_int
                & 3 as core::ffi::c_int) as WORD32;
            coeff_real[0 as core::ffi::c_int as usize] = ixheaac_hbe_post_anal_proc_interp_coeff[idx_0
                as usize][0 as core::ffi::c_int as usize];
            coeff_imag[0 as core::ffi::c_int as usize] = ixheaac_hbe_post_anal_proc_interp_coeff[idx_0
                as usize][1 as core::ffi::c_int as usize];
            coeff_real[1 as core::ffi::c_int as usize] = coeff_real[0 as core::ffi::c_int
                as usize];
            coeff_imag[1 as core::ffi::c_int as usize] = -coeff_imag[0
                as core::ffi::c_int as usize];
            vec_y_r[1 as core::ffi::c_int as usize] = *qmf_in_buf_ri
                .offset((2 as WORD32 * max_n2) as isize);
            vec_y_i[1 as core::ffi::c_int as usize] = *qmf_in_buf_ri
                .offset(
                    (2 as core::ffi::c_int * max_n2 as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                );
            addrshift = -(2 as core::ffi::c_int) as WORD32;
            temp_r = *(*((*ptr_hbe_txposer).qmf_in_buf)
                .offset(
                    (qmf_col_idx as core::ffi::c_int + addrshift as core::ffi::c_int
                        + HBE_ZERO_BAND_IDX) as isize,
                ))
                .offset((2 as WORD32 * max_n2) as isize);
            temp_i = *(*((*ptr_hbe_txposer).qmf_in_buf)
                .offset(
                    (qmf_col_idx as core::ffi::c_int + addrshift as core::ffi::c_int
                        + HBE_ZERO_BAND_IDX) as isize,
                ))
                .offset(
                    (2 as core::ffi::c_int * max_n2 as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                );
            vec_y_r[0 as core::ffi::c_int as usize] = coeff_real[1 as core::ffi::c_int
                as usize] * temp_r - coeff_imag[1 as core::ffi::c_int as usize] * temp_i;
            vec_y_i[0 as core::ffi::c_int as usize] = coeff_imag[1 as core::ffi::c_int
                as usize] * temp_r + coeff_real[1 as core::ffi::c_int as usize] * temp_i;
            temp_r = *(*((*ptr_hbe_txposer).qmf_in_buf)
                .offset(
                    (qmf_col_idx as core::ffi::c_int + addrshift as core::ffi::c_int
                        + 1 as core::ffi::c_int + HBE_ZERO_BAND_IDX) as isize,
                ))
                .offset((2 as WORD32 * max_n2) as isize);
            temp_i = *(*((*ptr_hbe_txposer).qmf_in_buf)
                .offset(
                    (qmf_col_idx as core::ffi::c_int + addrshift as core::ffi::c_int
                        + 1 as core::ffi::c_int + HBE_ZERO_BAND_IDX) as isize,
                ))
                .offset(
                    (2 as core::ffi::c_int * max_n2 as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                );
            vec_y_r[0 as core::ffi::c_int as usize]
                += coeff_real[0 as core::ffi::c_int as usize] * temp_r
                    - coeff_imag[0 as core::ffi::c_int as usize] * temp_i;
            vec_y_i[0 as core::ffi::c_int as usize]
                += coeff_imag[0 as core::ffi::c_int as usize] * temp_r
                    + coeff_real[0 as core::ffi::c_int as usize] * temp_i;
        } else {
            let mut idx_1: WORD32 = 0;
            d1 = 1.5f32;
            d2 = 0 as core::ffi::c_int as FLOAT32;
            mid_trans_fac = max_trans_fac;
            max_trans_fac = 3 as WORD32 - max_trans_fac;
            x_zero_band_r = *qmf_in_buf_ri.offset((2 as WORD32 * max_n2) as isize);
            x_zero_band_i = *qmf_in_buf_ri
                .offset(
                    (2 as core::ffi::c_int * max_n2 as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                );
            idx_1 = (max_n1 as core::ffi::c_int & 3 as core::ffi::c_int) as WORD32;
            idx_1 = (idx_1 as core::ffi::c_int + 1 as core::ffi::c_int
                & 3 as core::ffi::c_int) as WORD32;
            coeff_real[0 as core::ffi::c_int as usize] = ixheaac_hbe_post_anal_proc_interp_coeff[idx_1
                as usize][0 as core::ffi::c_int as usize];
            coeff_imag[0 as core::ffi::c_int as usize] = ixheaac_hbe_post_anal_proc_interp_coeff[idx_1
                as usize][1 as core::ffi::c_int as usize];
            coeff_real[1 as core::ffi::c_int as usize] = coeff_real[0 as core::ffi::c_int
                as usize];
            coeff_imag[1 as core::ffi::c_int as usize] = -coeff_imag[0
                as core::ffi::c_int as usize];
            vec_y_r[1 as core::ffi::c_int as usize] = *qmf_in_buf_ri
                .offset((2 as WORD32 * max_n1) as isize);
            vec_y_i[1 as core::ffi::c_int as usize] = *qmf_in_buf_ri
                .offset(
                    (2 as core::ffi::c_int * max_n1 as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                );
            addrshift = -(2 as core::ffi::c_int) as WORD32;
            temp_r = *(*((*ptr_hbe_txposer).qmf_in_buf)
                .offset(
                    (qmf_col_idx as core::ffi::c_int + addrshift as core::ffi::c_int
                        + HBE_ZERO_BAND_IDX) as isize,
                ))
                .offset((2 as WORD32 * max_n1) as isize);
            temp_i = *(*((*ptr_hbe_txposer).qmf_in_buf)
                .offset(
                    (qmf_col_idx as core::ffi::c_int + addrshift as core::ffi::c_int
                        + HBE_ZERO_BAND_IDX) as isize,
                ))
                .offset(
                    (2 as core::ffi::c_int * max_n1 as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                );
            vec_y_r[0 as core::ffi::c_int as usize] = coeff_real[1 as core::ffi::c_int
                as usize] * temp_r - coeff_imag[1 as core::ffi::c_int as usize] * temp_i;
            vec_y_i[0 as core::ffi::c_int as usize] = coeff_imag[1 as core::ffi::c_int
                as usize] * temp_r + coeff_real[1 as core::ffi::c_int as usize] * temp_i;
            temp_r = *(*((*ptr_hbe_txposer).qmf_in_buf)
                .offset(
                    (qmf_col_idx as core::ffi::c_int + addrshift as core::ffi::c_int
                        + 1 as core::ffi::c_int + HBE_ZERO_BAND_IDX) as isize,
                ))
                .offset((2 as WORD32 * max_n1) as isize);
            temp_i = *(*((*ptr_hbe_txposer).qmf_in_buf)
                .offset(
                    (qmf_col_idx as core::ffi::c_int + addrshift as core::ffi::c_int
                        + 1 as core::ffi::c_int + HBE_ZERO_BAND_IDX) as isize,
                ))
                .offset(
                    (2 as core::ffi::c_int * max_n1 as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                );
            vec_y_r[0 as core::ffi::c_int as usize]
                += coeff_real[0 as core::ffi::c_int as usize] * temp_r
                    - coeff_imag[0 as core::ffi::c_int as usize] * temp_i;
            vec_y_i[0 as core::ffi::c_int as usize]
                += coeff_imag[0 as core::ffi::c_int as usize] * temp_r
                    + coeff_real[0 as core::ffi::c_int as usize] * temp_i;
        }
        base = 1e-17f64 as FLOAT64;
        base = base + (x_zero_band_r * x_zero_band_r) as FLOAT64;
        base = base + (x_zero_band_i * x_zero_band_i) as FLOAT64;
        mag_scaling_fac = cbrt((1.0f32 / base as FLOAT32) as core::ffi::c_double)
            as FLOAT32;
        x_zero_band_r *= mag_scaling_fac;
        x_zero_band_i *= mag_scaling_fac;
        k = 0 as core::ffi::c_int as WORD32;
        while k < 2 as core::ffi::c_int {
            base = 1e-17f64 as FLOAT64;
            base = base + (vec_y_r[k as usize] * vec_y_r[k as usize]) as FLOAT64;
            base = base + (vec_y_i[k as usize] * vec_y_i[k as usize]) as FLOAT64;
            mag_scaling_fac = cbrt((1.0f32 / base as FLOAT32) as core::ffi::c_double)
                as FLOAT32;
            vec_y_r[k as usize] *= mag_scaling_fac;
            vec_y_i[k as usize] *= mag_scaling_fac;
            k += 1;
        }
        temp_r = x_zero_band_r;
        temp_i = x_zero_band_i;
        idx = 0 as core::ffi::c_int as WORD32;
        while idx < mid_trans_fac as core::ffi::c_int - 1 as core::ffi::c_int {
            let mut tmp: FLOAT32 = x_zero_band_r;
            x_zero_band_r = x_zero_band_r * temp_r - x_zero_band_i * temp_i;
            x_zero_band_i = tmp * temp_i + x_zero_band_i * temp_r;
            idx += 1;
        }
        k = 0 as core::ffi::c_int as WORD32;
        while k < 2 as core::ffi::c_int {
            temp_r = vec_y_r[k as usize];
            temp_i = vec_y_i[k as usize];
            idx = 0 as core::ffi::c_int as WORD32;
            while idx < max_trans_fac as core::ffi::c_int - 1 as core::ffi::c_int {
                let mut tmp_0: FLOAT32 = vec_y_r[k as usize];
                vec_y_r[k as usize] = vec_y_r[k as usize] * temp_r
                    - vec_y_i[k as usize] * temp_i;
                vec_y_i[k as usize] = tmp_0 * temp_i + vec_y_i[k as usize] * temp_r;
                idx += 1;
            }
            k += 1;
        }
        k = 0 as core::ffi::c_int as WORD32;
        while k < 2 as core::ffi::c_int {
            vec_o_r[k as usize] = vec_y_r[k as usize] * x_zero_band_r
                - vec_y_i[k as usize] * x_zero_band_i;
            vec_o_i[k as usize] = vec_y_r[k as usize] * x_zero_band_i
                + vec_y_i[k as usize] * x_zero_band_r;
            k += 1;
        }
        let mut cos_theta: FLOAT32 = ixheaac_hbe_x_prod_cos_table_trans_3[(((pitch_in_bins_idx
            as core::ffi::c_int) << 1 as core::ffi::c_int) + 0 as core::ffi::c_int)
            as usize];
        let mut sin_theta: FLOAT32 = ixheaac_hbe_x_prod_cos_table_trans_3[(((pitch_in_bins_idx
            as core::ffi::c_int) << 1 as core::ffi::c_int) + 1 as core::ffi::c_int)
            as usize];
        if d2 < d1 {
            sin_theta = -sin_theta;
        }
        temp_r = vec_o_r[0 as core::ffi::c_int as usize];
        temp_i = vec_o_i[0 as core::ffi::c_int as usize];
        vec_o_r[0 as core::ffi::c_int as usize] = cos_theta * temp_r
            - sin_theta * temp_i;
        vec_o_i[0 as core::ffi::c_int as usize] = cos_theta * temp_i
            + sin_theta * temp_r;
        k = 0 as core::ffi::c_int as WORD32;
        while k < 2 as core::ffi::c_int {
            *(*((*ptr_hbe_txposer).qmf_out_buf)
                .offset(
                    (qmf_col_idx as core::ffi::c_int * 2 as core::ffi::c_int
                        + (k as core::ffi::c_int + HBE_ZERO_BAND_IDX
                            - 1 as core::ffi::c_int)) as isize,
                ))
                .offset((2 as WORD32 * qmf_band_idx) as isize)
                += mag_cmplx_gain * vec_o_r[k as usize];
            *(*((*ptr_hbe_txposer).qmf_out_buf)
                .offset(
                    (qmf_col_idx as core::ffi::c_int * 2 as core::ffi::c_int
                        + (k as core::ffi::c_int + HBE_ZERO_BAND_IDX
                            - 1 as core::ffi::c_int)) as isize,
                ))
                .offset(
                    (2 as core::ffi::c_int * qmf_band_idx as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) += mag_cmplx_gain * vec_o_i[k as usize];
            k += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_xprod_proc_4(
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    mut qmf_band_idx: WORD32,
    mut qmf_col_idx: WORD32,
    mut p: FLOAT32,
    mut pitch_in_bins_idx: WORD32,
) -> VOID {
    let mut k: WORD32 = 0;
    let mut inp_band_idx: WORD32 = qmf_band_idx >> 1 as core::ffi::c_int;
    let mut tr: WORD32 = 0;
    let mut n1: WORD32 = 0;
    let mut n2: WORD32 = 0;
    let mut max_trans_fac: WORD32 = 0;
    let mut max_n1: WORD32 = 0;
    let mut max_n2: WORD32 = 0;
    let mut temp_fac: FLOAT64 = 0.;
    let mut max_mag_value: FLOAT32 = 0.;
    let mut mag_zero_band: FLOAT32 = 0.;
    let mut mag_n1_band: FLOAT32 = 0.;
    let mut mag_n2_band: FLOAT32 = 0.;
    let mut temp: FLOAT32 = 0.;
    let mut temp_r: FLOAT32 = 0.;
    let mut temp_i: FLOAT32 = 0.;
    let mut mag_cmplx_gain: FLOAT32 = 2.0f32;
    let mut qmf_in_buf_ri: *mut FLOAT32 = *((*ptr_hbe_txposer).qmf_in_buf)
        .offset((qmf_col_idx as core::ffi::c_int + HBE_ZERO_BAND_IDX) as isize);
    mag_zero_band = *qmf_in_buf_ri.offset((2 as WORD32 * inp_band_idx) as isize)
        * *qmf_in_buf_ri.offset((2 as WORD32 * inp_band_idx) as isize)
        + *qmf_in_buf_ri
            .offset(
                (2 as core::ffi::c_int * inp_band_idx as core::ffi::c_int
                    + 1 as core::ffi::c_int) as isize,
            )
            * *qmf_in_buf_ri
                .offset(
                    (2 as core::ffi::c_int * inp_band_idx as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                );
    max_mag_value = 0 as core::ffi::c_int as FLOAT32;
    max_trans_fac = 0 as core::ffi::c_int as WORD32;
    max_n2 = max_trans_fac;
    max_n1 = max_n2;
    tr = 1 as core::ffi::c_int as WORD32;
    while tr < 4 as core::ffi::c_int {
        temp_fac = ((2.0f64 * qmf_band_idx as core::ffi::c_double
            + 1 as core::ffi::c_int as core::ffi::c_double
            - (tr as FLOAT32 * p) as core::ffi::c_double) * 0.25f64) as FLOAT64;
        n1 = (temp_fac as WORD32) << 1 as core::ffi::c_int;
        n2 = ((temp_fac + p as FLOAT64) as WORD32) << 1 as core::ffi::c_int;
        mag_n1_band = *qmf_in_buf_ri.offset(n1 as isize)
            * *qmf_in_buf_ri.offset(n1 as isize)
            + *qmf_in_buf_ri
                .offset((n1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                * *qmf_in_buf_ri
                    .offset((n1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
        mag_n2_band = *qmf_in_buf_ri.offset(n2 as isize)
            * *qmf_in_buf_ri.offset(n2 as isize)
            + *qmf_in_buf_ri
                .offset((n2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                * *qmf_in_buf_ri
                    .offset((n2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
        temp = if mag_n1_band < mag_n2_band { mag_n1_band } else { mag_n2_band };
        if temp > max_mag_value {
            max_mag_value = temp;
            max_trans_fac = tr;
            max_n1 = n1;
            max_n2 = n2;
        }
        tr += 1;
    }
    if max_mag_value > mag_zero_band && max_n1 >= 0 as core::ffi::c_int
        && max_n2 < TWICE_QMF_SYNTH_CHANNELS_NUM
    {
        let mut vec_y_r: [FLOAT32; 2] = [0.; 2];
        let mut vec_y_i: [FLOAT32; 2] = [0.; 2];
        let mut vec_o_r: [FLOAT32; 2] = [0.; 2];
        let mut vec_o_i: [FLOAT32; 2] = [0.; 2];
        let mut d1: FLOAT32 = 0.;
        let mut d2: FLOAT32 = 0.;
        let mut mid_trans_fac: WORD32 = 0;
        let mut idx: WORD32 = 0;
        let mut x_zero_band_r: FLOAT32 = 0.;
        let mut x_zero_band_i: FLOAT32 = 0.;
        let mut base: FLOAT64 = 1e-17f64;
        let mut mag_scaling_fac: FLOAT32 = 0.0f32;
        x_zero_band_r = 0 as core::ffi::c_int as FLOAT32;
        x_zero_band_i = 0 as core::ffi::c_int as FLOAT32;
        mid_trans_fac = 4 as WORD32 - max_trans_fac;
        if max_trans_fac == 1 as core::ffi::c_int {
            d1 = 0 as core::ffi::c_int as FLOAT32;
            d2 = 2 as core::ffi::c_int as FLOAT32;
            x_zero_band_r = *qmf_in_buf_ri.offset(max_n1 as isize);
            x_zero_band_i = *qmf_in_buf_ri
                .offset((max_n1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            k = 0 as core::ffi::c_int as WORD32;
            while k < 2 as core::ffi::c_int {
                vec_y_r[k as usize] = *(*((*ptr_hbe_txposer).qmf_in_buf)
                    .offset(
                        (qmf_col_idx as core::ffi::c_int + HBE_ZERO_BAND_IDX
                            + 2 as core::ffi::c_int
                                * (k as core::ffi::c_int - 1 as core::ffi::c_int)) as isize,
                    ))
                    .offset(max_n2 as isize);
                vec_y_i[k as usize] = *(*((*ptr_hbe_txposer).qmf_in_buf)
                    .offset(
                        (qmf_col_idx as core::ffi::c_int + HBE_ZERO_BAND_IDX
                            + 2 as core::ffi::c_int
                                * (k as core::ffi::c_int - 1 as core::ffi::c_int)) as isize,
                    ))
                    .offset(
                        (max_n2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                    );
                k += 1;
            }
        } else if max_trans_fac == 2 as core::ffi::c_int {
            d1 = 0 as core::ffi::c_int as FLOAT32;
            d2 = 1 as core::ffi::c_int as FLOAT32;
            x_zero_band_r = *qmf_in_buf_ri.offset(max_n1 as isize);
            x_zero_band_i = *qmf_in_buf_ri
                .offset((max_n1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            k = 0 as core::ffi::c_int as WORD32;
            while k < 2 as core::ffi::c_int {
                vec_y_r[k as usize] = *(*((*ptr_hbe_txposer).qmf_in_buf)
                    .offset(
                        (qmf_col_idx as core::ffi::c_int + HBE_ZERO_BAND_IDX
                            + (k as core::ffi::c_int - 1 as core::ffi::c_int)) as isize,
                    ))
                    .offset(max_n2 as isize);
                vec_y_i[k as usize] = *(*((*ptr_hbe_txposer).qmf_in_buf)
                    .offset(
                        (qmf_col_idx as core::ffi::c_int + HBE_ZERO_BAND_IDX
                            + (k as core::ffi::c_int - 1 as core::ffi::c_int)) as isize,
                    ))
                    .offset(
                        (max_n2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                    );
                k += 1;
            }
        } else {
            d1 = 2 as core::ffi::c_int as FLOAT32;
            d2 = 0 as core::ffi::c_int as FLOAT32;
            mid_trans_fac = max_trans_fac;
            max_trans_fac = 4 as WORD32 - max_trans_fac;
            x_zero_band_r = *qmf_in_buf_ri.offset(max_n2 as isize);
            x_zero_band_i = *qmf_in_buf_ri
                .offset((max_n2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            k = 0 as core::ffi::c_int as WORD32;
            while k < 2 as core::ffi::c_int {
                vec_y_r[k as usize] = *(*((*ptr_hbe_txposer).qmf_in_buf)
                    .offset(
                        (qmf_col_idx as core::ffi::c_int + HBE_ZERO_BAND_IDX
                            + 2 as core::ffi::c_int
                                * (k as core::ffi::c_int - 1 as core::ffi::c_int)) as isize,
                    ))
                    .offset(max_n1 as isize);
                vec_y_i[k as usize] = *(*((*ptr_hbe_txposer).qmf_in_buf)
                    .offset(
                        (qmf_col_idx as core::ffi::c_int + HBE_ZERO_BAND_IDX
                            + 2 as core::ffi::c_int
                                * (k as core::ffi::c_int - 1 as core::ffi::c_int)) as isize,
                    ))
                    .offset(
                        (max_n1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                    );
                k += 1;
            }
        }
        base = 1e-17f64 as FLOAT64;
        base = base + (x_zero_band_r * x_zero_band_r) as FLOAT64;
        base = base + (x_zero_band_i * x_zero_band_i) as FLOAT64;
        temp = sqrt(sqrt(base as core::ffi::c_double)) as FLOAT32;
        mag_scaling_fac = temp * sqrt(temp as core::ffi::c_double) as FLOAT32;
        mag_scaling_fac = 1 as core::ffi::c_int as FLOAT32 / mag_scaling_fac;
        x_zero_band_r *= mag_scaling_fac;
        x_zero_band_i *= mag_scaling_fac;
        k = 0 as core::ffi::c_int as WORD32;
        while k < 2 as core::ffi::c_int {
            base = 1e-17f64 as FLOAT64;
            base = base + (vec_y_r[k as usize] * vec_y_r[k as usize]) as FLOAT64;
            base = base + (vec_y_i[k as usize] * vec_y_i[k as usize]) as FLOAT64;
            temp = sqrt(sqrt(base as core::ffi::c_double)) as FLOAT32;
            mag_scaling_fac = temp * sqrt(temp as core::ffi::c_double) as FLOAT32;
            mag_scaling_fac = 1 as core::ffi::c_int as FLOAT32 / mag_scaling_fac;
            vec_y_r[k as usize] *= mag_scaling_fac;
            vec_y_i[k as usize] *= mag_scaling_fac;
            k += 1;
        }
        temp_r = x_zero_band_r;
        temp_i = x_zero_band_i;
        idx = 0 as core::ffi::c_int as WORD32;
        while idx < mid_trans_fac as core::ffi::c_int - 1 as core::ffi::c_int {
            let mut tmp: FLOAT32 = x_zero_band_r;
            x_zero_band_r = x_zero_band_r * temp_r - x_zero_band_i * temp_i;
            x_zero_band_i = tmp * temp_i + x_zero_band_i * temp_r;
            idx += 1;
        }
        k = 0 as core::ffi::c_int as WORD32;
        while k < 2 as core::ffi::c_int {
            temp_r = vec_y_r[k as usize];
            temp_i = vec_y_i[k as usize];
            idx = 0 as core::ffi::c_int as WORD32;
            while idx < max_trans_fac as core::ffi::c_int - 1 as core::ffi::c_int {
                let mut tmp_0: FLOAT32 = vec_y_r[k as usize];
                vec_y_r[k as usize] = vec_y_r[k as usize] * temp_r
                    - vec_y_i[k as usize] * temp_i;
                vec_y_i[k as usize] = tmp_0 * temp_i + vec_y_i[k as usize] * temp_r;
                idx += 1;
            }
            k += 1;
        }
        k = 0 as core::ffi::c_int as WORD32;
        while k < 2 as core::ffi::c_int {
            vec_o_r[k as usize] = vec_y_r[k as usize] * x_zero_band_r
                - vec_y_i[k as usize] * x_zero_band_i;
            vec_o_i[k as usize] = vec_y_r[k as usize] * x_zero_band_i
                + vec_y_i[k as usize] * x_zero_band_r;
            k += 1;
        }
        let mut cos_theta: FLOAT32 = 0.;
        let mut sin_theta: FLOAT32 = 0.;
        if d2 == 1 as core::ffi::c_int as FLOAT32 {
            cos_theta = ixheaac_hbe_x_prod_cos_table_trans_4_1[(((pitch_in_bins_idx
                as core::ffi::c_int) << 1 as core::ffi::c_int) + 0 as core::ffi::c_int)
                as usize];
            sin_theta = ixheaac_hbe_x_prod_cos_table_trans_4_1[(((pitch_in_bins_idx
                as core::ffi::c_int) << 1 as core::ffi::c_int) + 1 as core::ffi::c_int)
                as usize];
        } else {
            cos_theta = ixheaac_hbe_x_prod_cos_table_trans_4[(((pitch_in_bins_idx
                as core::ffi::c_int) << 1 as core::ffi::c_int) + 0 as core::ffi::c_int)
                as usize];
            sin_theta = ixheaac_hbe_x_prod_cos_table_trans_4[(((pitch_in_bins_idx
                as core::ffi::c_int) << 1 as core::ffi::c_int) + 1 as core::ffi::c_int)
                as usize];
            if d2 < d1 {
                sin_theta = -sin_theta;
            }
        }
        temp_r = vec_o_r[0 as core::ffi::c_int as usize];
        temp_i = vec_o_i[0 as core::ffi::c_int as usize];
        vec_o_r[0 as core::ffi::c_int as usize] = cos_theta * temp_r
            - sin_theta * temp_i;
        vec_o_i[0 as core::ffi::c_int as usize] = cos_theta * temp_i
            + sin_theta * temp_r;
        k = 0 as core::ffi::c_int as WORD32;
        while k < 2 as core::ffi::c_int {
            *(*((*ptr_hbe_txposer).qmf_out_buf)
                .offset(
                    (qmf_col_idx as core::ffi::c_int * 2 as core::ffi::c_int
                        + (k as core::ffi::c_int + HBE_ZERO_BAND_IDX
                            - 1 as core::ffi::c_int)) as isize,
                ))
                .offset((2 as WORD32 * qmf_band_idx) as isize)
                += mag_cmplx_gain * vec_o_r[k as usize];
            *(*((*ptr_hbe_txposer).qmf_out_buf)
                .offset(
                    (qmf_col_idx as core::ffi::c_int * 2 as core::ffi::c_int
                        + (k as core::ffi::c_int + HBE_ZERO_BAND_IDX
                            - 1 as core::ffi::c_int)) as isize,
                ))
                .offset(
                    (2 as core::ffi::c_int * qmf_band_idx as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) += mag_cmplx_gain * vec_o_i[k as usize];
            k += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_post_anal_prod2(
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    mut qmf_voc_columns: WORD32,
    mut qmf_band_idx: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut norm_ptr: *mut FLOAT32 = &mut *(*((*ptr_hbe_txposer).norm_qmf_in_buf)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset((2 as WORD32 * qmf_band_idx) as isize) as *mut FLOAT32;
    let mut out_ptr: *mut FLOAT32 = &mut *(*((*ptr_hbe_txposer).qmf_out_buf)
        .offset(1 as core::ffi::c_int as isize))
        .offset((2 as WORD32 * qmf_band_idx) as isize) as *mut FLOAT32;
    let mut x_norm_ptr: *mut FLOAT32 = &mut *(*((*ptr_hbe_txposer).norm_qmf_in_buf)
        .as_mut_ptr()
        .offset(HBE_ZERO_BAND_IDX as isize))
        .as_mut_ptr()
        .offset((2 as WORD32 * qmf_band_idx) as isize) as *mut FLOAT32;
    ixheaacd_norm_qmf_in_buf_2(ptr_hbe_txposer, qmf_band_idx);
    while qmf_band_idx < (*ptr_hbe_txposer).x_over_qmf[1 as core::ffi::c_int as usize] {
        i = 0 as core::ffi::c_int as WORD32;
        while i < qmf_voc_columns {
            let mut k: WORD32 = 0;
            let mut x_zero_band_r: FLOAT32 = 0.;
            let mut x_zero_band_i: FLOAT32 = 0.;
            let fresh26 = x_norm_ptr;
            x_norm_ptr = x_norm_ptr.offset(1);
            x_zero_band_r = *fresh26;
            let fresh27 = x_norm_ptr;
            x_norm_ptr = x_norm_ptr.offset(1);
            x_zero_band_i = *fresh27;
            k = 0 as core::ffi::c_int as WORD32;
            while k < HBE_OPER_BLK_LEN_2 {
                let mut tmp_r: FLOAT32 = 0.;
                let mut tmp_i: FLOAT32 = 0.;
                let fresh28 = norm_ptr;
                norm_ptr = norm_ptr.offset(1);
                tmp_r = *fresh28;
                let fresh29 = norm_ptr;
                norm_ptr = norm_ptr.offset(1);
                tmp_i = *fresh29;
                let fresh30 = out_ptr;
                out_ptr = out_ptr.offset(1);
                *fresh30
                    += (tmp_r as core::ffi::c_float * x_zero_band_r as core::ffi::c_float
                        - tmp_i as core::ffi::c_float
                            * x_zero_band_i as core::ffi::c_float) * 0.3333333f32;
                let fresh31 = out_ptr;
                out_ptr = out_ptr.offset(1);
                *fresh31
                    += (tmp_r as core::ffi::c_float * x_zero_band_i as core::ffi::c_float
                        + tmp_i as core::ffi::c_float
                            * x_zero_band_r as core::ffi::c_float) * 0.3333333f32;
                norm_ptr = norm_ptr.offset(126 as core::ffi::c_int as isize);
                out_ptr = out_ptr.offset(126 as core::ffi::c_int as isize);
                k += 1;
            }
            norm_ptr = norm_ptr
                .offset(-((128 as core::ffi::c_int * 9 as core::ffi::c_int) as isize));
            out_ptr = out_ptr
                .offset(-((128 as core::ffi::c_int * 8 as core::ffi::c_int) as isize));
            x_norm_ptr = x_norm_ptr.offset(126 as core::ffi::c_int as isize);
            i += 1;
        }
        out_ptr = out_ptr
            .offset(
                -((128 as core::ffi::c_int * 2 as core::ffi::c_int
                    * qmf_voc_columns as core::ffi::c_int - 2 as core::ffi::c_int)
                    as isize),
            );
        norm_ptr = norm_ptr
            .offset(
                -((128 as core::ffi::c_int * qmf_voc_columns as core::ffi::c_int
                    - 2 as core::ffi::c_int) as isize),
            );
        x_norm_ptr = x_norm_ptr
            .offset(
                -((128 as core::ffi::c_int * qmf_voc_columns as core::ffi::c_int
                    - 2 as core::ffi::c_int) as isize),
            );
        qmf_band_idx += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_post_anal_prod3(
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    mut qmf_voc_columns: WORD32,
    mut qmf_band_idx: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut inp_band_idx: WORD32 = 0;
    let mut rem: WORD32 = 0;
    let mut out_buf: *mut FLOAT32 = &mut *(*((*ptr_hbe_txposer).qmf_out_buf)
        .offset(2 as core::ffi::c_int as isize))
        .offset((2 as WORD32 * qmf_band_idx) as isize) as *mut FLOAT32;
    while qmf_band_idx < (*ptr_hbe_txposer).x_over_qmf[2 as core::ffi::c_int as usize] {
        let mut temp_r: FLOAT32 = 0.;
        let mut temp_i: FLOAT32 = 0.;
        let mut temp_r1: FLOAT32 = 0.;
        let mut temp_i1: FLOAT32 = 0.;
        let mut ptr_sel: *const FLOAT32 = 0 as *const FLOAT32;
        let mut ptr_sel1: *const FLOAT32 = 0 as *const FLOAT32;
        inp_band_idx = (2 as core::ffi::c_int * qmf_band_idx as core::ffi::c_int
            / 3 as core::ffi::c_int) as WORD32;
        ptr_sel = &*(*ixheaac_sel_case
            .as_ptr()
            .offset(
                (inp_band_idx as core::ffi::c_int + 1 as core::ffi::c_int
                    & 3 as core::ffi::c_int) as isize,
            ))
            .as_ptr()
            .offset(0 as core::ffi::c_int as isize) as *const FLOAT32;
        ptr_sel1 = &*(*ixheaac_sel_case
            .as_ptr()
            .offset(
                ((inp_band_idx as core::ffi::c_int + 1 as core::ffi::c_int
                    & 3 as core::ffi::c_int) + 1 as core::ffi::c_int) as isize,
            ))
            .as_ptr()
            .offset(0 as core::ffi::c_int as isize) as *const FLOAT32;
        rem = 2 as WORD32 * qmf_band_idx - 3 as WORD32 * inp_band_idx;
        if rem == 0 as core::ffi::c_int || rem == 1 as core::ffi::c_int {
            let mut in_buf: *mut FLOAT32 = &mut *(*((*ptr_hbe_txposer).qmf_in_buf)
                .offset(0 as core::ffi::c_int as isize))
                .offset((2 as WORD32 * inp_band_idx) as isize) as *mut FLOAT32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < qmf_voc_columns {
                let mut k: WORD32 = 0;
                let mut vec_x: [FLOAT32; 26] = [0.; 26];
                let mut ptr_vec_x: *mut FLOAT32 = &mut *vec_x
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                let mut x_zero_band_r: FLOAT32 = 0.;
                let mut x_zero_band_i: FLOAT32 = 0.;
                let mut mag_scaling_fac: FLOAT32 = 0.;
                k = 0 as core::ffi::c_int as WORD32;
                while k < 8 as core::ffi::c_int {
                    let mut base1: FLOAT64 = 0.;
                    let mut base: FLOAT64 = 1e-17f64;
                    temp_r = *in_buf.offset(0 as core::ffi::c_int as isize);
                    temp_i = *in_buf.offset(1 as core::ffi::c_int as isize);
                    in_buf = in_buf.offset(256 as core::ffi::c_int as isize);
                    base1 = base + (temp_r * temp_r) as FLOAT64;
                    base1 = base1 + (temp_i * temp_i) as FLOAT64;
                    mag_scaling_fac = cbrt(
                        (1.0f32 / base1 as FLOAT32) as core::ffi::c_double,
                    ) as FLOAT32;
                    *ptr_vec_x.offset(0 as core::ffi::c_int as isize) = temp_r
                        * mag_scaling_fac;
                    *ptr_vec_x.offset(1 as core::ffi::c_int as isize) = temp_i
                        * mag_scaling_fac;
                    temp_r = *in_buf.offset(0 as core::ffi::c_int as isize);
                    temp_i = *in_buf.offset(1 as core::ffi::c_int as isize);
                    in_buf = in_buf.offset(-(128 as core::ffi::c_int as isize));
                    temp_r1 = *ptr_sel.offset(0 as core::ffi::c_int as isize) * temp_r
                        + *ptr_sel.offset(1 as core::ffi::c_int as isize) * temp_i;
                    temp_i1 = *ptr_sel.offset(2 as core::ffi::c_int as isize) * temp_r
                        + *ptr_sel.offset(3 as core::ffi::c_int as isize) * temp_i;
                    temp_r = *in_buf.offset(0 as core::ffi::c_int as isize);
                    temp_i = *in_buf.offset(1 as core::ffi::c_int as isize);
                    temp_r1
                        += *ptr_sel.offset(4 as core::ffi::c_int as isize) * temp_r
                            + *ptr_sel.offset(5 as core::ffi::c_int as isize) * temp_i;
                    temp_i1
                        += *ptr_sel.offset(6 as core::ffi::c_int as isize) * temp_r
                            + *ptr_sel.offset(7 as core::ffi::c_int as isize) * temp_i;
                    temp_r1 *= 0.3984033437f32;
                    temp_i1 *= 0.3984033437f32;
                    base1 = base + (temp_r1 * temp_r1) as FLOAT64;
                    base1 = base1 + (temp_i1 * temp_i1) as FLOAT64;
                    mag_scaling_fac = cbrt(
                        (1.0f32 / base1 as FLOAT32) as core::ffi::c_double,
                    ) as FLOAT32;
                    *ptr_vec_x.offset(2 as core::ffi::c_int as isize) = temp_r1
                        * mag_scaling_fac;
                    *ptr_vec_x.offset(3 as core::ffi::c_int as isize) = temp_i1
                        * mag_scaling_fac;
                    ptr_vec_x = ptr_vec_x.offset(4 as core::ffi::c_int as isize);
                    in_buf = in_buf.offset(256 as core::ffi::c_int as isize);
                    k += 2 as core::ffi::c_int;
                }
                ptr_vec_x = &mut *vec_x
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                temp_r = vec_x[(2 as core::ffi::c_int
                    * (HBE_ZERO_BAND_IDX - 2 as core::ffi::c_int)) as usize];
                temp_i = vec_x[(2 as core::ffi::c_int
                    * (HBE_ZERO_BAND_IDX - 2 as core::ffi::c_int)
                    + 1 as core::ffi::c_int) as usize];
                x_zero_band_r = temp_r * temp_r - temp_i * temp_i;
                x_zero_band_i = temp_r * temp_i + temp_i * temp_r;
                k = 0 as core::ffi::c_int as WORD32;
                while k < 8 as core::ffi::c_int {
                    temp_r = *ptr_vec_x.offset(0 as core::ffi::c_int as isize)
                        * x_zero_band_r
                        - *ptr_vec_x.offset(1 as core::ffi::c_int as isize)
                            * x_zero_band_i;
                    temp_i = *ptr_vec_x.offset(0 as core::ffi::c_int as isize)
                        * x_zero_band_i
                        + *ptr_vec_x.offset(1 as core::ffi::c_int as isize)
                            * x_zero_band_r;
                    let ref mut fresh22 = *out_buf
                        .offset(0 as core::ffi::c_int as isize);
                    *fresh22 += temp_r as core::ffi::c_float * 0.4714045f32;
                    let ref mut fresh23 = *out_buf
                        .offset(1 as core::ffi::c_int as isize);
                    *fresh23 += temp_i as core::ffi::c_float * 0.4714045f32;
                    ptr_vec_x = ptr_vec_x.offset(2 as core::ffi::c_int as isize);
                    out_buf = out_buf.offset(128 as core::ffi::c_int as isize);
                    k += 1;
                }
                in_buf = in_buf
                    .offset(
                        -((128 as core::ffi::c_int * 11 as core::ffi::c_int) as isize),
                    );
                out_buf = out_buf
                    .offset(
                        -((128 as core::ffi::c_int * 6 as core::ffi::c_int) as isize),
                    );
                i += 1 as core::ffi::c_int;
            }
        } else {
            let mut in_buf_0: *mut FLOAT32 = &mut *(*((*ptr_hbe_txposer).qmf_in_buf)
                .offset(0 as core::ffi::c_int as isize))
                .offset((2 as WORD32 * inp_band_idx) as isize) as *mut FLOAT32;
            let mut in_buf1: *mut FLOAT32 = &mut *(*((*ptr_hbe_txposer).qmf_in_buf)
                .offset(0 as core::ffi::c_int as isize))
                .offset(
                    (2 as core::ffi::c_int
                        * (inp_band_idx as core::ffi::c_int + 1 as core::ffi::c_int))
                        as isize,
                ) as *mut FLOAT32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < qmf_voc_columns {
                let mut k_0: WORD32 = 0;
                let mut vec_x_0: [FLOAT32; 26] = [0.; 26];
                let mut vec_x_cap: [FLOAT32; 26] = [0.; 26];
                let mut x_zero_band_r_0: FLOAT32 = 0.;
                let mut x_zero_band_i_0: FLOAT32 = 0.;
                let mut ptr_vec_x_0: *mut FLOAT32 = &mut *vec_x_0
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                let mut ptr_vec_x_cap: *mut FLOAT32 = &mut *vec_x_cap
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                let mut mag_scaling_fac_0: FLOAT32 = 0.;
                k_0 = 0 as core::ffi::c_int as WORD32;
                while k_0 < 8 as core::ffi::c_int {
                    let mut tmp_vr: FLOAT32 = 0.;
                    let mut tmp_vi: FLOAT32 = 0.;
                    let mut tmp_cr: FLOAT32 = 0.;
                    let mut tmp_ci: FLOAT32 = 0.;
                    let mut base1_0: FLOAT64 = 0.;
                    let mut base_0: FLOAT64 = 1e-17f64;
                    temp_r1 = *in_buf_0.offset(0 as core::ffi::c_int as isize);
                    temp_i1 = *in_buf_0.offset(1 as core::ffi::c_int as isize);
                    temp_r = *in_buf1.offset(0 as core::ffi::c_int as isize);
                    temp_i = *in_buf1.offset(1 as core::ffi::c_int as isize);
                    base1_0 = base_0 + (temp_r * temp_r) as FLOAT64;
                    base1_0 = base1_0 + (temp_i * temp_i) as FLOAT64;
                    mag_scaling_fac_0 = cbrt(
                        (1.0f32 / base1_0 as FLOAT32) as core::ffi::c_double,
                    ) as FLOAT32;
                    *ptr_vec_x_0.offset(0 as core::ffi::c_int as isize) = temp_r
                        * mag_scaling_fac_0;
                    *ptr_vec_x_0.offset(1 as core::ffi::c_int as isize) = temp_i
                        * mag_scaling_fac_0;
                    base1_0 = base_0 + (temp_r1 * temp_r1) as FLOAT64;
                    base1_0 = base1_0 + (temp_i1 * temp_i1) as FLOAT64;
                    mag_scaling_fac_0 = cbrt(
                        (1.0f32 / base1_0 as FLOAT32) as core::ffi::c_double,
                    ) as FLOAT32;
                    *ptr_vec_x_cap.offset(0 as core::ffi::c_int as isize) = temp_r1
                        * mag_scaling_fac_0;
                    *ptr_vec_x_cap.offset(1 as core::ffi::c_int as isize) = temp_i1
                        * mag_scaling_fac_0;
                    in_buf_0 = in_buf_0.offset(256 as core::ffi::c_int as isize);
                    temp_r = *in_buf_0.offset(0 as core::ffi::c_int as isize);
                    temp_i = *in_buf_0.offset(1 as core::ffi::c_int as isize);
                    temp_r1 = *ptr_sel.offset(0 as core::ffi::c_int as isize) * temp_r
                        + *ptr_sel.offset(1 as core::ffi::c_int as isize) * temp_i;
                    temp_i1 = *ptr_sel.offset(2 as core::ffi::c_int as isize) * temp_r
                        + *ptr_sel.offset(3 as core::ffi::c_int as isize) * temp_i;
                    in_buf_0 = in_buf_0.offset(-(128 as core::ffi::c_int as isize));
                    temp_r = *in_buf_0.offset(0 as core::ffi::c_int as isize);
                    temp_i = *in_buf_0.offset(1 as core::ffi::c_int as isize);
                    tmp_cr = temp_r1
                        + *ptr_sel.offset(4 as core::ffi::c_int as isize) * temp_r
                        + *ptr_sel.offset(5 as core::ffi::c_int as isize) * temp_i;
                    tmp_ci = temp_i1
                        + *ptr_sel.offset(6 as core::ffi::c_int as isize) * temp_r
                        + *ptr_sel.offset(7 as core::ffi::c_int as isize) * temp_i;
                    in_buf1 = in_buf1.offset(256 as core::ffi::c_int as isize);
                    temp_r = *in_buf1.offset(0 as core::ffi::c_int as isize);
                    temp_i = *in_buf1.offset(1 as core::ffi::c_int as isize);
                    temp_r1 = *ptr_sel1.offset(0 as core::ffi::c_int as isize) * temp_r
                        + *ptr_sel1.offset(1 as core::ffi::c_int as isize) * temp_i;
                    temp_i1 = *ptr_sel1.offset(2 as core::ffi::c_int as isize) * temp_r
                        + *ptr_sel1.offset(3 as core::ffi::c_int as isize) * temp_i;
                    in_buf1 = in_buf1.offset(-(128 as core::ffi::c_int as isize));
                    temp_r = *in_buf1.offset(0 as core::ffi::c_int as isize);
                    temp_i = *in_buf1.offset(1 as core::ffi::c_int as isize);
                    tmp_vr = temp_r1
                        + *ptr_sel1.offset(4 as core::ffi::c_int as isize) * temp_r
                        + *ptr_sel1.offset(5 as core::ffi::c_int as isize) * temp_i;
                    tmp_vi = temp_i1
                        + *ptr_sel1.offset(6 as core::ffi::c_int as isize) * temp_r
                        + *ptr_sel1.offset(7 as core::ffi::c_int as isize) * temp_i;
                    tmp_cr *= 0.3984033437f32;
                    tmp_ci *= 0.3984033437f32;
                    tmp_vr *= 0.3984033437f32;
                    tmp_vi *= 0.3984033437f32;
                    base1_0 = base_0 + (tmp_vr * tmp_vr) as FLOAT64;
                    base1_0 = base1_0 + (tmp_vi * tmp_vi) as FLOAT64;
                    mag_scaling_fac_0 = cbrt(
                        (1.0f32 / base1_0 as FLOAT32) as core::ffi::c_double,
                    ) as FLOAT32;
                    *ptr_vec_x_0.offset(2 as core::ffi::c_int as isize) = tmp_vr
                        * mag_scaling_fac_0;
                    *ptr_vec_x_0.offset(3 as core::ffi::c_int as isize) = tmp_vi
                        * mag_scaling_fac_0;
                    base1_0 = base_0 + (tmp_cr * tmp_cr) as FLOAT64;
                    base1_0 = base1_0 + (tmp_ci * tmp_ci) as FLOAT64;
                    mag_scaling_fac_0 = cbrt(
                        (1.0f32 / base1_0 as FLOAT32) as core::ffi::c_double,
                    ) as FLOAT32;
                    *ptr_vec_x_cap.offset(2 as core::ffi::c_int as isize) = tmp_cr
                        * mag_scaling_fac_0;
                    *ptr_vec_x_cap.offset(3 as core::ffi::c_int as isize) = tmp_ci
                        * mag_scaling_fac_0;
                    in_buf_0 = in_buf_0.offset(256 as core::ffi::c_int as isize);
                    in_buf1 = in_buf1.offset(256 as core::ffi::c_int as isize);
                    ptr_vec_x_0 = ptr_vec_x_0.offset(4 as core::ffi::c_int as isize);
                    ptr_vec_x_cap = ptr_vec_x_cap.offset(4 as core::ffi::c_int as isize);
                    k_0 += 2 as core::ffi::c_int;
                }
                ptr_vec_x_0 = &mut *vec_x_0
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                ptr_vec_x_cap = &mut *vec_x_cap
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                temp_r = vec_x_cap[(2 as core::ffi::c_int
                    * (HBE_ZERO_BAND_IDX - 2 as core::ffi::c_int)) as usize];
                temp_i = vec_x_cap[(2 as core::ffi::c_int
                    * (HBE_ZERO_BAND_IDX - 2 as core::ffi::c_int)
                    + 1 as core::ffi::c_int) as usize];
                temp_r1 = vec_x_0[(2 as core::ffi::c_int
                    * (HBE_ZERO_BAND_IDX - 2 as core::ffi::c_int)) as usize];
                temp_i1 = vec_x_0[(2 as core::ffi::c_int
                    * (HBE_ZERO_BAND_IDX - 2 as core::ffi::c_int)
                    + 1 as core::ffi::c_int) as usize];
                x_zero_band_r_0 = temp_r * temp_r - temp_i * temp_i;
                x_zero_band_i_0 = temp_r * temp_i + temp_i * temp_r;
                temp_r = temp_r1 * temp_r1 - temp_i1 * temp_i1;
                temp_i = temp_r1 * temp_i1 + temp_i1 * temp_r1;
                k_0 = 0 as core::ffi::c_int as WORD32;
                while k_0 < 8 as core::ffi::c_int {
                    temp_r1 = *ptr_vec_x_0.offset(0 as core::ffi::c_int as isize)
                        * x_zero_band_r_0
                        - *ptr_vec_x_0.offset(1 as core::ffi::c_int as isize)
                            * x_zero_band_i_0;
                    temp_i1 = *ptr_vec_x_0.offset(0 as core::ffi::c_int as isize)
                        * x_zero_band_i_0
                        + *ptr_vec_x_0.offset(1 as core::ffi::c_int as isize)
                            * x_zero_band_r_0;
                    temp_r1
                        += *ptr_vec_x_cap.offset(0 as core::ffi::c_int as isize) * temp_r
                            - *ptr_vec_x_cap.offset(1 as core::ffi::c_int as isize)
                                * temp_i;
                    temp_i1
                        += *ptr_vec_x_cap.offset(0 as core::ffi::c_int as isize) * temp_i
                            + *ptr_vec_x_cap.offset(1 as core::ffi::c_int as isize)
                                * temp_r;
                    let ref mut fresh24 = *out_buf
                        .offset(0 as core::ffi::c_int as isize);
                    *fresh24 += temp_r1 as core::ffi::c_float * 0.23570225f32;
                    let ref mut fresh25 = *out_buf
                        .offset(1 as core::ffi::c_int as isize);
                    *fresh25 += temp_i1 as core::ffi::c_float * 0.23570225f32;
                    out_buf = out_buf.offset(128 as core::ffi::c_int as isize);
                    ptr_vec_x_0 = ptr_vec_x_0.offset(2 as core::ffi::c_int as isize);
                    ptr_vec_x_cap = ptr_vec_x_cap.offset(2 as core::ffi::c_int as isize);
                    k_0 += 1;
                }
                in_buf_0 = in_buf_0
                    .offset(
                        -((128 as core::ffi::c_int * 11 as core::ffi::c_int) as isize),
                    );
                in_buf1 = in_buf1
                    .offset(
                        -((128 as core::ffi::c_int * 11 as core::ffi::c_int) as isize),
                    );
                out_buf = out_buf
                    .offset(
                        -((128 as core::ffi::c_int * 6 as core::ffi::c_int) as isize),
                    );
                i += 1;
            }
        }
        out_buf = out_buf
            .offset(
                -((256 as core::ffi::c_int * qmf_voc_columns as core::ffi::c_int
                    - 2 as core::ffi::c_int) as isize),
            );
        qmf_band_idx += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_post_anal_prod4(
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    mut qmf_voc_columns: WORD32,
    mut qmf_band_idx: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut inp_band_idx: WORD32 = 0;
    let mut out_ptr: *mut FLOAT32 = &mut *(*((*ptr_hbe_txposer).qmf_out_buf)
        .offset(3 as core::ffi::c_int as isize))
        .offset((2 as WORD32 * qmf_band_idx) as isize) as *mut FLOAT32;
    ixheaacd_norm_qmf_in_buf_4(
        ptr_hbe_txposer,
        (qmf_band_idx >> 1 as core::ffi::c_int) - 1 as WORD32,
    );
    while qmf_band_idx < (*ptr_hbe_txposer).x_over_qmf[3 as core::ffi::c_int as usize] {
        let mut ip_idx: WORD32 = 0;
        let mut temp: FLOAT32 = 0.;
        let mut temp_r: FLOAT32 = 0.;
        let mut temp_i: FLOAT32 = 0.;
        let mut norm_ptr: *mut FLOAT32 = 0 as *mut FLOAT32;
        let mut x_norm_ptr: *mut FLOAT32 = 0 as *mut FLOAT32;
        inp_band_idx = qmf_band_idx >> 1 as core::ffi::c_int;
        ip_idx = (if qmf_band_idx as core::ffi::c_int & 1 as core::ffi::c_int != 0 {
            inp_band_idx as core::ffi::c_int + 1 as core::ffi::c_int
        } else {
            inp_band_idx as core::ffi::c_int - 1 as core::ffi::c_int
        }) as WORD32;
        norm_ptr = &mut *(*((*ptr_hbe_txposer).norm_qmf_in_buf)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset((2 as WORD32 * ip_idx) as isize) as *mut FLOAT32;
        x_norm_ptr = &mut *(*((*ptr_hbe_txposer).norm_qmf_in_buf)
            .as_mut_ptr()
            .offset(HBE_ZERO_BAND_IDX as isize))
            .as_mut_ptr()
            .offset((2 as WORD32 * inp_band_idx) as isize) as *mut FLOAT32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < qmf_voc_columns {
            let mut k: WORD32 = 0;
            let mut x_zero_band_r: FLOAT32 = 0.;
            let mut x_zero_band_i: FLOAT32 = 0.;
            let fresh16 = x_norm_ptr;
            x_norm_ptr = x_norm_ptr.offset(1);
            x_zero_band_r = *fresh16;
            temp_r = x_zero_band_r;
            let fresh17 = x_norm_ptr;
            x_norm_ptr = x_norm_ptr.offset(1);
            x_zero_band_i = *fresh17;
            temp_i = x_zero_band_i;
            temp = x_zero_band_r * x_zero_band_r - x_zero_band_i * x_zero_band_i;
            x_zero_band_i = x_zero_band_r * x_zero_band_i
                + x_zero_band_i * x_zero_band_r;
            x_zero_band_r = temp_r * temp - temp_i * x_zero_band_i;
            x_zero_band_i = temp_r * x_zero_band_i + temp_i * temp;
            k = 0 as core::ffi::c_int as WORD32;
            while k < HBE_OPER_BLK_LEN_4 {
                let fresh18 = norm_ptr;
                norm_ptr = norm_ptr.offset(1);
                temp = *fresh18;
                let fresh19 = norm_ptr;
                norm_ptr = norm_ptr.offset(1);
                temp_i = *fresh19;
                temp_r = temp * x_zero_band_r - temp_i * x_zero_band_i;
                temp_i = temp * x_zero_band_i + temp_i * x_zero_band_r;
                let fresh20 = out_ptr;
                out_ptr = out_ptr.offset(1);
                *fresh20 += temp_r as core::ffi::c_float * 0.6666667f32;
                let fresh21 = out_ptr;
                out_ptr = out_ptr.offset(1);
                *fresh21 += temp_i as core::ffi::c_float * 0.6666667f32;
                norm_ptr = norm_ptr.offset(254 as core::ffi::c_int as isize);
                out_ptr = out_ptr.offset(126 as core::ffi::c_int as isize);
                k += 1;
            }
            norm_ptr = norm_ptr
                .offset(-((128 as core::ffi::c_int * 11 as core::ffi::c_int) as isize));
            out_ptr = out_ptr
                .offset(-((128 as core::ffi::c_int * 4 as core::ffi::c_int) as isize));
            x_norm_ptr = x_norm_ptr.offset(126 as core::ffi::c_int as isize);
            i += 1;
        }
        out_ptr = out_ptr
            .offset(
                -((128 as core::ffi::c_int * 2 as core::ffi::c_int
                    * qmf_voc_columns as core::ffi::c_int - 2 as core::ffi::c_int)
                    as isize),
            );
        qmf_band_idx += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_post_anal_xprod2(
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    mut qmf_voc_columns: WORD32,
    mut qmf_band_idx: WORD32,
    mut p: FLOAT32,
    mut cos_sin_theta: *mut FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut norm_ptr: *mut FLOAT32 = &mut *(*((*ptr_hbe_txposer).norm_qmf_in_buf)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset((2 as WORD32 * qmf_band_idx) as isize) as *mut FLOAT32;
    let mut out_ptr: *mut FLOAT32 = &mut *(*((*ptr_hbe_txposer).qmf_out_buf)
        .offset(1 as core::ffi::c_int as isize))
        .offset((2 as WORD32 * qmf_band_idx) as isize) as *mut FLOAT32;
    let mut x_norm_ptr: *mut FLOAT32 = &mut *(*((*ptr_hbe_txposer).norm_qmf_in_buf)
        .as_mut_ptr()
        .offset(HBE_ZERO_BAND_IDX as isize))
        .as_mut_ptr()
        .offset((2 as WORD32 * qmf_band_idx) as isize) as *mut FLOAT32;
    ixheaacd_norm_qmf_in_buf_2(ptr_hbe_txposer, qmf_band_idx);
    while qmf_band_idx < (*ptr_hbe_txposer).x_over_qmf[1 as core::ffi::c_int as usize] {
        let mut n1: WORD32 = 0;
        let mut n2: WORD32 = 0;
        let mut temp_fac: FLOAT64 = 0.;
        let mut mag_cmplx_gain: FLOAT32 = 1.666666667f32;
        temp_fac = ((2.0f64 * qmf_band_idx as core::ffi::c_double
            + 1 as core::ffi::c_int as core::ffi::c_double - p as core::ffi::c_double)
            * 0.5f64) as FLOAT64;
        n1 = (temp_fac as WORD32) << 1 as core::ffi::c_int;
        n2 = ((temp_fac + p as FLOAT64) as WORD32) << 1 as core::ffi::c_int;
        i = 0 as core::ffi::c_int as WORD32;
        while i < qmf_voc_columns {
            let mut k: WORD32 = 0;
            let mut x_zero_band_r: FLOAT32 = 0.;
            let mut x_zero_band_i: FLOAT32 = 0.;
            let fresh10 = x_norm_ptr;
            x_norm_ptr = x_norm_ptr.offset(1);
            x_zero_band_r = *fresh10;
            let fresh11 = x_norm_ptr;
            x_norm_ptr = x_norm_ptr.offset(1);
            x_zero_band_i = *fresh11;
            k = 1 as core::ffi::c_int as WORD32;
            while k < HBE_OPER_BLK_LEN_2 + 1 as core::ffi::c_int {
                let mut tmp_r: FLOAT32 = 0.;
                let mut tmp_i: FLOAT32 = 0.;
                let fresh12 = norm_ptr;
                norm_ptr = norm_ptr.offset(1);
                tmp_r = *fresh12;
                let fresh13 = norm_ptr;
                norm_ptr = norm_ptr.offset(1);
                tmp_i = *fresh13;
                let fresh14 = out_ptr;
                out_ptr = out_ptr.offset(1);
                *fresh14
                    += (tmp_r as core::ffi::c_float * x_zero_band_r as core::ffi::c_float
                        - tmp_i as core::ffi::c_float
                            * x_zero_band_i as core::ffi::c_float) * 0.3333333f32;
                let fresh15 = out_ptr;
                out_ptr = out_ptr.offset(1);
                *fresh15
                    += (tmp_r as core::ffi::c_float * x_zero_band_i as core::ffi::c_float
                        + tmp_i as core::ffi::c_float
                            * x_zero_band_r as core::ffi::c_float) * 0.3333333f32;
                norm_ptr = norm_ptr.offset(126 as core::ffi::c_int as isize);
                out_ptr = out_ptr.offset(126 as core::ffi::c_int as isize);
                k += 1;
            }
            norm_ptr = norm_ptr
                .offset(-((128 as core::ffi::c_int * 9 as core::ffi::c_int) as isize));
            out_ptr = out_ptr
                .offset(-((128 as core::ffi::c_int * 8 as core::ffi::c_int) as isize));
            x_norm_ptr = x_norm_ptr.offset(126 as core::ffi::c_int as isize);
            let mut max_trans_fac: WORD32 = 0;
            let mut max_n1: WORD32 = 0;
            let mut max_n2: WORD32 = 0;
            let mut max_mag_value: FLOAT32 = 0.;
            let mut mag_zero_band: FLOAT32 = 0.;
            let mut mag_n1_band: FLOAT32 = 0.;
            let mut mag_n2_band: FLOAT32 = 0.;
            let mut temp: FLOAT32 = 0.;
            let mut qmf_in_buf_ri: *mut FLOAT32 = *((*ptr_hbe_txposer).qmf_in_buf)
                .offset((i as core::ffi::c_int + HBE_ZERO_BAND_IDX) as isize);
            mag_zero_band = *qmf_in_buf_ri.offset((2 as WORD32 * qmf_band_idx) as isize)
                * *qmf_in_buf_ri.offset((2 as WORD32 * qmf_band_idx) as isize)
                + *qmf_in_buf_ri
                    .offset(
                        (2 as core::ffi::c_int * qmf_band_idx as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    )
                    * *qmf_in_buf_ri
                        .offset(
                            (2 as core::ffi::c_int * qmf_band_idx as core::ffi::c_int
                                + 1 as core::ffi::c_int) as isize,
                        );
            mag_n1_band = *qmf_in_buf_ri.offset(n1 as isize)
                * *qmf_in_buf_ri.offset(n1 as isize)
                + *qmf_in_buf_ri
                    .offset((n1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                    * *qmf_in_buf_ri
                        .offset(
                            (n1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                        );
            mag_n2_band = *qmf_in_buf_ri.offset(n2 as isize)
                * *qmf_in_buf_ri.offset(n2 as isize)
                + *qmf_in_buf_ri
                    .offset((n2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                    * *qmf_in_buf_ri
                        .offset(
                            (n2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                        );
            temp = if mag_n1_band < mag_n2_band { mag_n1_band } else { mag_n2_band };
            max_mag_value = 0 as core::ffi::c_int as FLOAT32;
            max_trans_fac = 0 as core::ffi::c_int as WORD32;
            max_n1 = 0 as core::ffi::c_int as WORD32;
            max_n2 = 0 as core::ffi::c_int as WORD32;
            if temp > 0 as core::ffi::c_int as FLOAT32 {
                max_mag_value = temp;
                max_trans_fac = 1 as core::ffi::c_int as WORD32;
                max_n1 = n1;
                max_n2 = n2;
            }
            if max_mag_value > mag_zero_band && max_n1 >= 0 as core::ffi::c_int
                && max_n2 < TWICE_QMF_SYNTH_CHANNELS_NUM
            {
                let mut vec_y_r: [FLOAT32; 2] = [0.; 2];
                let mut vec_y_i: [FLOAT32; 2] = [0.; 2];
                let mut temp_r: FLOAT32 = 0.;
                let mut temp_i: FLOAT32 = 0.;
                let mut tmp_r1: FLOAT32 = 0.;
                let mut mid_trans_fac: WORD32 = 0;
                let mut idx: WORD32 = 0;
                let mut base: FLOAT64 = 0.;
                let mut k_0: WORD32 = 0;
                let mut mag_scaling_fac: FLOAT32 = 0.0f32;
                let mut x_zero_band_r_0: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
                let mut x_zero_band_i_0: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
                mid_trans_fac = 2 as WORD32 - max_trans_fac;
                x_zero_band_r_0 = *qmf_in_buf_ri.offset(max_n1 as isize);
                x_zero_band_i_0 = *qmf_in_buf_ri
                    .offset(
                        (max_n1 as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                    );
                base = 1e-17f64 as FLOAT64;
                base = base + (x_zero_band_r_0 * x_zero_band_r_0) as FLOAT64;
                base = base + (x_zero_band_i_0 * x_zero_band_i_0) as FLOAT64;
                mag_scaling_fac = (1.0f64 / base) as FLOAT32;
                mag_scaling_fac = sqrt(sqrt(mag_scaling_fac as core::ffi::c_double))
                    as FLOAT32;
                x_zero_band_r_0 *= mag_scaling_fac;
                x_zero_band_i_0 *= mag_scaling_fac;
                temp_r = x_zero_band_r_0;
                temp_i = x_zero_band_i_0;
                idx = 0 as core::ffi::c_int as WORD32;
                while idx < mid_trans_fac as core::ffi::c_int - 1 as core::ffi::c_int {
                    let mut tmp: FLOAT32 = x_zero_band_r_0;
                    x_zero_band_r_0 = x_zero_band_r_0 * temp_r
                        - x_zero_band_i_0 * temp_i;
                    x_zero_band_i_0 = tmp * temp_i + x_zero_band_i_0 * temp_r;
                    idx += 1;
                }
                k_0 = 0 as core::ffi::c_int as WORD32;
                while k_0 < 2 as core::ffi::c_int {
                    temp_r = *(*((*ptr_hbe_txposer).qmf_in_buf)
                        .offset((i + HBE_ZERO_BAND_IDX - 1 as WORD32 + k_0) as isize))
                        .offset(max_n2 as isize);
                    temp_i = *(*((*ptr_hbe_txposer).qmf_in_buf)
                        .offset((i + HBE_ZERO_BAND_IDX - 1 as WORD32 + k_0) as isize))
                        .offset(
                            (max_n2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                        );
                    base = 1e-17f64 as FLOAT64;
                    base = base + (temp_r * temp_r) as FLOAT64;
                    base = base + (temp_i * temp_i) as FLOAT64;
                    mag_scaling_fac = (1.0f64 / base) as FLOAT32;
                    mag_scaling_fac = sqrt(sqrt(mag_scaling_fac as core::ffi::c_double))
                        as FLOAT32;
                    temp_r *= mag_scaling_fac;
                    temp_i *= mag_scaling_fac;
                    vec_y_r[k_0 as usize] = temp_r;
                    vec_y_i[k_0 as usize] = temp_i;
                    k_0 += 1;
                }
                temp_r = vec_y_r[0 as core::ffi::c_int as usize] * x_zero_band_r_0
                    - vec_y_i[0 as core::ffi::c_int as usize] * x_zero_band_i_0;
                temp_i = vec_y_r[0 as core::ffi::c_int as usize] * x_zero_band_i_0
                    + vec_y_i[0 as core::ffi::c_int as usize] * x_zero_band_r_0;
                tmp_r1 = *cos_sin_theta.offset(0 as core::ffi::c_int as isize) * temp_r
                    - *cos_sin_theta.offset(1 as core::ffi::c_int as isize) * temp_i;
                temp_i = *cos_sin_theta.offset(0 as core::ffi::c_int as isize) * temp_i
                    + *cos_sin_theta.offset(1 as core::ffi::c_int as isize) * temp_r;
                *(*((*ptr_hbe_txposer).qmf_out_buf)
                    .offset(
                        (i as core::ffi::c_int * 2 as core::ffi::c_int
                            + (HBE_ZERO_BAND_IDX - 1 as core::ffi::c_int)) as isize,
                    ))
                    .offset((2 as WORD32 * qmf_band_idx) as isize)
                    += mag_cmplx_gain * tmp_r1;
                *(*((*ptr_hbe_txposer).qmf_out_buf)
                    .offset(
                        (i as core::ffi::c_int * 2 as core::ffi::c_int
                            + (HBE_ZERO_BAND_IDX - 1 as core::ffi::c_int)) as isize,
                    ))
                    .offset(
                        (2 as core::ffi::c_int * qmf_band_idx as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    ) += mag_cmplx_gain * temp_i;
                temp_r = vec_y_r[1 as core::ffi::c_int as usize] * x_zero_band_r_0
                    - vec_y_i[1 as core::ffi::c_int as usize] * x_zero_band_i_0;
                temp_i = vec_y_r[1 as core::ffi::c_int as usize] * x_zero_band_i_0
                    + vec_y_i[1 as core::ffi::c_int as usize] * x_zero_band_r_0;
                *(*((*ptr_hbe_txposer).qmf_out_buf)
                    .offset(
                        (i as core::ffi::c_int * 2 as core::ffi::c_int
                            + (1 as core::ffi::c_int + HBE_ZERO_BAND_IDX
                                - 1 as core::ffi::c_int)) as isize,
                    ))
                    .offset((2 as WORD32 * qmf_band_idx) as isize)
                    += mag_cmplx_gain * temp_r;
                *(*((*ptr_hbe_txposer).qmf_out_buf)
                    .offset(
                        (i as core::ffi::c_int * 2 as core::ffi::c_int
                            + (1 as core::ffi::c_int + HBE_ZERO_BAND_IDX
                                - 1 as core::ffi::c_int)) as isize,
                    ))
                    .offset(
                        (2 as core::ffi::c_int * qmf_band_idx as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    ) += mag_cmplx_gain * temp_i;
            }
            i += 1;
        }
        out_ptr = out_ptr
            .offset(
                -((128 as core::ffi::c_int * 2 as core::ffi::c_int
                    * qmf_voc_columns as core::ffi::c_int - 2 as core::ffi::c_int)
                    as isize),
            );
        norm_ptr = norm_ptr
            .offset(
                -((128 as core::ffi::c_int * qmf_voc_columns as core::ffi::c_int
                    - 2 as core::ffi::c_int) as isize),
            );
        x_norm_ptr = x_norm_ptr
            .offset(
                -((128 as core::ffi::c_int * qmf_voc_columns as core::ffi::c_int
                    - 2 as core::ffi::c_int) as isize),
            );
        qmf_band_idx += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_post_anal_xprod3(
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    mut qmf_voc_columns: WORD32,
    mut qmf_band_idx: WORD32,
    mut p: FLOAT32,
    mut pitch_in_bins_idx: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut inp_band_idx: WORD32 = 0;
    let mut rem: WORD32 = 0;
    let mut out_buf: *mut FLOAT32 = &mut *(*((*ptr_hbe_txposer).qmf_out_buf)
        .offset(2 as core::ffi::c_int as isize))
        .offset((2 as WORD32 * qmf_band_idx) as isize) as *mut FLOAT32;
    while qmf_band_idx < (*ptr_hbe_txposer).x_over_qmf[2 as core::ffi::c_int as usize] {
        let mut temp_r: FLOAT32 = 0.;
        let mut temp_i: FLOAT32 = 0.;
        let mut temp_r1: FLOAT32 = 0.;
        let mut temp_i1: FLOAT32 = 0.;
        let mut ptr_sel: *const FLOAT32 = 0 as *const FLOAT32;
        let mut ptr_sel1: *const FLOAT32 = 0 as *const FLOAT32;
        inp_band_idx = (2 as core::ffi::c_int * qmf_band_idx as core::ffi::c_int
            / 3 as core::ffi::c_int) as WORD32;
        ptr_sel = &*(*ixheaac_sel_case
            .as_ptr()
            .offset(
                (inp_band_idx as core::ffi::c_int + 1 as core::ffi::c_int
                    & 3 as core::ffi::c_int) as isize,
            ))
            .as_ptr()
            .offset(0 as core::ffi::c_int as isize) as *const FLOAT32;
        ptr_sel1 = &*(*ixheaac_sel_case
            .as_ptr()
            .offset(
                ((inp_band_idx as core::ffi::c_int + 1 as core::ffi::c_int
                    & 3 as core::ffi::c_int) + 1 as core::ffi::c_int) as isize,
            ))
            .as_ptr()
            .offset(0 as core::ffi::c_int as isize) as *const FLOAT32;
        rem = 2 as WORD32 * qmf_band_idx - 3 as WORD32 * inp_band_idx;
        if rem == 0 as core::ffi::c_int || rem == 1 as core::ffi::c_int {
            let mut in_buf: *mut FLOAT32 = &mut *(*((*ptr_hbe_txposer).qmf_in_buf)
                .offset(0 as core::ffi::c_int as isize))
                .offset((2 as WORD32 * inp_band_idx) as isize) as *mut FLOAT32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < qmf_voc_columns {
                let mut k: WORD32 = 0;
                let mut vec_x: [FLOAT32; 26] = [0.; 26];
                let mut ptr_vec_x: *mut FLOAT32 = &mut *vec_x
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                let mut x_zero_band_r: FLOAT32 = 0.;
                let mut x_zero_band_i: FLOAT32 = 0.;
                let mut mag_scaling_fac: FLOAT32 = 0.;
                k = 0 as core::ffi::c_int as WORD32;
                while k < 8 as core::ffi::c_int {
                    let mut base1: FLOAT64 = 0.;
                    let mut base: FLOAT64 = 1e-17f64;
                    temp_r = *in_buf.offset(0 as core::ffi::c_int as isize);
                    temp_i = *in_buf.offset(1 as core::ffi::c_int as isize);
                    in_buf = in_buf.offset(256 as core::ffi::c_int as isize);
                    base1 = base + (temp_r * temp_r) as FLOAT64;
                    base1 = base1 + (temp_i * temp_i) as FLOAT64;
                    mag_scaling_fac = cbrt(
                        (1.0f32 / base1 as FLOAT32) as core::ffi::c_double,
                    ) as FLOAT32;
                    *ptr_vec_x.offset(0 as core::ffi::c_int as isize) = temp_r
                        * mag_scaling_fac;
                    *ptr_vec_x.offset(1 as core::ffi::c_int as isize) = temp_i
                        * mag_scaling_fac;
                    temp_r = *in_buf.offset(0 as core::ffi::c_int as isize);
                    temp_i = *in_buf.offset(1 as core::ffi::c_int as isize);
                    in_buf = in_buf.offset(-(128 as core::ffi::c_int as isize));
                    temp_r1 = *ptr_sel.offset(0 as core::ffi::c_int as isize) * temp_r
                        + *ptr_sel.offset(1 as core::ffi::c_int as isize) * temp_i;
                    temp_i1 = *ptr_sel.offset(2 as core::ffi::c_int as isize) * temp_r
                        + *ptr_sel.offset(3 as core::ffi::c_int as isize) * temp_i;
                    temp_r = *in_buf.offset(0 as core::ffi::c_int as isize);
                    temp_i = *in_buf.offset(1 as core::ffi::c_int as isize);
                    temp_r1
                        += *ptr_sel.offset(4 as core::ffi::c_int as isize) * temp_r
                            + *ptr_sel.offset(5 as core::ffi::c_int as isize) * temp_i;
                    temp_i1
                        += *ptr_sel.offset(6 as core::ffi::c_int as isize) * temp_r
                            + *ptr_sel.offset(7 as core::ffi::c_int as isize) * temp_i;
                    temp_r1 *= 0.3984033437f32;
                    temp_i1 *= 0.3984033437f32;
                    base1 = base + (temp_r1 * temp_r1) as FLOAT64;
                    base1 = base1 + (temp_i1 * temp_i1) as FLOAT64;
                    mag_scaling_fac = cbrt(
                        (1.0f32 / base1 as FLOAT32) as core::ffi::c_double,
                    ) as FLOAT32;
                    *ptr_vec_x.offset(2 as core::ffi::c_int as isize) = temp_r1
                        * mag_scaling_fac;
                    *ptr_vec_x.offset(3 as core::ffi::c_int as isize) = temp_i1
                        * mag_scaling_fac;
                    ptr_vec_x = ptr_vec_x.offset(4 as core::ffi::c_int as isize);
                    in_buf = in_buf.offset(256 as core::ffi::c_int as isize);
                    k += 2 as core::ffi::c_int;
                }
                ptr_vec_x = &mut *vec_x
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                temp_r = vec_x[(2 as core::ffi::c_int
                    * (HBE_ZERO_BAND_IDX - 2 as core::ffi::c_int)) as usize];
                temp_i = vec_x[(2 as core::ffi::c_int
                    * (HBE_ZERO_BAND_IDX - 2 as core::ffi::c_int)
                    + 1 as core::ffi::c_int) as usize];
                x_zero_band_r = temp_r * temp_r - temp_i * temp_i;
                x_zero_band_i = temp_r * temp_i + temp_i * temp_r;
                k = 0 as core::ffi::c_int as WORD32;
                while k < 8 as core::ffi::c_int {
                    temp_r = *ptr_vec_x.offset(0 as core::ffi::c_int as isize)
                        * x_zero_band_r
                        - *ptr_vec_x.offset(1 as core::ffi::c_int as isize)
                            * x_zero_band_i;
                    temp_i = *ptr_vec_x.offset(0 as core::ffi::c_int as isize)
                        * x_zero_band_i
                        + *ptr_vec_x.offset(1 as core::ffi::c_int as isize)
                            * x_zero_band_r;
                    let ref mut fresh6 = *out_buf.offset(0 as core::ffi::c_int as isize);
                    *fresh6 += temp_r as core::ffi::c_float * 0.4714045f32;
                    let ref mut fresh7 = *out_buf.offset(1 as core::ffi::c_int as isize);
                    *fresh7 += temp_i as core::ffi::c_float * 0.4714045f32;
                    ptr_vec_x = ptr_vec_x.offset(2 as core::ffi::c_int as isize);
                    out_buf = out_buf.offset(128 as core::ffi::c_int as isize);
                    k += 1;
                }
                ixheaacd_hbe_xprod_proc_3(
                    ptr_hbe_txposer,
                    qmf_band_idx,
                    i,
                    p,
                    pitch_in_bins_idx,
                );
                in_buf = in_buf
                    .offset(
                        -((128 as core::ffi::c_int * 11 as core::ffi::c_int) as isize),
                    );
                out_buf = out_buf
                    .offset(
                        -((128 as core::ffi::c_int * 6 as core::ffi::c_int) as isize),
                    );
                i += 1 as core::ffi::c_int;
            }
        } else {
            let mut in_buf_0: *mut FLOAT32 = &mut *(*((*ptr_hbe_txposer).qmf_in_buf)
                .offset(0 as core::ffi::c_int as isize))
                .offset((2 as WORD32 * inp_band_idx) as isize) as *mut FLOAT32;
            let mut in_buf1: *mut FLOAT32 = &mut *(*((*ptr_hbe_txposer).qmf_in_buf)
                .offset(0 as core::ffi::c_int as isize))
                .offset(
                    (2 as core::ffi::c_int
                        * (inp_band_idx as core::ffi::c_int + 1 as core::ffi::c_int))
                        as isize,
                ) as *mut FLOAT32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < qmf_voc_columns {
                let mut k_0: WORD32 = 0;
                let mut vec_x_0: [FLOAT32; 26] = [0.; 26];
                let mut vec_x_cap: [FLOAT32; 26] = [0.; 26];
                let mut x_zero_band_r_0: FLOAT32 = 0.;
                let mut x_zero_band_i_0: FLOAT32 = 0.;
                let mut ptr_vec_x_0: *mut FLOAT32 = &mut *vec_x_0
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                let mut ptr_vec_x_cap: *mut FLOAT32 = &mut *vec_x_cap
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                let mut mag_scaling_fac_0: FLOAT32 = 0.;
                k_0 = 0 as core::ffi::c_int as WORD32;
                while k_0 < 8 as core::ffi::c_int {
                    let mut tmp_vr: FLOAT32 = 0.;
                    let mut tmp_vi: FLOAT32 = 0.;
                    let mut tmp_cr: FLOAT32 = 0.;
                    let mut tmp_ci: FLOAT32 = 0.;
                    let mut base1_0: FLOAT64 = 0.;
                    let mut base_0: FLOAT64 = 1e-17f64;
                    temp_r1 = *in_buf_0.offset(0 as core::ffi::c_int as isize);
                    temp_i1 = *in_buf_0.offset(1 as core::ffi::c_int as isize);
                    temp_r = *in_buf1.offset(0 as core::ffi::c_int as isize);
                    temp_i = *in_buf1.offset(1 as core::ffi::c_int as isize);
                    base1_0 = base_0 + (temp_r * temp_r) as FLOAT64;
                    base1_0 = base1_0 + (temp_i * temp_i) as FLOAT64;
                    mag_scaling_fac_0 = cbrt(
                        (1.0f32 / base1_0 as FLOAT32) as core::ffi::c_double,
                    ) as FLOAT32;
                    *ptr_vec_x_0.offset(0 as core::ffi::c_int as isize) = temp_r
                        * mag_scaling_fac_0;
                    *ptr_vec_x_0.offset(1 as core::ffi::c_int as isize) = temp_i
                        * mag_scaling_fac_0;
                    base1_0 = base_0 + (temp_r1 * temp_r1) as FLOAT64;
                    base1_0 = base1_0 + (temp_i1 * temp_i1) as FLOAT64;
                    mag_scaling_fac_0 = cbrt(
                        (1.0f32 / base1_0 as FLOAT32) as core::ffi::c_double,
                    ) as FLOAT32;
                    *ptr_vec_x_cap.offset(0 as core::ffi::c_int as isize) = temp_r1
                        * mag_scaling_fac_0;
                    *ptr_vec_x_cap.offset(1 as core::ffi::c_int as isize) = temp_i1
                        * mag_scaling_fac_0;
                    in_buf_0 = in_buf_0.offset(256 as core::ffi::c_int as isize);
                    temp_r = *in_buf_0.offset(0 as core::ffi::c_int as isize);
                    temp_i = *in_buf_0.offset(1 as core::ffi::c_int as isize);
                    temp_r1 = *ptr_sel.offset(0 as core::ffi::c_int as isize) * temp_r
                        + *ptr_sel.offset(1 as core::ffi::c_int as isize) * temp_i;
                    temp_i1 = *ptr_sel.offset(2 as core::ffi::c_int as isize) * temp_r
                        + *ptr_sel.offset(3 as core::ffi::c_int as isize) * temp_i;
                    in_buf_0 = in_buf_0.offset(-(128 as core::ffi::c_int as isize));
                    temp_r = *in_buf_0.offset(0 as core::ffi::c_int as isize);
                    temp_i = *in_buf_0.offset(1 as core::ffi::c_int as isize);
                    tmp_cr = temp_r1
                        + *ptr_sel.offset(4 as core::ffi::c_int as isize) * temp_r
                        + *ptr_sel.offset(5 as core::ffi::c_int as isize) * temp_i;
                    tmp_ci = temp_i1
                        + *ptr_sel.offset(6 as core::ffi::c_int as isize) * temp_r
                        + *ptr_sel.offset(7 as core::ffi::c_int as isize) * temp_i;
                    in_buf1 = in_buf1.offset(256 as core::ffi::c_int as isize);
                    temp_r = *in_buf1.offset(0 as core::ffi::c_int as isize);
                    temp_i = *in_buf1.offset(1 as core::ffi::c_int as isize);
                    temp_r1 = *ptr_sel1.offset(0 as core::ffi::c_int as isize) * temp_r
                        + *ptr_sel1.offset(1 as core::ffi::c_int as isize) * temp_i;
                    temp_i1 = *ptr_sel1.offset(2 as core::ffi::c_int as isize) * temp_r
                        + *ptr_sel1.offset(3 as core::ffi::c_int as isize) * temp_i;
                    in_buf1 = in_buf1.offset(-(128 as core::ffi::c_int as isize));
                    temp_r = *in_buf1.offset(0 as core::ffi::c_int as isize);
                    temp_i = *in_buf1.offset(1 as core::ffi::c_int as isize);
                    tmp_vr = temp_r1
                        + *ptr_sel1.offset(4 as core::ffi::c_int as isize) * temp_r
                        + *ptr_sel1.offset(5 as core::ffi::c_int as isize) * temp_i;
                    tmp_vi = temp_i1
                        + *ptr_sel1.offset(6 as core::ffi::c_int as isize) * temp_r
                        + *ptr_sel1.offset(7 as core::ffi::c_int as isize) * temp_i;
                    tmp_cr *= 0.3984033437f32;
                    tmp_ci *= 0.3984033437f32;
                    tmp_vr *= 0.3984033437f32;
                    tmp_vi *= 0.3984033437f32;
                    base1_0 = base_0 + (tmp_vr * tmp_vr) as FLOAT64;
                    base1_0 = base1_0 + (tmp_vi * tmp_vi) as FLOAT64;
                    mag_scaling_fac_0 = cbrt(
                        (1.0f32 / base1_0 as FLOAT32) as core::ffi::c_double,
                    ) as FLOAT32;
                    *ptr_vec_x_0.offset(2 as core::ffi::c_int as isize) = tmp_vr
                        * mag_scaling_fac_0;
                    *ptr_vec_x_0.offset(3 as core::ffi::c_int as isize) = tmp_vi
                        * mag_scaling_fac_0;
                    base1_0 = base_0 + (tmp_cr * tmp_cr) as FLOAT64;
                    base1_0 = base1_0 + (tmp_ci * tmp_ci) as FLOAT64;
                    mag_scaling_fac_0 = cbrt(
                        (1.0f32 / base1_0 as FLOAT32) as core::ffi::c_double,
                    ) as FLOAT32;
                    *ptr_vec_x_cap.offset(2 as core::ffi::c_int as isize) = tmp_cr
                        * mag_scaling_fac_0;
                    *ptr_vec_x_cap.offset(3 as core::ffi::c_int as isize) = tmp_ci
                        * mag_scaling_fac_0;
                    in_buf_0 = in_buf_0.offset(256 as core::ffi::c_int as isize);
                    in_buf1 = in_buf1.offset(256 as core::ffi::c_int as isize);
                    ptr_vec_x_0 = ptr_vec_x_0.offset(4 as core::ffi::c_int as isize);
                    ptr_vec_x_cap = ptr_vec_x_cap.offset(4 as core::ffi::c_int as isize);
                    k_0 += 2 as core::ffi::c_int;
                }
                ptr_vec_x_0 = &mut *vec_x_0
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                ptr_vec_x_cap = &mut *vec_x_cap
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                temp_r = vec_x_cap[(2 as core::ffi::c_int
                    * (HBE_ZERO_BAND_IDX - 2 as core::ffi::c_int)) as usize];
                temp_i = vec_x_cap[(2 as core::ffi::c_int
                    * (HBE_ZERO_BAND_IDX - 2 as core::ffi::c_int)
                    + 1 as core::ffi::c_int) as usize];
                temp_r1 = vec_x_0[(2 as core::ffi::c_int
                    * (HBE_ZERO_BAND_IDX - 2 as core::ffi::c_int)) as usize];
                temp_i1 = vec_x_0[(2 as core::ffi::c_int
                    * (HBE_ZERO_BAND_IDX - 2 as core::ffi::c_int)
                    + 1 as core::ffi::c_int) as usize];
                x_zero_band_r_0 = temp_r * temp_r - temp_i * temp_i;
                x_zero_band_i_0 = temp_r * temp_i + temp_i * temp_r;
                temp_r = temp_r1 * temp_r1 - temp_i1 * temp_i1;
                temp_i = temp_r1 * temp_i1 + temp_i1 * temp_r1;
                k_0 = 0 as core::ffi::c_int as WORD32;
                while k_0 < 8 as core::ffi::c_int {
                    temp_r1 = *ptr_vec_x_0.offset(0 as core::ffi::c_int as isize)
                        * x_zero_band_r_0
                        - *ptr_vec_x_0.offset(1 as core::ffi::c_int as isize)
                            * x_zero_band_i_0;
                    temp_i1 = *ptr_vec_x_0.offset(0 as core::ffi::c_int as isize)
                        * x_zero_band_i_0
                        + *ptr_vec_x_0.offset(1 as core::ffi::c_int as isize)
                            * x_zero_band_r_0;
                    temp_r1
                        += *ptr_vec_x_cap.offset(0 as core::ffi::c_int as isize) * temp_r
                            - *ptr_vec_x_cap.offset(1 as core::ffi::c_int as isize)
                                * temp_i;
                    temp_i1
                        += *ptr_vec_x_cap.offset(0 as core::ffi::c_int as isize) * temp_i
                            + *ptr_vec_x_cap.offset(1 as core::ffi::c_int as isize)
                                * temp_r;
                    let ref mut fresh8 = *out_buf.offset(0 as core::ffi::c_int as isize);
                    *fresh8 += temp_r1 as core::ffi::c_float * 0.23570225f32;
                    let ref mut fresh9 = *out_buf.offset(1 as core::ffi::c_int as isize);
                    *fresh9 += temp_i1 as core::ffi::c_float * 0.23570225f32;
                    out_buf = out_buf.offset(128 as core::ffi::c_int as isize);
                    ptr_vec_x_0 = ptr_vec_x_0.offset(2 as core::ffi::c_int as isize);
                    ptr_vec_x_cap = ptr_vec_x_cap.offset(2 as core::ffi::c_int as isize);
                    k_0 += 1;
                }
                ixheaacd_hbe_xprod_proc_3(
                    ptr_hbe_txposer,
                    qmf_band_idx,
                    i,
                    p,
                    pitch_in_bins_idx,
                );
                in_buf_0 = in_buf_0
                    .offset(
                        -((128 as core::ffi::c_int * 11 as core::ffi::c_int) as isize),
                    );
                in_buf1 = in_buf1
                    .offset(
                        -((128 as core::ffi::c_int * 11 as core::ffi::c_int) as isize),
                    );
                out_buf = out_buf
                    .offset(
                        -((128 as core::ffi::c_int * 6 as core::ffi::c_int) as isize),
                    );
                i += 1;
            }
        }
        out_buf = out_buf
            .offset(
                -((256 as core::ffi::c_int * qmf_voc_columns as core::ffi::c_int
                    - 2 as core::ffi::c_int) as isize),
            );
        qmf_band_idx += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_post_anal_xprod4(
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    mut qmf_voc_columns: WORD32,
    mut qmf_band_idx: WORD32,
    mut p: FLOAT32,
    mut pitch_in_bins_idx: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut inp_band_idx: WORD32 = 0;
    let mut out_ptr: *mut FLOAT32 = &mut *(*((*ptr_hbe_txposer).qmf_out_buf)
        .offset(3 as core::ffi::c_int as isize))
        .offset((2 as WORD32 * qmf_band_idx) as isize) as *mut FLOAT32;
    ixheaacd_norm_qmf_in_buf_4(
        ptr_hbe_txposer,
        (qmf_band_idx >> 1 as core::ffi::c_int) - 1 as WORD32,
    );
    while qmf_band_idx < (*ptr_hbe_txposer).x_over_qmf[3 as core::ffi::c_int as usize] {
        let mut ip_idx: WORD32 = 0;
        let mut temp: FLOAT32 = 0.;
        let mut temp_r: FLOAT32 = 0.;
        let mut temp_i: FLOAT32 = 0.;
        let mut norm_ptr: *mut FLOAT32 = 0 as *mut FLOAT32;
        let mut x_norm_ptr: *mut FLOAT32 = 0 as *mut FLOAT32;
        inp_band_idx = qmf_band_idx >> 1 as core::ffi::c_int;
        ip_idx = (if qmf_band_idx as core::ffi::c_int & 1 as core::ffi::c_int != 0 {
            inp_band_idx as core::ffi::c_int + 1 as core::ffi::c_int
        } else {
            inp_band_idx as core::ffi::c_int - 1 as core::ffi::c_int
        }) as WORD32;
        norm_ptr = &mut *(*((*ptr_hbe_txposer).norm_qmf_in_buf)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset((2 as WORD32 * ip_idx) as isize) as *mut FLOAT32;
        x_norm_ptr = &mut *(*((*ptr_hbe_txposer).norm_qmf_in_buf)
            .as_mut_ptr()
            .offset(HBE_ZERO_BAND_IDX as isize))
            .as_mut_ptr()
            .offset((2 as WORD32 * inp_band_idx) as isize) as *mut FLOAT32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < qmf_voc_columns {
            let mut k: WORD32 = 0;
            let mut x_zero_band_r: FLOAT32 = 0.;
            let mut x_zero_band_i: FLOAT32 = 0.;
            let fresh0 = x_norm_ptr;
            x_norm_ptr = x_norm_ptr.offset(1);
            x_zero_band_r = *fresh0;
            temp_r = x_zero_band_r;
            let fresh1 = x_norm_ptr;
            x_norm_ptr = x_norm_ptr.offset(1);
            x_zero_band_i = *fresh1;
            temp_i = x_zero_band_i;
            temp = x_zero_band_r * x_zero_band_r - x_zero_band_i * x_zero_band_i;
            x_zero_band_i = x_zero_band_r * x_zero_band_i
                + x_zero_band_i * x_zero_band_r;
            x_zero_band_r = temp_r * temp - temp_i * x_zero_band_i;
            x_zero_band_i = temp_r * x_zero_band_i + temp_i * temp;
            k = 0 as core::ffi::c_int as WORD32;
            while k < HBE_OPER_BLK_LEN_4 {
                let fresh2 = norm_ptr;
                norm_ptr = norm_ptr.offset(1);
                temp = *fresh2;
                let fresh3 = norm_ptr;
                norm_ptr = norm_ptr.offset(1);
                temp_i = *fresh3;
                temp_r = temp * x_zero_band_r - temp_i * x_zero_band_i;
                temp_i = temp * x_zero_band_i + temp_i * x_zero_band_r;
                let fresh4 = out_ptr;
                out_ptr = out_ptr.offset(1);
                *fresh4 += temp_r as core::ffi::c_float * 0.6666667f32;
                let fresh5 = out_ptr;
                out_ptr = out_ptr.offset(1);
                *fresh5 += temp_i as core::ffi::c_float * 0.6666667f32;
                norm_ptr = norm_ptr.offset(254 as core::ffi::c_int as isize);
                out_ptr = out_ptr.offset(126 as core::ffi::c_int as isize);
                k += 1;
            }
            norm_ptr = norm_ptr
                .offset(-((128 as core::ffi::c_int * 11 as core::ffi::c_int) as isize));
            out_ptr = out_ptr
                .offset(-((128 as core::ffi::c_int * 4 as core::ffi::c_int) as isize));
            x_norm_ptr = x_norm_ptr.offset(126 as core::ffi::c_int as isize);
            ixheaacd_hbe_xprod_proc_4(
                ptr_hbe_txposer,
                qmf_band_idx,
                i,
                p,
                pitch_in_bins_idx,
            );
            i += 1;
        }
        out_ptr = out_ptr
            .offset(
                -((128 as core::ffi::c_int * 2 as core::ffi::c_int
                    * qmf_voc_columns as core::ffi::c_int - 2 as core::ffi::c_int)
                    as isize),
            );
        qmf_band_idx += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_post_anal_process(
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    mut pitch_in_bins: WORD32,
    mut sbr_upsamp_4_flg: WORD32,
) -> IA_ERRORCODE {
    let mut p: FLOAT32 = 0.;
    let mut trans_fac: WORD32 = 0;
    let mut qmf_voc_columns: WORD32 = (*ptr_hbe_txposer).no_bins / 2 as WORD32;
    let mut cos_sin_theta: [FLOAT32; 2] = [0.; 2];
    p = if sbr_upsamp_4_flg != 0 {
        (pitch_in_bins as core::ffi::c_double * 0.04166666666666f64) as FLOAT32
    } else {
        (pitch_in_bins as core::ffi::c_double * 0.08333333333333f64) as FLOAT32
    };
    if p < SBR_CONST_PMIN {
        trans_fac = 2 as core::ffi::c_int as WORD32;
        if trans_fac <= (*ptr_hbe_txposer).max_stretch {
            ixheaacd_hbe_post_anal_prod2(
                ptr_hbe_txposer,
                qmf_voc_columns,
                (*ptr_hbe_txposer).x_over_qmf[0 as core::ffi::c_int as usize],
            );
        }
        trans_fac = 3 as core::ffi::c_int as WORD32;
        if trans_fac <= (*ptr_hbe_txposer).max_stretch {
            ixheaacd_hbe_post_anal_prod3(
                ptr_hbe_txposer,
                qmf_voc_columns,
                (*ptr_hbe_txposer).x_over_qmf[1 as core::ffi::c_int as usize],
            );
        }
        trans_fac = 4 as core::ffi::c_int as WORD32;
        if trans_fac <= (*ptr_hbe_txposer).max_stretch {
            if (*ptr_hbe_txposer).x_over_qmf[2 as core::ffi::c_int as usize]
                <= 1 as core::ffi::c_int
            {
                return IA_FATAL_ERROR as IA_ERRORCODE;
            }
            ixheaacd_hbe_post_anal_prod4(
                ptr_hbe_txposer,
                qmf_voc_columns,
                (*ptr_hbe_txposer).x_over_qmf[2 as core::ffi::c_int as usize],
            );
        }
    } else {
        trans_fac = 2 as core::ffi::c_int as WORD32;
        if trans_fac <= (*ptr_hbe_txposer).max_stretch {
            cos_sin_theta[0 as core::ffi::c_int as usize] = ixheaac_hbe_x_prod_cos_table_trans_2[(((pitch_in_bins
                as core::ffi::c_int
                + sbr_upsamp_4_flg as core::ffi::c_int * 128 as core::ffi::c_int)
                << 1 as core::ffi::c_int) + 0 as core::ffi::c_int) as usize];
            cos_sin_theta[1 as core::ffi::c_int as usize] = ixheaac_hbe_x_prod_cos_table_trans_2[(((pitch_in_bins
                as core::ffi::c_int
                + sbr_upsamp_4_flg as core::ffi::c_int * 128 as core::ffi::c_int)
                << 1 as core::ffi::c_int) + 1 as core::ffi::c_int) as usize];
            ixheaacd_hbe_post_anal_xprod2(
                ptr_hbe_txposer,
                qmf_voc_columns,
                (*ptr_hbe_txposer).x_over_qmf[0 as core::ffi::c_int as usize],
                p,
                cos_sin_theta.as_mut_ptr(),
            );
        }
        trans_fac = 3 as core::ffi::c_int as WORD32;
        if trans_fac <= (*ptr_hbe_txposer).max_stretch {
            ixheaacd_hbe_post_anal_xprod3(
                ptr_hbe_txposer,
                qmf_voc_columns,
                (*ptr_hbe_txposer).x_over_qmf[1 as core::ffi::c_int as usize],
                p,
                pitch_in_bins + sbr_upsamp_4_flg * 128 as WORD32,
            );
        }
        trans_fac = 4 as core::ffi::c_int as WORD32;
        if trans_fac <= (*ptr_hbe_txposer).max_stretch {
            if (*ptr_hbe_txposer).x_over_qmf[2 as core::ffi::c_int as usize]
                <= 1 as core::ffi::c_int
            {
                return IA_FATAL_ERROR as IA_ERRORCODE;
            }
            ixheaacd_hbe_post_anal_xprod4(
                ptr_hbe_txposer,
                qmf_voc_columns,
                (*ptr_hbe_txposer).x_over_qmf[2 as core::ffi::c_int as usize],
                p,
                pitch_in_bins + sbr_upsamp_4_flg * 128 as WORD32,
            );
        }
    }
    return IA_NO_ERROR;
}
pub const MAX_NUM_PATCHES: core::ffi::c_int = 6 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
