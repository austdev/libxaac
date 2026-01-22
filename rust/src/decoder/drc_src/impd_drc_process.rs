extern "C" {
    fn fabs(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn memmove(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn impd_two_band_filter_process(
        str_two_band_bank: *mut ia_two_band_filt_struct,
        c: WORD32,
        size: WORD32,
        audio_in: *mut FLOAT32,
        audio_out: *mut *mut FLOAT32,
    ) -> VOID;
    fn impd_three_band_filter_process(
        str_three_band_bank: *mut ia_three_band_filt_struct,
        c: WORD32,
        size: WORD32,
        audio_in: *mut FLOAT32,
        audio_out: *mut *mut FLOAT32,
    ) -> VOID;
    fn impd_four_band_filter_process(
        str_four_band_bank: *mut ia_four_band_filt_struct,
        c: WORD32,
        size: WORD32,
        audio_in: *mut FLOAT32,
        audio_out: *mut *mut FLOAT32,
    ) -> VOID;
    fn impd_all_pass_cascade_process(
        str_all_pass_cascade: *mut ia_all_pass_cascade_struct,
        c: WORD32,
        size: WORD32,
        audio_in: *mut FLOAT32,
    ) -> VOID;
    fn impd_shape_filt_block_adapt(
        drc_gain: FLOAT32,
        shape_filter_block: *mut shape_filter_block,
    ) -> VOID;
    fn impd_shape_filt_block_time_process(
        shape_filter_block: *mut shape_filter_block,
        drc_gain: *mut FLOAT32,
        channel: WORD32,
        audio_in: *mut FLOAT32,
        start: WORD32,
        end: WORD32,
    ) -> VOID;
}
pub type size_t = usize;
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
pub struct ia_node_struct {
    pub loc_db_gain: FLOAT32,
    pub slope: FLOAT32,
    pub time: WORD32,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_interp_buf_struct {
    pub str_node: ia_node_struct,
    pub prev_node: ia_node_struct,
    pub lpcm_gains: [FLOAT32; 12692],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_gain_buffer_struct {
    pub buf_interpolation_count: WORD32,
    pub buf_interpolation: *mut ia_interp_buf_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sel_drc_struct {
    pub drc_instructions_index: WORD32,
    pub drc_coeff_idx: WORD32,
    pub dwnmix_instructions_index: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_params_struct {
    pub sample_rate: WORD32,
    pub delta_tmin_default: WORD32,
    pub drc_frame_size: WORD32,
    pub delay_mode: WORD32,
    pub sub_band_domain_mode: WORD32,
    pub gain_delay_samples: WORD32,
    pub parametric_drc_delay: WORD32,
    pub eq_delay: WORD32,
    pub audio_delay_samples: WORD32,
    pub drc_set_counter: WORD32,
    pub multiband_sel_drc_idx: WORD32,
    pub sel_drc_array: [ia_sel_drc_struct; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_audio_band_buffer_struct {
    pub multiband_audio_sig_count: WORD32,
    pub frame_size: WORD32,
    pub non_interleaved_audio: *mut *mut FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_audio_in_out_buf {
    pub audio_num_chan: WORD32,
    pub frame_size: WORD32,
    pub audio_sub_band_count: WORD32,
    pub audio_sub_band_frame_size: WORD32,
    pub audio_delay_samples: WORD32,
    pub audio_delay_sub_band_samples: WORD32,
    pub audio_io_buffer_delayed: *mut *mut FLOAT32,
    pub audio_buffer_delayed_real: *mut *mut FLOAT32,
    pub audio_buffer_delayed_imag: *mut *mut FLOAT32,
    pub audio_in_out_buf: *mut *mut FLOAT32,
    pub audio_real_buff: *mut *mut FLOAT32,
    pub audio_imag_buff: *mut *mut FLOAT32,
}
pub const MAX_SIGNAL_DELAY: core::ffi::c_int = 4500 as core::ffi::c_int;
pub const DELAY_MODE_LOW_DELAY: core::ffi::c_int = 1 as core::ffi::c_int;
pub const PARAM_ERROR: core::ffi::c_int = 3 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn impd_apply_gains_and_add(
    mut pstr_drc_instruction_arr: *mut ia_drc_instructions_struct,
    drc_instructions_index: WORD32,
    mut ia_drc_params_struct: *mut ia_drc_params_struct,
    mut pstr_gain_buf: *mut ia_gain_buffer_struct,
    mut shape_filter_block: *mut shape_filter_block,
    mut deinterleaved_audio: *mut *mut FLOAT32,
    mut channel_audio: *mut *mut FLOAT32,
    mut impd_apply_gains: WORD32,
) -> VOID {
    let mut c: WORD32 = 0;
    let mut b: WORD32 = 0;
    let mut g: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut offset: WORD32 = 0 as WORD32;
    let mut signal_index: WORD32 = 0 as WORD32;
    let mut gain_index_for_group: [WORD32; 24] = [0; 24];
    let mut signal_index_for_channel: [WORD32; 8] = [0; 8];
    let mut lpcm_gains: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut sum: FLOAT32 = 0.;
    let mut drc_gain_last: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
    let mut gain_thr: FLOAT32 = 0.;
    let mut i_end: WORD32 = 0;
    let mut i_start: WORD32 = 0;
    let mut str_drc_instruction_str: *mut ia_drc_instructions_struct = &mut *pstr_drc_instruction_arr
        .offset(drc_instructions_index as isize) as *mut ia_drc_instructions_struct;
    if drc_instructions_index >= 0 as core::ffi::c_int {
        str_drc_instruction_str = &mut *pstr_drc_instruction_arr
            .offset(drc_instructions_index as isize) as *mut ia_drc_instructions_struct;
        if (*str_drc_instruction_str).drc_set_id > 0 as core::ffi::c_int {
            if (*ia_drc_params_struct).delay_mode == DELAY_MODE_LOW_DELAY {
                offset = (*ia_drc_params_struct).drc_frame_size;
            }
            gain_index_for_group[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                as WORD32;
            g = 0 as core::ffi::c_int as WORD32;
            while g
                < (*str_drc_instruction_str).num_drc_ch_groups as core::ffi::c_int
                    - 1 as core::ffi::c_int
            {
                gain_index_for_group[(g as core::ffi::c_int + 1 as core::ffi::c_int)
                    as usize] = gain_index_for_group[g as usize]
                    + (*str_drc_instruction_str).band_count_of_ch_group[g as usize];
                g += 1;
            }
            signal_index_for_channel[0 as core::ffi::c_int as usize] = 0
                as core::ffi::c_int as WORD32;
            c = 0 as core::ffi::c_int as WORD32;
            while c
                < (*str_drc_instruction_str).audio_num_chan as core::ffi::c_int
                    - 1 as core::ffi::c_int
            {
                if (*str_drc_instruction_str).channel_group_of_ch[c as usize]
                    >= 0 as core::ffi::c_int
                {
                    signal_index_for_channel[(c as core::ffi::c_int
                        + 1 as core::ffi::c_int) as usize] = signal_index_for_channel[c
                        as usize]
                        + (*str_drc_instruction_str)
                            .band_count_of_ch_group[(*str_drc_instruction_str)
                            .channel_group_of_ch[c as usize] as usize];
                } else {
                    signal_index_for_channel[(c as core::ffi::c_int
                        + 1 as core::ffi::c_int) as usize] = (signal_index_for_channel[c
                        as usize] + 1 as core::ffi::c_int) as WORD32;
                }
                c += 1;
            }
            g = 0 as core::ffi::c_int as WORD32;
            while g < (*str_drc_instruction_str).num_drc_ch_groups {
                b = 0 as core::ffi::c_int as WORD32;
                while b < (*str_drc_instruction_str).band_count_of_ch_group[g as usize] {
                    if (*str_drc_instruction_str)
                        .ch_group_parametric_drc_flag[g as usize]
                        == 0 as core::ffi::c_int
                    {
                        lpcm_gains = ((*((*pstr_gain_buf).buf_interpolation)
                            .offset((gain_index_for_group[g as usize] + b) as isize))
                            .lpcm_gains)
                            .as_mut_ptr()
                            .offset(MAX_SIGNAL_DELAY as isize)
                            .offset(
                                -((*ia_drc_params_struct).gain_delay_samples as isize),
                            )
                            .offset(
                                -((*ia_drc_params_struct).audio_delay_samples as isize),
                            )
                            .offset(offset as isize);
                    } else {
                        lpcm_gains = ((*((*pstr_gain_buf).buf_interpolation)
                            .offset((gain_index_for_group[g as usize] + b) as isize))
                            .lpcm_gains)
                            .as_mut_ptr()
                            .offset(MAX_SIGNAL_DELAY as isize)
                            .offset(
                                (*str_drc_instruction_str)
                                    .parametric_drc_look_ahead_samples[g as usize] as isize,
                            )
                            .offset(
                                -((*ia_drc_params_struct).audio_delay_samples as isize),
                            );
                    }
                    i_end = 0 as core::ffi::c_int as WORD32;
                    i_start = 0 as core::ffi::c_int as WORD32;
                    while i_end < (*ia_drc_params_struct).drc_frame_size {
                        if (*shape_filter_block.offset(g as isize))
                            .shape_flter_block_flag != 0
                        {
                            drc_gain_last = (*shape_filter_block.offset(g as isize))
                                .drc_gain_last;
                            gain_thr = 0.0001f32 * drc_gain_last;
                            while i_end < (*ia_drc_params_struct).drc_frame_size
                                && fabs(
                                    (*lpcm_gains.offset(i_end as isize) - drc_gain_last)
                                        as core::ffi::c_double,
                                ) <= gain_thr as core::ffi::c_double
                            {
                                i_end += 1;
                            }
                        } else {
                            i_end = (*ia_drc_params_struct).drc_frame_size;
                        }
                        c = 0 as core::ffi::c_int as WORD32;
                        while c < (*str_drc_instruction_str).audio_num_chan {
                            if g
                                == (*str_drc_instruction_str)
                                    .channel_group_of_ch[c as usize]
                            {
                                signal_index = signal_index_for_channel[c as usize] + b;
                                if impd_apply_gains == 1 as core::ffi::c_int {
                                    impd_shape_filt_block_time_process(
                                        &mut *shape_filter_block.offset(g as isize),
                                        &mut *lpcm_gains.offset(0 as core::ffi::c_int as isize),
                                        signal_index,
                                        &mut *(*deinterleaved_audio.offset(signal_index as isize))
                                            .offset(0 as core::ffi::c_int as isize),
                                        i_start,
                                        i_end,
                                    );
                                } else {
                                    i = i_start;
                                    while i < i_end {
                                        *(*deinterleaved_audio.offset(signal_index as isize))
                                            .offset(i as isize) = *lpcm_gains.offset(i as isize);
                                        i += 1;
                                    }
                                }
                            }
                            c += 1;
                        }
                        if i_end < (*ia_drc_params_struct).drc_frame_size
                            && (*shape_filter_block.offset(g as isize))
                                .shape_flter_block_flag != 0
                        {
                            impd_shape_filt_block_adapt(
                                *lpcm_gains.offset(i_end as isize),
                                &mut *shape_filter_block.offset(g as isize),
                            );
                        }
                        if i_end == i_start
                            && drc_gain_last
                                == (*shape_filter_block.offset(g as isize)).drc_gain_last
                        {
                            break;
                        }
                        i_start = i_end;
                    }
                    b += 1;
                }
                g += 1;
            }
        }
    }
    signal_index = 0 as core::ffi::c_int as WORD32;
    if (*str_drc_instruction_str).drc_set_id > 0 as core::ffi::c_int {
        c = 0 as core::ffi::c_int as WORD32;
        while c < (*str_drc_instruction_str).audio_num_chan {
            g = (*str_drc_instruction_str).channel_group_of_ch[c as usize];
            if g >= 0 as core::ffi::c_int {
                i = 0 as core::ffi::c_int as WORD32;
                while i < (*ia_drc_params_struct).drc_frame_size {
                    sum = 0.0f32 as FLOAT32;
                    b = 0 as core::ffi::c_int as WORD32;
                    while b
                        < (*str_drc_instruction_str).band_count_of_ch_group[g as usize]
                    {
                        sum
                            += *(*deinterleaved_audio
                                .offset((signal_index + b) as isize))
                                .offset(i as isize);
                        b += 1;
                    }
                    *(*channel_audio.offset(c as isize)).offset(i as isize) = sum;
                    i += 1;
                }
                signal_index
                    += (*str_drc_instruction_str).band_count_of_ch_group[g as usize];
            } else {
                i = 0 as core::ffi::c_int as WORD32;
                while i < (*ia_drc_params_struct).drc_frame_size {
                    *(*channel_audio.offset(c as isize)).offset(i as isize) = *(*deinterleaved_audio
                        .offset(signal_index as isize))
                        .offset(i as isize);
                    i += 1;
                }
                signal_index += 1;
            }
            c += 1;
        }
    } else {
        c = 0 as core::ffi::c_int as WORD32;
        while c < (*str_drc_instruction_str).audio_num_chan {
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*ia_drc_params_struct).drc_frame_size {
                *(*channel_audio.offset(c as isize)).offset(i as isize) = *(*deinterleaved_audio
                    .offset(c as isize))
                    .offset(i as isize);
                i += 1;
            }
            c += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn impd_filter_banks_process(
    mut pstr_drc_instruction_arr: *mut ia_drc_instructions_struct,
    drc_instructions_index: WORD32,
    mut ia_drc_params_struct: *mut ia_drc_params_struct,
    mut audio_io_buf: *mut *mut FLOAT32,
    mut audio_band_buffer: *mut ia_audio_band_buffer_struct,
    mut ia_filter_banks_struct: *mut ia_filter_banks_struct,
    passThru: WORD32,
) -> WORD32 {
    let mut c: WORD32 = 0;
    let mut g: WORD32 = 0;
    let mut e: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut num_bands: WORD32 = 0;
    let mut audio_in: *mut FLOAT32 = 0 as *mut FLOAT32;
    let mut audio_out: *mut *mut FLOAT32 = 0 as *mut *mut FLOAT32;
    let mut str_drc_filter_bank: *mut ia_drc_filter_bank_struct = 0
        as *mut ia_drc_filter_bank_struct;
    let mut str_drc_instruction_str: *mut ia_drc_instructions_struct = 0
        as *mut ia_drc_instructions_struct;
    let mut drc_frame_size: WORD32 = (*ia_drc_params_struct).drc_frame_size;
    if drc_instructions_index >= 0 as core::ffi::c_int {
        str_drc_instruction_str = &mut *pstr_drc_instruction_arr
            .offset(drc_instructions_index as isize) as *mut ia_drc_instructions_struct;
    } else {
        return -(1 as WORD32)
    }
    e = 0 as core::ffi::c_int as WORD32;
    c = 0 as core::ffi::c_int as WORD32;
    while c < (*str_drc_instruction_str).audio_num_chan {
        str_drc_filter_bank = 0 as *mut ia_drc_filter_bank_struct;
        audio_in = *audio_io_buf.offset(c as isize);
        audio_out = &mut *((*audio_band_buffer).non_interleaved_audio).offset(e as isize)
            as *mut *mut FLOAT32;
        if passThru == 0 as core::ffi::c_int
            && drc_instructions_index >= 0 as core::ffi::c_int
        {
            if (*str_drc_instruction_str).drc_set_id < 0 as core::ffi::c_int {
                num_bands = 1 as core::ffi::c_int as WORD32;
            } else {
                g = (*str_drc_instruction_str).channel_group_of_ch[c as usize];
                if g == -(1 as core::ffi::c_int) {
                    num_bands = 1 as core::ffi::c_int as WORD32;
                    str_drc_filter_bank = &mut *((*ia_filter_banks_struct)
                        .str_drc_filter_bank)
                        .as_mut_ptr()
                        .offset((*str_drc_instruction_str).num_drc_ch_groups as isize)
                        as *mut ia_drc_filter_bank_struct;
                } else {
                    num_bands = (*str_drc_instruction_str)
                        .band_count_of_ch_group[g as usize];
                    str_drc_filter_bank = &mut *((*ia_filter_banks_struct)
                        .str_drc_filter_bank)
                        .as_mut_ptr()
                        .offset(g as isize) as *mut ia_drc_filter_bank_struct;
                }
                impd_all_pass_cascade_process(
                    &mut (*str_drc_filter_bank).str_all_pass_cascade,
                    c,
                    drc_frame_size,
                    audio_in,
                );
            }
        } else {
            num_bands = 1 as core::ffi::c_int as WORD32;
        }
        match num_bands {
            1 => {
                i = 0 as core::ffi::c_int as WORD32;
                while i < drc_frame_size {
                    *(*audio_out.offset(0 as core::ffi::c_int as isize))
                        .offset(i as isize) = *audio_in.offset(i as isize);
                    i += 1;
                }
                e += 1;
            }
            2 => {
                impd_two_band_filter_process(
                    &mut (*str_drc_filter_bank).str_two_band_bank,
                    c,
                    drc_frame_size,
                    audio_in,
                    audio_out as *mut *mut FLOAT32,
                );
                e += 2 as core::ffi::c_int;
            }
            3 => {
                impd_three_band_filter_process(
                    &mut (*str_drc_filter_bank).str_three_band_bank,
                    c,
                    drc_frame_size,
                    audio_in,
                    audio_out as *mut *mut FLOAT32,
                );
                e += 3 as core::ffi::c_int;
            }
            4 => {
                impd_four_band_filter_process(
                    &mut (*str_drc_filter_bank).str_four_band_bank,
                    c,
                    drc_frame_size,
                    audio_in,
                    audio_out as *mut *mut FLOAT32,
                );
                e += 4 as core::ffi::c_int;
            }
            _ => return 3 as WORD32,
        }
        c += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_store_audio_io_buffer_time(
    mut audio_in_out_buf: *mut *mut FLOAT32,
    mut audio_io_buf_internal: *mut ia_audio_in_out_buf,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    if (*audio_io_buf_internal).audio_delay_samples != 0 {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*audio_io_buf_internal).audio_num_chan {
            j = 0 as core::ffi::c_int as WORD32;
            while j < (*audio_io_buf_internal).frame_size {
                *(*((*audio_io_buf_internal).audio_io_buffer_delayed).offset(i as isize))
                    .offset(
                        ((*audio_io_buf_internal).audio_delay_samples + j) as isize,
                    ) = *(*audio_in_out_buf.offset(i as isize)).offset(j as isize);
                j += 1;
            }
            i += 1;
        }
    } else {
        (*audio_io_buf_internal).audio_io_buffer_delayed = audio_in_out_buf
            as *mut *mut FLOAT32;
        (*audio_io_buf_internal).audio_in_out_buf = audio_in_out_buf
            as *mut *mut FLOAT32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn impd_retrieve_audio_io_buffer_time(
    mut audio_in_out_buf: *mut *mut FLOAT32,
    mut audio_io_buf_internal: *mut ia_audio_in_out_buf,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    if (*audio_io_buf_internal).audio_delay_samples != 0 {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*audio_io_buf_internal).audio_num_chan {
            j = 0 as core::ffi::c_int as WORD32;
            while j < (*audio_io_buf_internal).frame_size {
                *(*audio_in_out_buf.offset(i as isize)).offset(j as isize) = *(*((*audio_io_buf_internal)
                    .audio_io_buffer_delayed)
                    .offset(i as isize))
                    .offset(j as isize);
                j += 1;
            }
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn impd_advance_audio_io_buffer_time(
    mut audio_io_buf_internal: *mut ia_audio_in_out_buf,
) -> VOID {
    let mut i: WORD32 = 0;
    if (*audio_io_buf_internal).audio_delay_samples != 0 {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*audio_io_buf_internal).audio_num_chan {
            memmove(
                *((*audio_io_buf_internal).audio_io_buffer_delayed).offset(i as isize)
                    as *mut core::ffi::c_void,
                &mut *(*((*audio_io_buf_internal).audio_io_buffer_delayed)
                    .offset(i as isize))
                    .offset((*audio_io_buf_internal).frame_size as isize) as *mut FLOAT32
                    as *const core::ffi::c_void,
                (::core::mem::size_of::<FLOAT32>() as size_t)
                    .wrapping_mul((*audio_io_buf_internal).audio_delay_samples as size_t),
            );
            i += 1;
        }
    }
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
