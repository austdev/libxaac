extern "C" {
    fn cos(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn sin(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn log10(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn pow(__x: core::ffi::c_double, __y: core::ffi::c_double) -> core::ffi::c_double;
    fn sqrt(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn fabs(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
pub type size_t = usize;
pub type WORD32 = core::ffi::c_int;
pub type FLOAT32 = core::ffi::c_float;
pub type FLOAT64 = core::ffi::c_double;
pub type VOID = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_filt_ele_struct {
    pub filt_ele_idx: WORD32,
    pub filt_ele_gain_flag: WORD32,
    pub filt_ele_gain: FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_filt_block_struct {
    pub filter_element_count: WORD32,
    pub str_filter_element: [ia_filt_ele_struct; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_unique_td_filt_element {
    pub eq_filter_format: WORD32,
    pub bs_real_zero_radius_one_count: WORD32,
    pub real_zero_count: WORD32,
    pub generic_zero_count: WORD32,
    pub real_pole_count: WORD32,
    pub cmplx_pole_count: WORD32,
    pub zero_sign: [WORD32; 14],
    pub real_zero_radius: [FLOAT32; 64],
    pub generic_zero_radius: [FLOAT32; 64],
    pub generic_zero_angle: [FLOAT32; 64],
    pub real_pole_radius: [FLOAT32; 16],
    pub complex_pole_radius: [FLOAT32; 16],
    pub complex_pole_angle: [FLOAT32; 16],
    pub fir_filt_order: WORD32,
    pub fir_symmetry: WORD32,
    pub fir_coeff: [FLOAT32; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_eq_subband_gain_spline_struct {
    pub num_eq_nodes: WORD32,
    pub eq_slope: [FLOAT32; 33],
    pub eq_freq_delta: [WORD32; 33],
    pub eq_gain_initial: FLOAT32,
    pub eq_gain_delta: [FLOAT32; 33],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_eq_subband_gain_vector {
    pub eq_subband_gain: [FLOAT32; 135],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_eq_coeff_struct {
    pub eq_delay_max_present: WORD32,
    pub eq_delay_max: WORD32,
    pub unique_filter_block_count: WORD32,
    pub str_filter_block: [ia_filt_block_struct; 16],
    pub unique_td_filter_element_count: WORD32,
    pub unique_td_filt_ele: [ia_unique_td_filt_element; 16],
    pub unique_eq_subband_gains_count: WORD32,
    pub eq_subband_gain_representation: WORD32,
    pub eq_subband_gain_format: WORD32,
    pub eq_subband_gain_count: WORD32,
    pub str_eq_subband_gain_spline: [ia_eq_subband_gain_spline_struct; 16],
    pub str_eq_subband_gain_vector: [ia_eq_subband_gain_vector; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_filter_block_refs_struct {
    pub filter_block_count: WORD32,
    pub filter_block_index: [WORD32; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_td_filter_cascade_struct {
    pub eq_cascade_gain_present: [WORD32; 4],
    pub eq_cascade_gain: [FLOAT32; 4],
    pub str_filter_block_refs: [ia_filter_block_refs_struct; 4],
    pub eq_phase_alignment_present: WORD32,
    pub eq_phase_alignment: [[WORD32; 4]; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_eq_instructions_struct {
    pub eq_set_id: WORD32,
    pub eq_set_complexity_level: WORD32,
    pub dwnmix_id_count: WORD32,
    pub downmix_id: [WORD32; 8],
    pub eq_apply_to_downmix: WORD32,
    pub drc_set_id_count: WORD32,
    pub drc_set_id: [WORD32; 16],
    pub eq_set_purpose: WORD32,
    pub depends_on_eq_set_present: WORD32,
    pub depends_on_eq_set: WORD32,
    pub no_independent_eq_use: WORD32,
    pub eq_channel_count: WORD32,
    pub eq_ch_group_count: WORD32,
    pub eq_ch_group_of_channel: [WORD32; 8],
    pub td_filter_cascade_present: WORD32,
    pub str_td_filter_cascade: ia_td_filter_cascade_struct,
    pub subband_gains_present: WORD32,
    pub subband_gains_index: [WORD32; 4],
    pub eq_transition_duration_present: WORD32,
    pub eq_transition_duration: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_audio_delay_struct {
    pub delay: WORD32,
    pub state: [[FLOAT32; 189]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_2nd_order_filt_params_struct {
    pub radius: FLOAT32,
    pub coeff: [FLOAT32; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_fir_filter_struct {
    pub coeff_count: WORD32,
    pub coeff: [FLOAT32; 128],
    pub state: [[FLOAT32; 128]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_subband_filt_struct {
    pub eq_frame_size_subband: WORD32,
    pub coeff_count: WORD32,
    pub subband_coeff: [FLOAT32; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_interm_filt_params_struct {
    pub filter_format: WORD32,
    pub filter_param_count_of_zeros: WORD32,
    pub ord_2_filt_params_of_zeros: [ia_2nd_order_filt_params_struct; 32],
    pub filter_param_count_of_poles: WORD32,
    pub ord_2_filt_params_of_poles: [ia_2nd_order_filt_params_struct; 32],
    pub filter_param_count_of_fir: WORD32,
    pub fir_filter: ia_fir_filter_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_filt_sect_state_struct {
    pub in_state_1: FLOAT32,
    pub in_state_2: FLOAT32,
    pub out_state_1: FLOAT32,
    pub out_state_2: FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_filt_sect_struct {
    pub a1: FLOAT32,
    pub a2: FLOAT32,
    pub b1: FLOAT32,
    pub b2: FLOAT32,
    pub filt_sect_state: [ia_filt_sect_state_struct; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_cascade_align_group_struct {
    pub member_count: WORD32,
    pub member_idx: [WORD32; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ph_alignment_filt_struct {
    pub validity_flag: WORD32,
    pub num_matches_filter: WORD32,
    pub matches_filter: [WORD32; 8],
    pub gain: FLOAT32,
    pub section_count: WORD32,
    pub filt_section: [ia_filt_sect_struct; 8],
    pub audio_delay: ia_audio_delay_struct,
}
pub type ia_matching_ph_filt_struct = ia_ph_alignment_filt_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_pole_zero_filt_struct {
    pub section_count: WORD32,
    pub filt_section: [ia_filt_sect_struct; 8],
    pub filt_coeffs_flag: WORD32,
    pub fir_filter: ia_fir_filter_struct,
    pub audio_delay: ia_audio_delay_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_eq_filt_ele_struct {
    pub elementGainLinear: FLOAT32,
    pub format: WORD32,
    pub pstr_pole_zero_filt: ia_pole_zero_filt_struct,
    pub fir_filter: ia_fir_filter_struct,
    pub num_ph_align_filt: WORD32,
    pub ph_alignment_filt: [ia_ph_alignment_filt_struct; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_eq_filt_block_struct {
    pub element_count: WORD32,
    pub eq_filt_element: [ia_eq_filt_ele_struct; 4],
    pub matching_ph_filt_ele_0: ia_matching_ph_filt_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_filt_cascade_td_struct {
    pub cascade_gain_linear: FLOAT32,
    pub block_count: WORD32,
    pub pstr_eq_filt_block: [ia_eq_filt_block_struct; 4],
    pub num_ph_align_filt: WORD32,
    pub ph_alignment_filt: [ia_ph_alignment_filt_struct; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_eq_set_struct {
    pub domain: WORD32,
    pub audio_num_chan: WORD32,
    pub eq_ch_group_count: WORD32,
    pub eq_ch_group_of_channel: [WORD32; 8],
    pub filt_cascade_td: [ia_filt_cascade_td_struct; 4],
    pub subband_filt: [ia_subband_filt_struct; 4],
}
pub const M_PI: core::ffi::c_double = 3.14159265358979323846f64;
pub const REAL_ZERO_COUNT_MAX: core::ffi::c_int = 64 as core::ffi::c_int;
pub const COMPLEX_ZERO_COUNT_MAX: core::ffi::c_int = 64 as core::ffi::c_int;
pub const REAL_POLE_COUNT_MAX: core::ffi::c_int = 16 as core::ffi::c_int;
pub const COMPLEX_POLE_COUNT_MAX: core::ffi::c_int = 16 as core::ffi::c_int;
pub const UNEXPECTED_ERROR: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SUBBAND_DOMAIN_MODE_OFF: core::ffi::c_int = 0 as core::ffi::c_int;
pub const SUBBAND_DOMAIN_MODE_QMF64: core::ffi::c_int = 1;
pub const SUBBAND_DOMAIN_MODE_QMF71: core::ffi::c_int = 2;
pub const SUBBAND_DOMAIN_MODE_STFT256: core::ffi::c_int = 3;
pub const AUDIO_CODEC_SUBBAND_COUNT_QMF64: core::ffi::c_int = 64 as core::ffi::c_int;
pub const AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_QMF64: core::ffi::c_int = 64
    as core::ffi::c_int;
pub const AUDIO_CODEC_SUBBAND_COUNT_QMF71: core::ffi::c_int = 71 as core::ffi::c_int;
pub const AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_QMF71: core::ffi::c_int = 64
    as core::ffi::c_int;
pub const AUDIO_CODEC_SUBBAND_COUNT_STFT256: core::ffi::c_int = 256 as core::ffi::c_int;
pub const AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_STFT256: core::ffi::c_int = 256
    as core::ffi::c_int;
pub const GAINFORMAT_QMF32: core::ffi::c_int = 1;
pub const GAINFORMAT_QMFHYBRID39: core::ffi::c_int = 2;
pub const GAINFORMAT_QMF64: core::ffi::c_int = 3;
pub const GAINFORMAT_QMFHYBRID71: core::ffi::c_int = 4;
pub const GAINFORMAT_QMF128: core::ffi::c_int = 5;
pub const GAINFORMAT_QMFHYBRID135: core::ffi::c_int = 6;
pub const GAINFORMAT_UNIFORM: core::ffi::c_int = 7;
pub const EQ_CHANNEL_COUNT_MAX: core::ffi::c_int = 8 as core::ffi::c_int;
pub const EQ_FILTER_DOMAIN_NONE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const EQ_FILTER_DOMAIN_TIME: core::ffi::c_int = (1 as core::ffi::c_int)
    << 0 as core::ffi::c_int;
pub const EQ_FILTER_DOMAIN_SUBBAND: core::ffi::c_int = (1 as core::ffi::c_int)
    << 1 as core::ffi::c_int;
pub const CONFIG_REAL_POLE: core::ffi::c_int = 0;
pub const CONFIG_COMPLEX_POLE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const STEP_RATIO_F_LO: core::ffi::c_float = 20.0f32;
pub const FILTER_ELEMENT_FORMAT_POLE_ZERO: core::ffi::c_int = 0 as core::ffi::c_int;
pub const FILTER_ELEMENT_FORMAT_FIR: core::ffi::c_int = 1 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn impd_derive_subband_center_freq(
    mut eq_subband_gain_count: WORD32,
    mut eq_subband_gain_format: WORD32,
    mut sample_rate: FLOAT32,
    mut subband_center_freq: *mut FLOAT32,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut width: FLOAT32 = 0.;
    let mut offset: FLOAT32 = 0.;
    match eq_subband_gain_format {
        GAINFORMAT_QMF32 | GAINFORMAT_QMF64 | GAINFORMAT_QMF128 | GAINFORMAT_UNIFORM => {
            width = 0.5f32 * sample_rate / eq_subband_gain_count as FLOAT32;
            offset = 0.5f32 * width;
            i = 0 as core::ffi::c_int as WORD32;
            while i < eq_subband_gain_count {
                *subband_center_freq.offset(i as isize) = offset;
                offset = offset + width;
                i += 1;
            }
        }
        GAINFORMAT_QMFHYBRID39 | GAINFORMAT_QMFHYBRID71 | GAINFORMAT_QMFHYBRID135 => {
            return 2 as WORD32;
        }
        _ => {}
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_calc_fir_filt_response(
    mut fir_order: WORD32,
    mut fir_symmetry: WORD32,
    mut coeff: *mut FLOAT32,
    mut frequency_radian: FLOAT32,
    mut response: *mut FLOAT32,
) -> VOID {
    let mut m: WORD32 = 0;
    let mut sum: FLOAT32 = 0.0f32;
    let mut o2: WORD32 = 0;
    if fir_order as core::ffi::c_int & 0x1 as core::ffi::c_int == 0 as core::ffi::c_int {
        o2 = (fir_order as core::ffi::c_int / 2 as core::ffi::c_int) as WORD32;
        if fir_symmetry == 0 as core::ffi::c_int {
            m = 1 as core::ffi::c_int as WORD32;
            while m <= o2 {
                sum
                    += *coeff.offset((o2 - m) as isize)
                        * cos((m as FLOAT32 * frequency_radian) as core::ffi::c_double)
                            as FLOAT32;
                m += 1;
            }
            sum += sum;
            sum = *coeff.offset(o2 as isize);
        } else {
            m = 1 as core::ffi::c_int as WORD32;
            while m <= o2 {
                sum
                    += *coeff.offset((o2 - m) as isize)
                        * sin((m as FLOAT32 * frequency_radian) as core::ffi::c_double)
                            as FLOAT32;
                m += 1;
            }
            sum += sum;
        }
    } else {
        o2 = ((fir_order as core::ffi::c_int + 1 as core::ffi::c_int)
            / 2 as core::ffi::c_int) as WORD32;
        if fir_symmetry == 0 as core::ffi::c_int {
            m = 1 as core::ffi::c_int as WORD32;
            while m <= o2 {
                sum
                    += *coeff.offset((o2 - m) as isize)
                        * cos(
                            ((m as FLOAT32 - 0.5f32) * frequency_radian)
                                as core::ffi::c_double,
                        ) as FLOAT32;
                m += 1;
            }
        } else {
            m = 1 as core::ffi::c_int as WORD32;
            while m <= o2 {
                sum
                    += *coeff.offset((o2 - m) as isize)
                        * sin(
                            ((m as FLOAT32 - 0.5f32) * frequency_radian)
                                as core::ffi::c_double,
                        ) as FLOAT32;
                m += 1;
            }
        }
        sum += sum;
    }
    *response = sum;
}
#[no_mangle]
pub unsafe extern "C" fn impd_calc_filt_ele_response(
    mut element: *mut ia_unique_td_filt_element,
    mut frequency_radian: FLOAT32,
    mut response: *mut FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut part_response: FLOAT32 = 0.;
    let mut radius: FLOAT32 = 0.;
    let mut angle_radian: FLOAT32 = 0.;
    let mut total_response: FLOAT64 = 1.0f64;
    if (*element).eq_filter_format == FILTER_ELEMENT_FORMAT_POLE_ZERO {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*element).bs_real_zero_radius_one_count {
            part_response = 1.0f32 + 1.0f32
                - 2.0f32 * 1.0f32
                    * cos(
                        (frequency_radian - (*element).zero_sign[i as usize] as FLOAT32)
                            as core::ffi::c_double,
                    ) as FLOAT32;
            total_response *= part_response as FLOAT64;
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*element).real_zero_count {
            if (*element).real_zero_radius[i as usize] < 0.0f32 {
                radius = -(*element).real_zero_radius[i as usize];
                angle_radian = M_PI as FLOAT32;
            } else {
                radius = (*element).real_zero_radius[i as usize];
                angle_radian = 0.0f32 as FLOAT32;
            }
            part_response = 1.0f32 + radius * radius
                - 2.0f32 * radius
                    * cos((frequency_radian - angle_radian) as core::ffi::c_double)
                        as FLOAT32;
            total_response *= part_response as FLOAT64;
            part_response = 1.0f32 + radius * radius
                - 2.0f32 * radius
                    * cos((frequency_radian - angle_radian) as core::ffi::c_double)
                        as FLOAT32;
            total_response *= part_response as FLOAT64;
            i += 1;
        }
        total_response = sqrt(total_response as core::ffi::c_double) as FLOAT64;
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*element).generic_zero_count {
            radius = (*element).generic_zero_radius[i as usize];
            part_response = 1.0f32 + radius * radius
                - 2.0f32 * radius
                    * cos(
                        (frequency_radian - (*element).generic_zero_angle[i as usize])
                            as core::ffi::c_double,
                    ) as FLOAT32;
            total_response *= part_response as FLOAT64;
            part_response = 1.0f32 + radius * radius
                - 2.0f32 * radius
                    * cos(
                        (frequency_radian - (*element).generic_zero_angle[i as usize])
                            as core::ffi::c_double,
                    ) as FLOAT32;
            total_response *= part_response as FLOAT64;
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*element).real_pole_count {
            if (*element).real_pole_radius[i as usize] < 0.0f32 {
                radius = -(*element).real_pole_radius[i as usize];
                angle_radian = -M_PI as FLOAT32;
            } else {
                radius = (*element).real_pole_radius[i as usize];
                angle_radian = 0.0f32 as FLOAT32;
            }
            part_response = 1 as core::ffi::c_int as FLOAT32
                / (1.0f32 + radius * radius
                    - 2.0f32 * radius
                        * cos((frequency_radian - angle_radian) as core::ffi::c_double)
                            as FLOAT32);
            total_response *= part_response as FLOAT64;
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*element).cmplx_pole_count {
            part_response = 1 as core::ffi::c_int as FLOAT32
                / (1.0f32
                    + (*element).real_pole_radius[i as usize]
                        * (*element).real_pole_radius[i as usize]
                    - 2.0f32 * (*element).real_pole_radius[i as usize]
                        * cos(
                            (frequency_radian
                                - (*element).complex_pole_angle[i as usize])
                                as core::ffi::c_double,
                        ) as FLOAT32);
            total_response *= (part_response * part_response) as FLOAT64;
            i += 1;
        }
    } else {
        impd_calc_fir_filt_response(
            (*element).fir_filt_order,
            (*element).fir_symmetry,
            ((*element).fir_coeff).as_mut_ptr(),
            frequency_radian,
            &mut part_response,
        );
        total_response *= part_response as FLOAT64;
    }
    *response = total_response as FLOAT32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_calc_filt_block_response(
    mut unique_td_filt_ele: *mut ia_unique_td_filt_element,
    mut str_filter_block: *mut ia_filt_block_struct,
    mut frequency_radian: FLOAT32,
    mut response: *mut FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut part_response: FLOAT32 = 0.;
    let mut total_response: FLOAT64 = 1.0f64;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*str_filter_block).filter_element_count {
        let mut str_filter_element: *mut ia_filt_ele_struct = &mut *((*str_filter_block)
            .str_filter_element)
            .as_mut_ptr()
            .offset(i as isize) as *mut ia_filt_ele_struct;
        impd_calc_filt_ele_response(
            &mut *unique_td_filt_ele.offset((*str_filter_element).filt_ele_idx as isize),
            frequency_radian,
            &mut part_response,
        );
        total_response *= part_response as FLOAT64;
        if (*str_filter_element).filt_ele_gain_flag == 1 as core::ffi::c_int {
            total_response
                *= pow(
                    10.0f64,
                    (0.05f32 * (*str_filter_element).filt_ele_gain)
                        as core::ffi::c_double,
                );
        }
        i += 1;
    }
    *response = total_response as FLOAT32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_calc_subband_gains_td_cascade(
    mut unique_td_filt_ele: *mut ia_unique_td_filt_element,
    mut str_filter_block: *mut ia_filt_block_struct,
    mut str_td_filter_cascade: *mut ia_td_filter_cascade_struct,
    mut eq_subband_gain_format: WORD32,
    mut eq_ch_group_count: WORD32,
    mut sample_rate: FLOAT32,
    mut eq_frame_size_subband: WORD32,
    mut subband_filt: *mut ia_subband_filt_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut err: WORD32 = 0 as WORD32;
    let mut g: WORD32 = 0;
    let mut b: WORD32 = 0;
    let mut response: FLOAT32 = 0.;
    let mut frequency_radian: FLOAT32 = 0.;
    let mut subband_center_freq: [FLOAT32; 256] = [0.; 256];
    let mut total_response: FLOAT64 = 0.;
    let mut eq_subband_gain_count: WORD32 = (*subband_filt).coeff_count;
    err = impd_derive_subband_center_freq(
        eq_subband_gain_count,
        eq_subband_gain_format,
        sample_rate,
        subband_center_freq.as_mut_ptr(),
    );
    if err != 0 {
        return err;
    }
    g = 0 as core::ffi::c_int as WORD32;
    while g < eq_ch_group_count {
        b = 0 as core::ffi::c_int as WORD32;
        while b < eq_subband_gain_count {
            total_response = pow(
                10.0f64,
                (0.05f32 * (*str_td_filter_cascade).eq_cascade_gain[g as usize])
                    as core::ffi::c_double,
            ) as FLOAT64;
            frequency_radian = (2.0f64 * M_PI
                * subband_center_freq[b as usize] as core::ffi::c_double
                / sample_rate as core::ffi::c_double) as FLOAT32;
            i = 0 as core::ffi::c_int as WORD32;
            while i
                < (*str_td_filter_cascade)
                    .str_filter_block_refs[g as usize]
                    .filter_block_count
            {
                impd_calc_filt_block_response(
                    unique_td_filt_ele,
                    &mut *str_filter_block
                        .offset(
                            *((*((*str_td_filter_cascade).str_filter_block_refs)
                                .as_mut_ptr()
                                .offset(g as isize))
                                .filter_block_index)
                                .as_mut_ptr()
                                .offset(i as isize) as isize,
                        ),
                    frequency_radian,
                    &mut response,
                );
                total_response *= response as FLOAT64;
                i += 1;
            }
            (*subband_filt.offset(g as isize)).subband_coeff[b as usize] = total_response
                as FLOAT32;
            b += 1;
        }
        (*subband_filt.offset(g as isize)).eq_frame_size_subband = eq_frame_size_subband;
        g += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_add_cascade(
    mut pstr_cascade_align_grp: *mut ia_cascade_align_group_struct,
    mut c1: WORD32,
    mut c2: WORD32,
    mut done: *mut WORD32,
) -> VOID {
    let mut m: WORD32 = 0;
    let mut n: WORD32 = 0;
    *done = 0 as core::ffi::c_int as WORD32;
    m = 0 as core::ffi::c_int as WORD32;
    while m < (*pstr_cascade_align_grp).member_count {
        if (*pstr_cascade_align_grp).member_idx[m as usize] == c1 {
            n = 0 as core::ffi::c_int as WORD32;
            while n < (*pstr_cascade_align_grp).member_count {
                if (*pstr_cascade_align_grp).member_idx[n as usize] == c2 {
                    *done = 1 as core::ffi::c_int as WORD32;
                }
                n += 1;
            }
            if *done == 0 as core::ffi::c_int {
                (*pstr_cascade_align_grp)
                    .member_idx[(*pstr_cascade_align_grp).member_count as usize] = c2;
                (*pstr_cascade_align_grp).member_count += 1;
                *done = 1 as core::ffi::c_int as WORD32;
            }
        }
        m += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn impd_calc_cascade_align_groups(
    mut eq_ch_group_count: WORD32,
    mut eq_phase_alignment_present: WORD32,
    mut eq_phase_alignment: *mut [WORD32; 4],
    mut cascade_align_grp_cnt: *mut WORD32,
    mut pstr_cascade_align_grp: *mut ia_cascade_align_group_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut g: WORD32 = 0;
    let mut group_count: WORD32 = 0;
    let mut done: WORD32 = 0;
    group_count = 0 as core::ffi::c_int as WORD32;
    if eq_phase_alignment_present == 0 as core::ffi::c_int {
        if eq_ch_group_count > 1 as core::ffi::c_int {
            i = 0 as core::ffi::c_int as WORD32;
            while i < eq_ch_group_count {
                (*pstr_cascade_align_grp.offset(group_count as isize))
                    .member_idx[i as usize] = i;
                i += 1;
            }
            (*pstr_cascade_align_grp.offset(group_count as isize)).member_count = eq_ch_group_count;
            group_count = 1 as core::ffi::c_int as WORD32;
        }
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < eq_ch_group_count {
            k = (i as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
            while k < eq_ch_group_count {
                if (*eq_phase_alignment.offset(i as isize))[k as usize]
                    == 1 as core::ffi::c_int
                {
                    done = 0 as core::ffi::c_int as WORD32;
                    g = 0 as core::ffi::c_int as WORD32;
                    while g < group_count {
                        impd_add_cascade(
                            &mut *pstr_cascade_align_grp.offset(g as isize),
                            i,
                            k,
                            &mut done,
                        );
                        if done == 0 as core::ffi::c_int {
                            impd_add_cascade(
                                &mut *pstr_cascade_align_grp.offset(g as isize),
                                k,
                                i,
                                &mut done,
                            );
                        }
                        g += 1;
                    }
                    if done == 0 as core::ffi::c_int {
                        (*pstr_cascade_align_grp.offset(group_count as isize))
                            .member_idx[0 as core::ffi::c_int as usize] = i;
                        (*pstr_cascade_align_grp.offset(group_count as isize))
                            .member_idx[1 as core::ffi::c_int as usize] = k;
                        (*pstr_cascade_align_grp.offset(group_count as isize))
                            .member_count = 2 as core::ffi::c_int as WORD32;
                        group_count += 1;
                    }
                }
                k += 1;
            }
            i += 1;
        }
    }
    *cascade_align_grp_cnt = group_count;
}
#[no_mangle]
pub unsafe extern "C" fn impd_calc_phase_filt_params(
    mut config: WORD32,
    mut radius: FLOAT32,
    mut angle: FLOAT32,
    mut ph_alignment_filt: *mut ia_ph_alignment_filt_struct,
) -> VOID {
    let mut channel: WORD32 = 0;
    let mut zReal: FLOAT32 = 0.;
    let mut zImag: FLOAT32 = 0.;
    let mut prod: FLOAT32 = 0.;
    let mut section: WORD32 = (*ph_alignment_filt).section_count;
    let mut filt_section: *mut ia_filt_sect_struct = &mut *((*ph_alignment_filt)
        .filt_section)
        .as_mut_ptr()
        .offset(section as isize) as *mut ia_filt_sect_struct;
    match config {
        CONFIG_REAL_POLE => {
            (*ph_alignment_filt).gain *= -radius;
            (*filt_section).a1 = -radius;
            (*filt_section).a2 = 0.0f32 as FLOAT32;
            (*filt_section).b1 = -1.0f32 / radius;
            (*filt_section).b2 = 0.0f32 as FLOAT32;
            (*ph_alignment_filt).section_count += 1;
        }
        CONFIG_COMPLEX_POLE => {
            zReal = radius * cos(M_PI * angle as core::ffi::c_double) as FLOAT32;
            zImag = radius * sin(M_PI * angle as core::ffi::c_double) as FLOAT32;
            prod = zReal * zReal + zImag * zImag;
            (*ph_alignment_filt).gain *= prod;
            (*filt_section).a1 = -2.0f32 * zReal;
            (*filt_section).a2 = prod;
            (*filt_section).b1 = -2.0f32 * zReal / prod;
            (*filt_section).b2 = 1.0f32 / prod;
            (*ph_alignment_filt).section_count += 1;
        }
        _ => {}
    }
    channel = 0 as core::ffi::c_int as WORD32;
    while channel < EQ_CHANNEL_COUNT_MAX {
        (*filt_section).filt_sect_state[channel as usize].in_state_1 = 0.0f32 as FLOAT32;
        (*filt_section).filt_sect_state[channel as usize].in_state_2 = 0.0f32 as FLOAT32;
        (*filt_section).filt_sect_state[channel as usize].out_state_1 = 0.0f32
            as FLOAT32;
        (*filt_section).filt_sect_state[channel as usize].out_state_2 = 0.0f32
            as FLOAT32;
        channel += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn impd_calc_phase_filt_delay(
    mut element: *mut ia_unique_td_filt_element,
    mut ph_alignment_filt: *mut ia_ph_alignment_filt_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut delay: WORD32 = 0 as WORD32;
    let mut channel: WORD32 = 0;
    if (*element).eq_filter_format == FILTER_ELEMENT_FORMAT_POLE_ZERO {
        if (*element).bs_real_zero_radius_one_count == 0 as core::ffi::c_int {
            delay = (*element).real_zero_count
                + 2 as WORD32 * (*element).generic_zero_count
                - (*element).real_pole_count - 2 as WORD32 * (*element).cmplx_pole_count;
            delay = (if 0 as core::ffi::c_int > delay {
                0 as core::ffi::c_int
            } else {
                delay as core::ffi::c_int
            }) as WORD32;
            (*ph_alignment_filt).validity_flag = 1 as core::ffi::c_int as WORD32;
        }
    }
    (*ph_alignment_filt).audio_delay.delay = delay;
    channel = 0 as core::ffi::c_int as WORD32;
    while channel < EQ_CHANNEL_COUNT_MAX {
        i = 0 as core::ffi::c_int as WORD32;
        while i < delay {
            (*ph_alignment_filt).audio_delay.state[channel as usize][i as usize] = 0.0f32
                as FLOAT32;
            i += 1;
        }
        channel += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn impd_calc_phase_filt(
    mut element: *mut ia_unique_td_filt_element,
    mut filt_ele_idx: WORD32,
    mut matching_ph_filt: *mut ia_matching_ph_filt_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    memset(
        matching_ph_filt as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<ia_matching_ph_filt_struct>() as size_t,
    );
    (*matching_ph_filt).gain = 1.0f32 as FLOAT32;
    if (*element).eq_filter_format == FILTER_ELEMENT_FORMAT_POLE_ZERO {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*element).real_pole_count {
            impd_calc_phase_filt_params(
                CONFIG_REAL_POLE,
                (*element).real_pole_radius[i as usize],
                0.0f32,
                matching_ph_filt as *mut ia_ph_alignment_filt_struct,
            );
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*element).cmplx_pole_count {
            impd_calc_phase_filt_params(
                CONFIG_COMPLEX_POLE,
                (*element).complex_pole_radius[i as usize],
                (*element).complex_pole_angle[i as usize],
                matching_ph_filt as *mut ia_ph_alignment_filt_struct,
            );
            i += 1;
        }
    }
    impd_calc_phase_filt_delay(
        element,
        matching_ph_filt as *mut ia_ph_alignment_filt_struct,
    );
    (*matching_ph_filt).num_matches_filter = 1 as core::ffi::c_int as WORD32;
    (*matching_ph_filt).matches_filter[0 as core::ffi::c_int as usize] = filt_ele_idx;
}
#[no_mangle]
pub unsafe extern "C" fn impd_calc_filt_params(
    mut element: *mut ia_unique_td_filt_element,
    mut interm_filt_params: *mut ia_interm_filt_params_struct,
) -> WORD32 {
    let mut zReal: FLOAT32 = 0.;
    let mut coeff: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut i: WORD32 = 0;
    let mut param_idx: WORD32 = 0 as WORD32;
    let mut pstr_2nd_oder_filt_params: *mut ia_2nd_order_filt_params_struct = &mut *((*interm_filt_params)
        .ord_2_filt_params_of_zeros)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut ia_2nd_order_filt_params_struct;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*element).bs_real_zero_radius_one_count {
        let mut radius: FLOAT32 = (*element)
            .zero_sign[(i as core::ffi::c_int + 0 as core::ffi::c_int) as usize]
            as FLOAT32;
        let mut angle: FLOAT32 = (*element)
            .zero_sign[(i as core::ffi::c_int + 1 as core::ffi::c_int) as usize]
            as FLOAT32;
        let mut angle1: FLOAT32 = radius;
        let mut angle2: FLOAT32 = angle;
        (*pstr_2nd_oder_filt_params).radius = 1.0f32 as FLOAT32;
        coeff = ((*pstr_2nd_oder_filt_params).coeff).as_mut_ptr();
        if angle1 != angle2 {
            *coeff.offset(0 as core::ffi::c_int as isize) = 0.0f32 as FLOAT32;
            *coeff.offset(1 as core::ffi::c_int as isize) = -1.0f32 as FLOAT32;
        } else if angle1 == 1.0f32 {
            *coeff.offset(0 as core::ffi::c_int as isize) = -2.0f32 as FLOAT32;
            *coeff.offset(1 as core::ffi::c_int as isize) = 1.0f32 as FLOAT32;
        } else {
            *coeff.offset(0 as core::ffi::c_int as isize) = 2.0f32 as FLOAT32;
            *coeff.offset(1 as core::ffi::c_int as isize) = 1.0f32 as FLOAT32;
        }
        pstr_2nd_oder_filt_params = pstr_2nd_oder_filt_params
            .offset(1 as core::ffi::c_int as isize);
        param_idx += 1 as core::ffi::c_int;
        i += 2 as core::ffi::c_int;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*element).real_zero_count {
        let mut radius_0: FLOAT32 = (*element).real_zero_radius[i as usize];
        (*pstr_2nd_oder_filt_params).radius = radius_0;
        if fabs(radius_0 as core::ffi::c_double) == 1.0f64 {
            return -(1 as WORD32)
        } else {
            coeff = ((*pstr_2nd_oder_filt_params).coeff).as_mut_ptr();
            *coeff.offset(0 as core::ffi::c_int as isize) = -(radius_0
                + 1.0f32 / radius_0);
            *coeff.offset(1 as core::ffi::c_int as isize) = 1.0f32 as FLOAT32;
        }
        pstr_2nd_oder_filt_params = pstr_2nd_oder_filt_params
            .offset(1 as core::ffi::c_int as isize);
        param_idx += 1 as core::ffi::c_int;
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*element).generic_zero_count {
        let mut radius_1: FLOAT32 = (*element).generic_zero_radius[i as usize];
        let mut angle_0: FLOAT32 = (*element).generic_zero_angle[i as usize];
        zReal = radius_1 * cos(M_PI * angle_0 as core::ffi::c_double) as FLOAT32;
        (*pstr_2nd_oder_filt_params).radius = radius_1;
        coeff = ((*pstr_2nd_oder_filt_params).coeff).as_mut_ptr();
        *coeff.offset(0 as core::ffi::c_int as isize) = -2.0f32 * zReal;
        *coeff.offset(1 as core::ffi::c_int as isize) = radius_1 * radius_1;
        pstr_2nd_oder_filt_params = pstr_2nd_oder_filt_params
            .offset(1 as core::ffi::c_int as isize);
        zReal = cos(M_PI * angle_0 as core::ffi::c_double) as FLOAT32 / radius_1;
        (*pstr_2nd_oder_filt_params).radius = radius_1;
        coeff = ((*pstr_2nd_oder_filt_params).coeff).as_mut_ptr();
        *coeff.offset(0 as core::ffi::c_int as isize) = -2.0f32 * zReal;
        *coeff.offset(1 as core::ffi::c_int as isize) = 1.0f32 / (radius_1 * radius_1);
        pstr_2nd_oder_filt_params = pstr_2nd_oder_filt_params
            .offset(1 as core::ffi::c_int as isize);
        param_idx += 2 as core::ffi::c_int;
        i += 1;
    }
    (*interm_filt_params).filter_param_count_of_zeros = param_idx;
    param_idx = 0 as core::ffi::c_int as WORD32;
    pstr_2nd_oder_filt_params = &mut *((*interm_filt_params).ord_2_filt_params_of_poles)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut ia_2nd_order_filt_params_struct;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*element).real_pole_count {
        let mut radius_2: FLOAT32 = (*element).real_pole_radius[i as usize];
        (*pstr_2nd_oder_filt_params).radius = radius_2;
        coeff = ((*pstr_2nd_oder_filt_params).coeff).as_mut_ptr();
        *coeff.offset(0 as core::ffi::c_int as isize) = -2.0f32 * radius_2;
        *coeff.offset(1 as core::ffi::c_int as isize) = radius_2 * radius_2;
        param_idx += 1 as core::ffi::c_int;
        pstr_2nd_oder_filt_params = pstr_2nd_oder_filt_params
            .offset(1 as core::ffi::c_int as isize);
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*element).cmplx_pole_count {
        let mut radius_3: FLOAT32 = (*element).complex_pole_radius[i as usize];
        let mut angle_1: FLOAT32 = (*element).complex_pole_angle[i as usize];
        zReal = radius_3 * cos(M_PI * angle_1 as core::ffi::c_double) as FLOAT32;
        (*pstr_2nd_oder_filt_params).radius = radius_3;
        coeff = ((*pstr_2nd_oder_filt_params).coeff).as_mut_ptr();
        *coeff.offset(0 as core::ffi::c_int as isize) = -2.0f32 * zReal;
        *coeff.offset(1 as core::ffi::c_int as isize) = radius_3 * radius_3;
        pstr_2nd_oder_filt_params = pstr_2nd_oder_filt_params
            .offset(1 as core::ffi::c_int as isize);
        (*pstr_2nd_oder_filt_params).radius = radius_3;
        (*pstr_2nd_oder_filt_params).coeff[0 as core::ffi::c_int as usize] = *coeff
            .offset(0 as core::ffi::c_int as isize);
        (*pstr_2nd_oder_filt_params).coeff[1 as core::ffi::c_int as usize] = *coeff
            .offset(1 as core::ffi::c_int as isize);
        pstr_2nd_oder_filt_params = pstr_2nd_oder_filt_params
            .offset(1 as core::ffi::c_int as isize);
        param_idx += 2 as core::ffi::c_int;
        i += 1;
    }
    (*interm_filt_params).filter_param_count_of_poles = param_idx;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_convert_fir_filt_params(
    mut fir_filt_order: WORD32,
    mut fir_symmetry: WORD32,
    mut fir_coeff: *mut FLOAT32,
    mut fir_filter: *mut ia_fir_filter_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut channel: WORD32 = 0;
    let mut coeff: *mut FLOAT32 = ((*fir_filter).coeff).as_mut_ptr();
    (*fir_filter).coeff_count = (fir_filt_order as core::ffi::c_int
        + 1 as core::ffi::c_int) as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i
        < fir_filt_order as core::ffi::c_int / 2 as core::ffi::c_int
            + 1 as core::ffi::c_int
    {
        *coeff.offset(i as isize) = *fir_coeff.offset(i as isize);
        i += 1;
    }
    if fir_symmetry == 1 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i
            < (fir_filt_order as core::ffi::c_int + 1 as core::ffi::c_int)
                / 2 as core::ffi::c_int
        {
            *coeff.offset((fir_filt_order - i) as isize) = -*coeff.offset(i as isize);
            i += 1;
        }
        if fir_filt_order as core::ffi::c_int & 1 as core::ffi::c_int
            == 0 as core::ffi::c_int
        {
            *coeff
                .offset(
                    (fir_filt_order as core::ffi::c_int / 2 as core::ffi::c_int) as isize,
                ) = 0.0f32 as FLOAT32;
        }
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i
            < (fir_filt_order as core::ffi::c_int + 1 as core::ffi::c_int)
                / 2 as core::ffi::c_int
        {
            *coeff.offset((fir_filt_order - i) as isize) = *coeff.offset(i as isize);
            i += 1;
        }
    }
    channel = 0 as core::ffi::c_int as WORD32;
    while channel < EQ_CHANNEL_COUNT_MAX {
        i = 0 as core::ffi::c_int as WORD32;
        while i < fir_filt_order as core::ffi::c_int + 1 as core::ffi::c_int {
            (*fir_filter).state[channel as usize][i as usize] = 0.0f32 as FLOAT32;
            i += 1;
        }
        channel += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn impd_calc_filt_params_all(
    mut element: *mut ia_unique_td_filt_element,
    mut interm_filt_params: *mut ia_interm_filt_params_struct,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    (*interm_filt_params).filter_format = (*element).eq_filter_format;
    if (*element).eq_filter_format == FILTER_ELEMENT_FORMAT_POLE_ZERO {
        err = impd_calc_filt_params(element, interm_filt_params);
        if err != 0 {
            return err;
        }
    } else {
        (*interm_filt_params).filter_param_count_of_zeros = 0 as core::ffi::c_int
            as WORD32;
        (*interm_filt_params).filter_param_count_of_poles = 0 as core::ffi::c_int
            as WORD32;
        impd_convert_fir_filt_params(
            (*element).fir_filt_order,
            (*element).fir_symmetry,
            ((*element).fir_coeff).as_mut_ptr(),
            &mut (*interm_filt_params).fir_filter,
        );
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_calc_eq_filt_elements(
    mut interm_filt_params: *mut ia_interm_filt_params_struct,
    mut eq_filt_element: *mut ia_eq_filt_ele_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut poles_idx: WORD32 = 0;
    let mut zeros_idx: WORD32 = 0;
    let mut pole_order: WORD32 = 0 as WORD32;
    let mut section: WORD32 = 0;
    let mut channel: WORD32 = 0;
    let mut poles_over: [WORD32; 32] = [0; 32];
    let mut zeros_over: [WORD32; 128] = [0; 128];
    let mut max_radius: FLOAT32 = 0.;
    let mut diff_radius: FLOAT32 = 0.;
    let mut coeff_count: WORD32 = 0;
    let mut coeff: *mut FLOAT32 = 0 as *mut FLOAT32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < REAL_POLE_COUNT_MAX + COMPLEX_POLE_COUNT_MAX {
        poles_over[i as usize] = 0 as core::ffi::c_int as WORD32;
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < REAL_ZERO_COUNT_MAX + COMPLEX_ZERO_COUNT_MAX {
        zeros_over[i as usize] = 0 as core::ffi::c_int as WORD32;
        i += 1;
    }
    section = 0 as core::ffi::c_int as WORD32;
    loop {
        max_radius = -1.0f64 as FLOAT32;
        poles_idx = -(1 as core::ffi::c_int) as WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*interm_filt_params).filter_param_count_of_poles {
            if poles_over[i as usize] == 0 as core::ffi::c_int {
                if (*interm_filt_params).filter_format == 0 as core::ffi::c_int {
                    if (max_radius as core::ffi::c_double)
                        < fabs(
                            (*interm_filt_params)
                                .ord_2_filt_params_of_poles[i as usize]
                                .radius as core::ffi::c_double,
                        )
                    {
                        max_radius = fabs(
                            (*interm_filt_params)
                                .ord_2_filt_params_of_poles[i as usize]
                                .radius as core::ffi::c_double,
                        ) as FLOAT32;
                        poles_idx = i;
                        if (*interm_filt_params)
                            .ord_2_filt_params_of_poles[i as usize]
                            .coeff[1 as core::ffi::c_int as usize] != 0.0f32
                        {
                            pole_order = 2 as core::ffi::c_int as WORD32;
                        } else {
                            pole_order = 1 as core::ffi::c_int as WORD32;
                        }
                    }
                }
            }
            i += 1;
        }
        if poles_idx >= 0 as core::ffi::c_int {
            diff_radius = 10.0f32 as FLOAT32;
            zeros_idx = -(1 as core::ffi::c_int) as WORD32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*interm_filt_params).filter_param_count_of_zeros {
                if zeros_over[i as usize] == 0 as core::ffi::c_int {
                    if (*interm_filt_params).filter_format == 0 as core::ffi::c_int {
                        if pole_order == 2 as core::ffi::c_int {
                            if (*interm_filt_params)
                                .ord_2_filt_params_of_zeros[i as usize]
                                .coeff[1 as core::ffi::c_int as usize] != 0.0f32
                            {
                                if diff_radius as core::ffi::c_double
                                    > fabs(
                                        fabs(
                                            (*interm_filt_params)
                                                .ord_2_filt_params_of_zeros[i as usize]
                                                .radius as core::ffi::c_double,
                                        ) - max_radius as core::ffi::c_double,
                                    )
                                {
                                    diff_radius = fabs(
                                        fabs(
                                            (*interm_filt_params)
                                                .ord_2_filt_params_of_zeros[i as usize]
                                                .radius as core::ffi::c_double,
                                        ) - max_radius as core::ffi::c_double,
                                    ) as FLOAT32;
                                    zeros_idx = i;
                                }
                            }
                        } else if (*interm_filt_params)
                            .ord_2_filt_params_of_zeros[i as usize]
                            .coeff[1 as core::ffi::c_int as usize] == 0.0f32
                        {
                            if diff_radius
                                > fabs(
                                    fabs(
                                        (*interm_filt_params)
                                            .ord_2_filt_params_of_zeros[i as usize]
                                            .radius as core::ffi::c_double,
                                    ) - max_radius as core::ffi::c_double,
                                ) as FLOAT32
                            {
                                diff_radius = fabs(
                                    fabs(
                                        (*interm_filt_params)
                                            .ord_2_filt_params_of_zeros[i as usize]
                                            .radius as core::ffi::c_double,
                                    ) - max_radius as core::ffi::c_double,
                                ) as FLOAT32;
                                zeros_idx = i;
                            }
                        }
                    }
                }
                i += 1;
            }
            if zeros_idx == -(1 as core::ffi::c_int) {
                i = 0 as core::ffi::c_int as WORD32;
                while i < (*interm_filt_params).filter_param_count_of_zeros {
                    if zeros_over[i as usize] == 0 as core::ffi::c_int {
                        if (*interm_filt_params).filter_format == 0 as core::ffi::c_int {
                            if pole_order == 2 as core::ffi::c_int {
                                if (*interm_filt_params)
                                    .ord_2_filt_params_of_zeros[i as usize]
                                    .coeff[1 as core::ffi::c_int as usize] == 0.0f32
                                {
                                    if diff_radius
                                        > fabs(
                                            fabs(
                                                (*interm_filt_params)
                                                    .ord_2_filt_params_of_zeros[i as usize]
                                                    .radius as core::ffi::c_double,
                                            ) - max_radius as core::ffi::c_double,
                                        ) as FLOAT32
                                    {
                                        diff_radius = fabs(
                                            fabs(
                                                (*interm_filt_params)
                                                    .ord_2_filt_params_of_zeros[i as usize]
                                                    .radius as core::ffi::c_double,
                                            ) - max_radius as core::ffi::c_double,
                                        ) as FLOAT32;
                                        zeros_idx = i;
                                    }
                                }
                            } else if (*interm_filt_params)
                                .ord_2_filt_params_of_zeros[i as usize]
                                .coeff[1 as core::ffi::c_int as usize] != 0.0f32
                            {
                                if diff_radius
                                    > fabs(
                                        fabs(
                                            (*interm_filt_params)
                                                .ord_2_filt_params_of_zeros[i as usize]
                                                .radius as core::ffi::c_double,
                                        ) - max_radius as core::ffi::c_double,
                                    ) as FLOAT32
                                {
                                    diff_radius = fabs(
                                        fabs(
                                            (*interm_filt_params)
                                                .ord_2_filt_params_of_zeros[i as usize]
                                                .radius as core::ffi::c_double,
                                        ) - max_radius as core::ffi::c_double,
                                    ) as FLOAT32;
                                    zeros_idx = i;
                                }
                            }
                        }
                    }
                    i += 1;
                }
            }
            (*eq_filt_element).pstr_pole_zero_filt.filt_section[section as usize].a1 = (*interm_filt_params)
                .ord_2_filt_params_of_poles[poles_idx as usize]
                .coeff[0 as core::ffi::c_int as usize];
            (*eq_filt_element).pstr_pole_zero_filt.filt_section[section as usize].a2 = (*interm_filt_params)
                .ord_2_filt_params_of_poles[poles_idx as usize]
                .coeff[1 as core::ffi::c_int as usize];
            if zeros_idx >= 0 as core::ffi::c_int {
                (*eq_filt_element)
                    .pstr_pole_zero_filt
                    .filt_section[section as usize]
                    .b1 = (*interm_filt_params)
                    .ord_2_filt_params_of_zeros[zeros_idx as usize]
                    .coeff[0 as core::ffi::c_int as usize];
                (*eq_filt_element)
                    .pstr_pole_zero_filt
                    .filt_section[section as usize]
                    .b2 = (*interm_filt_params)
                    .ord_2_filt_params_of_zeros[zeros_idx as usize]
                    .coeff[1 as core::ffi::c_int as usize];
            } else {
                (*eq_filt_element)
                    .pstr_pole_zero_filt
                    .filt_section[section as usize]
                    .b1 = 0.0f32 as FLOAT32;
                (*eq_filt_element)
                    .pstr_pole_zero_filt
                    .filt_section[section as usize]
                    .b2 = 0.0f32 as FLOAT32;
                (*eq_filt_element).pstr_pole_zero_filt.audio_delay.delay += 1;
            }
            channel = 0 as core::ffi::c_int as WORD32;
            while channel < EQ_CHANNEL_COUNT_MAX {
                (*eq_filt_element)
                    .pstr_pole_zero_filt
                    .filt_section[section as usize]
                    .filt_sect_state[channel as usize]
                    .in_state_1 = 0.0f32 as FLOAT32;
                (*eq_filt_element)
                    .pstr_pole_zero_filt
                    .filt_section[section as usize]
                    .filt_sect_state[channel as usize]
                    .in_state_2 = 0.0f32 as FLOAT32;
                (*eq_filt_element)
                    .pstr_pole_zero_filt
                    .filt_section[section as usize]
                    .filt_sect_state[channel as usize]
                    .out_state_1 = 0.0f32 as FLOAT32;
                (*eq_filt_element)
                    .pstr_pole_zero_filt
                    .filt_section[section as usize]
                    .filt_sect_state[channel as usize]
                    .out_state_2 = 0.0f32 as FLOAT32;
                channel += 1;
            }
            if zeros_idx >= 0 as core::ffi::c_int {
                zeros_over[zeros_idx as usize] = 1 as core::ffi::c_int as WORD32;
            }
            if poles_idx >= 0 as core::ffi::c_int {
                poles_over[poles_idx as usize] = 1 as core::ffi::c_int as WORD32;
            }
            section += 1;
        }
        if !(poles_idx >= 0 as core::ffi::c_int) {
            break;
        }
    }
    (*eq_filt_element).pstr_pole_zero_filt.section_count = section;
    coeff_count = 1 as core::ffi::c_int as WORD32;
    coeff = ((*eq_filt_element).pstr_pole_zero_filt.fir_filter.coeff).as_mut_ptr();
    *coeff.offset(0 as core::ffi::c_int as isize) = 1.0f32 as FLOAT32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*interm_filt_params).filter_param_count_of_zeros {
        if zeros_over[i as usize] == 0 as core::ffi::c_int {
            if (*interm_filt_params).filter_format == 0 as core::ffi::c_int {
                let mut k: WORD32 = 0;
                let mut b1: FLOAT32 = 0.;
                let mut b2: FLOAT32 = 0.;
                b1 = (*interm_filt_params)
                    .ord_2_filt_params_of_zeros[i as usize]
                    .coeff[0 as core::ffi::c_int as usize];
                b2 = (*interm_filt_params)
                    .ord_2_filt_params_of_zeros[i as usize]
                    .coeff[1 as core::ffi::c_int as usize];
                coeff_count += 2 as core::ffi::c_int;
                k = (coeff_count as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
                *coeff.offset(k as isize) = b2
                    * *coeff
                        .offset(
                            (k as core::ffi::c_int - 2 as core::ffi::c_int) as isize,
                        );
                k -= 1;
                if k > 1 as core::ffi::c_int {
                    *coeff.offset(k as isize) = b1
                        * *coeff
                            .offset(
                                (k as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            )
                        + b2
                            * *coeff
                                .offset(
                                    (k as core::ffi::c_int - 2 as core::ffi::c_int) as isize,
                                );
                    k -= 1;
                    while k > 1 as core::ffi::c_int {
                        *coeff.offset(k as isize)
                            += b1
                                * *coeff
                                    .offset(
                                        (k as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                                    )
                                + b2
                                    * *coeff
                                        .offset(
                                            (k as core::ffi::c_int - 2 as core::ffi::c_int) as isize,
                                        );
                        k -= 1;
                    }
                    *coeff.offset(1 as core::ffi::c_int as isize)
                        += b1 * *coeff.offset(0 as core::ffi::c_int as isize);
                } else {
                    *coeff.offset(1 as core::ffi::c_int as isize) = b1
                        * *coeff.offset(0 as core::ffi::c_int as isize);
                }
            }
        }
        zeros_over[i as usize] = 1 as core::ffi::c_int as WORD32;
        i += 1;
    }
    if coeff_count > 1 as core::ffi::c_int {
        (*eq_filt_element).pstr_pole_zero_filt.filt_coeffs_flag = 1 as core::ffi::c_int
            as WORD32;
        (*eq_filt_element).pstr_pole_zero_filt.fir_filter.coeff_count = coeff_count;
    } else {
        (*eq_filt_element).pstr_pole_zero_filt.filt_coeffs_flag = 0 as core::ffi::c_int
            as WORD32;
        (*eq_filt_element).pstr_pole_zero_filt.fir_filter.coeff_count = 0
            as core::ffi::c_int as WORD32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn impd_calc_filt_block(
    mut unique_td_filt_ele: *mut ia_unique_td_filt_element,
    mut str_filter_block: *mut ia_filt_block_struct,
    mut pstr_eq_filt_block: *mut ia_eq_filt_block_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut err: WORD32 = 0;
    let mut interm_filt_params: ia_interm_filt_params_struct = ia_interm_filt_params_struct {
        filter_format: 0,
        filter_param_count_of_zeros: 0,
        ord_2_filt_params_of_zeros: [ia_2nd_order_filt_params_struct {
            radius: 0.,
            coeff: [0.; 2],
        }; 32],
        filter_param_count_of_poles: 0,
        ord_2_filt_params_of_poles: [ia_2nd_order_filt_params_struct {
            radius: 0.,
            coeff: [0.; 2],
        }; 32],
        filter_param_count_of_fir: 0,
        fir_filter: ia_fir_filter_struct {
            coeff_count: 0,
            coeff: [0.; 128],
            state: [[0.; 128]; 8],
        },
    };
    let mut matching_ph_filt: [ia_matching_ph_filt_struct; 16] = [ia_ph_alignment_filt_struct {
        validity_flag: 0,
        num_matches_filter: 0,
        matches_filter: [0; 8],
        gain: 0.,
        section_count: 0,
        filt_section: [ia_filt_sect_struct {
            a1: 0.,
            a2: 0.,
            b1: 0.,
            b2: 0.,
            filt_sect_state: [ia_filt_sect_state_struct {
                in_state_1: 0.,
                in_state_2: 0.,
                out_state_1: 0.,
                out_state_2: 0.,
            }; 8],
        }; 8],
        audio_delay: ia_audio_delay_struct {
            delay: 0,
            state: [[0.; 189]; 8],
        },
    }; 16];
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*str_filter_block).filter_element_count {
        if (*unique_td_filt_ele
            .offset(
                (*str_filter_block).str_filter_element[i as usize].filt_ele_idx as isize,
            ))
            .eq_filter_format == FILTER_ELEMENT_FORMAT_FIR
            && (*str_filter_block).filter_element_count > 1 as core::ffi::c_int
        {
            return -(1 as WORD32);
        }
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*str_filter_block).filter_element_count {
        let mut eq_filt_element: *mut ia_eq_filt_ele_struct = &mut *((*pstr_eq_filt_block)
            .eq_filt_element)
            .as_mut_ptr()
            .offset(i as isize) as *mut ia_eq_filt_ele_struct;
        let mut str_filter_element: *mut ia_filt_ele_struct = &mut *((*str_filter_block)
            .str_filter_element)
            .as_mut_ptr()
            .offset(i as isize) as *mut ia_filt_ele_struct;
        let mut filterIndex: WORD32 = (*str_filter_element).filt_ele_idx;
        if (*unique_td_filt_ele.offset(filterIndex as isize)).eq_filter_format
            == FILTER_ELEMENT_FORMAT_POLE_ZERO
        {
            err = impd_calc_filt_params_all(
                &mut *unique_td_filt_ele.offset(filterIndex as isize),
                &mut interm_filt_params,
            );
            if err != 0 {
                return err;
            }
            impd_calc_eq_filt_elements(&mut interm_filt_params, eq_filt_element);
            (*eq_filt_element).format = FILTER_ELEMENT_FORMAT_POLE_ZERO as WORD32;
        } else {
            impd_convert_fir_filt_params(
                (*unique_td_filt_ele.offset(filterIndex as isize)).fir_filt_order,
                (*unique_td_filt_ele.offset(filterIndex as isize)).fir_symmetry,
                ((*unique_td_filt_ele.offset(filterIndex as isize)).fir_coeff)
                    .as_mut_ptr(),
                &mut (*eq_filt_element).fir_filter,
            );
            (*eq_filt_element).format = FILTER_ELEMENT_FORMAT_FIR as WORD32;
        }
        if (*str_filter_element).filt_ele_gain_flag == 1 as core::ffi::c_int {
            (*eq_filt_element).elementGainLinear = pow(
                10.0f64,
                (0.05f32 * (*str_filter_element).filt_ele_gain) as core::ffi::c_double,
            ) as FLOAT32;
        } else {
            (*eq_filt_element).elementGainLinear = 1.0f32 as FLOAT32;
        }
        k = 0 as core::ffi::c_int as WORD32;
        while k < (*unique_td_filt_ele.offset(filterIndex as isize)).real_zero_count {
            if (*unique_td_filt_ele.offset(filterIndex as isize))
                .real_zero_radius[k as usize] > 0.0f32
            {
                (*eq_filt_element).elementGainLinear = -(*eq_filt_element)
                    .elementGainLinear;
            }
            k += 1;
        }
        impd_calc_phase_filt(
            &mut *unique_td_filt_ele.offset(filterIndex as isize),
            i,
            &mut *matching_ph_filt.as_mut_ptr().offset(i as isize),
        );
        i += 1;
    }
    (*pstr_eq_filt_block).element_count = (*str_filter_block).filter_element_count;
    (*pstr_eq_filt_block).matching_ph_filt_ele_0 = matching_ph_filt[0 as core::ffi::c_int
        as usize];
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_calc_cascade_phase_align_filt(
    mut str_td_filter_cascade: *mut ia_td_filter_cascade_struct,
    mut ch_group_cnt: WORD32,
) -> VOID {
    let mut cascade_align_grp_cnt: WORD32 = 0 as WORD32;
    let mut pstr_cascade_align_grp: [ia_cascade_align_group_struct; 2] = [ia_cascade_align_group_struct {
        member_count: 0,
        member_idx: [0; 4],
    }; 2];
    impd_calc_cascade_align_groups(
        ch_group_cnt,
        (*str_td_filter_cascade).eq_phase_alignment_present,
        ((*str_td_filter_cascade).eq_phase_alignment).as_mut_ptr(),
        &mut cascade_align_grp_cnt,
        pstr_cascade_align_grp.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn impd_calc_filt_cascade(
    mut unique_td_filt_ele: *mut ia_unique_td_filt_element,
    mut str_filter_block: *mut ia_filt_block_struct,
    mut str_td_filter_cascade: *mut ia_td_filter_cascade_struct,
    mut ch_group_cnt: WORD32,
    mut filt_cascade_td: *mut ia_filt_cascade_td_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut err: WORD32 = 0;
    let mut g: WORD32 = 0;
    g = 0 as core::ffi::c_int as WORD32;
    while g < ch_group_cnt {
        i = 0 as core::ffi::c_int as WORD32;
        while i
            < (*str_td_filter_cascade)
                .str_filter_block_refs[g as usize]
                .filter_block_count
        {
            err = impd_calc_filt_block(
                unique_td_filt_ele,
                &mut *str_filter_block
                    .offset(
                        *((*((*str_td_filter_cascade).str_filter_block_refs)
                            .as_mut_ptr()
                            .offset(g as isize))
                            .filter_block_index)
                            .as_mut_ptr()
                            .offset(i as isize) as isize,
                    ),
                &mut *((*filt_cascade_td.offset(g as isize)).pstr_eq_filt_block)
                    .as_mut_ptr()
                    .offset(i as isize),
            );
            if err != 0 {
                return err;
            }
            i += 1;
        }
        (*filt_cascade_td.offset(g as isize)).block_count = i;
        (*filt_cascade_td.offset(g as isize)).cascade_gain_linear = pow(
            10.0f64,
            (0.05f32 * (*str_td_filter_cascade).eq_cascade_gain[g as usize])
                as core::ffi::c_double,
        ) as FLOAT32;
        g += 1;
    }
    impd_calc_cascade_phase_align_filt(str_td_filter_cascade, ch_group_cnt);
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_calc_subband_eq(
    mut str_eq_subband_gain_vector: *mut ia_eq_subband_gain_vector,
    mut eq_subband_gain_count: WORD32,
    mut subband_filt: *mut ia_subband_filt_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < eq_subband_gain_count {
        (*subband_filt).subband_coeff[i as usize] = (*str_eq_subband_gain_vector)
            .eq_subband_gain[i as usize];
        i += 1;
    }
    (*subband_filt).coeff_count = eq_subband_gain_count;
}
#[no_mangle]
pub unsafe extern "C" fn impd_decode_eq_node_freq(
    mut eq_node_freq_idx: WORD32,
) -> FLOAT32 {
    let mut step_ratio: FLOAT32 = 0.0739601809794f32;
    return pow(
        STEP_RATIO_F_LO as core::ffi::c_double,
        (1.0f32 + eq_node_freq_idx as FLOAT32 * step_ratio) as core::ffi::c_double,
    ) as FLOAT32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_calc_warp_freq_delta(
    mut fsubband: FLOAT32,
    mut node_freq: FLOAT32,
    mut eq_node_freq_idx: WORD32,
) -> FLOAT32 {
    let mut step_ratio: FLOAT32 = 0.0739601809794f32;
    return ((log10(fsubband as core::ffi::c_double)
        / log10(node_freq as core::ffi::c_double) - 1.0f64)
        / step_ratio as core::ffi::c_double
        - eq_node_freq_idx as FLOAT32 as core::ffi::c_double) as FLOAT32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_interpolate_eq_gain(
    mut band_step: WORD32,
    mut left_gain: FLOAT32,
    mut right_gain: FLOAT32,
    mut left_slope: FLOAT32,
    mut right_slope: FLOAT32,
    mut f: FLOAT32,
    mut interpolated_gain: *mut FLOAT32,
) -> VOID {
    let mut k1: FLOAT32 = 0.;
    let mut k2: FLOAT32 = 0.;
    let mut a: FLOAT32 = 0.;
    let mut b: FLOAT32 = 0.;
    let mut c: FLOAT32 = 0.;
    let mut d: FLOAT32 = 0.;
    let mut inv_band_step: FLOAT32 = (1.0f64
        / band_step as FLOAT32 as core::ffi::c_double) as FLOAT32;
    let mut inv_band_step_sqr: FLOAT32 = inv_band_step * inv_band_step;
    k1 = (right_gain - left_gain) * inv_band_step_sqr;
    left_slope = left_slope as core::ffi::c_float / 3.128f32;
    right_slope = right_slope as core::ffi::c_float / 3.128f32;
    k2 = right_slope + left_slope;
    a = inv_band_step * (inv_band_step * k2 - 2.0f32 * k1);
    b = 3.0f32 * k1 - inv_band_step * (k2 + left_slope);
    c = left_slope;
    d = left_gain;
    *interpolated_gain = ((a * f + b) * f + c) * f + d;
}
#[no_mangle]
pub unsafe extern "C" fn impd_interpolate_subband_spline(
    mut str_eq_subband_gain_spline: *mut ia_eq_subband_gain_spline_struct,
    mut eq_subband_gain_count: WORD32,
    mut eq_subband_gain_format: WORD32,
    mut sample_rate: FLOAT32,
    mut subband_filt: *mut ia_subband_filt_struct,
) -> WORD32 {
    let mut b: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut err: WORD32 = 0;
    let mut eq_gain: [FLOAT32; 32] = [0.; 32];
    let mut eq_node_freq_idx: [WORD32; 32] = [0; 32];
    let mut eq_node_freq: [FLOAT32; 32] = [0.; 32];
    let mut subband_center_freq: [FLOAT32; 256] = [0.; 256];
    let mut num_eq_nodes: WORD32 = (*str_eq_subband_gain_spline).num_eq_nodes;
    let mut eq_slope: *mut FLOAT32 = ((*str_eq_subband_gain_spline).eq_slope)
        .as_mut_ptr();
    let mut eq_freq_delta: *mut WORD32 = ((*str_eq_subband_gain_spline).eq_freq_delta)
        .as_mut_ptr();
    let mut eq_gain_initial: FLOAT32 = (*str_eq_subband_gain_spline).eq_gain_initial;
    let mut eq_gain_delta: *mut FLOAT32 = ((*str_eq_subband_gain_spline).eq_gain_delta)
        .as_mut_ptr();
    let mut subband_coeff: *mut FLOAT32 = ((*subband_filt).subband_coeff).as_mut_ptr();
    let mut max_eq_node_idx: WORD32 = 32 as WORD32;
    eq_gain[0 as core::ffi::c_int as usize] = eq_gain_initial;
    eq_node_freq_idx[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as WORD32;
    eq_node_freq[0 as core::ffi::c_int as usize] = impd_decode_eq_node_freq(
        eq_node_freq_idx[0 as core::ffi::c_int as usize],
    );
    n = 1 as core::ffi::c_int as WORD32;
    while n < num_eq_nodes {
        eq_gain[n as usize] = eq_gain[(n as core::ffi::c_int - 1 as core::ffi::c_int)
            as usize] + *eq_gain_delta.offset(n as isize);
        eq_node_freq_idx[n as usize] = eq_node_freq_idx[(n as core::ffi::c_int
            - 1 as core::ffi::c_int) as usize] + *eq_freq_delta.offset(n as isize);
        eq_node_freq[n as usize] = impd_decode_eq_node_freq(
            eq_node_freq_idx[n as usize],
        );
        n += 1;
    }
    if eq_node_freq[(num_eq_nodes as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
        < sample_rate as core::ffi::c_float * 0.5f32
        && eq_node_freq_idx[(num_eq_nodes as core::ffi::c_int - 1 as core::ffi::c_int)
            as usize] < max_eq_node_idx
    {
        *eq_slope.offset(num_eq_nodes as isize) = 0 as core::ffi::c_int as FLOAT32;
        eq_gain[num_eq_nodes as usize] = eq_gain[(num_eq_nodes as core::ffi::c_int
            - 1 as core::ffi::c_int) as usize];
        *eq_freq_delta.offset(num_eq_nodes as isize) = max_eq_node_idx
            - eq_node_freq_idx[(num_eq_nodes as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize];
        eq_node_freq_idx[num_eq_nodes as usize] = max_eq_node_idx;
        eq_node_freq[num_eq_nodes as usize] = impd_decode_eq_node_freq(
            eq_node_freq_idx[num_eq_nodes as usize],
        );
        num_eq_nodes += 1 as core::ffi::c_int;
    }
    err = impd_derive_subband_center_freq(
        eq_subband_gain_count,
        eq_subband_gain_format,
        sample_rate,
        subband_center_freq.as_mut_ptr(),
    );
    if err != 0 {
        return err;
    }
    n = 0 as core::ffi::c_int as WORD32;
    while n < num_eq_nodes as core::ffi::c_int - 1 as core::ffi::c_int {
        b = 0 as core::ffi::c_int as WORD32;
        while b < eq_subband_gain_count {
            let mut fSub: FLOAT32 = 0.;
            fSub = if subband_center_freq[b as usize]
                > eq_node_freq[0 as core::ffi::c_int as usize]
            {
                subband_center_freq[b as usize]
            } else {
                eq_node_freq[0 as core::ffi::c_int as usize]
            };
            fSub = if fSub
                < eq_node_freq[(num_eq_nodes as core::ffi::c_int - 1 as core::ffi::c_int)
                    as usize]
            {
                fSub
            } else {
                eq_node_freq[(num_eq_nodes as core::ffi::c_int - 1 as core::ffi::c_int)
                    as usize]
            };
            if fSub >= eq_node_freq[n as usize]
                && fSub
                    <= eq_node_freq[(n as core::ffi::c_int + 1 as core::ffi::c_int)
                        as usize]
            {
                let mut warpedDeltaFreq: FLOAT32 = impd_calc_warp_freq_delta(
                    fSub,
                    eq_node_freq[0 as core::ffi::c_int as usize],
                    eq_node_freq_idx[n as usize],
                );
                let mut gEqSubbandDb: FLOAT32 = 0.;
                impd_interpolate_eq_gain(
                    *eq_freq_delta
                        .offset(
                            (n as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                        ),
                    eq_gain[n as usize],
                    eq_gain[(n as core::ffi::c_int + 1 as core::ffi::c_int) as usize],
                    *eq_slope.offset(n as isize),
                    *eq_slope
                        .offset(
                            (n as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                        ),
                    warpedDeltaFreq,
                    &mut gEqSubbandDb,
                );
                *subband_coeff.offset(b as isize) = pow(
                    2.0f64,
                    (gEqSubbandDb as core::ffi::c_float / 6.0f32) as core::ffi::c_double,
                ) as FLOAT32;
            }
            b += 1;
        }
        n += 1;
    }
    (*subband_filt).coeff_count = eq_subband_gain_count;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_calc_subband_gains(
    mut str_eq_coeff: *mut ia_eq_coeff_struct,
    mut eq_ch_group_count: WORD32,
    mut subband_gains_index: *mut WORD32,
    mut sample_rate: FLOAT32,
    mut eq_frame_size_subband: WORD32,
    mut subband_filt: *mut ia_subband_filt_struct,
) -> WORD32 {
    let mut g: WORD32 = 0;
    let mut err: WORD32 = 0;
    let mut eq_subband_gain_representation: WORD32 = (*str_eq_coeff)
        .eq_subband_gain_representation;
    let mut eq_subband_gain_count: WORD32 = (*str_eq_coeff).eq_subband_gain_count;
    let mut eq_subband_gain_format: WORD32 = (*str_eq_coeff).eq_subband_gain_format;
    g = 0 as core::ffi::c_int as WORD32;
    while g < eq_ch_group_count {
        if eq_subband_gain_representation == 1 as core::ffi::c_int {
            err = impd_interpolate_subband_spline(
                &mut *((*str_eq_coeff).str_eq_subband_gain_spline)
                    .as_mut_ptr()
                    .offset(*subband_gains_index.offset(g as isize) as isize),
                eq_subband_gain_count,
                eq_subband_gain_format,
                sample_rate,
                &mut *subband_filt.offset(g as isize),
            );
            if err != 0 {
                return err;
            }
        } else {
            impd_calc_subband_eq(
                &mut *((*str_eq_coeff).str_eq_subband_gain_vector)
                    .as_mut_ptr()
                    .offset(*subband_gains_index.offset(g as isize) as isize),
                eq_subband_gain_count,
                &mut *subband_filt.offset(g as isize),
            );
        }
        (*subband_filt.offset(g as isize)).eq_frame_size_subband = eq_frame_size_subband;
        g += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_calc_filt_sect_delay(
    mut section_count: WORD32,
    mut filt_section: *mut ia_filt_sect_struct,
    mut delay: *mut FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut d: FLOAT32 = 0.0f32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < section_count {
        if (*filt_section.offset(i as isize)).b2 != 0.0f32 {
            d += 1.0f32;
        } else if (*filt_section.offset(i as isize)).b1 != 0.0f32 {
            d += 0.5f32;
        }
        i += 1;
    }
    *delay = d;
}
#[no_mangle]
pub unsafe extern "C" fn impd_get_eq_set_delay(
    mut eq_set: *mut ia_eq_set_struct,
    mut cascade_delay: *mut WORD32,
) -> VOID {
    let mut delay: FLOAT32 = 0.;
    let mut sect_delay: FLOAT32 = 0.;
    let mut k: WORD32 = 0;
    let mut g: WORD32 = 0;
    let mut b: WORD32 = 0;
    delay = 0 as core::ffi::c_int as FLOAT32;
    g = (*eq_set).eq_ch_group_of_channel[0 as core::ffi::c_int as usize];
    if g >= 0 as core::ffi::c_int {
        match (*eq_set).domain {
            EQ_FILTER_DOMAIN_TIME => {
                let mut filt_cascade_td: *mut ia_filt_cascade_td_struct = &mut *((*eq_set)
                    .filt_cascade_td)
                    .as_mut_ptr()
                    .offset(g as isize) as *mut ia_filt_cascade_td_struct;
                b = 0 as core::ffi::c_int as WORD32;
                while b < (*filt_cascade_td).block_count {
                    let mut eq_filt_element: *mut ia_eq_filt_ele_struct = &mut *((*((*filt_cascade_td)
                        .pstr_eq_filt_block)
                        .as_mut_ptr()
                        .offset(b as isize))
                        .eq_filt_element)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize)
                        as *mut ia_eq_filt_ele_struct;
                    match (*eq_filt_element).format {
                        FILTER_ELEMENT_FORMAT_POLE_ZERO => {
                            impd_calc_filt_sect_delay(
                                (*eq_filt_element).pstr_pole_zero_filt.section_count,
                                ((*eq_filt_element).pstr_pole_zero_filt.filt_section)
                                    .as_mut_ptr(),
                                &mut sect_delay,
                            );
                            delay += sect_delay;
                            if (*eq_filt_element).pstr_pole_zero_filt.filt_coeffs_flag
                                != 0
                            {
                                delay
                                    += 0.5f32
                                        * ((*eq_filt_element)
                                            .pstr_pole_zero_filt
                                            .fir_filter
                                            .coeff_count as core::ffi::c_int - 1 as core::ffi::c_int)
                                            as core::ffi::c_float;
                            }
                        }
                        FILTER_ELEMENT_FORMAT_FIR => {
                            delay
                                += 0.5f32
                                    * ((*eq_filt_element).fir_filter.coeff_count
                                        as core::ffi::c_int - 1 as core::ffi::c_int)
                                        as core::ffi::c_float;
                        }
                        _ => {}
                    }
                    k = 0 as core::ffi::c_int as WORD32;
                    while k < (*eq_filt_element).num_ph_align_filt {
                        let mut ph_alignment_filt: *mut ia_ph_alignment_filt_struct = &mut *((*eq_filt_element)
                            .ph_alignment_filt)
                            .as_mut_ptr()
                            .offset(k as isize) as *mut ia_ph_alignment_filt_struct;
                        impd_calc_filt_sect_delay(
                            (*ph_alignment_filt).section_count,
                            ((*ph_alignment_filt).filt_section).as_mut_ptr(),
                            &mut sect_delay,
                        );
                        delay += sect_delay;
                        k += 1;
                    }
                    b += 1;
                }
                b = 0 as core::ffi::c_int as WORD32;
                while b < (*filt_cascade_td).num_ph_align_filt {
                    let mut ph_alignment_filt_0: *mut ia_ph_alignment_filt_struct = &mut *((*filt_cascade_td)
                        .ph_alignment_filt)
                        .as_mut_ptr()
                        .offset(b as isize) as *mut ia_ph_alignment_filt_struct;
                    impd_calc_filt_sect_delay(
                        (*ph_alignment_filt_0).section_count,
                        ((*ph_alignment_filt_0).filt_section).as_mut_ptr(),
                        &mut sect_delay,
                    );
                    delay += sect_delay;
                    b += 1;
                }
            }
            EQ_FILTER_DOMAIN_SUBBAND | EQ_FILTER_DOMAIN_NONE | _ => {}
        }
    }
    *cascade_delay = delay as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_derive_eq_set(
    mut str_eq_coeff: *mut ia_eq_coeff_struct,
    mut str_eq_instructions: *mut ia_eq_instructions_struct,
    mut sample_rate: FLOAT32,
    mut drc_frame_size: WORD32,
    mut sub_band_domain_mode: WORD32,
    mut eq_set: *mut ia_eq_set_struct,
) -> WORD32 {
    let mut err: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut eq_frame_size_subband: WORD32 = 0;
    (*eq_set).domain = EQ_FILTER_DOMAIN_NONE as WORD32;
    if sub_band_domain_mode == SUBBAND_DOMAIN_MODE_OFF {
        if (*str_eq_instructions).td_filter_cascade_present == 1 as core::ffi::c_int {
            err = impd_calc_filt_cascade(
                ((*str_eq_coeff).unique_td_filt_ele).as_mut_ptr(),
                ((*str_eq_coeff).str_filter_block).as_mut_ptr(),
                &mut (*str_eq_instructions).str_td_filter_cascade,
                (*str_eq_instructions).eq_ch_group_count,
                ((*eq_set).filt_cascade_td).as_mut_ptr(),
            );
            if err != 0 {
                return err;
            }
        }
        (*eq_set).domain |= EQ_FILTER_DOMAIN_TIME;
    }
    if sub_band_domain_mode != SUBBAND_DOMAIN_MODE_OFF {
        match sub_band_domain_mode {
            SUBBAND_DOMAIN_MODE_QMF64 => {
                if (*str_eq_coeff).eq_subband_gain_count
                    != AUDIO_CODEC_SUBBAND_COUNT_QMF64
                {
                    return -(1 as WORD32);
                }
                eq_frame_size_subband = (drc_frame_size as core::ffi::c_int
                    / AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_QMF64) as WORD32;
            }
            SUBBAND_DOMAIN_MODE_QMF71 => {
                if (*str_eq_coeff).eq_subband_gain_count
                    != AUDIO_CODEC_SUBBAND_COUNT_QMF71
                {
                    return -(1 as WORD32);
                }
                eq_frame_size_subband = (drc_frame_size as core::ffi::c_int
                    / AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_QMF71) as WORD32;
            }
            SUBBAND_DOMAIN_MODE_STFT256 => {
                if (*str_eq_coeff).eq_subband_gain_count
                    != AUDIO_CODEC_SUBBAND_COUNT_STFT256
                {
                    return -(1 as WORD32);
                }
                eq_frame_size_subband = (drc_frame_size as core::ffi::c_int
                    / AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_STFT256) as WORD32;
            }
            _ => return -(1 as WORD32),
        }
        if (*str_eq_instructions).subband_gains_present == 1 as core::ffi::c_int {
            err = impd_calc_subband_gains(
                str_eq_coeff,
                (*str_eq_instructions).eq_ch_group_count,
                ((*str_eq_instructions).subband_gains_index).as_mut_ptr(),
                sample_rate,
                eq_frame_size_subband,
                ((*eq_set).subband_filt).as_mut_ptr(),
            );
            if err != 0 {
                return err;
            }
        } else if (*str_eq_instructions).td_filter_cascade_present
            == 1 as core::ffi::c_int
        {
            err = impd_calc_subband_gains_td_cascade(
                ((*str_eq_coeff).unique_td_filt_ele).as_mut_ptr(),
                ((*str_eq_coeff).str_filter_block).as_mut_ptr(),
                &mut (*str_eq_instructions).str_td_filter_cascade,
                (*str_eq_coeff).eq_subband_gain_format,
                (*str_eq_instructions).eq_ch_group_count,
                sample_rate,
                eq_frame_size_subband,
                ((*eq_set).subband_filt).as_mut_ptr(),
            );
            if err != 0 {
                return err;
            }
        }
        (*eq_set).domain |= EQ_FILTER_DOMAIN_SUBBAND;
    }
    (*eq_set).audio_num_chan = (*str_eq_instructions).eq_channel_count;
    (*eq_set).eq_ch_group_count = (*str_eq_instructions).eq_ch_group_count;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*str_eq_instructions).eq_channel_count {
        (*eq_set).eq_ch_group_of_channel[i as usize] = (*str_eq_instructions)
            .eq_ch_group_of_channel[i as usize];
        i += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_process_filt_sect(
    mut filt_section: *mut ia_filt_sect_struct,
    mut channel: WORD32,
    mut audio_out: *mut FLOAT32,
    mut section_count: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < section_count {
        let mut filt_sect_state: *mut ia_filt_sect_state_struct = &mut *((*filt_section
            .offset(i as isize))
            .filt_sect_state)
            .as_mut_ptr()
            .offset(channel as isize) as *mut ia_filt_sect_state_struct;
        let mut audio_in: FLOAT32 = *audio_out;
        *audio_out = audio_in
            + (*filt_section.offset(i as isize)).b1 * (*filt_sect_state).in_state_1
            + (*filt_section.offset(i as isize)).b2 * (*filt_sect_state).in_state_2
            - (*filt_section.offset(i as isize)).a1 * (*filt_sect_state).out_state_1
            - (*filt_section.offset(i as isize)).a2 * (*filt_sect_state).out_state_2;
        (*filt_sect_state).in_state_2 = (*filt_sect_state).in_state_1;
        (*filt_sect_state).in_state_1 = audio_in;
        (*filt_sect_state).out_state_2 = (*filt_sect_state).out_state_1;
        (*filt_sect_state).out_state_1 = *audio_out;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn impd_fir_filt_process(
    mut fir_filter: *mut ia_fir_filter_struct,
    mut channel: WORD32,
    mut audio_in: FLOAT32,
    mut audio_out: *mut FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut coeff: *mut FLOAT32 = ((*fir_filter).coeff).as_mut_ptr();
    let mut state: *mut FLOAT32 = ((*fir_filter).state[channel as usize]).as_mut_ptr();
    let mut sum: FLOAT32 = 0.;
    sum = *coeff.offset(0 as core::ffi::c_int as isize) * audio_in;
    i = 1 as core::ffi::c_int as WORD32;
    while i < (*fir_filter).coeff_count {
        sum
            += *coeff.offset(i as isize)
                * *state
                    .offset((i as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
        i += 1;
    }
    *audio_out = sum;
    i = ((*fir_filter).coeff_count as core::ffi::c_int - 2 as core::ffi::c_int)
        as WORD32;
    while i > 0 as core::ffi::c_int {
        *state.offset(i as isize) = *state
            .offset((i as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
        i -= 1;
    }
    *state.offset(0 as core::ffi::c_int as isize) = audio_in;
}
#[no_mangle]
pub unsafe extern "C" fn impd_audio_delay_process(
    mut audio_delay: *mut ia_audio_delay_struct,
    mut channel: WORD32,
    mut audio_in: FLOAT32,
    mut ptr_audio_out: *mut FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut state: *mut FLOAT32 = ((*audio_delay).state[channel as usize]).as_mut_ptr();
    if (*audio_delay).delay > 0 as core::ffi::c_int {
        *ptr_audio_out = *state
            .offset(
                ((*audio_delay).delay as core::ffi::c_int - 1 as core::ffi::c_int)
                    as isize,
            );
        i = ((*audio_delay).delay as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        while i > 0 as core::ffi::c_int {
            *state.offset(i as isize) = *state
                .offset((i as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
            i -= 1;
        }
        *state.offset(0 as core::ffi::c_int as isize) = audio_in;
    } else {
        *ptr_audio_out = audio_in;
    };
}
#[no_mangle]
pub unsafe extern "C" fn impd_pole_zero_filt_process(
    mut pstr_pole_zero_filt: *mut ia_pole_zero_filt_struct,
    mut channel: WORD32,
    mut audio_in: FLOAT32,
    mut ptr_audio_out: *mut FLOAT32,
) -> VOID {
    let mut inp: FLOAT32 = audio_in;
    let mut out: FLOAT32 = inp;
    impd_process_filt_sect(
        ((*pstr_pole_zero_filt).filt_section).as_mut_ptr(),
        channel,
        &mut out,
        (*pstr_pole_zero_filt).section_count,
    );
    inp = out;
    if (*pstr_pole_zero_filt).filt_coeffs_flag == 1 as core::ffi::c_int {
        impd_fir_filt_process(
            &mut (*pstr_pole_zero_filt).fir_filter,
            channel,
            inp,
            &mut out,
        );
        inp = out;
    }
    impd_audio_delay_process(
        &mut (*pstr_pole_zero_filt).audio_delay,
        channel,
        inp,
        &mut out,
    );
    *ptr_audio_out = out;
}
#[no_mangle]
pub unsafe extern "C" fn impd_phase_align_filt_process(
    mut ph_alignment_filt: *mut ia_ph_alignment_filt_struct,
    mut channel: WORD32,
    mut ptr_audio_out: *mut FLOAT32,
) -> VOID {
    let mut audio_in: FLOAT32 = *ptr_audio_out;
    let mut inp: FLOAT32 = audio_in;
    let mut out: FLOAT32 = inp;
    impd_process_filt_sect(
        ((*ph_alignment_filt).filt_section).as_mut_ptr(),
        channel,
        &mut out,
        (*ph_alignment_filt).section_count,
    );
    inp = out;
    impd_audio_delay_process(
        &mut (*ph_alignment_filt).audio_delay,
        channel,
        inp,
        &mut out,
    );
    *ptr_audio_out = out * (*ph_alignment_filt).gain;
}
#[no_mangle]
pub unsafe extern "C" fn impd_eq_filt_element_process(
    mut str_eq_filt_block: *mut ia_eq_filt_block_struct,
    mut channel: WORD32,
    mut audio_in: FLOAT32,
    mut ptr_audio_out: *mut FLOAT32,
    mut block_count: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut inp: FLOAT32 = audio_in;
    let mut out: FLOAT32 = inp;
    let mut k: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut element_count: WORD32 = 0;
    j = 0 as core::ffi::c_int as WORD32;
    while j < block_count {
        let mut sum: FLOAT32 = 0.0f32;
        element_count = (*str_eq_filt_block.offset(j as isize)).element_count;
        k = 0 as core::ffi::c_int as WORD32;
        while k < element_count {
            match (*str_eq_filt_block.offset(j as isize))
                .eq_filt_element[k as usize]
                .format
            {
                FILTER_ELEMENT_FORMAT_POLE_ZERO => {
                    impd_pole_zero_filt_process(
                        &mut (*((*str_eq_filt_block.offset(j as isize)).eq_filt_element)
                            .as_mut_ptr()
                            .offset(k as isize))
                            .pstr_pole_zero_filt,
                        channel,
                        inp,
                        &mut out,
                    );
                }
                FILTER_ELEMENT_FORMAT_FIR => {
                    impd_fir_filt_process(
                        &mut (*((*str_eq_filt_block.offset(j as isize)).eq_filt_element)
                            .as_mut_ptr()
                            .offset(k as isize))
                            .fir_filter,
                        channel,
                        inp,
                        &mut out,
                    );
                }
                _ => {}
            }
            out
                *= (*str_eq_filt_block.offset(j as isize))
                    .eq_filt_element[k as usize]
                    .elementGainLinear;
            i = 0 as core::ffi::c_int as WORD32;
            while i
                < (*str_eq_filt_block.offset(j as isize))
                    .eq_filt_element[k as usize]
                    .num_ph_align_filt
            {
                inp = out;
                impd_phase_align_filt_process(
                    &mut *((*((*str_eq_filt_block.offset(j as isize)).eq_filt_element)
                        .as_mut_ptr()
                        .offset(k as isize))
                        .ph_alignment_filt)
                        .as_mut_ptr()
                        .offset(i as isize),
                    channel,
                    &mut out,
                );
                i += 1;
            }
            sum += out;
            k += 1;
        }
        inp = sum;
        j += 1;
    }
    *ptr_audio_out = inp;
}
#[no_mangle]
pub unsafe extern "C" fn impd_process_eq_set_time_domain(
    mut pstr_eq_set: *mut ia_eq_set_struct,
    mut channel: WORD32,
    mut ptr_audio_in: *mut FLOAT32,
    mut ptr_audio_out: *mut FLOAT32,
    mut frame_size: WORD32,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut g: WORD32 = 0 as WORD32;
    if pstr_eq_set.is_null() {
        return 0 as WORD32;
    }
    g = (*pstr_eq_set).eq_ch_group_of_channel[channel as usize];
    if g < 0 as core::ffi::c_int {
        return 0 as WORD32;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < frame_size {
        impd_eq_filt_element_process(
            ((*pstr_eq_set).filt_cascade_td[g as usize].pstr_eq_filt_block).as_mut_ptr(),
            channel,
            *ptr_audio_in.offset(i as isize),
            &mut *ptr_audio_out.offset(i as isize),
            (*pstr_eq_set).filt_cascade_td[g as usize].block_count,
        );
        j = 0 as core::ffi::c_int as WORD32;
        while j < (*pstr_eq_set).filt_cascade_td[g as usize].num_ph_align_filt {
            impd_phase_align_filt_process(
                &mut *((*((*pstr_eq_set).filt_cascade_td)
                    .as_mut_ptr()
                    .offset(g as isize))
                    .ph_alignment_filt)
                    .as_mut_ptr()
                    .offset(j as isize),
                channel,
                &mut *ptr_audio_out.offset(i as isize),
            );
            j += 1;
        }
        *ptr_audio_out.offset(i as isize) = *ptr_audio_out.offset(i as isize)
            * (*pstr_eq_set).filt_cascade_td[g as usize].cascade_gain_linear;
        i += 1;
    }
    return 0 as WORD32;
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
