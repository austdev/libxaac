extern "C" {
    fn log10(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn pow(__x: core::ffi::c_double, __y: core::ffi::c_double) -> core::ffi::c_double;
    static normal_cross_freq: [ia_filter_bank_params_struct; 16];
}
pub type WORD32 = core::ffi::c_int;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_delta_time_code_table_entry_struct {
    pub size: WORD32,
    pub code: WORD32,
    pub value: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_tables_struct {
    pub delta_time_code_table: [ia_delta_time_code_table_entry_struct; 526],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_gain_params_struct {
    pub gain_seq_idx: WORD32,
    pub drc_characteristic: WORD32,
    pub drc_characteristic_present: WORD32,
    pub drc_characteristic_format_is_cicp: WORD32,
    pub drc_characteristic_left_index: WORD32,
    pub drc_characteristic_right_index: WORD32,
    pub crossover_freq_idx: WORD32,
    pub start_subband_index: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ducking_modifiers_struct {
    pub ducking_scaling_flag: WORD32,
    pub ducking_scaling: FLOAT32,
    pub ducking_scaling_quantized: FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_gain_modifiers_struct {
    pub target_characteristic_left_present: [WORD32; 8],
    pub target_characteristic_left_index: [WORD32; 8],
    pub target_characteristic_right_present: [WORD32; 8],
    pub target_characteristic_right_index: [WORD32; 8],
    pub shape_filter_flag: WORD32,
    pub shape_filter_idx: WORD32,
    pub gain_scaling_flag: [WORD32; 8],
    pub attn_scaling: [FLOAT32; 8],
    pub ampl_scaling: [FLOAT32; 8],
    pub gain_offset_flag: [WORD32; 8],
    pub gain_offset: [FLOAT32; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_gain_set_params_struct {
    pub gain_coding_profile: WORD32,
    pub gain_interpolation_type: WORD32,
    pub full_frame: WORD32,
    pub time_alignment: WORD32,
    pub time_delt_min_flag: WORD32,
    pub time_delt_min_val: WORD32,
    pub band_count: WORD32,
    pub drc_band_type: WORD32,
    pub gain_params: [ia_gain_params_struct; 8],
    pub num_gain_max_values: WORD32,
    pub str_tables: ia_tables_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_split_drc_characteristic_struct {
    pub characteristic_format: WORD32,
    pub in_out_ratio: FLOAT32,
    pub gain: FLOAT32,
    pub exp: FLOAT32,
    pub flip_sign: WORD32,
    pub characteristic_node_count: WORD32,
    pub node_level: [FLOAT32; 5],
    pub node_gain: [FLOAT32; 5],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_uni_drc_coeffs_struct {
    pub version: WORD32,
    pub drc_location: WORD32,
    pub drc_frame_size_present: WORD32,
    pub drc_frame_size: WORD32,
    pub gain_set_count: WORD32,
    pub gain_set_params: [ia_gain_set_params_struct; 24],
    pub drc_characteristic_left_present: WORD32,
    pub characteristic_left_count: WORD32,
    pub str_split_characteristic_left: [ia_split_drc_characteristic_struct; 8],
    pub drc_characteristic_right_present: WORD32,
    pub characteristic_right_count: WORD32,
    pub str_split_characteristic_right: [ia_split_drc_characteristic_struct; 8],
    pub shape_filters_present: WORD32,
    pub shape_num_filter: WORD32,
    pub str_shape_filter_block_params: [ia_shape_filter_block_params_struct; 9],
    pub gain_sequence_count: WORD32,
    pub gain_set_params_index_for_gain_sequence: [WORD32; 24],
    pub gain_set_count_plus: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_instructions_struct {
    pub drc_set_id: WORD32,
    pub drc_set_complexity_level: WORD32,
    pub requires_eq: WORD32,
    pub drc_apply_to_dwnmix: WORD32,
    pub drc_location: WORD32,
    pub dwnmix_id_count: WORD32,
    pub downmix_id: [WORD32; 8],
    pub depends_on_drc_set_present: WORD32,
    pub depends_on_drc_set: WORD32,
    pub no_independent_use: WORD32,
    pub drc_set_effect: WORD32,
    pub gain_set_index: [WORD32; 8],
    pub str_gain_modifiers_of_ch_group: [ia_gain_modifiers_struct; 24],
    pub str_ducking_modifiers_for_channel: [ia_ducking_modifiers_struct; 8],
    pub limiter_peak_target_present: WORD32,
    pub limiter_peak_target: FLOAT32,
    pub drc_set_target_loudness_present: WORD32,
    pub drc_set_target_loudness_value_upper: WORD32,
    pub drc_set_target_loudness_value_lower_present: WORD32,
    pub drc_set_target_loudness_value_lower: WORD32,
    pub audio_num_chan: WORD32,
    pub num_drc_ch_groups: WORD32,
    pub gain_set_index_for_channel_group: [WORD32; 24],
    pub band_count_of_ch_group: [WORD32; 24],
    pub gain_interpolation_type_for_channel_group: [WORD32; 24],
    pub time_delta_min_for_channel_group: [WORD32; 24],
    pub time_alignment_for_channel_group: [WORD32; 24],
    pub str_ducking_modifiers_for_channel_group: [ia_ducking_modifiers_struct; 24],
    pub channel_group_of_ch: [WORD32; 8],
    pub num_chan_per_ch_group: [WORD32; 24],
    pub gain_element_count: WORD32,
    pub multiband_audio_sig_count: WORD32,
    pub ch_group_parametric_drc_flag: [WORD32; 24],
    pub gain_set_idx_of_ch_group_parametric_drc: [WORD32; 24],
    pub parametric_drc_look_ahead_samples: [WORD32; 24],
    pub parametric_drc_look_ahead_samples_max: WORD32,
    pub leveling_present: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_filter_bank_params_struct {
    pub f_cross_norm: FLOAT32,
    pub gamma: FLOAT32,
    pub delta: FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_band_overlap_params_struct {
    pub overlap_weight: [FLOAT32; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_group_overlap_params_struct {
    pub str_band_overlap_params: [ia_band_overlap_params_struct; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_overlap_params_struct {
    pub str_group_overlap_params: [ia_group_overlap_params_struct; 24],
}
pub const SUBBAND_DOMAIN_MODE_QMF64: core::ffi::c_int = 1;
pub const SUBBAND_DOMAIN_MODE_QMF71: core::ffi::c_int = 2;
pub const SUBBAND_DOMAIN_MODE_STFT256: core::ffi::c_int = 3;
pub const AUDIO_CODEC_SUBBAND_COUNT_QMF64: core::ffi::c_int = 64 as core::ffi::c_int;
pub const AUDIO_CODEC_SUBBAND_COUNT_QMF71: core::ffi::c_int = 71 as core::ffi::c_int;
pub const AUDIO_CODEC_SUBBAND_COUNT_STFT256: core::ffi::c_int = 256 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn impd_fcenter_norm_sb_init(
    mut num_subbands: WORD32,
    mut fcenter_norm_subband: *mut FLOAT32,
) -> VOID {
    let mut s: WORD32 = 0;
    s = 0 as core::ffi::c_int as WORD32;
    while s < num_subbands {
        *fcenter_norm_subband.offset(s as isize) = ((s as core::ffi::c_float + 0.5f32)
            / (2.0f32 * num_subbands as core::ffi::c_float)) as FLOAT32;
        s += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn impd_generate_slope(
    mut num_sub_bands: WORD32,
    mut fcenter_norm_subband: *mut FLOAT32,
    mut fcross_norm_lo: FLOAT32,
    mut fcross_norm_hi: FLOAT32,
    mut response: *mut FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut filter_slope: FLOAT32 = -24.0f32;
    let mut inv_log10_2: FLOAT32 = 3.32192809f32;
    let mut norm: FLOAT32 = 0.05f32 * filter_slope * inv_log10_2;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_sub_bands {
        if *fcenter_norm_subband.offset(i as isize) < fcross_norm_lo {
            *response.offset(i as isize) = pow(
                10.0f64,
                norm as core::ffi::c_double
                    * log10(
                        (fcross_norm_lo / *fcenter_norm_subband.offset(i as isize))
                            as core::ffi::c_double,
                    ),
            ) as FLOAT32;
        } else if *fcenter_norm_subband.offset(i as isize) < fcross_norm_hi {
            *response.offset(i as isize) = 1.0f32 as FLOAT32;
        } else {
            *response.offset(i as isize) = pow(
                10.0f64,
                norm as core::ffi::c_double
                    * log10(
                        (*fcenter_norm_subband.offset(i as isize) / fcross_norm_hi)
                            as core::ffi::c_double,
                    ),
            ) as FLOAT32;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn impd_generate_overlap_weights(
    mut num_drc_bands: WORD32,
    mut drc_band_type: WORD32,
    mut gain_params: *mut ia_gain_params_struct,
    mut dec_subband_count: WORD32,
    mut pstr_group_overlap_params: *mut ia_group_overlap_params_struct,
) -> VOID {
    let mut fcenter_norm_subband: [FLOAT32; 256] = [0.; 256];
    let mut w_norm: [FLOAT32; 256] = [0.; 256];
    let mut fcross_norm_lo: FLOAT32 = 0.;
    let mut fcross_norm_hi: FLOAT32 = 0.;
    let mut b: WORD32 = 0;
    let mut s: WORD32 = 0;
    let mut start_subband_index: WORD32 = 0 as WORD32;
    let mut stop_sub_band_index: WORD32 = 0 as WORD32;
    impd_fcenter_norm_sb_init(dec_subband_count, fcenter_norm_subband.as_mut_ptr());
    if drc_band_type == 1 as core::ffi::c_int {
        fcross_norm_lo = 0.0f32 as FLOAT32;
        b = 0 as core::ffi::c_int as WORD32;
        while b < num_drc_bands {
            if b < num_drc_bands as core::ffi::c_int - 1 as core::ffi::c_int {
                fcross_norm_hi = normal_cross_freq[(*gain_params
                        .offset(
                            (b as core::ffi::c_int + 1 as core::ffi::c_int) as isize,
                        ))
                        .crossover_freq_idx as usize]
                    .f_cross_norm;
            } else {
                fcross_norm_hi = 0.5f32 as FLOAT32;
            }
            impd_generate_slope(
                dec_subband_count,
                fcenter_norm_subband.as_mut_ptr(),
                fcross_norm_lo,
                fcross_norm_hi,
                ((*pstr_group_overlap_params)
                    .str_band_overlap_params[b as usize]
                    .overlap_weight)
                    .as_mut_ptr(),
            );
            fcross_norm_lo = fcross_norm_hi;
            b += 1;
        }
        s = 0 as core::ffi::c_int as WORD32;
        while s < dec_subband_count {
            w_norm[s as usize] = (*pstr_group_overlap_params)
                .str_band_overlap_params[0 as core::ffi::c_int as usize]
                .overlap_weight[s as usize];
            b = 1 as core::ffi::c_int as WORD32;
            while b < num_drc_bands {
                w_norm[s as usize]
                    += (*pstr_group_overlap_params)
                        .str_band_overlap_params[b as usize]
                        .overlap_weight[s as usize];
                b += 1;
            }
            s += 1;
        }
        s = 0 as core::ffi::c_int as WORD32;
        while s < dec_subband_count {
            b = 0 as core::ffi::c_int as WORD32;
            while b < num_drc_bands {
                (*pstr_group_overlap_params)
                    .str_band_overlap_params[b as usize]
                    .overlap_weight[s as usize] /= w_norm[s as usize];
                b += 1;
            }
            s += 1;
        }
    } else {
        start_subband_index = 0 as core::ffi::c_int as WORD32;
        b = 0 as core::ffi::c_int as WORD32;
        while b < num_drc_bands {
            if b < num_drc_bands as core::ffi::c_int - 1 as core::ffi::c_int {
                stop_sub_band_index = ((*gain_params
                    .offset((b as core::ffi::c_int + 1 as core::ffi::c_int) as isize))
                    .start_subband_index as core::ffi::c_int - 1 as core::ffi::c_int)
                    as WORD32;
            } else {
                stop_sub_band_index = (dec_subband_count as core::ffi::c_int
                    - 1 as core::ffi::c_int) as WORD32;
            }
            s = 0 as core::ffi::c_int as WORD32;
            while s < dec_subband_count {
                if s >= start_subband_index && s <= stop_sub_band_index {
                    (*pstr_group_overlap_params)
                        .str_band_overlap_params[b as usize]
                        .overlap_weight[s as usize] = 1.0f32;
                } else {
                    (*pstr_group_overlap_params)
                        .str_band_overlap_params[b as usize]
                        .overlap_weight[s as usize] = 0.0f32;
                }
                s += 1;
            }
            start_subband_index = (stop_sub_band_index as core::ffi::c_int
                + 1 as core::ffi::c_int) as WORD32;
            b += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn impd_init_overlap_weight(
    mut str_p_loc_drc_coefficients_uni_drc: *mut ia_uni_drc_coeffs_struct,
    mut str_drc_instruction_str: *mut ia_drc_instructions_struct,
    mut sub_band_domain_mode: WORD32,
    mut pstr_overlap_params: *mut ia_overlap_params_struct,
) -> VOID {
    let mut g: WORD32 = 0;
    let mut dec_subband_count: WORD32 = 0 as WORD32;
    match sub_band_domain_mode {
        SUBBAND_DOMAIN_MODE_QMF64 => {
            dec_subband_count = AUDIO_CODEC_SUBBAND_COUNT_QMF64 as WORD32;
        }
        SUBBAND_DOMAIN_MODE_QMF71 => {
            dec_subband_count = AUDIO_CODEC_SUBBAND_COUNT_QMF71 as WORD32;
        }
        SUBBAND_DOMAIN_MODE_STFT256 => {
            dec_subband_count = AUDIO_CODEC_SUBBAND_COUNT_STFT256 as WORD32;
        }
        _ => {}
    }
    g = 0 as core::ffi::c_int as WORD32;
    while g < (*str_drc_instruction_str).num_drc_ch_groups {
        if (*str_drc_instruction_str).band_count_of_ch_group[g as usize]
            > 1 as core::ffi::c_int
        {
            impd_generate_overlap_weights(
                (*str_drc_instruction_str).band_count_of_ch_group[g as usize],
                (*str_p_loc_drc_coefficients_uni_drc)
                    .gain_set_params[(*str_drc_instruction_str)
                        .gain_set_index_for_channel_group[g as usize] as usize]
                    .drc_band_type,
                ((*str_p_loc_drc_coefficients_uni_drc)
                    .gain_set_params[(*str_drc_instruction_str)
                        .gain_set_index_for_channel_group[g as usize] as usize]
                    .gain_params)
                    .as_mut_ptr(),
                dec_subband_count,
                &mut *((*pstr_overlap_params).str_group_overlap_params)
                    .as_mut_ptr()
                    .offset(g as isize),
            );
        }
        g += 1;
    }
}
