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
    static ixheaac_cos_table_trans_qmf: [[FLOAT32; 64]; 7];
}
pub type size_t = usize;
pub type WORD32 = core::ffi::c_int;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
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
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const HBE_OPER_WIN_LEN: core::ffi::c_int = 13 as core::ffi::c_int;
pub const NO_QMF_SYNTH_CHANNELS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const TWICE_QMF_SYNTH_CHANNELS_NUM: core::ffi::c_int = 128 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_complex_anal_filt(
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
) -> WORD32 {
    let mut idx: WORD32 = 0;
    let mut anal_size: WORD32 = 2 as WORD32 * (*ptr_hbe_txposer).synth_size;
    let mut N: WORD32 = 10 as WORD32 * anal_size;
    let mut no_bins: WORD32 = (*ptr_hbe_txposer).no_bins >> 1 as core::ffi::c_int;
    if (*ptr_hbe_txposer).esbr_hq != 0 as core::ffi::c_int {
        anal_size = 2 as WORD32 * (*ptr_hbe_txposer).analy_size;
        no_bins = (*ptr_hbe_txposer).no_bins;
    }
    idx = 0 as core::ffi::c_int as WORD32;
    while idx < no_bins {
        let mut i: WORD32 = 0;
        let mut j: WORD32 = 0;
        let mut k: WORD32 = 0;
        let mut l: WORD32 = 0;
        let mut window_output: [FLOAT32; 640] = [0.; 640];
        let mut u: [FLOAT32; 128] = [0.; 128];
        let mut u_in: [FLOAT32; 256] = [0.; 256];
        let mut u_out: [FLOAT32; 256] = [0.; 256];
        let mut accu_r: FLOAT32 = 0.;
        let mut accu_i: FLOAT32 = 0.;
        let mut inp_signal: *const FLOAT32 = 0 as *const FLOAT32;
        let mut anal_buf: *mut FLOAT32 = 0 as *mut FLOAT32;
        let mut analy_cos_sin_tab: *mut FLOAT32 = (*ptr_hbe_txposer).analy_cos_sin_tab;
        let mut interp_window_coeff: *const FLOAT32 = (*ptr_hbe_txposer)
            .analy_wind_coeff;
        let mut x: *mut FLOAT32 = ((*ptr_hbe_txposer).analy_buf).as_mut_ptr();
        if (*ptr_hbe_txposer).esbr_hq != 0 as core::ffi::c_int {
            memset(
                *((*ptr_hbe_txposer).qmf_in_buf).offset(idx as isize)
                    as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (TWICE_QMF_SYNTH_CHANNELS_NUM as size_t)
                    .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
            );
            inp_signal = ((*ptr_hbe_txposer).ptr_output_buf)
                .offset((idx * (*ptr_hbe_txposer).analy_size) as isize)
                .offset(1 as core::ffi::c_int as isize);
            anal_buf = &mut *(*((*ptr_hbe_txposer).qmf_in_buf).offset(idx as isize))
                .offset((4 as WORD32 * (*ptr_hbe_txposer).a_start) as isize)
                as *mut FLOAT32;
        } else {
            memset(
                *((*ptr_hbe_txposer).qmf_in_buf)
                    .offset(
                        (idx as core::ffi::c_int + HBE_OPER_WIN_LEN
                            - 1 as core::ffi::c_int) as isize,
                    ) as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (TWICE_QMF_SYNTH_CHANNELS_NUM as size_t)
                    .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
            );
            inp_signal = ((*ptr_hbe_txposer).ptr_input_buf)
                .offset((idx * 2 as WORD32 * (*ptr_hbe_txposer).synth_size) as isize)
                .offset(1 as core::ffi::c_int as isize);
            anal_buf = &mut *(*((*ptr_hbe_txposer).qmf_in_buf)
                .offset(
                    (idx as core::ffi::c_int + HBE_OPER_WIN_LEN - 1 as core::ffi::c_int)
                        as isize,
                ))
                .offset((4 as WORD32 * (*ptr_hbe_txposer).k_start) as isize)
                as *mut FLOAT32;
        }
        i = (N as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while i >= anal_size {
            *x.offset(i as isize) = *x.offset((i - anal_size) as isize);
            i -= 1;
        }
        i = (anal_size as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while i >= 0 as core::ffi::c_int {
            *x.offset(i as isize) = *inp_signal
                .offset((anal_size - 1 as WORD32 - i) as isize);
            i -= 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < N {
            window_output[i as usize] = *x.offset(i as isize)
                * *interp_window_coeff.offset(i as isize);
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < 2 as WORD32 * anal_size {
            accu_r = 0.0f32;
            j = 0 as core::ffi::c_int as WORD32;
            while j < 5 as core::ffi::c_int {
                accu_r = accu_r
                    + window_output[(i + j * 2 as WORD32 * anal_size) as usize];
                j += 1;
            }
            u[i as usize] = accu_r;
            i += 1;
        }
        if anal_size == 40 as core::ffi::c_int || anal_size == 56 as core::ffi::c_int {
            i = 1 as core::ffi::c_int as WORD32;
            while i < anal_size {
                let mut temp1: FLOAT32 = u[i as usize]
                    + u[(2 as WORD32 * anal_size - i) as usize];
                let mut temp2: FLOAT32 = u[i as usize]
                    - u[(2 as WORD32 * anal_size - i) as usize];
                u[i as usize] = temp1;
                u[(2 as WORD32 * anal_size - i) as usize] = temp2;
                i += 1;
            }
            k = 0 as core::ffi::c_int as WORD32;
            while k < anal_size {
                accu_r = u[anal_size as usize];
                if k as core::ffi::c_int & 1 as core::ffi::c_int != 0 {
                    accu_i = u[0 as core::ffi::c_int as usize];
                } else {
                    accu_i = -u[0 as core::ffi::c_int as usize];
                }
                l = 1 as core::ffi::c_int as WORD32;
                while l < anal_size {
                    accu_r = accu_r
                        + u[(0 as WORD32 + l) as usize]
                            * *analy_cos_sin_tab
                                .offset(
                                    (2 as core::ffi::c_int * l as core::ffi::c_int
                                        + 0 as core::ffi::c_int) as isize,
                                );
                    accu_i = accu_i
                        + u[(2 as WORD32 * anal_size - l) as usize]
                            * *analy_cos_sin_tab
                                .offset(
                                    (2 as core::ffi::c_int * l as core::ffi::c_int
                                        + 1 as core::ffi::c_int) as isize,
                                );
                    l += 1;
                }
                analy_cos_sin_tab = analy_cos_sin_tab
                    .offset((2 as WORD32 * anal_size) as isize);
                let fresh0 = anal_buf;
                anal_buf = anal_buf.offset(1);
                *fresh0 = accu_r;
                let fresh1 = anal_buf;
                anal_buf = anal_buf.offset(1);
                *fresh1 = accu_i;
                k += 1;
            }
        } else {
            let mut ptr_u: *mut FLOAT32 = u_in.as_mut_ptr();
            let mut ptr_v: *mut FLOAT32 = u_out.as_mut_ptr();
            k = 0 as core::ffi::c_int as WORD32;
            while k < anal_size as core::ffi::c_int * 2 as core::ffi::c_int {
                let fresh2 = analy_cos_sin_tab;
                analy_cos_sin_tab = analy_cos_sin_tab.offset(1);
                let fresh3 = ptr_u;
                ptr_u = ptr_u.offset(1);
                *fresh3 = *fresh2 * u[k as usize];
                let fresh4 = analy_cos_sin_tab;
                analy_cos_sin_tab = analy_cos_sin_tab.offset(1);
                let fresh5 = ptr_u;
                ptr_u = ptr_u.offset(1);
                *fresh5 = *fresh4 * u[k as usize];
                k += 1;
            }
            if ((*ptr_hbe_txposer).ixheaacd_cmplx_anal_fft).is_some() {
                (Some(
                    ((*ptr_hbe_txposer).ixheaacd_cmplx_anal_fft)
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(u_in.as_mut_ptr(), u_out.as_mut_ptr(), anal_size * 2 as WORD32);
            } else {
                return -(1 as WORD32)
            }
            k = 0 as core::ffi::c_int as WORD32;
            while k < anal_size as core::ffi::c_int / 2 as core::ffi::c_int {
                let fresh6 = ptr_v;
                ptr_v = ptr_v.offset(1);
                *anal_buf.offset(1 as core::ffi::c_int as isize) = -*fresh6;
                let fresh7 = ptr_v;
                ptr_v = ptr_v.offset(1);
                *anal_buf = *fresh7;
                anal_buf = anal_buf.offset(2 as core::ffi::c_int as isize);
                let fresh8 = ptr_v;
                ptr_v = ptr_v.offset(1);
                *anal_buf.offset(1 as core::ffi::c_int as isize) = *fresh8;
                let fresh9 = ptr_v;
                ptr_v = ptr_v.offset(1);
                *anal_buf = -*fresh9;
                anal_buf = anal_buf.offset(2 as core::ffi::c_int as isize);
                k += 1;
            }
        }
        idx += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_real_synth_filt(
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    mut num_columns: WORD32,
    mut qmf_buf_real: *mut [FLOAT32; 64],
    mut qmf_buf_imag: *mut [FLOAT32; 64],
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut l: WORD32 = 0;
    let mut idx: WORD32 = 0;
    let mut g: [FLOAT32; 640] = [0.; 640];
    let mut w: [FLOAT32; 640] = [0.; 640];
    let mut synth_out: [FLOAT32; 128] = [0.; 128];
    let mut accu_r: FLOAT32 = 0.;
    let mut synth_size: WORD32 = (*ptr_hbe_txposer).synth_size;
    let mut ptr_cos_tab_trans_qmf: *mut FLOAT32 = (&*(*ixheaac_cos_table_trans_qmf
        .as_ptr()
        .offset(0 as core::ffi::c_int as isize))
        .as_ptr()
        .offset(0 as core::ffi::c_int as isize) as *const FLOAT32 as *mut FLOAT32)
        .offset(
            ((*ptr_hbe_txposer).k_start as core::ffi::c_int * 32 as core::ffi::c_int)
                as isize,
        );
    let mut buffer: *mut FLOAT32 = ((*ptr_hbe_txposer).synth_buf).as_mut_ptr();
    let mut ptr_inp_buf: *mut FLOAT32 = ((*ptr_hbe_txposer).ptr_input_buf)
        .offset(
            (*ptr_hbe_txposer).ana_fft_size[0 as core::ffi::c_int as usize] as isize,
        );
    idx = 0 as core::ffi::c_int as WORD32;
    while idx < num_columns {
        let mut loc_qmf_buf: [FLOAT32; 64] = [0.; 64];
        let mut synth_buf_r: *mut FLOAT32 = loc_qmf_buf.as_mut_ptr();
        let mut out_buf: *mut FLOAT32 = 0 as *mut FLOAT32;
        if (*ptr_hbe_txposer).esbr_hq == 1 as core::ffi::c_int {
            out_buf = ptr_inp_buf
                .offset(((idx - 1 as WORD32) * (*ptr_hbe_txposer).synth_size) as isize);
        } else {
            out_buf = ((*ptr_hbe_txposer).ptr_input_buf)
                .offset(((idx + 1 as WORD32) * (*ptr_hbe_txposer).synth_size) as isize);
        }
        let mut synth_cos_tab: *mut FLOAT32 = (*ptr_hbe_txposer).synth_cos_tab;
        let mut interp_window_coeff: *const FLOAT32 = (*ptr_hbe_txposer)
            .synth_wind_coeff;
        if (*ptr_hbe_txposer).k_start < 0 as core::ffi::c_int {
            return -(1 as WORD32);
        }
        k = 0 as core::ffi::c_int as WORD32;
        while k < synth_size {
            let mut ki: WORD32 = (*ptr_hbe_txposer).k_start + k;
            *synth_buf_r.offset(k as isize) = *ptr_cos_tab_trans_qmf
                .offset(
                    (((k as core::ffi::c_int) << 1 as core::ffi::c_int)
                        + 0 as core::ffi::c_int) as isize,
                ) * (*qmf_buf_real.offset(idx as isize))[ki as usize]
                + *ptr_cos_tab_trans_qmf
                    .offset(
                        (((k as core::ffi::c_int) << 1 as core::ffi::c_int)
                            + 1 as core::ffi::c_int) as isize,
                    ) * (*qmf_buf_imag.offset(idx as isize))[ki as usize];
            *synth_buf_r.offset((k + (*ptr_hbe_txposer).synth_size) as isize) = 0
                as core::ffi::c_int as FLOAT32;
            k += 1;
        }
        l = (20 as core::ffi::c_int * synth_size as core::ffi::c_int
            - 1 as core::ffi::c_int) as WORD32;
        while l >= 2 as WORD32 * synth_size {
            *buffer.offset(l as isize) = *buffer
                .offset((l - 2 as WORD32 * synth_size) as isize);
            l -= 1;
        }
        if synth_size == 20 as core::ffi::c_int {
            let mut psynth_cos_tab: *mut FLOAT32 = synth_cos_tab;
            l = 0 as core::ffi::c_int as WORD32;
            while l < synth_size as core::ffi::c_int + 1 as core::ffi::c_int {
                accu_r = 0.0f32;
                k = 0 as core::ffi::c_int as WORD32;
                while k < synth_size {
                    accu_r
                        += *synth_buf_r.offset(k as isize)
                            * *psynth_cos_tab.offset(k as isize);
                    k += 1;
                }
                *buffer.offset((0 as WORD32 + l) as isize) = accu_r;
                *buffer.offset((synth_size - l) as isize) = accu_r;
                psynth_cos_tab = psynth_cos_tab.offset(synth_size as isize);
                l += 1;
            }
            l = (synth_size as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
            while l
                < 2 as core::ffi::c_int * synth_size as core::ffi::c_int
                    - synth_size as core::ffi::c_int / 2 as core::ffi::c_int
            {
                accu_r = 0.0f32;
                k = 0 as core::ffi::c_int as WORD32;
                while k < synth_size {
                    accu_r
                        += *synth_buf_r.offset(k as isize)
                            * *psynth_cos_tab.offset(k as isize);
                    k += 1;
                }
                *buffer.offset((0 as WORD32 + l) as isize) = accu_r;
                *buffer.offset((3 as WORD32 * synth_size - l) as isize) = -accu_r;
                psynth_cos_tab = psynth_cos_tab.offset(synth_size as isize);
                l += 1;
            }
            accu_r = 0.0f32;
            k = 0 as core::ffi::c_int as WORD32;
            while k < synth_size {
                accu_r
                    += *synth_buf_r.offset(k as isize)
                        * *psynth_cos_tab.offset(k as isize);
                k += 1;
            }
            *buffer
                .offset((3 as WORD32 * synth_size >> 1 as core::ffi::c_int) as isize) = accu_r;
        } else {
            let mut tmp: FLOAT32 = 0.;
            let mut ptr_u: *mut FLOAT32 = synth_out.as_mut_ptr();
            let mut kmax: WORD32 = synth_size >> 1 as core::ffi::c_int;
            let mut syn_buf: *mut FLOAT32 = &mut *buffer.offset(kmax as isize)
                as *mut FLOAT32;
            kmax += synth_size;
            if ((*ptr_hbe_txposer).ixheaacd_real_synth_fft).is_some() {
                (Some(
                    ((*ptr_hbe_txposer).ixheaacd_real_synth_fft)
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(synth_buf_r, synth_out.as_mut_ptr(), synth_size * 2 as WORD32);
            } else {
                return -(1 as WORD32)
            }
            k = 0 as core::ffi::c_int as WORD32;
            while k < kmax {
                let fresh10 = ptr_u;
                ptr_u = ptr_u.offset(1);
                let fresh11 = synth_cos_tab;
                synth_cos_tab = synth_cos_tab.offset(1);
                tmp = *fresh10 * *fresh11;
                let fresh12 = ptr_u;
                ptr_u = ptr_u.offset(1);
                let fresh13 = synth_cos_tab;
                synth_cos_tab = synth_cos_tab.offset(1);
                tmp -= *fresh12 * *fresh13;
                let fresh14 = syn_buf;
                syn_buf = syn_buf.offset(1);
                *fresh14 = tmp;
                k += 1;
            }
            syn_buf = &mut *buffer.offset(0 as core::ffi::c_int as isize)
                as *mut FLOAT32;
            kmax -= synth_size;
            k = 0 as core::ffi::c_int as WORD32;
            while k < kmax {
                let fresh15 = ptr_u;
                ptr_u = ptr_u.offset(1);
                let fresh16 = synth_cos_tab;
                synth_cos_tab = synth_cos_tab.offset(1);
                tmp = *fresh15 * *fresh16;
                let fresh17 = ptr_u;
                ptr_u = ptr_u.offset(1);
                let fresh18 = synth_cos_tab;
                synth_cos_tab = synth_cos_tab.offset(1);
                tmp -= *fresh17 * *fresh18;
                let fresh19 = syn_buf;
                syn_buf = syn_buf.offset(1);
                *fresh19 = tmp;
                k += 1;
            }
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < 5 as core::ffi::c_int {
            memcpy(
                &mut *g
                    .as_mut_ptr()
                    .offset(((2 as WORD32 * i + 0 as WORD32) * synth_size) as isize)
                    as *mut FLOAT32 as *mut core::ffi::c_void,
                &mut *buffer
                    .offset(((4 as WORD32 * i + 0 as WORD32) * synth_size) as isize)
                    as *mut FLOAT32 as *const core::ffi::c_void,
                (::core::mem::size_of::<FLOAT32>() as size_t)
                    .wrapping_mul(synth_size as size_t),
            );
            memcpy(
                &mut *g
                    .as_mut_ptr()
                    .offset(((2 as WORD32 * i + 1 as WORD32) * synth_size) as isize)
                    as *mut FLOAT32 as *mut core::ffi::c_void,
                &mut *buffer
                    .offset(((4 as WORD32 * i + 3 as WORD32) * synth_size) as isize)
                    as *mut FLOAT32 as *const core::ffi::c_void,
                (::core::mem::size_of::<FLOAT32>() as size_t)
                    .wrapping_mul(synth_size as size_t),
            );
            i += 1;
        }
        k = 0 as core::ffi::c_int as WORD32;
        while k < 10 as WORD32 * synth_size {
            w[k as usize] = g[k as usize] * *interp_window_coeff.offset(k as isize);
            k += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < synth_size {
            accu_r = 0.0f32;
            j = 0 as core::ffi::c_int as WORD32;
            while j < 10 as core::ffi::c_int {
                accu_r = accu_r + w[(synth_size * j + i) as usize];
                j += 1;
            }
            *out_buf.offset(i as isize) = accu_r;
            i += 1;
        }
        idx += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dft_hbe_cplx_anal_filt(
    mut ptr_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    mut qmf_buf_real: *mut [FLOAT32; 64],
    mut qmf_buf_imag: *mut [FLOAT32; 64],
) -> WORD32 {
    let mut idx: WORD32 = 0;
    let mut anal_size: WORD32 = (*ptr_hbe_txposer).analy_size;
    let mut N: WORD32 = 10 as WORD32 * (*ptr_hbe_txposer).analy_size;
    idx = 0 as core::ffi::c_int as WORD32;
    while idx < (*ptr_hbe_txposer).no_bins {
        let mut i: WORD32 = 0;
        let mut j: WORD32 = 0;
        let mut k: WORD32 = 0;
        let mut l: WORD32 = 0;
        let mut window_output: [FLOAT32; 640] = [0.; 640];
        let mut u: [FLOAT32; 128] = [0.; 128];
        let mut accu_r: FLOAT32 = 0.;
        let mut accu_i: FLOAT32 = 0.;
        let mut inp_signal: *const FLOAT32 = 0 as *const FLOAT32;
        let mut qmf_buf_r: *mut FLOAT32 = &mut *(*qmf_buf_real.offset(idx as isize))
            .as_mut_ptr()
            .offset((*ptr_hbe_txposer).a_start as isize) as *mut FLOAT32;
        let mut qmf_buf_i: *mut FLOAT32 = &mut *(*qmf_buf_imag.offset(idx as isize))
            .as_mut_ptr()
            .offset((*ptr_hbe_txposer).a_start as isize) as *mut FLOAT32;
        let mut interp_window_coeff: *const FLOAT32 = (*ptr_hbe_txposer)
            .analy_wind_coeff;
        let mut x: *mut FLOAT32 = ((*ptr_hbe_txposer).analy_buf).as_mut_ptr();
        memset(
            &mut *(*qmf_buf_real.offset(idx as isize))
                .as_mut_ptr()
                .offset((*ptr_hbe_txposer).a_start as isize) as *mut FLOAT32
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ((NO_QMF_SYNTH_CHANNELS - (*ptr_hbe_txposer).a_start) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        memset(
            &mut *(*qmf_buf_imag.offset(idx as isize))
                .as_mut_ptr()
                .offset((*ptr_hbe_txposer).a_start as isize) as *mut FLOAT32
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (TWICE_QMF_SYNTH_CHANNELS_NUM as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        inp_signal = ((*ptr_hbe_txposer).ptr_output_buf)
            .offset((idx * (*ptr_hbe_txposer).analy_size) as isize)
            .offset(1 as core::ffi::c_int as isize);
        i = (N as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while i >= anal_size {
            *x.offset(i as isize) = *x.offset((i - anal_size) as isize);
            i -= 1;
        }
        i = (anal_size as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while i >= 0 as core::ffi::c_int {
            *x.offset(i as isize) = *inp_signal
                .offset((anal_size - 1 as WORD32 - i) as isize);
            i -= 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < N {
            window_output[i as usize] = *x.offset(i as isize)
                * *interp_window_coeff.offset(i as isize);
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < 2 as WORD32 * anal_size {
            accu_r = 0.0f32;
            j = 0 as core::ffi::c_int as WORD32;
            while j < 5 as core::ffi::c_int {
                accu_r = accu_r
                    + window_output[(i + j * 2 as WORD32 * anal_size) as usize];
                j += 1;
            }
            u[i as usize] = accu_r;
            i += 1;
        }
        k = 0 as core::ffi::c_int as WORD32;
        while k < anal_size {
            accu_r = 0 as core::ffi::c_int as FLOAT32;
            accu_i = 0 as core::ffi::c_int as FLOAT32;
            l = 0 as core::ffi::c_int as WORD32;
            while l < 2 as WORD32 * anal_size {
                accu_r = accu_r
                    + u[l as usize]
                        * (*ptr_hbe_txposer)
                            .str_dft_hbe_anal_coeff
                            .real[k as usize][l as usize];
                accu_i = accu_i
                    + u[l as usize]
                        * (*ptr_hbe_txposer)
                            .str_dft_hbe_anal_coeff
                            .imag[k as usize][l as usize];
                l += 1;
            }
            *qmf_buf_r.offset(k as isize) = accu_r;
            *qmf_buf_i.offset(k as isize) = accu_i;
            k += 1;
        }
        idx += 1;
    }
    return 0 as WORD32;
}
