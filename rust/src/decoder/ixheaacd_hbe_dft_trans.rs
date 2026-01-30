extern "C" {
    fn acos(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn atan2(__y: core::ffi::c_double, __x: core::ffi::c_double) -> core::ffi::c_double;
    fn cos(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn sin(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn pow(__x: core::ffi::c_double, __y: core::ffi::c_double) -> core::ffi::c_double;
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
    fn ixheaacd_hbe_apply_cfftn(
        re: *mut FLOAT32,
        scratch: *mut FLOAT32,
        n_pass: WORD32,
        i_sign: WORD32,
    ) -> VOID;
    fn ixheaacd_hbe_apply_cfftn_gen(
        re: *mut FLOAT32,
        scratch: *mut FLOAT32,
        n_pass: WORD32,
        i_sign: WORD32,
    ) -> VOID;
    fn ixheaacd_hbe_apply_fft_288(
        inp: *mut FLOAT32,
        scratch: *mut FLOAT32,
        len: WORD32,
        i_sign: WORD32,
    ) -> VOID;
    fn ixheaacd_hbe_apply_ifft_224(
        inp: *mut FLOAT32,
        scratch: *mut FLOAT32,
        len: WORD32,
        i_sign: WORD32,
    ) -> VOID;
    fn ixheaacd_hbe_apply_ifft_336(
        inp: *mut FLOAT32,
        ptr_scratch: *mut FLOAT32,
        len: WORD32,
        i_sign: WORD32,
    ) -> VOID;
    static ixheaac_start_subband2kL_tbl: [WORD32; 33];
    fn ixheaacd_real_synth_filt(
        ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
        num_columns: WORD32,
        qmf_buf_real: *mut [FLOAT32; 64],
        qmf_buf_imag: *mut [FLOAT32; 64],
    ) -> WORD32;
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
    fn ixheaacd_dft_hbe_cplx_anal_filt(
        ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
        qmf_buf_real: *mut [FLOAT32; 64],
        qmf_buf_imag: *mut [FLOAT32; 64],
    ) -> WORD32;
    static ixheaac_synth_cos_table_kl_4: [FLOAT32; 16];
    static ixheaac_synth_cos_table_kl_8: [FLOAT32; 32];
    static ixheaac_synth_cos_table_kl_12: [FLOAT32; 48];
    static ixheaac_synth_cos_table_kl_16: [FLOAT32; 64];
    static ixheaac_synth_cos_table_kl_20: [FLOAT32; 800];
    static ixheaac_dft_hbe_window_ts_12: [FLOAT32; 13];
    static ixheaac_dft_hbe_window_ts_18: [FLOAT32; 19];
    static ixheaac_sine_pi_n_by_1024: [FLOAT32; 1024];
    static ixheaac_sine_pi_n_by_960: [FLOAT32; 960];
    static ixheaac_sine_pi_n_by_896: [FLOAT32; 896];
    static ixheaac_sine_pi_n_by_832: [FLOAT32; 832];
    static ixheaac_sine_pi_n_by_768: [FLOAT32; 768];
    static ixheaac_sine_pi_n_by_704: [FLOAT32; 704];
    static ixheaac_sine_pi_n_by_640: [FLOAT32; 640];
    static ixheaac_sine_pi_n_by_576: [FLOAT32; 576];
    static ixheaac_sine_pi_by_2_N: [FLOAT32; 0];
    static ixheaac_sin_cos_448: [FLOAT32; 0];
    static ixheaac_sin_cos_672: [FLOAT32; 0];
    static ixheaac_sin_cos_512: [FLOAT32; 0];
    static ixheaac_sin_cos_576: [FLOAT32; 0];
    static ixheaac_sin_cos_384: [FLOAT32; 0];
    static ixheaac_sin_cos_768: [FLOAT32; 0];
    static ixheaac_sub_samp_qmf_window_coeff_28_36: [FLOAT32; 640];
}
pub type size_t = usize;
pub type WORD16 = core::ffi::c_short;
pub type WORD32 = core::ffi::c_int;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type WORD = core::ffi::c_int;
pub type IA_ERRORCODE = WORD32;
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
pub const PI: core::ffi::c_double = 3.14159265358979323846264338327950288f64;
pub const MAX_STRETCH: core::ffi::c_int = 4 as core::ffi::c_int;
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_FATAL_ERROR: core::ffi::c_uint = 0x80000000 as core::ffi::c_uint;
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
        28 => {
            return &*ixheaac_sub_samp_qmf_window_coeff_28_36
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const FLOAT32 as *mut FLOAT32;
        }
        32 => {
            return &*ixheaac_sub_samp_qmf_window_coeff
                .as_ptr()
                .offset(840 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
        }
        36 => {
            return &*ixheaac_sub_samp_qmf_window_coeff_28_36
                .as_ptr()
                .offset(280 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
        }
        40 => {
            return &*ixheaac_sub_samp_qmf_window_coeff
                .as_ptr()
                .offset(1160 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
        }
        44 => {
            return &*ixheaac_sub_samp_qmf_window_coeff
                .as_ptr()
                .offset(1560 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
        }
        _ => {
            return &*ixheaac_sub_samp_qmf_window_coeff
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const FLOAT32 as *mut FLOAT32;
        }
    };
}
unsafe extern "C" fn ixheaacd_create_dft_hbe_window(
    mut win: *mut FLOAT32,
    mut x_over_bin1: WORD32,
    mut x_over_bin2: WORD32,
    mut ts: WORD32,
    mut size: WORD32,
) -> VOID {
    let mut ptr_freq_domain_win: *const FLOAT32 = 0 as *const FLOAT32;
    let mut n: WORD32 = 0;
    if ts == 12 as core::ffi::c_int {
        ptr_freq_domain_win = &*ixheaac_dft_hbe_window_ts_12
            .as_ptr()
            .offset(0 as core::ffi::c_int as isize) as *const FLOAT32;
    } else {
        ptr_freq_domain_win = &*ixheaac_dft_hbe_window_ts_18
            .as_ptr()
            .offset(0 as core::ffi::c_int as isize) as *const FLOAT32;
    }
    n = 0 as core::ffi::c_int as WORD32;
    while n
        < x_over_bin1 as core::ffi::c_int
            - ts as core::ffi::c_int / 2 as core::ffi::c_int
    {
        *win.offset(n as isize) = 0 as core::ffi::c_int as FLOAT32;
        n += 1;
    }
    n = (x_over_bin1 as core::ffi::c_int
        - ts as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
    while n
        <= x_over_bin1 as core::ffi::c_int
            + ts as core::ffi::c_int / 2 as core::ffi::c_int
    {
        *win.offset(n as isize) = *ptr_freq_domain_win
            .offset(
                (n as core::ffi::c_int
                    - (x_over_bin1 as core::ffi::c_int
                        - ts as core::ffi::c_int / 2 as core::ffi::c_int)) as isize,
            );
        n += 1;
    }
    n = (x_over_bin1 as core::ffi::c_int + ts as core::ffi::c_int / 2 as core::ffi::c_int
        + 1 as core::ffi::c_int) as WORD32;
    while n
        < x_over_bin2 as core::ffi::c_int
            - ts as core::ffi::c_int / 2 as core::ffi::c_int
    {
        *win.offset(n as isize) = 1.0f32;
        n += 1;
    }
    n = (x_over_bin2 as core::ffi::c_int
        - ts as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
    while n
        <= x_over_bin2 as core::ffi::c_int
            + ts as core::ffi::c_int / 2 as core::ffi::c_int
    {
        *win.offset(n as isize) = 1.0f32
            - *ptr_freq_domain_win
                .offset(
                    (n as core::ffi::c_int
                        - (x_over_bin2 as core::ffi::c_int
                            - ts as core::ffi::c_int / 2 as core::ffi::c_int)) as isize,
                );
        n += 1;
    }
    n = (x_over_bin2 as core::ffi::c_int + ts as core::ffi::c_int / 2 as core::ffi::c_int
        + 1 as core::ffi::c_int) as WORD32;
    while n < size {
        *win.offset(n as isize) = 0.0f32 as FLOAT32;
        n += 1;
    }
}
unsafe extern "C" fn ixheaacd_calc_anal_synth_window(
    mut fft_size: WORD32,
    mut ptr_window: *mut FLOAT32,
) -> WORD32 {
    let mut sin_pi_2_N: FLOAT32 = 0.0f32;
    let mut cos_pi_2_N: FLOAT32 = 0.0f32;
    let mut ptr_sin_pi_n_by_N: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut hop_stride: WORD32 = 1 as WORD32;
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut l_fft_stride: WORD32 = 512 as WORD32;
    match fft_size {
        64 => {
            hop_stride = 16 as core::ffi::c_int as WORD32;
            ptr_sin_pi_n_by_N = &*ixheaac_sine_pi_n_by_1024
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
            sin_pi_2_N = *ptr_sin_pi_n_by_N
                .offset((hop_stride >> 1 as core::ffi::c_int) as isize);
            cos_pi_2_N = *ptr_sin_pi_n_by_N
                .offset(
                    (512 as WORD32 + (hop_stride >> 1 as core::ffi::c_int)) as isize,
                );
            l_fft_stride = 512 as core::ffi::c_int as WORD32;
        }
        128 => {
            hop_stride = 8 as core::ffi::c_int as WORD32;
            ptr_sin_pi_n_by_N = &*ixheaac_sine_pi_n_by_1024
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
            sin_pi_2_N = *ptr_sin_pi_n_by_N
                .offset((hop_stride >> 1 as core::ffi::c_int) as isize);
            cos_pi_2_N = *ptr_sin_pi_n_by_N
                .offset(
                    (512 as WORD32 + (hop_stride >> 1 as core::ffi::c_int)) as isize,
                );
            l_fft_stride = 512 as core::ffi::c_int as WORD32;
        }
        256 => {
            hop_stride = 4 as core::ffi::c_int as WORD32;
            ptr_sin_pi_n_by_N = &*ixheaac_sine_pi_n_by_1024
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
            sin_pi_2_N = *ptr_sin_pi_n_by_N
                .offset((hop_stride >> 1 as core::ffi::c_int) as isize);
            cos_pi_2_N = *ptr_sin_pi_n_by_N
                .offset(
                    (512 as WORD32 + (hop_stride >> 1 as core::ffi::c_int)) as isize,
                );
            l_fft_stride = 512 as core::ffi::c_int as WORD32;
        }
        512 => {
            hop_stride = 2 as core::ffi::c_int as WORD32;
            ptr_sin_pi_n_by_N = &*ixheaac_sine_pi_n_by_1024
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
            sin_pi_2_N = *ptr_sin_pi_n_by_N.offset(1 as core::ffi::c_int as isize);
            cos_pi_2_N = *ptr_sin_pi_n_by_N
                .offset((512 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            l_fft_stride = 512 as core::ffi::c_int as WORD32;
        }
        1024 => {
            hop_stride = 1 as core::ffi::c_int as WORD32;
            ptr_sin_pi_n_by_N = &*ixheaac_sine_pi_n_by_1024
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
            sin_pi_2_N = *ixheaac_sine_pi_by_2_N
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize);
            cos_pi_2_N = *ixheaac_sine_pi_by_2_N
                .as_ptr()
                .offset(1 as core::ffi::c_int as isize);
            l_fft_stride = 512 as core::ffi::c_int as WORD32;
        }
        192 => {
            hop_stride = 4 as core::ffi::c_int as WORD32;
            ptr_sin_pi_n_by_N = &*ixheaac_sine_pi_n_by_768
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
            sin_pi_2_N = *ptr_sin_pi_n_by_N
                .offset((hop_stride >> 1 as core::ffi::c_int) as isize);
            cos_pi_2_N = *ptr_sin_pi_n_by_N
                .offset(
                    (384 as WORD32 + (hop_stride >> 1 as core::ffi::c_int)) as isize,
                );
            l_fft_stride = 384 as core::ffi::c_int as WORD32;
        }
        384 => {
            hop_stride = 2 as core::ffi::c_int as WORD32;
            ptr_sin_pi_n_by_N = &*ixheaac_sine_pi_n_by_768
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
            sin_pi_2_N = *ptr_sin_pi_n_by_N.offset(1 as core::ffi::c_int as isize);
            cos_pi_2_N = *ptr_sin_pi_n_by_N
                .offset((384 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            l_fft_stride = 384 as core::ffi::c_int as WORD32;
        }
        768 => {
            hop_stride = 1 as core::ffi::c_int as WORD32;
            ptr_sin_pi_n_by_N = &*ixheaac_sine_pi_n_by_768
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
            sin_pi_2_N = *ixheaac_sine_pi_by_2_N
                .as_ptr()
                .offset(8 as core::ffi::c_int as isize);
            cos_pi_2_N = *ixheaac_sine_pi_by_2_N
                .as_ptr()
                .offset(9 as core::ffi::c_int as isize);
            l_fft_stride = 384 as core::ffi::c_int as WORD32;
        }
        320 => {
            hop_stride = 3 as core::ffi::c_int as WORD32;
            ptr_sin_pi_n_by_N = &*ixheaac_sine_pi_n_by_960
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
            sin_pi_2_N = *ixheaac_sine_pi_by_2_N
                .as_ptr()
                .offset(16 as core::ffi::c_int as isize);
            cos_pi_2_N = *ixheaac_sine_pi_by_2_N
                .as_ptr()
                .offset(17 as core::ffi::c_int as isize);
            l_fft_stride = 480 as core::ffi::c_int as WORD32;
        }
        960 => {
            hop_stride = 1 as core::ffi::c_int as WORD32;
            ptr_sin_pi_n_by_N = &*ixheaac_sine_pi_n_by_960
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
            sin_pi_2_N = *ixheaac_sine_pi_by_2_N
                .as_ptr()
                .offset(2 as core::ffi::c_int as isize);
            cos_pi_2_N = *ixheaac_sine_pi_by_2_N
                .as_ptr()
                .offset(3 as core::ffi::c_int as isize);
            l_fft_stride = 480 as core::ffi::c_int as WORD32;
        }
        448 => {
            hop_stride = 2 as core::ffi::c_int as WORD32;
            ptr_sin_pi_n_by_N = &*ixheaac_sine_pi_n_by_896
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
            sin_pi_2_N = *ptr_sin_pi_n_by_N.offset(1 as core::ffi::c_int as isize);
            cos_pi_2_N = *ptr_sin_pi_n_by_N
                .offset((448 as core::ffi::c_int + 1 as core::ffi::c_int) as isize);
            l_fft_stride = 448 as core::ffi::c_int as WORD32;
        }
        896 => {
            hop_stride = 1 as core::ffi::c_int as WORD32;
            ptr_sin_pi_n_by_N = &*ixheaac_sine_pi_n_by_896
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
            sin_pi_2_N = *ixheaac_sine_pi_by_2_N
                .as_ptr()
                .offset(4 as core::ffi::c_int as isize);
            cos_pi_2_N = *ixheaac_sine_pi_by_2_N
                .as_ptr()
                .offset(5 as core::ffi::c_int as isize);
            l_fft_stride = 448 as core::ffi::c_int as WORD32;
        }
        576 => {
            hop_stride = 1 as core::ffi::c_int as WORD32;
            ptr_sin_pi_n_by_N = &*ixheaac_sine_pi_n_by_576
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
            sin_pi_2_N = *ixheaac_sine_pi_by_2_N
                .as_ptr()
                .offset(14 as core::ffi::c_int as isize);
            cos_pi_2_N = *ixheaac_sine_pi_by_2_N
                .as_ptr()
                .offset(15 as core::ffi::c_int as isize);
            l_fft_stride = 288 as core::ffi::c_int as WORD32;
        }
        640 => {
            hop_stride = 1 as core::ffi::c_int as WORD32;
            ptr_sin_pi_n_by_N = &*ixheaac_sine_pi_n_by_640
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
            sin_pi_2_N = *ixheaac_sine_pi_by_2_N
                .as_ptr()
                .offset(12 as core::ffi::c_int as isize);
            cos_pi_2_N = *ixheaac_sine_pi_by_2_N
                .as_ptr()
                .offset(13 as core::ffi::c_int as isize);
            l_fft_stride = 320 as core::ffi::c_int as WORD32;
        }
        704 => {
            hop_stride = 1 as core::ffi::c_int as WORD32;
            ptr_sin_pi_n_by_N = &*ixheaac_sine_pi_n_by_704
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
            sin_pi_2_N = *ixheaac_sine_pi_by_2_N
                .as_ptr()
                .offset(10 as core::ffi::c_int as isize);
            cos_pi_2_N = *ixheaac_sine_pi_by_2_N
                .as_ptr()
                .offset(11 as core::ffi::c_int as isize);
            l_fft_stride = 352 as core::ffi::c_int as WORD32;
        }
        832 => {
            hop_stride = 1 as core::ffi::c_int as WORD32;
            ptr_sin_pi_n_by_N = &*ixheaac_sine_pi_n_by_832
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const FLOAT32
                as *mut FLOAT32;
            sin_pi_2_N = *ixheaac_sine_pi_by_2_N
                .as_ptr()
                .offset(6 as core::ffi::c_int as isize);
            cos_pi_2_N = *ixheaac_sine_pi_by_2_N
                .as_ptr()
                .offset(7 as core::ffi::c_int as isize);
            l_fft_stride = 416 as core::ffi::c_int as WORD32;
        }
        _ => return -(1 as WORD32),
    }
    i = 0 as core::ffi::c_int as WORD32;
    j = 0 as core::ffi::c_int as WORD32;
    while j < fft_size >> 1 as core::ffi::c_int {
        let mut cos_val: FLOAT32 = *ptr_sin_pi_n_by_N
            .offset((i + l_fft_stride) as isize);
        let mut sin_val: FLOAT32 = *ptr_sin_pi_n_by_N.offset(i as isize);
        *ptr_window.offset(j as isize) = cos_val * sin_pi_2_N + sin_val * cos_pi_2_N;
        i += hop_stride;
        j += 1;
    }
    while j < fft_size {
        let mut cos_val_0: FLOAT32 = *ptr_sin_pi_n_by_N
            .offset((i - l_fft_stride) as isize);
        let mut sin_val_0: FLOAT32 = *ptr_sin_pi_n_by_N.offset(i as isize);
        *ptr_window.offset(j as isize) = sin_val_0 * cos_pi_2_N - cos_val_0 * sin_pi_2_N;
        j += 1;
        i += hop_stride;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dft_hbe_data_reinit(
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    mut p_freq_band_tab: *mut *mut WORD16,
    mut p_num_sfb: *mut WORD16,
) -> WORD32 {
    let mut sfb: WORD32 = 0;
    let mut patch: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut temp_start: WORD32 = 0;
    let mut fb_ratio: FLOAT32 = 0.;
    let mut stop_patch: WORD32 = 0;
    let mut in_hop_size_divisor: WORD32 = 8 as WORD32;
    static mut trans_samp: [WORD32; 2] = [
        12 as core::ffi::c_int,
        18 as core::ffi::c_int,
    ];
    let mut err: WORD32 = 0 as WORD32;
    (*ptr_hbe_txposer).start_band = *(*p_freq_band_tab.offset(LOW as isize))
        .offset(0 as core::ffi::c_int as isize) as WORD32;
    (*ptr_hbe_txposer).end_band = *(*p_freq_band_tab.offset(LOW as isize))
        .offset(*p_num_sfb.offset(LOW as isize) as isize) as WORD32;
    (*ptr_hbe_txposer).esbr_hq = 1 as core::ffi::c_int as WORD32;
    (*ptr_hbe_txposer).synth_size = (4 as core::ffi::c_int
        * (((*ptr_hbe_txposer).start_band as core::ffi::c_int + 4 as core::ffi::c_int)
            / 8 as core::ffi::c_int + 1 as core::ffi::c_int)) as WORD32;
    (*ptr_hbe_txposer).k_start = ixheaac_start_subband2kL_tbl[(*ptr_hbe_txposer)
        .start_band as usize];
    fb_ratio = ((*ptr_hbe_txposer).synth_size as core::ffi::c_float / 32.0f32)
        as FLOAT32;
    (*ptr_hbe_txposer).ana_fft_size[0 as core::ffi::c_int as usize] = (fb_ratio
        * (*ptr_hbe_txposer).fft_size[0 as core::ffi::c_int as usize] as FLOAT32)
        as WORD32;
    (*ptr_hbe_txposer).ana_fft_size[1 as core::ffi::c_int as usize] = (fb_ratio
        * (*ptr_hbe_txposer).fft_size[1 as core::ffi::c_int as usize] as FLOAT32)
        as WORD32;
    (*ptr_hbe_txposer).in_hop_size = (*ptr_hbe_txposer)
        .ana_fft_size[0 as core::ffi::c_int as usize] / in_hop_size_divisor;
    (*ptr_hbe_txposer).synth_window = &mut *((*ptr_hbe_txposer).synthesis_window_buf)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
    (*ptr_hbe_txposer).anal_window = &mut *((*ptr_hbe_txposer).analysis_window_buf)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
    err = ixheaacd_calc_anal_synth_window(
        (*ptr_hbe_txposer).ana_fft_size[0 as core::ffi::c_int as usize],
        (*ptr_hbe_txposer).anal_window,
    );
    if err != 0 {
        return err;
    }
    memset(
        ((*ptr_hbe_txposer).synth_buf).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (1280 as size_t).wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
    (*ptr_hbe_txposer).synth_wind_coeff = ixheaacd_map_prot_filter(
        (*ptr_hbe_txposer).synth_size,
    );
    temp_start = (2 as core::ffi::c_int
        * (((*ptr_hbe_txposer).start_band as core::ffi::c_int - 1 as core::ffi::c_int)
            / 2 as core::ffi::c_int)) as WORD32;
    (*ptr_hbe_txposer).analy_size = (4 as core::ffi::c_int
        * (((if (64 as core::ffi::c_int)
            < (*ptr_hbe_txposer).end_band as core::ffi::c_int + 1 as core::ffi::c_int
        {
            64 as core::ffi::c_int
        } else {
            (*ptr_hbe_txposer).end_band as core::ffi::c_int + 1 as core::ffi::c_int
        }) - temp_start as core::ffi::c_int - 1 as core::ffi::c_int)
            / 4 as core::ffi::c_int + 1 as core::ffi::c_int)) as WORD32;
    (*ptr_hbe_txposer).a_start = (temp_start as core::ffi::c_int
        - (if 0 as core::ffi::c_int
            > temp_start as core::ffi::c_int
                + (*ptr_hbe_txposer).analy_size as core::ffi::c_int
                - 64 as core::ffi::c_int
        {
            0 as core::ffi::c_int
        } else {
            temp_start as core::ffi::c_int
                + (*ptr_hbe_txposer).analy_size as core::ffi::c_int
                - 64 as core::ffi::c_int
        })) as WORD32;
    fb_ratio = ((*ptr_hbe_txposer).analy_size as core::ffi::c_float / 64.0f32)
        as FLOAT32;
    (*ptr_hbe_txposer).syn_fft_size[0 as core::ffi::c_int as usize] = (fb_ratio
        * (*ptr_hbe_txposer).fft_size[0 as core::ffi::c_int as usize] as FLOAT32)
        as WORD32;
    (*ptr_hbe_txposer).syn_fft_size[1 as core::ffi::c_int as usize] = (fb_ratio
        * (*ptr_hbe_txposer).fft_size[1 as core::ffi::c_int as usize] as FLOAT32)
        as WORD32;
    (*ptr_hbe_txposer).out_hop_size = 2 as WORD32
        * (*ptr_hbe_txposer).syn_fft_size[0 as core::ffi::c_int as usize]
        / in_hop_size_divisor;
    err = ixheaacd_calc_anal_synth_window(
        (*ptr_hbe_txposer).syn_fft_size[0 as core::ffi::c_int as usize],
        (*ptr_hbe_txposer).synth_window,
    );
    if err != 0 {
        return err;
    }
    (*ptr_hbe_txposer).analy_wind_coeff = ixheaacd_map_prot_filter(
        (*ptr_hbe_txposer).analy_size,
    );
    memset(
        &mut *((*ptr_hbe_txposer).x_over_qmf)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut WORD32
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[WORD32; 6]>() as size_t,
    );
    i = 0 as core::ffi::c_int as WORD32;
    while i < MAX_STRETCH {
        memset(
            &mut *(*((*ptr_hbe_txposer).x_over_bin).as_mut_ptr().offset(i as isize))
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut WORD32
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (2 as size_t).wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
        );
        i += 1;
    }
    sfb = 0 as core::ffi::c_int as WORD32;
    stop_patch = MAX_STRETCH as WORD32;
    match (*ptr_hbe_txposer).synth_size {
        4 => {
            (*ptr_hbe_txposer).synth_cos_tab = ixheaac_synth_cos_table_kl_4.as_ptr()
                as *mut FLOAT32;
            (*ptr_hbe_txposer).ixheaacd_real_synth_fft = Some(
                ixheaac_real_synth_fft_p2
                    as unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
            )
                as Option<
                    unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
                >;
        }
        8 => {
            (*ptr_hbe_txposer).synth_cos_tab = ixheaac_synth_cos_table_kl_8.as_ptr()
                as *mut FLOAT32;
            (*ptr_hbe_txposer).ixheaacd_real_synth_fft = Some(
                ixheaac_real_synth_fft_p2
                    as unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
            )
                as Option<
                    unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
                >;
        }
        12 => {
            (*ptr_hbe_txposer).synth_cos_tab = ixheaac_synth_cos_table_kl_12.as_ptr()
                as *mut FLOAT32;
            (*ptr_hbe_txposer).ixheaacd_real_synth_fft = Some(
                ixheaac_real_synth_fft_p3
                    as unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
            )
                as Option<
                    unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
                >;
        }
        16 => {
            (*ptr_hbe_txposer).synth_cos_tab = ixheaac_synth_cos_table_kl_16.as_ptr()
                as *mut FLOAT32;
            (*ptr_hbe_txposer).ixheaacd_real_synth_fft = Some(
                ixheaac_real_synth_fft_p2
                    as unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
            )
                as Option<
                    unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
                >;
        }
        20 => {
            (*ptr_hbe_txposer).synth_cos_tab = ixheaac_synth_cos_table_kl_20.as_ptr()
                as *mut FLOAT32;
        }
        28 => {
            (*ptr_hbe_txposer).synth_cos_tab = ixheaac_synth_cos_table_kl_20.as_ptr()
                as *mut FLOAT32;
        }
        _ => {
            (*ptr_hbe_txposer).synth_cos_tab = ixheaac_synth_cos_table_kl_4.as_ptr()
                as *mut FLOAT32;
            (*ptr_hbe_txposer).ixheaacd_real_synth_fft = Some(
                ixheaac_real_synth_fft_p2
                    as unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
            )
                as Option<
                    unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
                >;
        }
    }
    let mut l: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut L: WORD32 = (*ptr_hbe_txposer).analy_size;
    k = 0 as core::ffi::c_int as WORD32;
    while k < L {
        l = 0 as core::ffi::c_int as WORD32;
        while l < 2 as WORD32 * L {
            (*ptr_hbe_txposer).str_dft_hbe_anal_coeff.real[k as usize][l as usize] = cos(
                PI / (2 as WORD32 * L) as core::ffi::c_double
                    * ((k as core::ffi::c_double + 0.5f64)
                        * ((2 as WORD32 * l) as core::ffi::c_double
                            - L as core::ffi::c_double / 64.0f64)
                        - L as core::ffi::c_double / 64.0f64
                            * (*ptr_hbe_txposer).a_start as core::ffi::c_double),
            ) as FLOAT32;
            (*ptr_hbe_txposer).str_dft_hbe_anal_coeff.imag[k as usize][l as usize] = sin(
                PI / (2 as WORD32 * L) as core::ffi::c_double
                    * ((k as core::ffi::c_double + 0.5f64)
                        * ((2 as WORD32 * l) as core::ffi::c_double
                            - L as core::ffi::c_double / 64.0f64)
                        - L as core::ffi::c_double / 64.0f64
                            * (*ptr_hbe_txposer).a_start as core::ffi::c_double),
            ) as FLOAT32;
            l += 1;
        }
        k += 1;
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
                    .offset((sfb as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int <= 3 as core::ffi::c_int
            {
                (*ptr_hbe_txposer)
                    .x_over_qmf[(patch as core::ffi::c_int - 1 as core::ffi::c_int)
                    as usize] = *(*p_freq_band_tab.offset(LOW as isize))
                    .offset((sfb as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                    as WORD32;
                if patch <= MAX_STRETCH {
                    (*ptr_hbe_txposer)
                        .x_over_bin[(patch as core::ffi::c_int - 1 as core::ffi::c_int)
                        as usize][0 as core::ffi::c_int as usize] = (((*ptr_hbe_txposer)
                        .fft_size[0 as core::ffi::c_int as usize]
                        * *(*p_freq_band_tab.offset(LOW as isize))
                            .offset(
                                (sfb as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ) as core::ffi::c_int / 128 as core::ffi::c_int)
                        as core::ffi::c_double + 0.5f64) as WORD32;
                    (*ptr_hbe_txposer)
                        .x_over_bin[(patch as core::ffi::c_int - 1 as core::ffi::c_int)
                        as usize][1 as core::ffi::c_int as usize] = (((*ptr_hbe_txposer)
                        .fft_size[1 as core::ffi::c_int as usize]
                        * *(*p_freq_band_tab.offset(LOW as isize))
                            .offset(
                                (sfb as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ) as core::ffi::c_int / 128 as core::ffi::c_int)
                        as core::ffi::c_double + 0.5f64) as WORD32;
                }
            } else {
                let mut sfb_0: WORD32 = 0 as WORD32;
                while sfb_0 <= *p_num_sfb.offset(HIGH as isize) as core::ffi::c_int
                    && *(*p_freq_band_tab.offset(HIGH as isize)).offset(sfb_0 as isize)
                        as core::ffi::c_int <= patch * (*ptr_hbe_txposer).start_band
                {
                    sfb_0 += 1;
                }
                (*ptr_hbe_txposer)
                    .x_over_qmf[(patch as core::ffi::c_int - 1 as core::ffi::c_int)
                    as usize] = *(*p_freq_band_tab.offset(HIGH as isize))
                    .offset((sfb_0 as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                    as WORD32;
                if patch <= MAX_STRETCH {
                    (*ptr_hbe_txposer)
                        .x_over_bin[(patch as core::ffi::c_int - 1 as core::ffi::c_int)
                        as usize][0 as core::ffi::c_int as usize] = (((*ptr_hbe_txposer)
                        .fft_size[0 as core::ffi::c_int as usize]
                        * *(*p_freq_band_tab.offset(HIGH as isize))
                            .offset(
                                (sfb_0 as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ) as core::ffi::c_int / 128 as core::ffi::c_int)
                        as core::ffi::c_double + 0.5f64) as WORD32;
                    (*ptr_hbe_txposer)
                        .x_over_bin[(patch as core::ffi::c_int - 1 as core::ffi::c_int)
                        as usize][1 as core::ffi::c_int as usize] = (((*ptr_hbe_txposer)
                        .fft_size[1 as core::ffi::c_int as usize]
                        * *(*p_freq_band_tab.offset(HIGH as isize))
                            .offset(
                                (sfb_0 as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ) as core::ffi::c_int / 128 as core::ffi::c_int)
                        as core::ffi::c_double + 0.5f64) as WORD32;
                }
            }
            patch += 1;
        } else {
            (*ptr_hbe_txposer)
                .x_over_qmf[(patch as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize] = (*ptr_hbe_txposer).end_band;
            if patch <= MAX_STRETCH {
                (*ptr_hbe_txposer)
                    .x_over_bin[(patch as core::ffi::c_int - 1 as core::ffi::c_int)
                    as usize][0 as core::ffi::c_int as usize] = (((*ptr_hbe_txposer)
                    .fft_size[0 as core::ffi::c_int as usize]
                    * (*ptr_hbe_txposer).end_band as core::ffi::c_int
                    / 128 as core::ffi::c_int) as core::ffi::c_double + 0.5f64)
                    as WORD32;
                (*ptr_hbe_txposer)
                    .x_over_bin[(patch as core::ffi::c_int - 1 as core::ffi::c_int)
                    as usize][1 as core::ffi::c_int as usize] = (((*ptr_hbe_txposer)
                    .fft_size[1 as core::ffi::c_int as usize]
                    * (*ptr_hbe_txposer).end_band as core::ffi::c_int
                    / 128 as core::ffi::c_int) as core::ffi::c_double + 0.5f64)
                    as WORD32;
            }
            (*ptr_hbe_txposer).max_stretch = (if patch < 4 as core::ffi::c_int {
                patch as core::ffi::c_int
            } else {
                4 as core::ffi::c_int
            }) as WORD32;
            break;
        }
    }
    patch = 0 as core::ffi::c_int as WORD32;
    while patch
        < (*ptr_hbe_txposer).max_stretch as core::ffi::c_int - 1 as core::ffi::c_int
    {
        i = 0 as core::ffi::c_int as WORD32;
        while i < 2 as core::ffi::c_int {
            ixheaacd_create_dft_hbe_window(
                ((*ptr_hbe_txposer).fd_win_buf[patch as usize][i as usize]).as_mut_ptr(),
                (*ptr_hbe_txposer).x_over_bin[patch as usize][i as usize],
                (*ptr_hbe_txposer)
                    .x_over_bin[(patch as core::ffi::c_int + 1 as core::ffi::c_int)
                    as usize][i as usize],
                trans_samp[i as usize],
                (*ptr_hbe_txposer).fft_size[i as usize],
            );
            i += 1;
        }
        patch += 1;
    }
    return 0 as WORD32;
}
unsafe extern "C" fn ixheaacd_dft_hbe_apply_win(
    mut inp1: *const FLOAT32,
    mut inp2: *const FLOAT32,
    mut out: *mut FLOAT32,
    mut n: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < n {
        *out.offset(i as isize) = *inp1.offset(i as isize) * *inp2.offset(i as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dft_hbe_fft_memmove(
    mut ptr_spectrum: *mut FLOAT32,
    mut size: WORD32,
) -> VOID {
    let mut n: WORD32 = 0;
    n = 0 as core::ffi::c_int as WORD32;
    while n < size as core::ffi::c_int / 2 as core::ffi::c_int {
        let mut tmp: FLOAT32 = *ptr_spectrum.offset(n as isize);
        *ptr_spectrum.offset(n as isize) = *ptr_spectrum
            .offset(
                (n as core::ffi::c_int
                    + size as core::ffi::c_int / 2 as core::ffi::c_int) as isize,
            );
        *ptr_spectrum
            .offset(
                (n as core::ffi::c_int
                    + size as core::ffi::c_int / 2 as core::ffi::c_int) as isize,
            ) = tmp;
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_karth2polar(
    mut spectrum: *mut FLOAT32,
    mut mag: *mut FLOAT32,
    mut phase: *mut FLOAT32,
    mut fft_size: WORD32,
) -> VOID {
    let mut n: WORD32 = 0;
    n = 1 as core::ffi::c_int as WORD32;
    while n < fft_size as core::ffi::c_int / 2 as core::ffi::c_int {
        *phase.offset(n as isize) = atan2(
            *spectrum
                .offset(
                    (2 as core::ffi::c_int * n as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) as core::ffi::c_double,
            *spectrum.offset((2 as WORD32 * n) as isize) as core::ffi::c_double,
        ) as FLOAT32;
        *mag.offset(n as isize) = sqrt(
            (*spectrum.offset((2 as WORD32 * n) as isize)
                * *spectrum.offset((2 as WORD32 * n) as isize)
                + *spectrum
                    .offset(
                        (2 as core::ffi::c_int * n as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    )
                    * *spectrum
                        .offset(
                            (2 as core::ffi::c_int * n as core::ffi::c_int
                                + 1 as core::ffi::c_int) as isize,
                        )) as core::ffi::c_double,
        ) as FLOAT32;
        n += 1;
    }
    if *spectrum.offset(0 as core::ffi::c_int as isize)
        < 0 as core::ffi::c_int as FLOAT32
    {
        *phase.offset(0 as core::ffi::c_int as isize) = acos(
            -(1 as core::ffi::c_int) as core::ffi::c_double,
        ) as FLOAT32;
        *mag.offset(0 as core::ffi::c_int as isize) = -*spectrum
            .offset(0 as core::ffi::c_int as isize);
    } else {
        *phase.offset(0 as core::ffi::c_int as isize) = 0 as core::ffi::c_int as FLOAT32;
        *mag.offset(0 as core::ffi::c_int as isize) = *spectrum
            .offset(0 as core::ffi::c_int as isize);
    }
    if *spectrum.offset(1 as core::ffi::c_int as isize)
        < 0 as core::ffi::c_int as FLOAT32
    {
        *phase.offset((fft_size as core::ffi::c_int / 2 as core::ffi::c_int) as isize) = acos(
            -(1 as core::ffi::c_int) as core::ffi::c_double,
        ) as FLOAT32;
        *mag.offset((fft_size as core::ffi::c_int / 2 as core::ffi::c_int) as isize) = -*spectrum
            .offset(1 as core::ffi::c_int as isize);
    } else {
        *phase.offset((fft_size as core::ffi::c_int / 2 as core::ffi::c_int) as isize) = 0
            as core::ffi::c_int as FLOAT32;
        *mag.offset((fft_size as core::ffi::c_int / 2 as core::ffi::c_int) as isize) = *spectrum
            .offset(1 as core::ffi::c_int as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_fft_table(
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
) -> VOID {
    let mut oversampling_flag: WORD32 = (*ptr_hbe_txposer).oversampling_flag;
    let mut ana_fft_size: WORD32 = (*ptr_hbe_txposer)
        .ana_fft_size[oversampling_flag as usize];
    let mut syn_fft_size: WORD32 = (*ptr_hbe_txposer)
        .syn_fft_size[oversampling_flag as usize];
    match ana_fft_size {
        576 => {
            (*ptr_hbe_txposer).ana_cos_sin_tab = ixheaac_sin_cos_576.as_ptr()
                as *mut FLOAT32;
        }
        384 => {
            (*ptr_hbe_txposer).ana_cos_sin_tab = ixheaac_sin_cos_384.as_ptr()
                as *mut FLOAT32;
        }
        512 => {
            (*ptr_hbe_txposer).ana_cos_sin_tab = ixheaac_sin_cos_512.as_ptr()
                as *mut FLOAT32;
        }
        768 => {
            (*ptr_hbe_txposer).ana_cos_sin_tab = ixheaac_sin_cos_768.as_ptr()
                as *mut FLOAT32;
        }
        _ => {}
    }
    match syn_fft_size {
        448 => {
            (*ptr_hbe_txposer).syn_cos_sin_tab = ixheaac_sin_cos_448.as_ptr()
                as *mut FLOAT32;
        }
        512 => {
            (*ptr_hbe_txposer).syn_cos_sin_tab = ixheaac_sin_cos_512.as_ptr()
                as *mut FLOAT32;
        }
        768 => {
            (*ptr_hbe_txposer).syn_cos_sin_tab = ixheaac_sin_cos_768.as_ptr()
                as *mut FLOAT32;
        }
        672 => {
            (*ptr_hbe_txposer).syn_cos_sin_tab = ixheaac_sin_cos_672.as_ptr()
                as *mut FLOAT32;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_hbe_fft_map(
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
) -> IA_ERRORCODE {
    let mut oversampling_flag: WORD32 = (*ptr_hbe_txposer).oversampling_flag;
    let mut ana_fft_size: WORD32 = (*ptr_hbe_txposer)
        .ana_fft_size[oversampling_flag as usize];
    let mut syn_fft_size: WORD32 = (*ptr_hbe_txposer)
        .syn_fft_size[oversampling_flag as usize];
    match ana_fft_size {
        576 => {
            (*ptr_hbe_txposer).ana_cos_sin_tab = ixheaac_sin_cos_576.as_ptr()
                as *mut FLOAT32;
            (*ptr_hbe_txposer).ixheaacd_hbe_anal_fft = Some(
                ixheaacd_hbe_apply_fft_288
                    as unsafe extern "C" fn(
                        *mut FLOAT32,
                        *mut FLOAT32,
                        WORD32,
                        WORD32,
                    ) -> VOID,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut FLOAT32,
                        *mut FLOAT32,
                        WORD32,
                        WORD32,
                    ) -> VOID,
                >;
        }
        384 => {
            (*ptr_hbe_txposer).ana_cos_sin_tab = ixheaac_sin_cos_384.as_ptr()
                as *mut FLOAT32;
            (*ptr_hbe_txposer).ixheaacd_hbe_anal_fft = Some(
                ixheaacd_hbe_apply_cfftn_gen
                    as unsafe extern "C" fn(
                        *mut FLOAT32,
                        *mut FLOAT32,
                        WORD32,
                        WORD32,
                    ) -> VOID,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut FLOAT32,
                        *mut FLOAT32,
                        WORD32,
                        WORD32,
                    ) -> VOID,
                >;
        }
        512 => {
            (*ptr_hbe_txposer).ana_cos_sin_tab = ixheaac_sin_cos_512.as_ptr()
                as *mut FLOAT32;
            (*ptr_hbe_txposer).ixheaacd_hbe_anal_fft = Some(
                ixheaacd_hbe_apply_cfftn
                    as unsafe extern "C" fn(
                        *mut FLOAT32,
                        *mut FLOAT32,
                        WORD32,
                        WORD32,
                    ) -> VOID,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut FLOAT32,
                        *mut FLOAT32,
                        WORD32,
                        WORD32,
                    ) -> VOID,
                >;
        }
        768 => {
            (*ptr_hbe_txposer).ana_cos_sin_tab = ixheaac_sin_cos_768.as_ptr()
                as *mut FLOAT32;
            (*ptr_hbe_txposer).ixheaacd_hbe_anal_fft = Some(
                ixheaacd_hbe_apply_cfftn_gen
                    as unsafe extern "C" fn(
                        *mut FLOAT32,
                        *mut FLOAT32,
                        WORD32,
                        WORD32,
                    ) -> VOID,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut FLOAT32,
                        *mut FLOAT32,
                        WORD32,
                        WORD32,
                    ) -> VOID,
                >;
        }
        _ => return IA_FATAL_ERROR as IA_ERRORCODE,
    }
    match syn_fft_size {
        448 => {
            (*ptr_hbe_txposer).syn_cos_sin_tab = ixheaac_sin_cos_448.as_ptr()
                as *mut FLOAT32;
            (*ptr_hbe_txposer).ixheaacd_hbe_synth_ifft = Some(
                ixheaacd_hbe_apply_ifft_224
                    as unsafe extern "C" fn(
                        *mut FLOAT32,
                        *mut FLOAT32,
                        WORD32,
                        WORD32,
                    ) -> VOID,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut FLOAT32,
                        *mut FLOAT32,
                        WORD32,
                        WORD32,
                    ) -> VOID,
                >;
        }
        512 => {
            (*ptr_hbe_txposer).syn_cos_sin_tab = ixheaac_sin_cos_512.as_ptr()
                as *mut FLOAT32;
            (*ptr_hbe_txposer).ixheaacd_hbe_synth_ifft = Some(
                ixheaacd_hbe_apply_cfftn
                    as unsafe extern "C" fn(
                        *mut FLOAT32,
                        *mut FLOAT32,
                        WORD32,
                        WORD32,
                    ) -> VOID,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut FLOAT32,
                        *mut FLOAT32,
                        WORD32,
                        WORD32,
                    ) -> VOID,
                >;
        }
        768 => {
            (*ptr_hbe_txposer).syn_cos_sin_tab = ixheaac_sin_cos_768.as_ptr()
                as *mut FLOAT32;
            (*ptr_hbe_txposer).ixheaacd_hbe_synth_ifft = Some(
                ixheaacd_hbe_apply_cfftn_gen
                    as unsafe extern "C" fn(
                        *mut FLOAT32,
                        *mut FLOAT32,
                        WORD32,
                        WORD32,
                    ) -> VOID,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut FLOAT32,
                        *mut FLOAT32,
                        WORD32,
                        WORD32,
                    ) -> VOID,
                >;
        }
        672 => {
            (*ptr_hbe_txposer).syn_cos_sin_tab = ixheaac_sin_cos_672.as_ptr()
                as *mut FLOAT32;
            (*ptr_hbe_txposer).ixheaacd_hbe_synth_ifft = Some(
                ixheaacd_hbe_apply_ifft_336
                    as unsafe extern "C" fn(
                        *mut FLOAT32,
                        *mut FLOAT32,
                        WORD32,
                        WORD32,
                    ) -> VOID,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut FLOAT32,
                        *mut FLOAT32,
                        WORD32,
                        WORD32,
                    ) -> VOID,
                >;
        }
        _ => return IA_FATAL_ERROR as IA_ERRORCODE,
    }
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dft_hbe_apply_polar_t2(
    mut trans_fac: WORD32,
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    mut pitch_in_bins: WORD32,
    mut out_transform_size: WORD,
) -> VOID {
    let mut tr: WORD32 = 0;
    let mut ti: WORD32 = 0;
    let mut m_tr: WORD32 = 0;
    let mut p: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut mag_t: FLOAT32 = 0.;
    let mut phase_t: FLOAT32 = 0.;
    let mut m_val: FLOAT32 = 0.;
    let mut fd_win_buf: *mut [[[FLOAT32; 1536]; 3]; 3] = &mut (*ptr_hbe_txposer)
        .fd_win_buf;
    let mut phase: *mut FLOAT32 = (*ptr_hbe_txposer).phase;
    let mut oversampling_flag: WORD32 = (*ptr_hbe_txposer).oversampling_flag;
    let mut fft_size: WORD32 = (*ptr_hbe_txposer).fft_size[oversampling_flag as usize];
    let mut ptr_spectrum_tx: *mut FLOAT32 = (*ptr_hbe_txposer).ptr_spectrum_tx;
    let mut mag: *mut FLOAT32 = (*ptr_hbe_txposer).mag;
    let mut p_flt: FLOAT32 = (fft_size * pitch_in_bins) as FLOAT32 / 1536.0f32;
    p = p_flt as WORD32;
    let mut q_thr: FLOAT32 = 4.0f32;
    m_tr = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i <= out_transform_size {
        let mut utk: WORD32 = i;
        mag_t = (*fd_win_buf)[(trans_fac as core::ffi::c_int - 2 as core::ffi::c_int)
            as usize][oversampling_flag as usize][i as usize]
            * *mag.offset(utk as isize);
        phase_t = trans_fac as FLOAT32 * *phase.offset(utk as isize);
        if phase_t as core::ffi::c_double == 0.0f64 {
            *ptr_spectrum_tx.offset((2 as WORD32 * i) as isize) += mag_t;
        } else {
            *ptr_spectrum_tx.offset((2 as WORD32 * i) as isize)
                += mag_t * cos(phase_t as core::ffi::c_double) as FLOAT32;
            *ptr_spectrum_tx
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) += mag_t * sin(phase_t as core::ffi::c_double) as FLOAT32;
        }
        if p > 0 as core::ffi::c_int {
            m_val = 0 as core::ffi::c_int as FLOAT32;
            tr = 1 as core::ffi::c_int as WORD32;
            while tr < trans_fac {
                let mut temp: FLOAT32 = 0.;
                ti = ((2.0f32 * i as core::ffi::c_float
                    - tr as core::ffi::c_float * p_flt as core::ffi::c_float)
                    / trans_fac as core::ffi::c_float + 0.5f32) as WORD32;
                if !(ti < 0 as core::ffi::c_int
                    || ti + p > fft_size as core::ffi::c_int / 2 as core::ffi::c_int)
                {
                    temp = if *mag.offset(ti as isize) < *mag.offset((ti + p) as isize) {
                        *mag.offset(ti as isize)
                    } else {
                        *mag.offset((ti + p) as isize)
                    };
                    if temp > m_val {
                        m_val = temp;
                        m_tr = tr;
                        utk = ti;
                    }
                }
                tr += 1;
            }
            if m_val > q_thr * *mag.offset((2 as WORD32 * i / trans_fac) as isize) {
                mag_t = ((*fd_win_buf)[(trans_fac as core::ffi::c_int
                    - 2 as core::ffi::c_int)
                    as usize][oversampling_flag as usize][i as usize]
                    as core::ffi::c_double
                    * sqrt(*mag.offset(utk as isize) as core::ffi::c_double)
                    * sqrt(*mag.offset((utk + p) as isize) as core::ffi::c_double))
                    as FLOAT32;
                phase_t = (trans_fac - m_tr) as FLOAT32 * *phase.offset(utk as isize)
                    + m_tr as FLOAT32 * *phase.offset((utk + p) as isize);
                *ptr_spectrum_tx.offset((2 as WORD32 * i) as isize)
                    += (mag_t as core::ffi::c_double
                        * cos(phase_t as core::ffi::c_double)) as FLOAT32;
                *ptr_spectrum_tx
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    )
                    += (mag_t as core::ffi::c_double
                        * sin(phase_t as core::ffi::c_double)) as FLOAT32;
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dft_hbe_apply_polar_t3(
    mut trans_fac: WORD32,
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    mut pitch_in_bins: WORD32,
    mut out_transform_size: WORD,
) -> VOID {
    let mut tr: WORD32 = 0;
    let mut ti: WORD32 = 0;
    let mut m_tr: WORD32 = 0 as WORD32;
    let mut p: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut mag_t: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
    let mut phase_t: FLOAT32 = 0.;
    let mut m_val: FLOAT32 = 0.;
    let mut fd_win_buf: *mut [[[FLOAT32; 1536]; 3]; 3] = &mut (*ptr_hbe_txposer)
        .fd_win_buf;
    let mut phase: *mut FLOAT32 = (*ptr_hbe_txposer).phase;
    let mut oversampling_flag: WORD32 = (*ptr_hbe_txposer).oversampling_flag;
    let mut fft_size: WORD32 = (*ptr_hbe_txposer).fft_size[oversampling_flag as usize];
    let mut ptr_spectrum_tx: *mut FLOAT32 = (*ptr_hbe_txposer).ptr_spectrum_tx;
    let mut mag: *mut FLOAT32 = (*ptr_hbe_txposer).mag;
    let mut p_flt: FLOAT32 = (fft_size * pitch_in_bins) as FLOAT32 / 1536.0f32;
    p = p_flt as WORD32;
    let mut q_thr: FLOAT32 = 4.0f32;
    i = 0 as core::ffi::c_int as WORD32;
    while i <= out_transform_size {
        let mut utk: WORD32 = 2 as WORD32 * i / trans_fac;
        let mut ptk: FLOAT32 = 2.0f32 * i as FLOAT32 / trans_fac as FLOAT32
            - utk as FLOAT32;
        let mut k: FLOAT32 = 0.;
        if i as core::ffi::c_int % 3 as core::ffi::c_int == 0 as core::ffi::c_int {
            mag_t = (*fd_win_buf)[(trans_fac as core::ffi::c_int - 2 as core::ffi::c_int)
                as usize][oversampling_flag as usize][i as usize]
                * *mag.offset(utk as isize);
        } else if i as core::ffi::c_int % 3 as core::ffi::c_int == 1 as core::ffi::c_int
        {
            k = cbrt(*mag.offset(utk as isize) as core::ffi::c_double) as FLOAT32;
            mag_t = (*fd_win_buf)[(trans_fac as core::ffi::c_int - 2 as core::ffi::c_int)
                as usize][oversampling_flag as usize][i as usize] * k
                * pow(
                    *mag
                        .offset(
                            (utk as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_double,
                    ptk as core::ffi::c_double,
                ) as FLOAT32;
        } else if i as core::ffi::c_int % 3 as core::ffi::c_int == 2 as core::ffi::c_int
        {
            k = cbrt(
                *mag.offset((utk as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_double,
            ) as FLOAT32;
            mag_t = (*fd_win_buf)[(trans_fac as core::ffi::c_int - 2 as core::ffi::c_int)
                as usize][oversampling_flag as usize][i as usize]
                * pow(
                    *mag.offset(utk as isize) as core::ffi::c_double,
                    1.0f64 - ptk as core::ffi::c_double,
                ) as FLOAT32 * k;
        }
        phase_t = trans_fac as FLOAT32
            * ((1 as core::ffi::c_int as FLOAT32 - ptk) * *phase.offset(utk as isize)
                + ptk
                    * *phase
                        .offset(
                            (utk as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                        ));
        *ptr_spectrum_tx.offset((2 as WORD32 * i) as isize)
            += mag_t * cos(phase_t as core::ffi::c_double) as FLOAT32;
        *ptr_spectrum_tx
            .offset(
                (2 as core::ffi::c_int * i as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) += mag_t * sin(phase_t as core::ffi::c_double) as FLOAT32;
        if p > 0 as core::ffi::c_int {
            m_val = 0 as core::ffi::c_int as FLOAT32;
            tr = 1 as core::ffi::c_int as WORD32;
            while tr < trans_fac {
                let mut temp: FLOAT32 = 0.;
                ti = ((2.0f32 * i as core::ffi::c_float
                    - tr as core::ffi::c_float * p_flt as core::ffi::c_float)
                    / trans_fac as core::ffi::c_float + 0.5f32) as WORD32;
                if !(ti < 0 as core::ffi::c_int
                    || ti + p > fft_size as core::ffi::c_int / 2 as core::ffi::c_int)
                {
                    temp = if *mag.offset(ti as isize) < *mag.offset((ti + p) as isize) {
                        *mag.offset(ti as isize)
                    } else {
                        *mag.offset((ti + p) as isize)
                    };
                    if temp > m_val {
                        m_val = temp;
                        m_tr = tr;
                        utk = ti;
                    }
                }
                tr += 1;
            }
            if m_val > q_thr * *mag.offset((2 as WORD32 * i / trans_fac) as isize) {
                let mut r: FLOAT32 = m_tr as FLOAT32 / trans_fac as FLOAT32;
                if m_tr == 1 as core::ffi::c_int {
                    k = cbrt(*mag.offset((utk + p) as isize) as core::ffi::c_double)
                        as FLOAT32;
                    mag_t = (*fd_win_buf)[(trans_fac as core::ffi::c_int
                        - 2 as core::ffi::c_int)
                        as usize][oversampling_flag as usize][i as usize]
                        * pow(
                            *mag.offset(utk as isize) as core::ffi::c_double,
                            1.0f64 - r as core::ffi::c_double,
                        ) as FLOAT32 * k;
                    phase_t = (trans_fac - m_tr) as FLOAT32 * *phase.offset(utk as isize)
                        + *phase.offset((utk + p) as isize);
                } else if m_tr == 2 as core::ffi::c_int {
                    k = cbrt(*mag.offset(utk as isize) as core::ffi::c_double)
                        as FLOAT32;
                    mag_t = (*fd_win_buf)[(trans_fac as core::ffi::c_int
                        - 2 as core::ffi::c_int)
                        as usize][oversampling_flag as usize][i as usize] * k
                        * pow(
                            *mag.offset((utk + p) as isize) as core::ffi::c_double,
                            r as core::ffi::c_double,
                        ) as FLOAT32;
                    phase_t = *phase.offset(utk as isize)
                        + m_tr as FLOAT32 * *phase.offset((utk + p) as isize);
                }
                *ptr_spectrum_tx.offset((2 as WORD32 * i) as isize)
                    += mag_t * cos(phase_t as core::ffi::c_double) as FLOAT32;
                *ptr_spectrum_tx
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    ) += mag_t * sin(phase_t as core::ffi::c_double) as FLOAT32;
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dft_hbe_apply_polar_t(
    mut trans_fac: WORD32,
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    mut pitch_in_bins: WORD32,
    mut out_transform_size: WORD,
) -> VOID {
    let mut tr: WORD32 = 0;
    let mut ti: WORD32 = 0;
    let mut m_tr: WORD32 = 0;
    let mut p: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut mag_t: FLOAT32 = 0.;
    let mut phase_t: FLOAT32 = 0.;
    let mut m_val: FLOAT32 = 0.;
    let mut fd_win_buf: *mut [[[FLOAT32; 1536]; 3]; 3] = &mut (*ptr_hbe_txposer)
        .fd_win_buf;
    let mut phase: *mut FLOAT32 = (*ptr_hbe_txposer).phase;
    let mut oversampling_flag: WORD32 = (*ptr_hbe_txposer).oversampling_flag;
    let mut fft_size: WORD32 = (*ptr_hbe_txposer).fft_size[oversampling_flag as usize];
    let mut ptr_spectrum_tx: *mut FLOAT32 = (*ptr_hbe_txposer).ptr_spectrum_tx;
    let mut mag: *mut FLOAT32 = (*ptr_hbe_txposer).mag;
    let mut p_flt: FLOAT32 = (fft_size * pitch_in_bins) as FLOAT32 / 1536.0f32;
    p = p_flt as WORD32;
    let mut q_thr: FLOAT32 = 4.0f32;
    m_tr = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i <= out_transform_size {
        let mut utk: WORD32 = 2 as WORD32 * i / trans_fac;
        let mut ptk: FLOAT32 = 2.0f32 * i as FLOAT32 / trans_fac as FLOAT32
            - utk as FLOAT32;
        mag_t = (*fd_win_buf)[(trans_fac as core::ffi::c_int - 2 as core::ffi::c_int)
            as usize][oversampling_flag as usize][i as usize]
            * pow(
                *mag.offset(utk as isize) as core::ffi::c_double,
                (1.0f32 - ptk) as core::ffi::c_double,
            ) as FLOAT32
            * pow(
                *mag.offset((utk as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_double,
                ptk as core::ffi::c_double,
            ) as FLOAT32;
        phase_t = trans_fac as FLOAT32
            * ((1 as core::ffi::c_int as FLOAT32 - ptk) * *phase.offset(utk as isize)
                + ptk
                    * *phase
                        .offset(
                            (utk as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                        ));
        *ptr_spectrum_tx.offset((2 as WORD32 * i) as isize)
            += mag_t * cos(phase_t as core::ffi::c_double) as FLOAT32;
        *ptr_spectrum_tx
            .offset(
                (2 as core::ffi::c_int * i as core::ffi::c_int + 1 as core::ffi::c_int)
                    as isize,
            ) += mag_t * sin(phase_t as core::ffi::c_double) as FLOAT32;
        if p > 0 as core::ffi::c_int {
            m_val = 0 as core::ffi::c_int as FLOAT32;
            tr = 1 as core::ffi::c_int as WORD32;
            while tr < trans_fac {
                let mut temp: FLOAT32 = 0.;
                ti = ((2.0f32 * i as core::ffi::c_float
                    - tr as core::ffi::c_float * p_flt as core::ffi::c_float)
                    / trans_fac as core::ffi::c_float + 0.5f32) as WORD32;
                if !(ti < 0 as core::ffi::c_int
                    || ti + p > fft_size as core::ffi::c_int / 2 as core::ffi::c_int)
                {
                    temp = if *mag.offset(ti as isize) < *mag.offset((ti + p) as isize) {
                        *mag.offset(ti as isize)
                    } else {
                        *mag.offset((ti + p) as isize)
                    };
                    if temp > m_val {
                        m_val = temp;
                        m_tr = tr;
                        utk = ti;
                    }
                }
                tr += 1;
            }
            if m_val > q_thr * *mag.offset((2 as WORD32 * i / trans_fac) as isize) {
                let mut r: FLOAT32 = m_tr as FLOAT32 / trans_fac as FLOAT32;
                mag_t = (*fd_win_buf)[(trans_fac as core::ffi::c_int
                    - 2 as core::ffi::c_int)
                    as usize][oversampling_flag as usize][i as usize]
                    * pow(
                        *mag.offset(utk as isize) as core::ffi::c_double,
                        1.0f64 - r as core::ffi::c_double,
                    ) as FLOAT32
                    * pow(
                        *mag.offset((utk + p) as isize) as core::ffi::c_double,
                        r as core::ffi::c_double,
                    ) as FLOAT32;
                phase_t = (trans_fac - m_tr) as FLOAT32 * *phase.offset(utk as isize)
                    + m_tr as FLOAT32 * *phase.offset((utk + p) as isize);
                *ptr_spectrum_tx.offset((2 as WORD32 * i) as isize)
                    += mag_t * cos(phase_t as core::ffi::c_double) as FLOAT32;
                *ptr_spectrum_tx
                    .offset(
                        (2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    ) += mag_t * sin(phase_t as core::ffi::c_double) as FLOAT32;
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dft_hbe_apply(
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    mut qmf_buf_real: *mut [FLOAT32; 64],
    mut qmf_buf_imag: *mut [FLOAT32; 64],
    mut num_columns: WORD32,
    mut pv_qmf_buf_real: *mut [FLOAT32; 64],
    mut pv_qmf_buf_imag: *mut [FLOAT32; 64],
    mut pitch_in_bins: WORD32,
    mut dft_hbe_scratch_buf: *mut FLOAT32,
) -> WORD32 {
    let mut in_offset: WORD32 = 0 as WORD32;
    let mut out_offset: WORD32 = 0 as WORD32;
    let mut in_hop_size: WORD32 = (*ptr_hbe_txposer).in_hop_size;
    let mut oversampling_flag: WORD32 = (*ptr_hbe_txposer).oversampling_flag;
    let mut fft_size: WORD32 = (*ptr_hbe_txposer).fft_size[oversampling_flag as usize];
    let mut out_hop_size: WORD32 = (*ptr_hbe_txposer).out_hop_size;
    let mut num_in_samples: WORD32 = num_columns * (*ptr_hbe_txposer).synth_size;
    let mut ana_fft_size: WORD32 = (*ptr_hbe_txposer)
        .ana_fft_size[oversampling_flag as usize];
    let mut syn_fft_size: WORD32 = (*ptr_hbe_txposer)
        .syn_fft_size[oversampling_flag as usize];
    let mut ana_pad_size: WORD32 = (ana_fft_size
        - (*ptr_hbe_txposer).ana_fft_size[0 as core::ffi::c_int as usize]) / 2 as WORD32;
    let mut syn_pad_size: WORD32 = (syn_fft_size
        - (*ptr_hbe_txposer).syn_fft_size[0 as core::ffi::c_int as usize]) / 2 as WORD32;
    let mut ptr_input_buf: *mut FLOAT32 = (*ptr_hbe_txposer).ptr_input_buf;
    let mut ptr_output_buf: *mut FLOAT32 = (*ptr_hbe_txposer).ptr_output_buf;
    let mut ptr_spectrum: *mut FLOAT32 = (*ptr_hbe_txposer).ptr_spectrum;
    let mut ptr_spectrum_tx: *mut FLOAT32 = (*ptr_hbe_txposer).ptr_spectrum_tx;
    let mut mag: *mut FLOAT32 = (*ptr_hbe_txposer).mag;
    let mut phase: *mut FLOAT32 = (*ptr_hbe_txposer).phase;
    let mut i: WORD32 = 0;
    let mut trans_fac: WORD32 = 0;
    let mut ptr_cos_fft: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut ptr_cos_ifft: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut ana_fft_offset: WORD32 = (*ptr_hbe_txposer).k_start * fft_size
        / 32 as WORD32;
    let mut syn_fft_offset: WORD32 = (*ptr_hbe_txposer).a_start * fft_size
        / 64 as WORD32;
    let mut err_code: WORD32 = IA_NO_ERROR;
    memcpy(
        (*ptr_hbe_txposer).ptr_input_buf as *mut core::ffi::c_void,
        ((*ptr_hbe_txposer).ptr_input_buf)
            .offset(
                (*ptr_hbe_txposer).ana_fft_size[0 as core::ffi::c_int as usize] as isize,
            ) as *const core::ffi::c_void,
        ((*ptr_hbe_txposer).ana_fft_size[0 as core::ffi::c_int as usize] as size_t)
            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
    ixheaacd_real_synth_filt(ptr_hbe_txposer, num_columns, qmf_buf_real, qmf_buf_imag);
    memcpy(
        ptr_output_buf as *mut core::ffi::c_void,
        ptr_output_buf
            .offset(
                (2 as WORD32
                    * (*ptr_hbe_txposer).syn_fft_size[0 as core::ffi::c_int as usize])
                    as isize,
            ) as *const core::ffi::c_void,
        ((2 as WORD32 * (*ptr_hbe_txposer).syn_fft_size[0 as core::ffi::c_int as usize])
            as size_t)
            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
    memset(
        ptr_output_buf
            .offset(
                (2 as WORD32
                    * (*ptr_hbe_txposer).syn_fft_size[0 as core::ffi::c_int as usize])
                    as isize,
            ) as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ((2 as WORD32 * (*ptr_hbe_txposer).syn_fft_size[0 as core::ffi::c_int as usize])
            as size_t)
            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
    err_code = ixheaacd_hbe_fft_map(ptr_hbe_txposer) as WORD32;
    if err_code != 0 {
        return err_code;
    }
    while in_offset < num_in_samples {
        memset(
            ptr_spectrum as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (fft_size as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        memset(
            ptr_spectrum_tx as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ((fft_size as core::ffi::c_int + 2 as core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        memset(
            mag as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ((fft_size as core::ffi::c_int / 2 as core::ffi::c_int
                + 2 as core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        memset(
            phase as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ((fft_size as core::ffi::c_int / 2 as core::ffi::c_int
                + 2 as core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        ixheaacd_dft_hbe_apply_win(
            ptr_input_buf.offset(in_offset as isize),
            (*ptr_hbe_txposer).anal_window,
            ptr_spectrum.offset(ana_pad_size as isize).offset(ana_fft_offset as isize),
            (*ptr_hbe_txposer).ana_fft_size[0 as core::ffi::c_int as usize],
        );
        ixheaacd_dft_hbe_fft_memmove(
            ptr_spectrum.offset(ana_fft_offset as isize),
            ana_fft_size,
        );
        let mut len: WORD32 = ana_fft_size;
        ptr_cos_fft = (*ptr_hbe_txposer).ana_cos_sin_tab;
        let mut ptr_fft_data: *mut FLOAT32 = ptr_spectrum
            .offset(ana_fft_offset as isize);
        let mut tmp1: FLOAT32 = 0.;
        let mut tmp2: FLOAT32 = 0.;
        let mut tmp3: FLOAT32 = 0.;
        let mut tmp4: FLOAT32 = 0.;
        (Some(
            ((*ptr_hbe_txposer).ixheaacd_hbe_anal_fft)
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(ptr_fft_data, dft_hbe_scratch_buf, len / 2 as WORD32, -(1 as WORD32));
        tmp1 = *ptr_fft_data.offset(0 as core::ffi::c_int as isize)
            + *ptr_fft_data.offset(1 as core::ffi::c_int as isize);
        *ptr_fft_data.offset(1 as core::ffi::c_int as isize) = *ptr_fft_data
            .offset(0 as core::ffi::c_int as isize)
            - *ptr_fft_data.offset(1 as core::ffi::c_int as isize);
        *ptr_fft_data.offset(0 as core::ffi::c_int as isize) = tmp1;
        i = 1 as core::ffi::c_int as WORD32;
        while i
            <= len as core::ffi::c_int / 4 as core::ffi::c_int
                + len as core::ffi::c_int % 4 as core::ffi::c_int / 2 as core::ffi::c_int
        {
            tmp1 = *ptr_fft_data.offset((2 as WORD32 * i) as isize)
                - *ptr_fft_data.offset((len - 2 as WORD32 * i) as isize);
            tmp2 = *ptr_fft_data
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                )
                + *ptr_fft_data
                    .offset(
                        (len as core::ffi::c_int
                            - 2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    );
            tmp3 = *ptr_cos_fft * tmp1
                - *ptr_cos_fft.offset(1 as core::ffi::c_int as isize) * tmp2;
            tmp4 = *ptr_cos_fft.offset(1 as core::ffi::c_int as isize) * tmp1
                + *ptr_cos_fft * tmp2;
            ptr_cos_fft = ptr_cos_fft.offset(2 as core::ffi::c_int as isize);
            tmp1 = *ptr_fft_data.offset((2 as WORD32 * i) as isize)
                + *ptr_fft_data.offset((len - 2 as WORD32 * i) as isize);
            tmp2 = *ptr_fft_data
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                )
                - *ptr_fft_data
                    .offset(
                        (len as core::ffi::c_int
                            - 2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    );
            *ptr_fft_data
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) = 0.5f32 * (tmp1 - tmp3);
            *ptr_fft_data
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = 0.5f32 * (tmp2 - tmp4);
            *ptr_fft_data
                .offset(
                    (len as core::ffi::c_int
                        - 2 as core::ffi::c_int * i as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) = 0.5f32 * (tmp1 + tmp3);
            *ptr_fft_data
                .offset(
                    (len as core::ffi::c_int
                        - 2 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = -0.5f32 * (tmp2 + tmp4);
            i += 1;
        }
        ixheaacd_karth2polar(
            ptr_spectrum.offset(ana_fft_offset as isize),
            mag
                .offset(
                    (ana_fft_offset as core::ffi::c_int / 2 as core::ffi::c_int) as isize,
                ),
            phase
                .offset(
                    (ana_fft_offset as core::ffi::c_int / 2 as core::ffi::c_int) as isize,
                ),
            ana_fft_size,
        );
        trans_fac = 2 as core::ffi::c_int as WORD32;
        while trans_fac <= (*ptr_hbe_txposer).max_stretch {
            let mut out_transform_size: WORD32 = 0;
            out_transform_size = (fft_size as core::ffi::c_int / 2 as core::ffi::c_int)
                as WORD32;
            match trans_fac {
                2 => {
                    ixheaacd_dft_hbe_apply_polar_t2(
                        trans_fac,
                        ptr_hbe_txposer,
                        pitch_in_bins,
                        out_transform_size as WORD,
                    );
                }
                3 => {
                    ixheaacd_dft_hbe_apply_polar_t3(
                        trans_fac,
                        ptr_hbe_txposer,
                        pitch_in_bins,
                        out_transform_size as WORD,
                    );
                }
                _ => {
                    ixheaacd_dft_hbe_apply_polar_t(
                        trans_fac,
                        ptr_hbe_txposer,
                        pitch_in_bins,
                        out_transform_size as WORD,
                    );
                }
            }
            trans_fac += 1;
        }
        *ptr_spectrum_tx
            .offset(
                (syn_fft_offset as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
            ) = *ptr_spectrum_tx.offset((syn_fft_offset + syn_fft_size) as isize);
        let mut len_0: WORD32 = syn_fft_size;
        ptr_cos_ifft = (*ptr_hbe_txposer).syn_cos_sin_tab;
        let mut ptr_fft_data_0: *mut FLOAT32 = ptr_spectrum_tx
            .offset(syn_fft_offset as isize);
        let mut tmp1_0: FLOAT32 = 0.;
        let mut tmp2_0: FLOAT32 = 0.;
        let mut tmp3_0: FLOAT32 = 0.;
        let mut tmp4_0: FLOAT32 = 0.;
        let mut scale: FLOAT32 = 1.0f32 / len_0 as FLOAT32;
        tmp1_0 = *ptr_fft_data_0.offset(0 as core::ffi::c_int as isize)
            + *ptr_fft_data_0.offset(1 as core::ffi::c_int as isize);
        *ptr_fft_data_0.offset(1 as core::ffi::c_int as isize) = scale
            * (*ptr_fft_data_0.offset(0 as core::ffi::c_int as isize)
                - *ptr_fft_data_0.offset(1 as core::ffi::c_int as isize));
        *ptr_fft_data_0.offset(0 as core::ffi::c_int as isize) = scale * tmp1_0;
        i = 1 as core::ffi::c_int as WORD32;
        while i
            <= len_0 as core::ffi::c_int / 4 as core::ffi::c_int
                + len_0 as core::ffi::c_int % 4 as core::ffi::c_int
                    / 2 as core::ffi::c_int
        {
            tmp1_0 = *ptr_fft_data_0.offset((2 as WORD32 * i) as isize)
                - *ptr_fft_data_0.offset((len_0 - 2 as WORD32 * i) as isize);
            tmp2_0 = *ptr_fft_data_0
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                )
                + *ptr_fft_data_0
                    .offset(
                        (len_0 as core::ffi::c_int
                            - 2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    );
            tmp3_0 = *ptr_cos_ifft * tmp1_0
                + *ptr_cos_ifft.offset(1 as core::ffi::c_int as isize) * tmp2_0;
            tmp4_0 = -*ptr_cos_ifft.offset(1 as core::ffi::c_int as isize) * tmp1_0
                + *ptr_cos_ifft * tmp2_0;
            ptr_cos_ifft = ptr_cos_ifft.offset(2 as core::ffi::c_int as isize);
            tmp1_0 = *ptr_fft_data_0.offset((2 as WORD32 * i) as isize)
                + *ptr_fft_data_0.offset((len_0 - 2 as WORD32 * i) as isize);
            tmp2_0 = *ptr_fft_data_0
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                )
                - *ptr_fft_data_0
                    .offset(
                        (len_0 as core::ffi::c_int
                            - 2 as core::ffi::c_int * i as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    );
            *ptr_fft_data_0.offset((2 as WORD32 * i) as isize) = scale
                * (tmp1_0 - tmp3_0);
            *ptr_fft_data_0
                .offset(
                    (2 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = scale * (tmp2_0 - tmp4_0);
            *ptr_fft_data_0.offset((len_0 - 2 as WORD32 * i) as isize) = scale
                * (tmp1_0 + tmp3_0);
            *ptr_fft_data_0
                .offset(
                    (len_0 as core::ffi::c_int
                        - 2 as core::ffi::c_int * i as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ) = -scale * (tmp2_0 + tmp4_0);
            i += 1;
        }
        (Some(
            ((*ptr_hbe_txposer).ixheaacd_hbe_synth_ifft)
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(ptr_fft_data_0, dft_hbe_scratch_buf, len_0 / 2 as WORD32, 1 as WORD32);
        ixheaacd_dft_hbe_fft_memmove(
            ptr_spectrum_tx.offset(syn_fft_offset as isize),
            syn_fft_size,
        );
        ixheaacd_dft_hbe_apply_win(
            ptr_spectrum_tx
                .offset(syn_pad_size as isize)
                .offset(syn_fft_offset as isize),
            (*ptr_hbe_txposer).synth_window,
            ptr_spectrum_tx
                .offset(syn_pad_size as isize)
                .offset(syn_fft_offset as isize),
            (*ptr_hbe_txposer).syn_fft_size[0 as core::ffi::c_int as usize],
        );
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*ptr_hbe_txposer).syn_fft_size[0 as core::ffi::c_int as usize] {
            *ptr_output_buf.offset((out_offset + i) as isize)
                += *ptr_spectrum_tx.offset((syn_pad_size + syn_fft_offset + i) as isize);
            i += 1;
        }
        in_offset += in_hop_size;
        out_offset += out_hop_size;
    }
    err_code = ixheaacd_dft_hbe_cplx_anal_filt(
        ptr_hbe_txposer,
        pv_qmf_buf_real,
        pv_qmf_buf_imag,
    );
    return err_code;
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
