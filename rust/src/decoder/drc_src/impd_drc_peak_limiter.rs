extern "C" {
    fn pow(__x: core::ffi::c_double, __y: core::ffi::c_double) -> core::ffi::c_double;
    fn fabs(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
pub type size_t = usize;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type FLOAT32 = core::ffi::c_float;
pub type FLOAT64 = core::ffi::c_double;
pub type VOID = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_peak_limiter_struct {
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
    pub buffer: *mut FLOAT32,
}
#[no_mangle]
pub unsafe extern "C" fn impd_peak_limiter_init(
    mut peak_limiter: *mut ia_drc_peak_limiter_struct,
    mut attack_time: FLOAT32,
    mut release_time: FLOAT32,
    mut limit_threshold: FLOAT32,
    mut num_channels: UWORD32,
    mut sample_rate: UWORD32,
    mut buffer: *mut FLOAT32,
) -> WORD32 {
    let mut attack: UWORD32 = 0;
    attack = (attack_time * sample_rate as FLOAT32 / 1000 as core::ffi::c_int as FLOAT32)
        as UWORD32;
    if attack < 1 as UWORD32 {
        return 0 as WORD32;
    }
    (*peak_limiter).max_buf = buffer;
    (*peak_limiter).delayed_input = buffer
        .offset(attack.wrapping_mul(4 as UWORD32) as isize)
        .offset(32 as core::ffi::c_int as isize);
    (*peak_limiter).delayed_input_index = 0 as UWORD32;
    (*peak_limiter).attack_time = attack_time;
    (*peak_limiter).release_time = release_time;
    (*peak_limiter).attack_time_samples = attack;
    (*peak_limiter).attack_constant = pow(
        0.1f64,
        1.0f64 / attack.wrapping_add(1 as UWORD32) as core::ffi::c_double,
    ) as FLOAT32;
    (*peak_limiter).release_constant = pow(
        0.1f64,
        1.0f64
            / (release_time * sample_rate as FLOAT32
                / 1000 as core::ffi::c_int as FLOAT32 + 1 as core::ffi::c_int as FLOAT32)
                as core::ffi::c_double,
    ) as FLOAT32;
    (*peak_limiter).limit_threshold = limit_threshold;
    (*peak_limiter).num_channels = num_channels;
    (*peak_limiter).sample_rate = sample_rate;
    (*peak_limiter).min_gain = 1.0f32 as FLOAT32;
    (*peak_limiter).limiter_on = 1 as UWORD32;
    (*peak_limiter).pre_smoothed_gain = 1.0f64;
    (*peak_limiter).gain_modified = 1.0f32 as FLOAT32;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_peak_limiter_reinit(
    mut peak_limiter: *mut ia_drc_peak_limiter_struct,
) -> VOID {
    if !peak_limiter.is_null() {
        (*peak_limiter).delayed_input_index = 0 as UWORD32;
        (*peak_limiter).pre_smoothed_gain = 1.0f64;
        (*peak_limiter).gain_modified = 1.0f32 as FLOAT32;
        (*peak_limiter).min_gain = 1.0f32 as FLOAT32;
        memset(
            (*peak_limiter).max_buf as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (((*peak_limiter).attack_time_samples).wrapping_add(1 as UWORD32) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
        memset(
            (*peak_limiter).delayed_input as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (((*peak_limiter).attack_time_samples)
                .wrapping_mul((*peak_limiter).num_channels) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn impd_limiter_process(
    mut peak_limiter: *mut ia_drc_peak_limiter_struct,
    mut samples: *mut FLOAT32,
    mut frame_len: UWORD32,
) -> VOID {
    let mut i: UWORD32 = 0;
    let mut j: UWORD32 = 0;
    let mut tmp: FLOAT32 = 0.;
    let mut gain: FLOAT32 = 0.;
    let mut min_gain: FLOAT32 = 1 as core::ffi::c_int as FLOAT32;
    let mut maximum: FLOAT32 = 0.;
    let mut sectionMaximum: FLOAT32 = 0.;
    let mut num_channels: UWORD32 = (*peak_limiter).num_channels;
    let mut attack_time_samples: UWORD32 = (*peak_limiter).attack_time_samples;
    let mut attack_constant: FLOAT32 = (*peak_limiter).attack_constant;
    let mut release_constant: FLOAT32 = (*peak_limiter).release_constant;
    let mut limit_threshold: FLOAT32 = (*peak_limiter).limit_threshold;
    let mut max_buf: *mut FLOAT32 = (*peak_limiter).max_buf;
    let mut gain_modified: FLOAT32 = (*peak_limiter).gain_modified;
    let mut delayed_input: *mut FLOAT32 = (*peak_limiter).delayed_input;
    let mut delayed_input_index: UWORD32 = (*peak_limiter).delayed_input_index;
    let mut pre_smoothed_gain: FLOAT64 = (*peak_limiter).pre_smoothed_gain;
    if (*peak_limiter).limiter_on != 0 || (pre_smoothed_gain as FLOAT32) < 1.0f32 {
        i = 0 as UWORD32;
        while i < frame_len {
            tmp = 0.0f32 as FLOAT32;
            j = 0 as UWORD32;
            while j < num_channels {
                tmp = if tmp
                    > fabs(
                        *samples
                            .offset(
                                i.wrapping_mul(num_channels).wrapping_add(j) as isize,
                            ) as core::ffi::c_double,
                    ) as FLOAT32
                {
                    tmp
                } else {
                    fabs(
                        *samples
                            .offset(
                                i.wrapping_mul(num_channels).wrapping_add(j) as isize,
                            ) as core::ffi::c_double,
                    ) as FLOAT32
                };
                j = j.wrapping_add(1);
            }
            j = attack_time_samples;
            while j > 0 as UWORD32 {
                *max_buf.offset(j as isize) = *max_buf
                    .offset(j.wrapping_sub(1 as UWORD32) as isize);
                j = j.wrapping_sub(1);
            }
            *max_buf.offset(0 as core::ffi::c_int as isize) = tmp;
            sectionMaximum = tmp;
            j = 1 as UWORD32;
            while j < attack_time_samples.wrapping_add(1 as UWORD32) {
                if *max_buf.offset(j as isize) > sectionMaximum {
                    sectionMaximum = *max_buf.offset(j as isize);
                }
                j = j.wrapping_add(1);
            }
            maximum = sectionMaximum;
            if maximum > limit_threshold {
                gain = limit_threshold / maximum;
            } else {
                gain = 1 as core::ffi::c_int as FLOAT32;
            }
            if (gain as FLOAT64) < pre_smoothed_gain {
                gain_modified = (if gain_modified
                    < (gain as core::ffi::c_float
                        - 0.1f32 * pre_smoothed_gain as core::ffi::c_float)
                        * 1.11111111f32
                {
                    gain_modified as core::ffi::c_float
                } else {
                    (gain as core::ffi::c_float
                        - 0.1f32 * pre_smoothed_gain as core::ffi::c_float)
                        * 1.11111111f32
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
                    ) = *samples
                    .offset(i.wrapping_mul(num_channels).wrapping_add(j) as isize);
                tmp *= gain;
                if tmp > limit_threshold {
                    tmp = limit_threshold;
                } else if tmp < -limit_threshold {
                    tmp = -limit_threshold;
                }
                *samples.offset(i.wrapping_mul(num_channels).wrapping_add(j) as isize) = tmp;
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
                    ) = *samples
                    .offset(i.wrapping_mul(num_channels).wrapping_add(j) as isize);
                *samples.offset(i.wrapping_mul(num_channels).wrapping_add(j) as isize) = tmp;
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
