extern "C" {
    fn cos(__x: core::ffi::c_double) -> core::ffi::c_double;
    static shape_filt_lf_y1_bound_tbl: [[FLOAT32; 3]; 0];
    static shape_filt_hf_y1_bound_tbl: [[FLOAT32; 3]; 0];
    static shape_filt_lf_gain_offset_tbl: [[FLOAT32; 3]; 0];
    static shape_filt_hf_gain_offset_tbl: [[FLOAT32; 3]; 0];
    static shape_filt_lf_radius_tbl: [FLOAT32; 0];
    static shape_filt_hf_radius_tbl: [FLOAT32; 0];
    static shape_filt_cutoff_freq_norm_hf_tbl: [FLOAT32; 0];
}
pub type WORD32 = core::ffi::c_int;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_shape_filter_struct {
    pub type_0: WORD32,
    pub gain_offset: FLOAT32,
    pub y1_bound: FLOAT32,
    pub warped_gain_max: FLOAT32,
    pub factor: FLOAT32,
    pub coeff_sum: FLOAT32,
    pub partial_coeff_sum: FLOAT32,
    pub g_norm: FLOAT32,
    pub a1: FLOAT32,
    pub a2: FLOAT32,
    pub b1: FLOAT32,
    pub b2: FLOAT32,
    pub audio_in_state_1: [FLOAT32; 8],
    pub audio_in_state_2: [FLOAT32; 8],
    pub audio_out_state_1: [FLOAT32; 8],
    pub audio_out_state_2: [FLOAT32; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shape_filter_block {
    pub shape_flter_block_flag: WORD32,
    pub drc_gain_last: FLOAT32,
    pub shape_filter: [ia_shape_filter_struct; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_shape_filter_params_struct {
    pub corner_freq_index: WORD32,
    pub filter_strength_index: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_shape_filter_block_params_struct {
    pub lf_cut_filter_present: WORD32,
    pub str_lf_cut_params: ia_shape_filter_params_struct,
    pub lf_boost_filter_present: WORD32,
    pub str_lf_boost_params: ia_shape_filter_params_struct,
    pub hf_cut_filter_present: WORD32,
    pub str_hfCutParams: ia_shape_filter_params_struct,
    pub hf_boost_filter_present: WORD32,
    pub str_hf_boost_params: ia_shape_filter_params_struct,
}
pub const M_PI: core::ffi::c_double = 3.14159265358979323846f64;
pub const MAX_CHANNEL_COUNT: core::ffi::c_int = 8 as core::ffi::c_int;
pub const SHAPE_FILTER_TYPE_OFF: core::ffi::c_int = 0 as core::ffi::c_int;
pub const SHAPE_FILTER_TYPE_LF_CUT: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SHAPE_FILTER_TYPE_LF_BOOST: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SHAPE_FILTER_TYPE_HF_CUT: core::ffi::c_int = 3 as core::ffi::c_int;
pub const SHAPE_FILTER_TYPE_HF_BOOST: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SHAPE_FILTER_DRC_GAIN_MAX_MINUS_ONE: core::ffi::c_float = 1583.8931924611f32;
#[no_mangle]
pub unsafe extern "C" fn impd_shape_filt_block_adapt(
    drc_gain: FLOAT32,
    mut shape_filter_block: *mut shape_filter_block,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut warped_gain: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
    let mut x1: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
    let mut y1: FLOAT32 = 0.;
    (*shape_filter_block).drc_gain_last = drc_gain;
    i = 0 as core::ffi::c_int as WORD32;
    while i < 4 as core::ffi::c_int {
        if !((*shape_filter_block).shape_filter[i as usize].type_0
            == SHAPE_FILTER_TYPE_OFF)
        {
            if (*shape_filter_block).shape_filter[i as usize].type_0
                == SHAPE_FILTER_TYPE_LF_CUT
                || (*shape_filter_block).shape_filter[i as usize].type_0
                    == SHAPE_FILTER_TYPE_HF_CUT
            {
                if drc_gain < 1.0f32 {
                    warped_gain = -1.0f32 as FLOAT32;
                } else {
                    warped_gain = (drc_gain - 1.0f32)
                        / (drc_gain - 1.0f32
                            + (*shape_filter_block)
                                .shape_filter[i as usize]
                                .gain_offset);
                }
                x1 = (*shape_filter_block).shape_filter[i as usize].a1;
            } else if (*shape_filter_block).shape_filter[i as usize].type_0
                == SHAPE_FILTER_TYPE_LF_BOOST
                || (*shape_filter_block).shape_filter[i as usize].type_0
                    == SHAPE_FILTER_TYPE_HF_BOOST
            {
                if drc_gain >= 1.0f32 {
                    warped_gain = -1.0f32 as FLOAT32;
                } else {
                    warped_gain = ((1.0f32 - drc_gain as core::ffi::c_float)
                        / (1.0f32
                            + drc_gain as core::ffi::c_float
                                * ((*shape_filter_block)
                                    .shape_filter[i as usize]
                                    .gain_offset as core::ffi::c_float - 1.0f32))) as FLOAT32;
                }
                x1 = (*shape_filter_block).shape_filter[i as usize].b1;
            }
            if warped_gain <= 0.0f32 {
                y1 = x1;
            } else if warped_gain
                < (*shape_filter_block).shape_filter[i as usize].warped_gain_max
            {
                y1 = x1
                    + (*shape_filter_block).shape_filter[i as usize].factor
                        * warped_gain;
            } else {
                y1 = (*shape_filter_block).shape_filter[i as usize].y1_bound;
            }
            if (*shape_filter_block).shape_filter[i as usize].type_0
                == SHAPE_FILTER_TYPE_LF_CUT
            {
                (*shape_filter_block).shape_filter[i as usize].b1 = y1;
            } else if (*shape_filter_block).shape_filter[i as usize].type_0
                == SHAPE_FILTER_TYPE_HF_CUT
            {
                (*shape_filter_block).shape_filter[i as usize].g_norm = (*shape_filter_block)
                    .shape_filter[i as usize]
                    .coeff_sum
                    / ((*shape_filter_block).shape_filter[i as usize].partial_coeff_sum
                        + y1);
                (*shape_filter_block).shape_filter[i as usize].b1 = y1;
            } else if (*shape_filter_block).shape_filter[i as usize].type_0
                == SHAPE_FILTER_TYPE_HF_BOOST
            {
                (*shape_filter_block).shape_filter[i as usize].g_norm = ((*shape_filter_block)
                    .shape_filter[i as usize]
                    .partial_coeff_sum + y1)
                    / (*shape_filter_block).shape_filter[i as usize].coeff_sum;
                (*shape_filter_block).shape_filter[i as usize].a1 = y1;
            } else if (*shape_filter_block).shape_filter[i as usize].type_0
                == SHAPE_FILTER_TYPE_LF_BOOST
            {
                (*shape_filter_block).shape_filter[i as usize].a1 = y1;
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn resetshape_flter_block(
    mut shape_filter_block: *mut shape_filter_block,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut c: WORD32 = 0;
    (*shape_filter_block).drc_gain_last = -1.0f32 as FLOAT32;
    impd_shape_filt_block_adapt(1.0f32, shape_filter_block);
    i = 0 as core::ffi::c_int as WORD32;
    while i < 4 as core::ffi::c_int {
        c = 0 as core::ffi::c_int as WORD32;
        while c < MAX_CHANNEL_COUNT {
            (*shape_filter_block)
                .shape_filter[i as usize]
                .audio_in_state_1[c as usize] = 0.0f32 as FLOAT32;
            (*shape_filter_block)
                .shape_filter[i as usize]
                .audio_in_state_2[c as usize] = 0.0f32 as FLOAT32;
            (*shape_filter_block)
                .shape_filter[i as usize]
                .audio_out_state_1[c as usize] = 0.0f32 as FLOAT32;
            (*shape_filter_block)
                .shape_filter[i as usize]
                .audio_out_state_2[c as usize] = 0.0f32 as FLOAT32;
            c += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn impd_shape_filt_block_init(
    mut shape_flter_block_params: *mut ia_shape_filter_block_params_struct,
    mut shape_filter_block: *mut shape_filter_block,
) -> VOID {
    let mut x1: FLOAT32 = 0.;
    let mut x2: FLOAT32 = 0.0f32;
    let mut radius: FLOAT32 = 0.;
    if (*shape_flter_block_params).lf_cut_filter_present != 0 {
        let mut params: *mut ia_shape_filter_params_struct = &mut (*shape_flter_block_params)
            .str_lf_cut_params;
        (*shape_filter_block).shape_filter[0 as core::ffi::c_int as usize].type_0 = SHAPE_FILTER_TYPE_LF_CUT
            as WORD32;
        (*shape_filter_block).shape_filter[0 as core::ffi::c_int as usize].gain_offset = (*shape_filt_lf_gain_offset_tbl
            .as_ptr()
            .offset(
                (*params).corner_freq_index as isize,
            ))[(*params).filter_strength_index as usize];
        (*shape_filter_block).shape_filter[0 as core::ffi::c_int as usize].y1_bound = (*shape_filt_lf_y1_bound_tbl
            .as_ptr()
            .offset(
                (*params).corner_freq_index as isize,
            ))[(*params).filter_strength_index as usize];
        x1 = -*shape_filt_lf_radius_tbl
            .as_ptr()
            .offset((*params).corner_freq_index as isize);
        (*shape_filter_block)
            .shape_filter[0 as core::ffi::c_int as usize]
            .warped_gain_max = SHAPE_FILTER_DRC_GAIN_MAX_MINUS_ONE
            / (SHAPE_FILTER_DRC_GAIN_MAX_MINUS_ONE
                + (*shape_filter_block)
                    .shape_filter[0 as core::ffi::c_int as usize]
                    .gain_offset);
        (*shape_filter_block).shape_filter[0 as core::ffi::c_int as usize].factor = ((*shape_filter_block)
            .shape_filter[0 as core::ffi::c_int as usize]
            .y1_bound - x1)
            / (*shape_filter_block)
                .shape_filter[0 as core::ffi::c_int as usize]
                .warped_gain_max;
        (*shape_filter_block).shape_filter[0 as core::ffi::c_int as usize].a1 = x1;
    } else {
        (*shape_filter_block).shape_filter[0 as core::ffi::c_int as usize].type_0 = SHAPE_FILTER_TYPE_OFF
            as WORD32;
    }
    if (*shape_flter_block_params).lf_boost_filter_present != 0 {
        let mut params_0: *mut ia_shape_filter_params_struct = &mut (*shape_flter_block_params)
            .str_lf_boost_params;
        (*shape_filter_block).shape_filter[1 as core::ffi::c_int as usize].type_0 = SHAPE_FILTER_TYPE_LF_BOOST
            as WORD32;
        (*shape_filter_block).shape_filter[1 as core::ffi::c_int as usize].gain_offset = (*shape_filt_lf_gain_offset_tbl
            .as_ptr()
            .offset(
                (*params_0).corner_freq_index as isize,
            ))[(*params_0).filter_strength_index as usize];
        (*shape_filter_block).shape_filter[1 as core::ffi::c_int as usize].y1_bound = (*shape_filt_lf_y1_bound_tbl
            .as_ptr()
            .offset(
                (*params_0).corner_freq_index as isize,
            ))[(*params_0).filter_strength_index as usize];
        x1 = -*shape_filt_lf_radius_tbl
            .as_ptr()
            .offset((*params_0).corner_freq_index as isize);
        (*shape_filter_block)
            .shape_filter[1 as core::ffi::c_int as usize]
            .warped_gain_max = SHAPE_FILTER_DRC_GAIN_MAX_MINUS_ONE
            / (SHAPE_FILTER_DRC_GAIN_MAX_MINUS_ONE
                + (*shape_filter_block)
                    .shape_filter[1 as core::ffi::c_int as usize]
                    .gain_offset);
        (*shape_filter_block).shape_filter[1 as core::ffi::c_int as usize].factor = ((*shape_filter_block)
            .shape_filter[1 as core::ffi::c_int as usize]
            .y1_bound - x1)
            / (*shape_filter_block)
                .shape_filter[1 as core::ffi::c_int as usize]
                .warped_gain_max;
        (*shape_filter_block).shape_filter[1 as core::ffi::c_int as usize].b1 = x1;
    } else {
        (*shape_filter_block).shape_filter[1 as core::ffi::c_int as usize].type_0 = SHAPE_FILTER_TYPE_OFF
            as WORD32;
    }
    if (*shape_flter_block_params).hf_cut_filter_present != 0 {
        let mut params_1: *mut ia_shape_filter_params_struct = &mut (*shape_flter_block_params)
            .str_hfCutParams;
        (*shape_filter_block).shape_filter[2 as core::ffi::c_int as usize].type_0 = SHAPE_FILTER_TYPE_HF_CUT
            as WORD32;
        (*shape_filter_block).shape_filter[2 as core::ffi::c_int as usize].gain_offset = (*shape_filt_hf_gain_offset_tbl
            .as_ptr()
            .offset(
                (*params_1).corner_freq_index as isize,
            ))[(*params_1).filter_strength_index as usize];
        (*shape_filter_block).shape_filter[2 as core::ffi::c_int as usize].y1_bound = (*shape_filt_hf_y1_bound_tbl
            .as_ptr()
            .offset(
                (*params_1).corner_freq_index as isize,
            ))[(*params_1).filter_strength_index as usize];
        radius = *shape_filt_hf_radius_tbl
            .as_ptr()
            .offset((*params_1).corner_freq_index as isize);
        x1 = ((-2.0f32 * radius) as core::ffi::c_double
            * cos(
                2.0f64 * M_PI
                    * *shape_filt_cutoff_freq_norm_hf_tbl
                        .as_ptr()
                        .offset((*params_1).corner_freq_index as isize)
                        as core::ffi::c_double,
            )) as FLOAT32;
        x2 = radius * radius;
        (*shape_filter_block)
            .shape_filter[2 as core::ffi::c_int as usize]
            .warped_gain_max = SHAPE_FILTER_DRC_GAIN_MAX_MINUS_ONE
            / (SHAPE_FILTER_DRC_GAIN_MAX_MINUS_ONE
                + (*shape_filter_block)
                    .shape_filter[2 as core::ffi::c_int as usize]
                    .gain_offset);
        (*shape_filter_block).shape_filter[2 as core::ffi::c_int as usize].factor = ((*shape_filter_block)
            .shape_filter[2 as core::ffi::c_int as usize]
            .y1_bound - x1)
            / (*shape_filter_block)
                .shape_filter[2 as core::ffi::c_int as usize]
                .warped_gain_max;
        (*shape_filter_block).shape_filter[2 as core::ffi::c_int as usize].coeff_sum = 1.0f32
            + x1 + x2;
        (*shape_filter_block)
            .shape_filter[2 as core::ffi::c_int as usize]
            .partial_coeff_sum = 1.0f32 + x2;
        (*shape_filter_block).shape_filter[2 as core::ffi::c_int as usize].a1 = x1;
        (*shape_filter_block).shape_filter[2 as core::ffi::c_int as usize].a2 = x2;
        (*shape_filter_block).shape_filter[2 as core::ffi::c_int as usize].b2 = x2;
    } else {
        (*shape_filter_block).shape_filter[2 as core::ffi::c_int as usize].type_0 = SHAPE_FILTER_TYPE_OFF
            as WORD32;
    }
    if (*shape_flter_block_params).hf_boost_filter_present != 0 {
        let mut params_2: *mut ia_shape_filter_params_struct = &mut (*shape_flter_block_params)
            .str_hf_boost_params;
        (*shape_filter_block).shape_filter[3 as core::ffi::c_int as usize].type_0 = SHAPE_FILTER_TYPE_HF_BOOST
            as WORD32;
        (*shape_filter_block).shape_filter[3 as core::ffi::c_int as usize].gain_offset = (*shape_filt_hf_gain_offset_tbl
            .as_ptr()
            .offset(
                (*params_2).corner_freq_index as isize,
            ))[(*params_2).filter_strength_index as usize];
        (*shape_filter_block).shape_filter[3 as core::ffi::c_int as usize].y1_bound = (*shape_filt_hf_y1_bound_tbl
            .as_ptr()
            .offset(
                (*params_2).corner_freq_index as isize,
            ))[(*params_2).filter_strength_index as usize];
        radius = *shape_filt_hf_radius_tbl
            .as_ptr()
            .offset((*params_2).corner_freq_index as isize);
        x1 = ((-2.0f32 * radius) as core::ffi::c_double
            * cos(
                2.0f64 * M_PI
                    * *shape_filt_cutoff_freq_norm_hf_tbl
                        .as_ptr()
                        .offset((*params_2).corner_freq_index as isize)
                        as core::ffi::c_double,
            )) as FLOAT32;
        x2 = radius * radius;
        (*shape_filter_block)
            .shape_filter[3 as core::ffi::c_int as usize]
            .warped_gain_max = SHAPE_FILTER_DRC_GAIN_MAX_MINUS_ONE
            / (SHAPE_FILTER_DRC_GAIN_MAX_MINUS_ONE
                + (*shape_filter_block)
                    .shape_filter[3 as core::ffi::c_int as usize]
                    .gain_offset);
        (*shape_filter_block).shape_filter[3 as core::ffi::c_int as usize].factor = ((*shape_filter_block)
            .shape_filter[3 as core::ffi::c_int as usize]
            .y1_bound - x1)
            / (*shape_filter_block)
                .shape_filter[3 as core::ffi::c_int as usize]
                .warped_gain_max;
        (*shape_filter_block).shape_filter[3 as core::ffi::c_int as usize].coeff_sum = 1.0f32
            + x1 + x2;
        (*shape_filter_block)
            .shape_filter[3 as core::ffi::c_int as usize]
            .partial_coeff_sum = 1.0f32 + x2;
        (*shape_filter_block).shape_filter[3 as core::ffi::c_int as usize].b1 = x1;
        (*shape_filter_block).shape_filter[3 as core::ffi::c_int as usize].b2 = x2;
        (*shape_filter_block).shape_filter[3 as core::ffi::c_int as usize].a2 = x2;
    } else {
        (*shape_filter_block).shape_filter[3 as core::ffi::c_int as usize].type_0 = SHAPE_FILTER_TYPE_OFF
            as WORD32;
    }
    resetshape_flter_block(shape_filter_block);
    (*shape_filter_block).shape_flter_block_flag = 1 as core::ffi::c_int as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_shape_filt_block_time_process(
    mut shape_filter_block: *mut shape_filter_block,
    mut drc_gain: *mut FLOAT32,
    channel: WORD32,
    mut audio_in: *mut FLOAT32,
    mut start: WORD32,
    mut end: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut audio_out: FLOAT32 = 0.;
    if (*shape_filter_block).shape_flter_block_flag != 0 {
        i = start;
        while i < end {
            let mut tmp: FLOAT32 = *audio_in.offset(i as isize);
            j = 0 as core::ffi::c_int as WORD32;
            while j < 4 as core::ffi::c_int {
                if (*shape_filter_block).shape_filter[j as usize].type_0
                    == SHAPE_FILTER_TYPE_LF_CUT
                    || (*shape_filter_block).shape_filter[j as usize].type_0
                        == SHAPE_FILTER_TYPE_LF_BOOST
                {
                    audio_out = tmp
                        + (*shape_filter_block).shape_filter[j as usize].b1
                            * (*shape_filter_block)
                                .shape_filter[j as usize]
                                .audio_in_state_1[channel as usize]
                        - (*shape_filter_block).shape_filter[j as usize].a1
                            * (*shape_filter_block)
                                .shape_filter[j as usize]
                                .audio_out_state_1[channel as usize];
                    (*shape_filter_block)
                        .shape_filter[j as usize]
                        .audio_in_state_1[channel as usize] = tmp;
                    (*shape_filter_block)
                        .shape_filter[j as usize]
                        .audio_out_state_1[channel as usize] = audio_out;
                } else if (*shape_filter_block).shape_filter[j as usize].type_0
                    == SHAPE_FILTER_TYPE_HF_CUT
                    || (*shape_filter_block).shape_filter[j as usize].type_0
                        == SHAPE_FILTER_TYPE_HF_BOOST
                {
                    audio_out = (*shape_filter_block).shape_filter[j as usize].g_norm
                        * tmp
                        + (*shape_filter_block).shape_filter[j as usize].b1
                            * (*shape_filter_block)
                                .shape_filter[j as usize]
                                .audio_in_state_1[channel as usize]
                        + (*shape_filter_block).shape_filter[j as usize].b2
                            * (*shape_filter_block)
                                .shape_filter[j as usize]
                                .audio_in_state_2[channel as usize]
                        - (*shape_filter_block).shape_filter[j as usize].a1
                            * (*shape_filter_block)
                                .shape_filter[j as usize]
                                .audio_out_state_1[channel as usize]
                        - (*shape_filter_block).shape_filter[j as usize].a2
                            * (*shape_filter_block)
                                .shape_filter[j as usize]
                                .audio_out_state_2[channel as usize];
                    (*shape_filter_block)
                        .shape_filter[j as usize]
                        .audio_in_state_2[channel as usize] = (*shape_filter_block)
                        .shape_filter[j as usize]
                        .audio_in_state_1[channel as usize];
                    (*shape_filter_block)
                        .shape_filter[j as usize]
                        .audio_in_state_1[channel as usize] = (*shape_filter_block)
                        .shape_filter[j as usize]
                        .g_norm * tmp;
                    (*shape_filter_block)
                        .shape_filter[j as usize]
                        .audio_out_state_2[channel as usize] = (*shape_filter_block)
                        .shape_filter[j as usize]
                        .audio_out_state_1[channel as usize];
                    (*shape_filter_block)
                        .shape_filter[j as usize]
                        .audio_out_state_1[channel as usize] = audio_out;
                } else {
                    audio_out = tmp;
                }
                tmp = audio_out;
                j += 1;
            }
            *audio_in.offset(i as isize) = audio_out * *drc_gain.offset(i as isize);
            i += 1;
        }
    } else {
        i = start;
        while i < end {
            *audio_in.offset(i as isize) = *audio_in.offset(i as isize)
                * *drc_gain.offset(i as isize);
            i += 1;
        }
    };
}
