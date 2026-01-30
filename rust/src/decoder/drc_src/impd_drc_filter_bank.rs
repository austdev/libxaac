extern "C" {
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
pub struct ia_iir_filter_struct {
    pub a0: FLOAT32,
    pub a1: FLOAT32,
    pub a2: FLOAT32,
    pub b0: FLOAT32,
    pub b1: FLOAT32,
    pub b2: FLOAT32,
    pub x_p: [FLOAT32; 16],
    pub y_p: [FLOAT32; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_two_band_filt_struct {
    pub low_pass: ia_iir_filter_struct,
    pub high_pass: ia_iir_filter_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_three_band_filt_struct {
    pub str_low_pass_stage_1: ia_iir_filter_struct,
    pub str_high_pass_stage_1: ia_iir_filter_struct,
    pub str_low_pass_stage_2: ia_iir_filter_struct,
    pub str_high_pass_stage_2: ia_iir_filter_struct,
    pub str_all_pass_stage_2: ia_iir_filter_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_four_band_filt_struct {
    pub str_low_pass_stage_1: ia_iir_filter_struct,
    pub str_high_pass_stage_1: ia_iir_filter_struct,
    pub str_all_pass_stage_2_high: ia_iir_filter_struct,
    pub str_all_pass_stage_2_low: ia_iir_filter_struct,
    pub str_low_pass_stage_3_high: ia_iir_filter_struct,
    pub str_high_pass_stage_3_high: ia_iir_filter_struct,
    pub str_low_pass_stage_3_low: ia_iir_filter_struct,
    pub str_high_pass_stage_3_low: ia_iir_filter_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_all_pass_filter_sturct {
    pub str_all_pass_stage: ia_iir_filter_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_all_pass_cascade_struct {
    pub str_all_pass_cascade_filter: [ia_all_pass_filter_sturct; 9],
    pub num_filter: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_filter_bank_struct {
    pub num_bands: WORD32,
    pub complexity: WORD32,
    pub str_two_band_bank: ia_two_band_filt_struct,
    pub str_three_band_bank: ia_three_band_filt_struct,
    pub str_four_band_bank: ia_four_band_filt_struct,
    pub str_all_pass_cascade: ia_all_pass_cascade_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_filter_banks_struct {
    pub nfilter_banks: WORD32,
    pub num_ph_align_ch_groups: WORD32,
    pub complexity: WORD32,
    pub str_drc_filter_bank: [ia_drc_filter_bank_struct; 8],
}
pub const SEQUENCE_COUNT_MAX: core::ffi::c_int = 24 as core::ffi::c_int;
pub const CHANNEL_GROUP_COUNT_MAX: core::ffi::c_int = SEQUENCE_COUNT_MAX;
#[no_mangle]
pub unsafe extern "C" fn impd_compute_filt_coeff(
    mut crossover_freq_idx: WORD32,
    mut pstr_lp_filt_coeff: *mut ia_iir_filter_struct,
    mut pstr_hp_filt_coeff: *mut ia_iir_filter_struct,
    mut pstr_ap_filt_coeff: *mut ia_iir_filter_struct,
    mut filter_type: WORD32,
) -> VOID {
    let mut gamma: FLOAT32 = normal_cross_freq[crossover_freq_idx as usize].gamma;
    let mut delta: FLOAT32 = normal_cross_freq[crossover_freq_idx as usize].delta;
    if filter_type == 0 as core::ffi::c_int || filter_type == 2 as core::ffi::c_int {
        (*pstr_lp_filt_coeff).a0 = 1.0f32 as FLOAT32;
        (*pstr_lp_filt_coeff).a1 = 2.0f32 * (gamma - delta);
        (*pstr_lp_filt_coeff).a2 = (2.0f32
            * (gamma as core::ffi::c_float + delta as core::ffi::c_float) - 1.0f32)
            as FLOAT32;
        (*pstr_lp_filt_coeff).b0 = gamma;
        (*pstr_lp_filt_coeff).b1 = 2.0f32 * gamma;
        (*pstr_lp_filt_coeff).b2 = gamma;
        (*pstr_hp_filt_coeff).a0 = 1.0f32 as FLOAT32;
        (*pstr_hp_filt_coeff).a1 = (*pstr_lp_filt_coeff).a1;
        (*pstr_hp_filt_coeff).a2 = (*pstr_lp_filt_coeff).a2;
        (*pstr_hp_filt_coeff).b0 = delta;
        (*pstr_hp_filt_coeff).b1 = -2.0f32 * delta;
        (*pstr_hp_filt_coeff).b2 = delta;
    }
    if filter_type == 1 as core::ffi::c_int || filter_type == 2 as core::ffi::c_int {
        (*pstr_ap_filt_coeff).a0 = 1.0f32 as FLOAT32;
        (*pstr_ap_filt_coeff).a1 = 2.0f32 * (gamma - delta);
        (*pstr_ap_filt_coeff).a2 = (2.0f32
            * (gamma as core::ffi::c_float + delta as core::ffi::c_float) - 1.0f32)
            as FLOAT32;
        (*pstr_ap_filt_coeff).b0 = (*pstr_ap_filt_coeff).a2;
        (*pstr_ap_filt_coeff).b1 = (*pstr_ap_filt_coeff).a1;
        (*pstr_ap_filt_coeff).b2 = (*pstr_ap_filt_coeff).a0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn impd_initialize_filt_bank(
    mut num_sub_bands: WORD32,
    mut gain_params: *mut ia_gain_params_struct,
    mut drc_filter_bank: *mut ia_drc_filter_bank_struct,
) -> WORD32 {
    let mut str_two_band_bank: *mut ia_two_band_filt_struct = 0
        as *mut ia_two_band_filt_struct;
    let mut str_three_band_bank: *mut ia_three_band_filt_struct = 0
        as *mut ia_three_band_filt_struct;
    let mut str_four_band_bank: *mut ia_four_band_filt_struct = 0
        as *mut ia_four_band_filt_struct;
    (*drc_filter_bank).complexity = 0 as core::ffi::c_int as WORD32;
    (*drc_filter_bank).num_bands = num_sub_bands;
    if num_sub_bands == 1 as core::ffi::c_int {
        return 0 as WORD32
    } else if num_sub_bands == 2 as core::ffi::c_int {
        str_two_band_bank = &mut (*drc_filter_bank).str_two_band_bank;
        impd_compute_filt_coeff(
            (*gain_params.offset(1 as core::ffi::c_int as isize)).crossover_freq_idx,
            &mut (*str_two_band_bank).low_pass,
            &mut (*str_two_band_bank).high_pass,
            0 as *mut ia_iir_filter_struct,
            0 as WORD32,
        );
    } else if num_sub_bands == 3 as core::ffi::c_int {
        str_three_band_bank = &mut (*drc_filter_bank).str_three_band_bank;
        impd_compute_filt_coeff(
            (*gain_params.offset(1 as core::ffi::c_int as isize)).crossover_freq_idx,
            &mut (*str_three_band_bank).str_low_pass_stage_2,
            &mut (*str_three_band_bank).str_high_pass_stage_2,
            &mut (*str_three_band_bank).str_all_pass_stage_2,
            2 as WORD32,
        );
        impd_compute_filt_coeff(
            (*gain_params.offset(2 as core::ffi::c_int as isize)).crossover_freq_idx,
            &mut (*str_three_band_bank).str_low_pass_stage_1,
            &mut (*str_three_band_bank).str_high_pass_stage_1,
            0 as *mut ia_iir_filter_struct,
            0 as WORD32,
        );
    } else if num_sub_bands == 4 as core::ffi::c_int {
        str_four_band_bank = &mut (*drc_filter_bank).str_four_band_bank;
        impd_compute_filt_coeff(
            (*gain_params.offset(1 as core::ffi::c_int as isize)).crossover_freq_idx,
            &mut (*str_four_band_bank).str_low_pass_stage_3_low,
            &mut (*str_four_band_bank).str_high_pass_stage_3_low,
            &mut (*str_four_band_bank).str_all_pass_stage_2_high,
            2 as WORD32,
        );
        impd_compute_filt_coeff(
            (*gain_params.offset(2 as core::ffi::c_int as isize)).crossover_freq_idx,
            &mut (*str_four_band_bank).str_low_pass_stage_1,
            &mut (*str_four_band_bank).str_high_pass_stage_1,
            0 as *mut ia_iir_filter_struct,
            0 as WORD32,
        );
        impd_compute_filt_coeff(
            (*gain_params.offset(3 as core::ffi::c_int as isize)).crossover_freq_idx,
            &mut (*str_four_band_bank).str_low_pass_stage_3_high,
            &mut (*str_four_band_bank).str_high_pass_stage_3_high,
            &mut (*str_four_band_bank).str_all_pass_stage_2_low,
            2 as WORD32,
        );
    } else {
        return -(1 as WORD32)
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_init_all_filter_banks(
    mut str_p_loc_drc_coefficients_uni_drc: *mut ia_uni_drc_coeffs_struct,
    mut str_drc_instruction_str: *mut ia_drc_instructions_struct,
    mut ia_filter_banks_struct: *mut ia_filter_banks_struct,
) -> WORD32 {
    let mut err_code: WORD32 = 0 as WORD32;
    let mut b: WORD32 = 0;
    let mut g: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut m: WORD32 = 0;
    let mut s: WORD32 = 0;
    let mut crossover_freq_idx: WORD32 = 0;
    let mut num_ch_in_groups: WORD32 = 0;
    let mut num_ph_align_ch_groups: WORD32 = 0;
    let mut match_found: WORD32 = 0 as WORD32;
    let mut num_filter: WORD32 = 0;
    let mut cascade_cross_idx: [[WORD32; 72]; 25] = [[0; 72]; 25];
    let mut count: [WORD32; 25] = [0; 25];
    num_ch_in_groups = 0 as core::ffi::c_int as WORD32;
    num_ph_align_ch_groups = (*str_drc_instruction_str).num_drc_ch_groups;
    g = 0 as core::ffi::c_int as WORD32;
    while g < (*str_drc_instruction_str).num_drc_ch_groups {
        num_ch_in_groups += (*str_drc_instruction_str).num_chan_per_ch_group[g as usize];
        g += 1;
    }
    if num_ch_in_groups < (*str_drc_instruction_str).audio_num_chan {
        num_ph_align_ch_groups += 1;
    }
    (*ia_filter_banks_struct).nfilter_banks = (*str_drc_instruction_str)
        .num_drc_ch_groups;
    (*ia_filter_banks_struct).num_ph_align_ch_groups = num_ph_align_ch_groups;
    if str_p_loc_drc_coefficients_uni_drc.is_null() {
        (*((*ia_filter_banks_struct).str_drc_filter_bank).as_mut_ptr()).num_bands = 1
            as core::ffi::c_int as WORD32;
    } else {
        g = 0 as core::ffi::c_int as WORD32;
        while g < (*str_drc_instruction_str).num_drc_ch_groups {
            err_code = impd_initialize_filt_bank(
                (*str_p_loc_drc_coefficients_uni_drc)
                    .gain_set_params[(*str_drc_instruction_str)
                        .gain_set_index_for_channel_group[g as usize] as usize]
                    .band_count,
                ((*str_p_loc_drc_coefficients_uni_drc)
                    .gain_set_params[(*str_drc_instruction_str)
                        .gain_set_index_for_channel_group[g as usize] as usize]
                    .gain_params)
                    .as_mut_ptr(),
                &mut *((*ia_filter_banks_struct).str_drc_filter_bank)
                    .as_mut_ptr()
                    .offset(g as isize),
            );
            if err_code != 0 as core::ffi::c_int {
                return err_code;
            }
            g += 1;
        }
    }
    g = 0 as core::ffi::c_int as WORD32;
    while g < CHANNEL_GROUP_COUNT_MAX + 1 as core::ffi::c_int {
        count[g as usize] = 0 as core::ffi::c_int as WORD32;
        g += 1;
    }
    g = 0 as core::ffi::c_int as WORD32;
    while g < (*str_drc_instruction_str).num_drc_ch_groups {
        b = 1 as core::ffi::c_int as WORD32;
        while b
            < (*str_p_loc_drc_coefficients_uni_drc)
                .gain_set_params[(*str_drc_instruction_str)
                    .gain_set_index_for_channel_group[g as usize] as usize]
                .band_count
        {
            crossover_freq_idx = (*str_p_loc_drc_coefficients_uni_drc)
                .gain_set_params[(*str_drc_instruction_str)
                    .gain_set_index_for_channel_group[g as usize] as usize]
                .gain_params[b as usize]
                .crossover_freq_idx;
            k = 0 as core::ffi::c_int as WORD32;
            while k < num_ph_align_ch_groups {
                if k != g {
                    cascade_cross_idx[k as usize][count[k as usize] as usize] = crossover_freq_idx;
                    count[k as usize] += 1;
                    if count[k as usize]
                        > CHANNEL_GROUP_COUNT_MAX * 3 as core::ffi::c_int
                    {
                        return -(1 as WORD32);
                    }
                }
                k += 1;
            }
            b += 1;
        }
        g += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < count[0 as core::ffi::c_int as usize] {
        crossover_freq_idx = cascade_cross_idx[0 as core::ffi::c_int
            as usize][i as usize];
        match_found = 0 as core::ffi::c_int as WORD32;
        g = 1 as core::ffi::c_int as WORD32;
        while g < num_ph_align_ch_groups {
            match_found = 0 as core::ffi::c_int as WORD32;
            k = 0 as core::ffi::c_int as WORD32;
            while k < count[g as usize] {
                if cascade_cross_idx[g as usize][k as usize] == crossover_freq_idx {
                    match_found = 1 as core::ffi::c_int as WORD32;
                    break;
                } else {
                    k += 1;
                }
            }
            if match_found == 0 as core::ffi::c_int {
                break;
            }
            g += 1;
        }
        if match_found == 1 as core::ffi::c_int {
            g = 0 as core::ffi::c_int as WORD32;
            while g < num_ph_align_ch_groups {
                m = 0 as core::ffi::c_int as WORD32;
                while m < count[g as usize] {
                    if cascade_cross_idx[g as usize][m as usize] == crossover_freq_idx {
                        s = (m as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
                        while s < count[g as usize] {
                            cascade_cross_idx[g
                                as usize][(s as core::ffi::c_int - 1 as core::ffi::c_int)
                                as usize] = cascade_cross_idx[g as usize][s as usize];
                            s += 1;
                        }
                        count[g as usize] -= 1;
                        break;
                    } else {
                        m += 1;
                    }
                }
                g += 1;
            }
            i = 0 as core::ffi::c_int as WORD32;
        } else {
            i += 1;
        }
    }
    g = 0 as core::ffi::c_int as WORD32;
    while g < num_ph_align_ch_groups {
        num_filter = count[g as usize];
        if num_filter > 0 as core::ffi::c_int {
            i = 0 as core::ffi::c_int as WORD32;
            while i < num_filter {
                impd_compute_filt_coeff(
                    cascade_cross_idx[g as usize][i as usize],
                    0 as *mut ia_iir_filter_struct,
                    0 as *mut ia_iir_filter_struct,
                    &mut (*((*((*ia_filter_banks_struct).str_drc_filter_bank)
                        .as_mut_ptr()
                        .offset(g as isize))
                        .str_all_pass_cascade
                        .str_all_pass_cascade_filter)
                        .as_mut_ptr()
                        .offset(i as isize))
                        .str_all_pass_stage,
                    1 as WORD32,
                );
                i += 1;
            }
            (*ia_filter_banks_struct)
                .str_drc_filter_bank[g as usize]
                .str_all_pass_cascade
                .num_filter = num_filter;
        }
        if err_code != 0 as core::ffi::c_int {
            return err_code;
        }
        g += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_iir_second_order_filter_all_pass(
    mut filter: *mut ia_iir_filter_struct,
    mut chan_idx: WORD32,
    mut frame_len: WORD32,
    mut input: *mut FLOAT32,
    mut output: *mut FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut tmp: FLOAT32 = 0.;
    let mut a1: FLOAT32 = (*filter).a1;
    let mut a2: FLOAT32 = (*filter).a2;
    let mut b0: FLOAT32 = (*filter).b0;
    let mut b1: FLOAT32 = (*filter).b1;
    let mut b2: FLOAT32 = (*filter).b2;
    let mut st1: FLOAT32 = (*filter)
        .x_p[(chan_idx as core::ffi::c_int * 2 as core::ffi::c_int) as usize];
    let mut st2: FLOAT32 = (*filter)
        .y_p[(chan_idx as core::ffi::c_int * 2 as core::ffi::c_int) as usize];
    i = 0 as core::ffi::c_int as WORD32;
    while i < frame_len {
        tmp = *input.offset(i as isize);
        *output.offset(i as isize) = b0 * tmp + st1;
        st1 = b1 * tmp - a1 * *output.offset(i as isize) + st2;
        st2 = b2 * tmp - a2 * *output.offset(i as isize);
        i += 1;
    }
    (*filter).x_p[(chan_idx as core::ffi::c_int * 2 as core::ffi::c_int) as usize] = st1;
    (*filter).y_p[(chan_idx as core::ffi::c_int * 2 as core::ffi::c_int) as usize] = st2;
}
#[no_mangle]
pub unsafe extern "C" fn impd_apply_low_high_filter(
    mut pstr_lp_filt_coeff: *mut ia_iir_filter_struct,
    mut pstr_hp_filt_coeff: *mut ia_iir_filter_struct,
    mut chan_idx: WORD32,
    mut frame_len: WORD32,
    mut input: *mut FLOAT32,
    mut output: *mut *mut FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut tmp: FLOAT32 = 0.;
    let mut tmp1: FLOAT32 = 0.;
    let mut a1_l: FLOAT32 = (*pstr_lp_filt_coeff).a1;
    let mut a2_l: FLOAT32 = (*pstr_lp_filt_coeff).a2;
    let mut b0_l: FLOAT32 = (*pstr_lp_filt_coeff).b0;
    let mut b1_l: FLOAT32 = (*pstr_lp_filt_coeff).b1;
    let mut b2_l: FLOAT32 = (*pstr_lp_filt_coeff).b2;
    let mut st1_l: FLOAT32 = (*pstr_lp_filt_coeff)
        .x_p[(chan_idx as core::ffi::c_int * 2 as core::ffi::c_int
        + 0 as core::ffi::c_int) as usize];
    let mut st2_l: FLOAT32 = (*pstr_lp_filt_coeff)
        .x_p[(chan_idx as core::ffi::c_int * 2 as core::ffi::c_int
        + 1 as core::ffi::c_int) as usize];
    let mut st3_l: FLOAT32 = (*pstr_lp_filt_coeff)
        .y_p[(chan_idx as core::ffi::c_int * 2 as core::ffi::c_int
        + 0 as core::ffi::c_int) as usize];
    let mut st4_l: FLOAT32 = (*pstr_lp_filt_coeff)
        .y_p[(chan_idx as core::ffi::c_int * 2 as core::ffi::c_int
        + 1 as core::ffi::c_int) as usize];
    let mut a1_h: FLOAT32 = (*pstr_hp_filt_coeff).a1;
    let mut a2_h: FLOAT32 = (*pstr_hp_filt_coeff).a2;
    let mut b0_h: FLOAT32 = (*pstr_hp_filt_coeff).b0;
    let mut b1_h: FLOAT32 = (*pstr_hp_filt_coeff).b1;
    let mut b2_h: FLOAT32 = (*pstr_hp_filt_coeff).b2;
    let mut st1_h: FLOAT32 = (*pstr_hp_filt_coeff)
        .x_p[(chan_idx as core::ffi::c_int * 2 as core::ffi::c_int
        + 0 as core::ffi::c_int) as usize];
    let mut st2_h: FLOAT32 = (*pstr_hp_filt_coeff)
        .x_p[(chan_idx as core::ffi::c_int * 2 as core::ffi::c_int
        + 1 as core::ffi::c_int) as usize];
    let mut st3_h: FLOAT32 = (*pstr_hp_filt_coeff)
        .y_p[(chan_idx as core::ffi::c_int * 2 as core::ffi::c_int
        + 0 as core::ffi::c_int) as usize];
    let mut st4_h: FLOAT32 = (*pstr_hp_filt_coeff)
        .y_p[(chan_idx as core::ffi::c_int * 2 as core::ffi::c_int
        + 1 as core::ffi::c_int) as usize];
    let mut output_low: *mut FLOAT32 = *output.offset(0 as core::ffi::c_int as isize);
    let mut output_high: *mut FLOAT32 = *output.offset(1 as core::ffi::c_int as isize);
    i = 0 as core::ffi::c_int as WORD32;
    while i < frame_len {
        tmp1 = *input.offset(i as isize);
        tmp = b0_l * tmp1 + st1_l;
        st1_l = b1_l * tmp1 - a1_l * tmp + st2_l;
        st2_l = b2_l * tmp1 - a2_l * tmp;
        *output_low.offset(i as isize) = b0_l * tmp + st3_l;
        st3_l = b1_l * tmp - a1_l * *output_low.offset(i as isize) + st4_l;
        st4_l = b2_l * tmp - a2_l * *output_low.offset(i as isize);
        tmp = b0_h * tmp1 + st1_h;
        st1_h = b1_h * tmp1 - a1_h * tmp + st2_h;
        st2_h = b2_h * tmp1 - a2_h * tmp;
        *output_high.offset(i as isize) = b0_h * tmp + st3_h;
        st3_h = b1_h * tmp - a1_h * *output_high.offset(i as isize) + st4_h;
        st4_h = b2_h * tmp - a2_h * *output_high.offset(i as isize);
        i += 1;
    }
    (*pstr_lp_filt_coeff)
        .x_p[(chan_idx as core::ffi::c_int * 2 as core::ffi::c_int
        + 0 as core::ffi::c_int) as usize] = st1_l;
    (*pstr_lp_filt_coeff)
        .x_p[(chan_idx as core::ffi::c_int * 2 as core::ffi::c_int
        + 1 as core::ffi::c_int) as usize] = st2_l;
    (*pstr_lp_filt_coeff)
        .y_p[(chan_idx as core::ffi::c_int * 2 as core::ffi::c_int
        + 0 as core::ffi::c_int) as usize] = st3_l;
    (*pstr_lp_filt_coeff)
        .y_p[(chan_idx as core::ffi::c_int * 2 as core::ffi::c_int
        + 1 as core::ffi::c_int) as usize] = st4_l;
    (*pstr_hp_filt_coeff)
        .x_p[(chan_idx as core::ffi::c_int * 2 as core::ffi::c_int
        + 0 as core::ffi::c_int) as usize] = st1_h;
    (*pstr_hp_filt_coeff)
        .x_p[(chan_idx as core::ffi::c_int * 2 as core::ffi::c_int
        + 1 as core::ffi::c_int) as usize] = st2_h;
    (*pstr_hp_filt_coeff)
        .y_p[(chan_idx as core::ffi::c_int * 2 as core::ffi::c_int
        + 0 as core::ffi::c_int) as usize] = st3_h;
    (*pstr_hp_filt_coeff)
        .y_p[(chan_idx as core::ffi::c_int * 2 as core::ffi::c_int
        + 1 as core::ffi::c_int) as usize] = st4_h;
}
#[no_mangle]
pub unsafe extern "C" fn impd_two_band_filter_process(
    mut str_two_band_bank: *mut ia_two_band_filt_struct,
    mut chan_idx: WORD32,
    mut frame_len: WORD32,
    mut input: *mut FLOAT32,
    mut output: *mut *mut FLOAT32,
) -> VOID {
    let mut pstr_lp_filt_coeff: *mut ia_iir_filter_struct = &mut (*str_two_band_bank)
        .low_pass;
    let mut pstr_hp_filt_coeff: *mut ia_iir_filter_struct = &mut (*str_two_band_bank)
        .high_pass;
    impd_apply_low_high_filter(
        pstr_lp_filt_coeff,
        pstr_hp_filt_coeff,
        chan_idx,
        frame_len,
        input,
        output,
    );
}
#[no_mangle]
pub unsafe extern "C" fn impd_three_band_filter_process(
    mut str_three_band_bank: *mut ia_three_band_filt_struct,
    mut c: WORD32,
    mut size: WORD32,
    mut input: *mut FLOAT32,
    mut output: *mut *mut FLOAT32,
) -> VOID {
    let mut all_pass_filter: *mut ia_iir_filter_struct = 0 as *mut ia_iir_filter_struct;
    let mut pstr_lp_filt_coeff: *mut ia_iir_filter_struct = &mut (*str_three_band_bank)
        .str_low_pass_stage_1;
    let mut pstr_hp_filt_coeff: *mut ia_iir_filter_struct = &mut (*str_three_band_bank)
        .str_high_pass_stage_1;
    let mut output1: [*mut FLOAT32; 2] = [0 as *mut FLOAT32; 2];
    output1[0 as core::ffi::c_int as usize] = *output
        .offset(0 as core::ffi::c_int as isize);
    output1[1 as core::ffi::c_int as usize] = *output
        .offset(1 as core::ffi::c_int as isize);
    impd_apply_low_high_filter(
        pstr_lp_filt_coeff,
        pstr_hp_filt_coeff,
        c,
        size,
        input,
        output1.as_mut_ptr(),
    );
    all_pass_filter = &mut (*str_three_band_bank).str_all_pass_stage_2;
    impd_iir_second_order_filter_all_pass(
        all_pass_filter,
        c,
        size,
        output1[1 as core::ffi::c_int as usize],
        *output.offset(2 as core::ffi::c_int as isize),
    );
    pstr_lp_filt_coeff = &mut (*str_three_band_bank).str_low_pass_stage_2;
    pstr_hp_filt_coeff = &mut (*str_three_band_bank).str_high_pass_stage_2;
    impd_apply_low_high_filter(
        pstr_lp_filt_coeff,
        pstr_hp_filt_coeff,
        c,
        size,
        output1[0 as core::ffi::c_int as usize],
        output1.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn impd_four_band_filter_process(
    mut str_four_band_bank: *mut ia_four_band_filt_struct,
    mut cha_idx: WORD32,
    mut win_size: WORD32,
    mut input: *mut FLOAT32,
    mut output: *mut *mut FLOAT32,
) -> VOID {
    let mut all_pass_filter: *mut ia_iir_filter_struct = 0 as *mut ia_iir_filter_struct;
    let mut pstr_lp_filt_coeff: *mut ia_iir_filter_struct = &mut (*str_four_band_bank)
        .str_low_pass_stage_1;
    let mut pstr_hp_filt_coeff: *mut ia_iir_filter_struct = &mut (*str_four_band_bank)
        .str_high_pass_stage_1;
    let mut output1: [*mut FLOAT32; 2] = [0 as *mut FLOAT32; 2];
    let mut output2: [*mut FLOAT32; 2] = [0 as *mut FLOAT32; 2];
    output1[0 as core::ffi::c_int as usize] = *output
        .offset(0 as core::ffi::c_int as isize);
    output1[1 as core::ffi::c_int as usize] = *output
        .offset(1 as core::ffi::c_int as isize);
    output2[0 as core::ffi::c_int as usize] = *output
        .offset(2 as core::ffi::c_int as isize);
    output2[1 as core::ffi::c_int as usize] = *output
        .offset(3 as core::ffi::c_int as isize);
    impd_apply_low_high_filter(
        pstr_lp_filt_coeff,
        pstr_hp_filt_coeff,
        cha_idx,
        win_size,
        input,
        output1.as_mut_ptr(),
    );
    all_pass_filter = &mut (*str_four_band_bank).str_all_pass_stage_2_low;
    impd_iir_second_order_filter_all_pass(
        all_pass_filter,
        cha_idx,
        win_size,
        output1[0 as core::ffi::c_int as usize],
        output1[0 as core::ffi::c_int as usize],
    );
    all_pass_filter = &mut (*str_four_band_bank).str_all_pass_stage_2_high;
    impd_iir_second_order_filter_all_pass(
        all_pass_filter,
        cha_idx,
        win_size,
        output1[1 as core::ffi::c_int as usize],
        output2[0 as core::ffi::c_int as usize],
    );
    pstr_lp_filt_coeff = &mut (*str_four_band_bank).str_low_pass_stage_3_low;
    pstr_hp_filt_coeff = &mut (*str_four_band_bank).str_high_pass_stage_3_low;
    impd_apply_low_high_filter(
        pstr_lp_filt_coeff,
        pstr_hp_filt_coeff,
        cha_idx,
        win_size,
        output1[0 as core::ffi::c_int as usize],
        output1.as_mut_ptr(),
    );
    pstr_lp_filt_coeff = &mut (*str_four_band_bank).str_low_pass_stage_3_high;
    pstr_hp_filt_coeff = &mut (*str_four_band_bank).str_high_pass_stage_3_high;
    impd_apply_low_high_filter(
        pstr_lp_filt_coeff,
        pstr_hp_filt_coeff,
        cha_idx,
        win_size,
        output2[0 as core::ffi::c_int as usize],
        output2.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn impd_all_pass_cascade_process(
    mut str_all_pass_cascade: *mut ia_all_pass_cascade_struct,
    mut ch_idx: WORD32,
    mut win_size: WORD32,
    mut input: *mut FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*str_all_pass_cascade).num_filter {
        impd_iir_second_order_filter_all_pass(
            &mut (*((*str_all_pass_cascade).str_all_pass_cascade_filter)
                .as_mut_ptr()
                .offset(i as isize))
                .str_all_pass_stage,
            ch_idx,
            win_size,
            input,
            input,
        );
        i += 1;
    }
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
