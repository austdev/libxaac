extern "C" {
    fn pow(__x: core::ffi::c_double, __y: core::ffi::c_double) -> core::ffi::c_double;
    fn fabs(__x: core::ffi::c_double) -> core::ffi::c_double;
}
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type WORD64 = core::ffi::c_longlong;
pub type FLOAT32 = core::ffi::c_float;
pub type FLOAT64 = core::ffi::c_double;
pub type VOID = ();
pub type WORD = core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_peak_limiter_struct {
    pub attack_time: FLOAT32,
    pub release_time: FLOAT32,
    pub attack_constant: FLOAT32,
    pub release_constant: FLOAT32,
    pub limit_threshold: FLOAT32,
    pub num_channels: UWORD32,
    pub sample_rate: UWORD32,
    pub attack_time_samples: UWORD32,
    pub limiter_on: UWORD32,
    pub gain_modified: FLOAT32,
    pub pre_smoothed_gain: FLOAT64,
    pub delayed_input: *mut FLOAT32,
    pub delayed_input_index: UWORD32,
    pub max_buf: *mut FLOAT32,
    pub min_gain: FLOAT32,
    pub buffer: [FLOAT32; 16384],
    pub max_idx: WORD32,
    pub cir_buf_pnt: WORD32,
}
pub const DEFAULT_ATTACK_TIME_MS: core::ffi::c_float = 5.0f32;
pub const DEFAULT_RELEASE_TIME_MS: core::ffi::c_float = 50.0f32;
pub const PEAK_LIM_THR_FLOAT: core::ffi::c_float = 29203.6f32;
pub const PEAK_LIM_THR_FIX: core::ffi::c_int = 2147483647 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_peak_limiter_init(
    mut peak_limiter: *mut ia_peak_limiter_struct,
    mut num_channels: UWORD32,
    mut sample_rate: UWORD32,
    mut buffer: *mut FLOAT32,
    mut delay_in_samples: *mut UWORD32,
) -> WORD32 {
    let mut attack: UWORD32 = 0;
    attack = (DEFAULT_ATTACK_TIME_MS * sample_rate as core::ffi::c_float
        / 1000 as core::ffi::c_int as core::ffi::c_float) as UWORD32;
    *delay_in_samples = attack;
    if attack < 1 as UWORD32 {
        return 0 as WORD32;
    }
    (*peak_limiter).max_buf = buffer;
    (*peak_limiter).max_idx = 0 as core::ffi::c_int as WORD32;
    (*peak_limiter).cir_buf_pnt = 0 as core::ffi::c_int as WORD32;
    (*peak_limiter).delayed_input = buffer
        .offset(attack.wrapping_mul(4 as UWORD32) as isize)
        .offset(32 as core::ffi::c_int as isize);
    (*peak_limiter).delayed_input_index = 0 as UWORD32;
    (*peak_limiter).attack_time = DEFAULT_ATTACK_TIME_MS as FLOAT32;
    (*peak_limiter).release_time = DEFAULT_RELEASE_TIME_MS as FLOAT32;
    (*peak_limiter).attack_time_samples = attack;
    (*peak_limiter).attack_constant = pow(
        0.1f64,
        1.0f64 / attack.wrapping_add(1 as UWORD32) as core::ffi::c_double,
    ) as FLOAT32;
    (*peak_limiter).release_constant = pow(
        0.1f64,
        1.0f64
            / (DEFAULT_RELEASE_TIME_MS * sample_rate as core::ffi::c_float
                / 1000 as core::ffi::c_int as core::ffi::c_float
                + 1 as core::ffi::c_int as core::ffi::c_float) as core::ffi::c_double,
    ) as FLOAT32;
    (*peak_limiter).num_channels = num_channels;
    (*peak_limiter).sample_rate = sample_rate;
    (*peak_limiter).min_gain = 1.0f32 as FLOAT32;
    (*peak_limiter).limiter_on = 1 as UWORD32;
    (*peak_limiter).pre_smoothed_gain = 1.0f64;
    (*peak_limiter).gain_modified = 1.0f32 as FLOAT32;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_peak_limiter_process_float(
    mut peak_limiter: *mut ia_peak_limiter_struct,
    mut samples: *mut [FLOAT32; 4096],
    mut frame_len: UWORD32,
) -> VOID {
    let mut i: UWORD32 = 0;
    let mut j: UWORD32 = 0;
    let mut tmp: FLOAT32 = 0.;
    let mut gain: FLOAT32 = 0.;
    let mut min_gain: FLOAT32 = 1.0f32;
    let mut maximum: FLOAT32 = 0.;
    let mut num_channels: UWORD32 = (*peak_limiter).num_channels;
    let mut attack_time_samples: UWORD32 = (*peak_limiter).attack_time_samples;
    let mut attack_constant: FLOAT32 = (*peak_limiter).attack_constant;
    let mut release_constant: FLOAT32 = (*peak_limiter).release_constant;
    let mut max_buf: *mut FLOAT32 = (*peak_limiter).max_buf;
    let mut gain_modified: FLOAT32 = (*peak_limiter).gain_modified;
    let mut delayed_input: *mut FLOAT32 = (*peak_limiter).delayed_input;
    let mut delayed_input_index: UWORD32 = (*peak_limiter).delayed_input_index;
    let mut pre_smoothed_gain: FLOAT64 = (*peak_limiter).pre_smoothed_gain;
    let mut limit_threshold: FLOAT32 = PEAK_LIM_THR_FLOAT;
    if (*peak_limiter).limiter_on != 0 || pre_smoothed_gain as FLOAT32 != 0. {
        i = 0 as UWORD32;
        while i < frame_len {
            tmp = 0.0f32 as FLOAT32;
            j = 0 as UWORD32;
            while j < num_channels {
                tmp = (if tmp as core::ffi::c_double
                    > fabs(
                        (*samples.offset(j as isize))[i as usize] as core::ffi::c_double,
                    )
                {
                    tmp as core::ffi::c_double
                } else {
                    fabs(
                        (*samples.offset(j as isize))[i as usize] as core::ffi::c_double,
                    )
                }) as FLOAT32;
                j = j.wrapping_add(1);
            }
            *max_buf.offset((*peak_limiter).cir_buf_pnt as isize) = tmp;
            if (*peak_limiter).max_idx == (*peak_limiter).cir_buf_pnt {
                (*peak_limiter).max_idx = 0 as core::ffi::c_int as WORD32;
                j = 1 as UWORD32;
                while j < attack_time_samples {
                    if *max_buf.offset(j as isize)
                        > *max_buf.offset((*peak_limiter).max_idx as isize)
                    {
                        (*peak_limiter).max_idx = j as WORD32;
                    }
                    j = j.wrapping_add(1);
                }
            } else if tmp >= *max_buf.offset((*peak_limiter).max_idx as isize) {
                (*peak_limiter).max_idx = (*peak_limiter).cir_buf_pnt;
            }
            (*peak_limiter).cir_buf_pnt += 1;
            if (*peak_limiter).cir_buf_pnt == attack_time_samples as WORD32 {
                (*peak_limiter).cir_buf_pnt = 0 as core::ffi::c_int as WORD32;
            }
            maximum = *max_buf.offset((*peak_limiter).max_idx as isize);
            if maximum > limit_threshold {
                gain = limit_threshold / maximum;
            } else {
                gain = 1 as core::ffi::c_int as FLOAT32;
            }
            if (gain as FLOAT64) < pre_smoothed_gain {
                gain_modified = (if gain_modified
                    > (gain as core::ffi::c_float
                        - 0.1f32 * pre_smoothed_gain as core::ffi::c_float)
                        * 1.11111111f32
                {
                    (gain as core::ffi::c_float
                        - 0.1f32 * pre_smoothed_gain as core::ffi::c_float)
                        * 1.11111111f32
                } else {
                    gain_modified as core::ffi::c_float
                }) as FLOAT32;
            } else {
                gain_modified = gain;
            }
            if (gain_modified as FLOAT64) < pre_smoothed_gain {
                pre_smoothed_gain = attack_constant as FLOAT64
                    * (pre_smoothed_gain - gain_modified as FLOAT64)
                    + gain_modified as FLOAT64;
                pre_smoothed_gain = if pre_smoothed_gain > gain as FLOAT64 {
                    pre_smoothed_gain
                } else {
                    gain as FLOAT64
                };
            } else {
                pre_smoothed_gain = release_constant as FLOAT64
                    * (pre_smoothed_gain - gain_modified as FLOAT64)
                    + gain_modified as FLOAT64;
            }
            gain = pre_smoothed_gain as FLOAT32;
            j = 0 as UWORD32;
            while j < num_channels {
                tmp = *delayed_input
                    .offset(
                        delayed_input_index.wrapping_mul(num_channels).wrapping_add(j)
                            as isize,
                    );
                *delayed_input
                    .offset(
                        delayed_input_index.wrapping_mul(num_channels).wrapping_add(j)
                            as isize,
                    ) = (*samples.offset(j as isize))[i as usize];
                tmp *= gain;
                if tmp > limit_threshold {
                    tmp = limit_threshold;
                } else if tmp < -limit_threshold {
                    tmp = -limit_threshold;
                }
                (*samples.offset(j as isize))[i as usize] = tmp;
                j = j.wrapping_add(1);
            }
            delayed_input_index = delayed_input_index.wrapping_add(1);
            if delayed_input_index >= attack_time_samples {
                delayed_input_index = 0 as UWORD32;
            }
            if gain < min_gain {
                min_gain = gain;
            }
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as UWORD32;
        while i < frame_len {
            j = 0 as UWORD32;
            while j < num_channels {
                tmp = *delayed_input
                    .offset(
                        delayed_input_index.wrapping_mul(num_channels).wrapping_add(j)
                            as isize,
                    );
                *delayed_input
                    .offset(
                        delayed_input_index.wrapping_mul(num_channels).wrapping_add(j)
                            as isize,
                    ) = (*samples.offset(j as isize))[i as usize];
                (*samples.offset(j as isize))[i as usize] = tmp;
                j = j.wrapping_add(1);
            }
            delayed_input_index = delayed_input_index.wrapping_add(1);
            if delayed_input_index >= attack_time_samples {
                delayed_input_index = 0 as UWORD32;
            }
            i = i.wrapping_add(1);
        }
    }
    (*peak_limiter).gain_modified = gain_modified;
    (*peak_limiter).delayed_input_index = delayed_input_index;
    (*peak_limiter).pre_smoothed_gain = pre_smoothed_gain;
    (*peak_limiter).min_gain = min_gain;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_peak_limiter_process(
    mut peak_limiter: *mut ia_peak_limiter_struct,
    mut samples_t: *mut core::ffi::c_void,
    mut frame_len: UWORD32,
    mut qshift_adj: *mut UWORD8,
) -> VOID {
    let mut i: UWORD32 = 0;
    let mut j: UWORD32 = 0;
    let mut tmp: FLOAT32 = 0.;
    let mut gain: FLOAT32 = 0.;
    let mut min_gain: FLOAT32 = 1.0f32;
    let mut maximum: FLOAT32 = 0.;
    let mut num_channels: UWORD32 = (*peak_limiter).num_channels;
    let mut attack_time_samples: UWORD32 = (*peak_limiter).attack_time_samples;
    let mut attack_constant: FLOAT32 = (*peak_limiter).attack_constant;
    let mut release_constant: FLOAT32 = (*peak_limiter).release_constant;
    let mut max_buf: *mut FLOAT32 = (*peak_limiter).max_buf;
    let mut gain_modified: FLOAT32 = (*peak_limiter).gain_modified;
    let mut delayed_input: *mut FLOAT32 = (*peak_limiter).delayed_input;
    let mut delayed_input_index: UWORD32 = (*peak_limiter).delayed_input_index;
    let mut pre_smoothed_gain: FLOAT64 = (*peak_limiter).pre_smoothed_gain;
    let mut limit_threshold: WORD32 = PEAK_LIM_THR_FIX;
    let mut samples: *mut WORD32 = samples_t as *mut WORD32;
    if (*peak_limiter).limiter_on != 0 || pre_smoothed_gain as FLOAT32 != 0. {
        i = 0 as UWORD32;
        while i < frame_len {
            tmp = 0.0f32 as FLOAT32;
            j = 0 as UWORD32;
            while j < num_channels {
                let mut gain_t: FLOAT32 = ((1 as core::ffi::c_int)
                    << *qshift_adj.offset(j as isize) as core::ffi::c_int) as FLOAT32;
                tmp = (if tmp as core::ffi::c_double
                    > fabs(
                        (*samples
                            .offset(
                                i.wrapping_mul(num_channels).wrapping_add(j) as isize,
                            ) as FLOAT32 * gain_t) as core::ffi::c_double,
                    )
                {
                    tmp as core::ffi::c_double
                } else {
                    fabs(
                        (*samples
                            .offset(
                                i.wrapping_mul(num_channels).wrapping_add(j) as isize,
                            ) as FLOAT32 * gain_t) as core::ffi::c_double,
                    )
                }) as FLOAT32;
                j = j.wrapping_add(1);
            }
            *max_buf.offset((*peak_limiter).cir_buf_pnt as isize) = tmp;
            if (*peak_limiter).max_idx == (*peak_limiter).cir_buf_pnt {
                (*peak_limiter).max_idx = 0 as core::ffi::c_int as WORD32;
                j = 1 as UWORD32;
                while j < attack_time_samples {
                    if *max_buf.offset(j as isize)
                        > *max_buf.offset((*peak_limiter).max_idx as isize)
                    {
                        (*peak_limiter).max_idx = j as WORD32;
                    }
                    j = j.wrapping_add(1);
                }
            } else if tmp >= *max_buf.offset((*peak_limiter).max_idx as isize) {
                (*peak_limiter).max_idx = (*peak_limiter).cir_buf_pnt;
            }
            (*peak_limiter).cir_buf_pnt += 1;
            if (*peak_limiter).cir_buf_pnt == attack_time_samples as WORD32 {
                (*peak_limiter).cir_buf_pnt = 0 as core::ffi::c_int as WORD32;
            }
            maximum = *max_buf.offset((*peak_limiter).max_idx as isize);
            if maximum > limit_threshold as FLOAT32 {
                gain = limit_threshold as FLOAT32 / maximum;
            } else {
                gain = 1 as core::ffi::c_int as FLOAT32;
            }
            if (gain as FLOAT64) < pre_smoothed_gain {
                gain_modified = (if gain_modified
                    > (gain as core::ffi::c_float
                        - 0.1f32 * pre_smoothed_gain as core::ffi::c_float)
                        * 1.11111111f32
                {
                    (gain as core::ffi::c_float
                        - 0.1f32 * pre_smoothed_gain as core::ffi::c_float)
                        * 1.11111111f32
                } else {
                    gain_modified as core::ffi::c_float
                }) as FLOAT32;
            } else {
                gain_modified = gain;
            }
            if (gain_modified as FLOAT64) < pre_smoothed_gain {
                pre_smoothed_gain = attack_constant as FLOAT64
                    * (pre_smoothed_gain - gain_modified as FLOAT64)
                    + gain_modified as FLOAT64;
                pre_smoothed_gain = if pre_smoothed_gain > gain as FLOAT64 {
                    pre_smoothed_gain
                } else {
                    gain as FLOAT64
                };
            } else {
                pre_smoothed_gain = release_constant as FLOAT64
                    * (pre_smoothed_gain - gain_modified as FLOAT64)
                    + gain_modified as FLOAT64;
            }
            gain = pre_smoothed_gain as FLOAT32;
            j = 0 as UWORD32;
            while j < num_channels {
                let mut tmp_fix: WORD64 = 0;
                tmp = *delayed_input
                    .offset(
                        delayed_input_index.wrapping_mul(num_channels).wrapping_add(j)
                            as isize,
                    );
                let mut gain_t_0: FLOAT32 = ((1 as core::ffi::c_int)
                    << *qshift_adj.offset(j as isize) as core::ffi::c_int) as FLOAT32;
                *delayed_input
                    .offset(
                        delayed_input_index.wrapping_mul(num_channels).wrapping_add(j)
                            as isize,
                    ) = *samples
                    .offset(i.wrapping_mul(num_channels).wrapping_add(j) as isize)
                    as FLOAT32 * gain_t_0;
                tmp *= gain;
                tmp_fix = tmp as WORD64;
                if tmp_fix > limit_threshold as WORD64 {
                    tmp_fix = limit_threshold as WORD64;
                } else if tmp_fix < -limit_threshold as WORD64 {
                    tmp_fix = -limit_threshold as WORD64;
                }
                *samples.offset(i.wrapping_mul(num_channels).wrapping_add(j) as isize) = tmp_fix
                    as WORD32;
                j = j.wrapping_add(1);
            }
            delayed_input_index = delayed_input_index.wrapping_add(1);
            if delayed_input_index >= attack_time_samples {
                delayed_input_index = 0 as UWORD32;
            }
            if gain < min_gain {
                min_gain = gain;
            }
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as UWORD32;
        while i < frame_len {
            j = 0 as UWORD32;
            while j < num_channels {
                tmp = *delayed_input
                    .offset(
                        delayed_input_index.wrapping_mul(num_channels).wrapping_add(j)
                            as isize,
                    );
                let mut gain_t_1: FLOAT32 = ((1 as core::ffi::c_int)
                    << *qshift_adj.offset(j as isize) as core::ffi::c_int) as FLOAT32;
                *delayed_input
                    .offset(
                        delayed_input_index.wrapping_mul(num_channels).wrapping_add(j)
                            as isize,
                    ) = *samples
                    .offset(i.wrapping_mul(num_channels).wrapping_add(j) as isize)
                    as FLOAT32 * gain_t_1;
                *samples.offset(i.wrapping_mul(num_channels).wrapping_add(j) as isize) = tmp
                    as WORD32;
                j = j.wrapping_add(1);
            }
            delayed_input_index = delayed_input_index.wrapping_add(1);
            if delayed_input_index >= attack_time_samples {
                delayed_input_index = 0 as UWORD32;
            }
            i = i.wrapping_add(1);
        }
    }
    (*peak_limiter).gain_modified = gain_modified;
    (*peak_limiter).delayed_input_index = delayed_input_index;
    (*peak_limiter).pre_smoothed_gain = pre_smoothed_gain;
    (*peak_limiter).min_gain = min_gain;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_scale_adjust(
    mut samples: *mut WORD32,
    mut frame_len: UWORD32,
    mut qshift_adj: *mut WORD8,
    mut num_channels: WORD,
) -> VOID {
    let mut i: UWORD32 = 0;
    let mut j: WORD32 = 0;
    i = 0 as UWORD32;
    while i < frame_len {
        j = 0 as core::ffi::c_int as WORD32;
        while j < num_channels {
            let mut gain_t: WORD32 = (1 as core::ffi::c_int)
                << *qshift_adj.offset(j as isize) as core::ffi::c_int;
            *samples
                .offset(
                    i.wrapping_mul(num_channels as UWORD32).wrapping_add(j as UWORD32)
                        as isize,
                ) = *samples
                .offset(
                    i.wrapping_mul(num_channels as UWORD32).wrapping_add(j as UWORD32)
                        as isize,
                ) * gain_t;
            j += 1;
        }
        i = i.wrapping_add(1);
    }
}
