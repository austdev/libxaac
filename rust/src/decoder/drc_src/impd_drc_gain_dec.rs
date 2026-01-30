extern "C" {
    fn pow(__x: core::ffi::c_double, __y: core::ffi::c_double) -> core::ffi::c_double;
    fn fabs(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn memmove(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn impd_parametric_drc_instance_process(
        audio_in_out_buf: *mut *mut FLOAT32,
        audio_real_buff: *mut *mut FLOAT32,
        audio_imag_buff: *mut *mut FLOAT32,
        pstr_parametric_drc_params: *mut ia_parametric_drc_params_struct,
        pstr_parametric_drc_instance_params: *mut ia_parametric_drc_instance_params_struct,
    ) -> WORD32;
    fn impd_parametric_lim_type_drc_process(
        audio_in_out_buf: *mut *mut FLOAT32,
        loudness_normalization_gain_db: FLOAT32,
        pstr_parametric_lim_type_drc_params: *mut ia_parametric_drc_type_lim_params_struct,
        lpcm_gains: *mut FLOAT32,
    ) -> VOID;
}
pub type size_t = usize;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type FLOAT32 = core::ffi::c_float;
pub type FLOAT64 = core::ffi::c_double;
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
pub struct ia_parametric_drc_type_feed_forward_struct {
    pub level_estim_k_weighting_type: WORD32,
    pub level_estim_integration_time_present: WORD32,
    pub level_estim_integration_time: WORD32,
    pub drc_curve_definition_type: WORD32,
    pub drc_characteristic: WORD32,
    pub node_count: WORD32,
    pub node_level: [WORD32; 9],
    pub node_gain: [WORD32; 9],
    pub drc_gain_smooth_parameters_present: WORD32,
    pub gain_smooth_attack_time_slow: WORD32,
    pub gain_smooth_release_time_slow: WORD32,
    pub gain_smooth_time_fast_present: WORD32,
    pub gain_smooth_attack_time_fast: WORD32,
    pub gain_smooth_release_time_fast: WORD32,
    pub gain_smooth_threshold_present: WORD32,
    pub gain_smooth_attack_threshold: WORD32,
    pub gain_smooth_rel_threshold: WORD32,
    pub gain_smooth_hold_off_count_present: WORD32,
    pub gain_smooth_hold_off: WORD32,
    pub disable_paramteric_drc: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_parametric_drc_lim_struct {
    pub parametric_lim_threshold_present: WORD32,
    pub parametric_lim_threshold: FLOAT32,
    pub parametric_lim_attack: WORD32,
    pub parametric_lim_release_present: WORD32,
    pub parametric_lim_release: WORD32,
    pub drc_characteristic: WORD32,
    pub disable_paramteric_drc: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_parametric_drc_instructions_struct {
    pub parametric_drc_id: WORD32,
    pub parametric_drc_look_ahead_flag: WORD32,
    pub parametric_drc_look_ahead: WORD32,
    pub parametric_drc_preset_id_present: WORD32,
    pub parametric_drc_preset_id: WORD32,
    pub parametric_drc_type: WORD32,
    pub len_bit_size: WORD32,
    pub str_parametric_drc_type_feed_forward: ia_parametric_drc_type_feed_forward_struct,
    pub parametric_drc_lim: ia_parametric_drc_lim_struct,
    pub drc_characteristic: WORD32,
    pub disable_paramteric_drc: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_parametric_drc_gain_set_params_struct {
    pub parametric_drc_id: WORD32,
    pub side_chain_config_type: WORD32,
    pub downmix_id: WORD32,
    pub level_estim_channel_weight_format: WORD32,
    pub level_estim_ch_weight: [FLOAT32; 8],
    pub drc_input_loudness_present: WORD32,
    pub drc_input_loudness: FLOAT32,
    pub ch_count_from_dwnmix_id: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_coeff_parametric_drc_struct {
    pub drc_location: WORD32,
    pub parametric_drc_frame_size_format: WORD32,
    pub parametric_drc_frame_size: WORD32,
    pub parametric_drc_delay_max_present: WORD32,
    pub parametric_drc_delay_max: WORD32,
    pub reset_parametric_drc: WORD32,
    pub parametric_drc_gain_set_count: WORD32,
    pub str_parametric_drc_gain_set_params: [ia_parametric_drc_gain_set_params_struct; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_channel_layout_struct {
    pub base_channel_count: WORD32,
    pub layout_signaling_present: WORD32,
    pub defined_layout: WORD32,
    pub speaker_position: [WORD32; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_downmix_instructions_struct {
    pub downmix_id: WORD32,
    pub target_channel_count: WORD32,
    pub target_layout: WORD32,
    pub downmix_coefficients_present: WORD32,
    pub downmix_coefficient: [FLOAT32; 1024],
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
pub struct ia_drc_coefficients_basic_struct {
    pub drc_location: WORD32,
    pub drc_characteristic: WORD32,
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
pub struct ia_drc_instructions_basic_struct {
    pub drc_set_id: WORD32,
    pub drc_location: WORD32,
    pub dwnmix_id_count: WORD32,
    pub downmix_id: [WORD32; 8],
    pub drc_set_effect: WORD32,
    pub limiter_peak_target_present: WORD32,
    pub limiter_peak_target: FLOAT32,
    pub drc_set_target_loudness_present: WORD32,
    pub drc_set_target_loudness_value_upper: WORD32,
    pub drc_set_target_loudness_value_lower_present: WORD32,
    pub drc_set_target_loudness_value_lower: WORD32,
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
pub struct ia_loud_eq_instructions_struct {
    pub loud_eq_set_id: WORD32,
    pub drc_location: WORD32,
    pub dwnmix_id_count: WORD32,
    pub downmix_id: [WORD32; 8],
    pub drc_set_id_count: WORD32,
    pub drc_set_id: [WORD32; 16],
    pub eq_set_id_count: WORD32,
    pub eq_set_id: [WORD32; 8],
    pub loudness_after_drc: WORD32,
    pub loudness_after_eq: WORD32,
    pub loud_eq_gain_sequence_count: WORD32,
    pub gain_seq_idx: [WORD32; 4],
    pub drc_characteristic_format_is_cicp: [WORD32; 4],
    pub drc_characteristic: [WORD32; 4],
    pub drc_characteristic_left_index: [WORD32; 4],
    pub drc_characteristic_right_index: [WORD32; 4],
    pub frequency_range_index: [WORD32; 4],
    pub loud_eq_scaling: [FLOAT32; 4],
    pub loud_eq_offset: [FLOAT32; 4],
}
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
pub struct ia_drc_config_ext {
    pub drc_config_ext_type: [WORD32; 8],
    pub ext_bit_size: [WORD32; 7],
    pub parametric_drc_present: WORD32,
    pub str_drc_coeff_param_drc: ia_drc_coeff_parametric_drc_struct,
    pub parametric_drc_instructions_count: WORD32,
    pub str_parametric_drc_instructions: [ia_parametric_drc_instructions_struct; 8],
    pub drc_extension_v1_present: WORD32,
    pub loud_eq_instructions_flag: WORD32,
    pub loud_eq_instructions_count: WORD32,
    pub loud_eq_instructions: [ia_loud_eq_instructions_struct; 8],
    pub eq_flag: WORD32,
    pub str_eq_coeff: ia_eq_coeff_struct,
    pub eq_instructions_count: WORD32,
    pub str_eq_instructions: [ia_eq_instructions_struct; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_config {
    pub sample_rate_present: WORD32,
    pub sampling_rate: WORD32,
    pub dwnmix_instructions_count: WORD32,
    pub drc_coefficients_drc_count: WORD32,
    pub drc_instructions_uni_drc_count: WORD32,
    pub drc_instructions_count_plus: WORD32,
    pub drc_description_basic_present: WORD32,
    pub drc_coefficients_basic_count: WORD32,
    pub drc_instructions_basic_count: WORD32,
    pub drc_config_ext_present: WORD32,
    pub apply_drc: WORD32,
    pub str_drc_config_ext: ia_drc_config_ext,
    pub str_drc_coefficients_basic: [ia_drc_coefficients_basic_struct; 8],
    pub str_drc_instructions_basic: [ia_drc_instructions_basic_struct; 36],
    pub str_p_loc_drc_coefficients_uni_drc: [ia_uni_drc_coeffs_struct; 8],
    pub str_drc_instruction_str: [ia_drc_instructions_struct; 36],
    pub channel_layout: ia_channel_layout_struct,
    pub dwnmix_instructions: [ia_downmix_instructions_struct; 16],
    pub is_config_changed: WORD32,
    pub ln_gain_changed: WORD32,
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
pub struct ia_spline_nodes_struct {
    pub drc_gain_coding_mode: WORD32,
    pub num_nodes: WORD32,
    pub str_node: [ia_node_struct; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_gain_sequence_struct {
    pub str_spline_nodes: [ia_spline_nodes_struct; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_uni_drc_gain_ext_struct {
    pub uni_drc_gain_ext_type: [WORD32; 8],
    pub ext_bit_size: [WORD32; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_gain_struct {
    pub num_drc_gain_sequences: WORD32,
    pub drc_gain_sequence: [ia_drc_gain_sequence_struct; 24],
    pub uni_drc_gain_ext_flag: WORD32,
    pub uni_drc_gain_ext: ia_uni_drc_gain_ext_struct,
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
pub struct ia_drc_gain_buffers_struct {
    pub pstr_gain_buf: [ia_gain_buffer_struct; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_interp_params_struct {
    pub gain_interpolation_type: WORD32,
    pub gain_modification_flag: WORD32,
    pub ducking_flag: WORD32,
    pub clipping_flag: WORD32,
    pub pstr_ducking_modifiers: *mut ia_ducking_modifiers_struct,
    pub pstr_gain_modifiers: *mut ia_gain_modifiers_struct,
    pub drc_characteristic_present: WORD32,
    pub drc_source_characteristic_cicp_format: WORD32,
    pub source_drc_characteristic: WORD32,
    pub split_source_characteristic_left: *mut ia_split_drc_characteristic_struct,
    pub split_source_characteristic_right: *mut ia_split_drc_characteristic_struct,
    pub drc_target_characteristic_cicp_format: WORD32,
    pub target_drc_characteristic: WORD32,
    pub split_target_characteristic_left: *mut ia_split_drc_characteristic_struct,
    pub split_target_characteristic_right: *mut ia_split_drc_characteristic_struct,
    pub interpolation_loud_eq: WORD32,
    pub limiter_peak_target_present: WORD32,
    pub limiter_peak_target: FLOAT32,
    pub loudness_normalization_gain_db: FLOAT32,
    pub delta_tmin: WORD32,
    pub characteristic_index: WORD32,
    pub compress: FLOAT32,
    pub boost: FLOAT32,
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
pub struct ia_2nd_order_filt_coeff_struct_t {
    pub b0: FLOAT32,
    pub b1: FLOAT32,
    pub b2: FLOAT32,
    pub a1: FLOAT32,
    pub a2: FLOAT32,
}
pub type ia_2nd_order_filt_coeff_struct = ia_2nd_order_filt_coeff_struct_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_2nd_order_filt_state_struct_t {
    pub z1: FLOAT32,
    pub z2: FLOAT32,
}
pub type ia_2nd_order_filt_state_struct = ia_2nd_order_filt_state_struct_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_parametric_drc_type_ff_params_struct_t {
    pub audio_num_chan: WORD32,
    pub frame_size: WORD32,
    pub sub_band_domain_mode: WORD32,
    pub sub_band_count: WORD32,
    pub level_estim_integration_time: WORD32,
    pub level_estim_frame_index: WORD32,
    pub level_estim_frame_count: WORD32,
    pub level: [FLOAT32; 64],
    pub start_up_phase: WORD32,
    pub level_estim_ch_weight: [FLOAT32; 8],
    pub level_estim_k_weighting_type: WORD32,
    pub pre_filt_coeff: ia_2nd_order_filt_coeff_struct,
    pub rlb_filt_coeff: ia_2nd_order_filt_coeff_struct,
    pub pre_filt_state: [ia_2nd_order_filt_state_struct; 8],
    pub rlb_filt_state: [ia_2nd_order_filt_state_struct; 8],
    pub weighting_filt: [FLOAT32; 256],
    pub sub_band_compensation_type: WORD32,
    pub filt_coeff_subband: ia_2nd_order_filt_coeff_struct,
    pub filt_state_subband_real: [ia_2nd_order_filt_state_struct; 8],
    pub filt_state_subband_imag: [ia_2nd_order_filt_state_struct; 8],
    pub ref_level_parametric_drc: FLOAT32,
    pub node_count: WORD32,
    pub node_level: [WORD32; 9],
    pub node_gain: [WORD32; 9],
    pub gain_smooth_attack_alpha_slow: FLOAT32,
    pub gain_smooth_rel_alpha_slow: FLOAT32,
    pub gain_smooth_attack_alpha_fast: FLOAT32,
    pub gain_smooth_rel_alpha_fast: FLOAT32,
    pub gain_smooth_attack_threshold: WORD32,
    pub gain_smooth_rel_threshold: WORD32,
    pub gain_smooth_hold_off_count: WORD32,
    pub db_level_smooth: FLOAT32,
    pub db_gain_smooth: FLOAT32,
    pub hold_counter: WORD32,
}
pub type ia_parametric_drc_type_ff_params_struct = ia_parametric_drc_type_ff_params_struct_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_parametric_drc_type_lim_params_struct_t {
    pub audio_num_chan: WORD32,
    pub frame_size: WORD32,
    pub level_estim_ch_weight: [FLOAT32; 8],
    pub attack: UWORD32,
    pub attack_constant: FLOAT32,
    pub release_constant: FLOAT32,
    pub attack_ms: FLOAT32,
    pub release_ms: FLOAT32,
    pub threshold: FLOAT32,
    pub channels: UWORD32,
    pub sampling_rate: UWORD32,
    pub cor: FLOAT32,
    pub max_buf: *mut FLOAT32,
    pub max_buf_slow: *mut FLOAT32,
    pub max_buf_idx: UWORD32,
    pub max_buf_slow_idx: UWORD32,
    pub sec_len: UWORD32,
    pub num_max_buf_sec: UWORD32,
    pub max_buf_sec_idx: UWORD32,
    pub max_buf_sec_ctr: UWORD32,
    pub smooth_state_0: FLOAT64,
}
pub type ia_parametric_drc_type_lim_params_struct = ia_parametric_drc_type_lim_params_struct_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_parametric_drc_instance_params_struct_t {
    pub disable_paramteric_drc: WORD32,
    pub parametric_drc_type: WORD32,
    pub str_spline_nodes: ia_spline_nodes_struct,
    pub str_parametric_drc_type_ff_params: ia_parametric_drc_type_ff_params_struct,
    pub str_parametric_drc_type_lim_params: ia_parametric_drc_type_lim_params_struct,
}
pub type ia_parametric_drc_instance_params_struct = ia_parametric_drc_instance_params_struct_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_parametric_drc_params_struct_t {
    pub sampling_rate: WORD32,
    pub audio_num_chan: WORD32,
    pub sub_band_domain_mode: WORD32,
    pub sub_band_count: WORD32,
    pub num_nodes: WORD32,
    pub drc_frame_size: WORD32,
    pub parametric_drc_frame_size: WORD32,
    pub parametric_drc_look_ahead_samples_max: WORD32,
    pub reset_parametric_drc: WORD32,
    pub parametric_drc_instance_count: WORD32,
    pub parametric_drc_idx: [WORD32; 8],
    pub gain_set_index: [WORD32; 8],
    pub dwnmix_id_from_drc_instructions: [WORD32; 8],
    pub channel_map: [[WORD32; 8]; 8],
    pub str_parametric_drc_instance_params: [ia_parametric_drc_instance_params_struct; 8],
}
pub type ia_parametric_drc_params_struct = ia_parametric_drc_params_struct_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_audio_delay_struct {
    pub delay: WORD32,
    pub state: [[FLOAT32; 189]; 8],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_gain_dec_struct {
    pub audio_num_chan: WORD32,
    pub ia_drc_params_struct: ia_drc_params_struct,
    pub drc_gain_buffers: ia_drc_gain_buffers_struct,
    pub audio_band_buffer: ia_audio_band_buffer_struct,
    pub str_overlap_params: ia_overlap_params_struct,
    pub ia_filter_banks_struct: ia_filter_banks_struct,
    pub audio_in_out_buf: ia_audio_in_out_buf,
    pub parametricdrc_params: ia_parametric_drc_params_struct,
    pub shape_filter_block: [shape_filter_block; 24],
    pub eq_set: *mut ia_eq_set_struct,
}
pub const MAX_SIGNAL_DELAY: core::ffi::c_int = 4500 as core::ffi::c_int;
pub const UNEXPECTED_ERROR: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SUBBAND_DOMAIN_MODE_OFF: core::ffi::c_int = 0 as core::ffi::c_int;
pub const PARAM_DRC_TYPE_LIM: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const SLOPE_FACTOR_DB_TO_LINEAR: core::ffi::c_float = 0.1151f32;
pub const EFFECT_BIT_CLIPPING: core::ffi::c_int = 0x100 as core::ffi::c_int;
pub const EFFECT_BIT_FADE: core::ffi::c_int = 0x200 as core::ffi::c_int;
pub const EFFECT_BIT_DUCK_OTHER: core::ffi::c_int = 0x400 as core::ffi::c_int;
pub const EFFECT_BIT_DUCK_SELF: core::ffi::c_int = 0x800 as core::ffi::c_int;
pub const GAIN_INTERPOLATION_TYPE_SPLINE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const CHARACTERISTIC_SIGMOID: core::ffi::c_int = 0;
pub const CHARACTERISTIC_NODES: core::ffi::c_int = 1;
pub const CHARACTERISTIC_PASS_THRU: core::ffi::c_int = 2;
pub const DRC_INPUT_LOUDNESS_TARGET: core::ffi::c_float = -31.0f32;
#[no_mangle]
pub unsafe extern "C" fn impd_gain_db_to_lin(
    mut interp_params_str: *mut ia_interp_params_struct,
    mut drc_band: WORD32,
    mut in_param_db_gain: FLOAT32,
    mut in_param_db_slope: FLOAT32,
    mut out_param_lin_gain: *mut FLOAT32,
    mut out_param_lin_slope: *mut FLOAT32,
) -> VOID {
    let mut loc_db_gain: FLOAT32 = in_param_db_gain;
    let mut gain_ratio: FLOAT32 = 1.0f32;
    let mut pstr_gain_modifiers: *mut ia_gain_modifiers_struct = (*interp_params_str)
        .pstr_gain_modifiers;
    if (*interp_params_str).gain_modification_flag != 0 {
        if (*interp_params_str).characteristic_index > 0 as core::ffi::c_int
            && loc_db_gain != 0.0f32
        {
            gain_ratio = 1.0f32 as FLOAT32;
        }
        if loc_db_gain < 0.0f32 {
            gain_ratio *= (*interp_params_str).compress;
        } else {
            gain_ratio *= (*interp_params_str).boost;
        }
    }
    if (*pstr_gain_modifiers).gain_scaling_flag[drc_band as usize]
        == 1 as core::ffi::c_int
    {
        if (loc_db_gain as core::ffi::c_double) < 0.0f64 {
            gain_ratio *= (*pstr_gain_modifiers).attn_scaling[drc_band as usize];
        } else {
            gain_ratio *= (*pstr_gain_modifiers).ampl_scaling[drc_band as usize];
        }
    }
    if (*(*interp_params_str).pstr_ducking_modifiers).ducking_scaling_flag
        == 1 as core::ffi::c_int
        && (*interp_params_str).ducking_flag == 1 as core::ffi::c_int
    {
        gain_ratio *= (*(*interp_params_str).pstr_ducking_modifiers).ducking_scaling;
    }
    *out_param_lin_gain = pow(
        2.0f64,
        (gain_ratio as core::ffi::c_float * loc_db_gain as core::ffi::c_float / 6.0f32)
            as core::ffi::c_double,
    ) as FLOAT32;
    *out_param_lin_slope = SLOPE_FACTOR_DB_TO_LINEAR * gain_ratio * *out_param_lin_gain
        * in_param_db_slope;
    if (*pstr_gain_modifiers).gain_offset_flag[drc_band as usize]
        == 1 as core::ffi::c_int
    {
        *out_param_lin_gain
            *= pow(
                2.0f64,
                ((*pstr_gain_modifiers).gain_offset[drc_band as usize] / 6.0f32)
                    as core::ffi::c_double,
            ) as FLOAT32;
    }
    if (*interp_params_str).limiter_peak_target_present == 1 as core::ffi::c_int
        && (*interp_params_str).clipping_flag == 1 as core::ffi::c_int
    {
        *out_param_lin_gain
            *= pow(
                2.0f64,
                (if 0.0f64
                    > (-(*interp_params_str).limiter_peak_target
                        - (*interp_params_str).loudness_normalization_gain_db)
                        as core::ffi::c_double
                {
                    0.0f64
                } else {
                    (-(*interp_params_str).limiter_peak_target
                        - (*interp_params_str).loudness_normalization_gain_db)
                        as core::ffi::c_double
                }) / 6.0f64,
            ) as FLOAT32;
        if *out_param_lin_gain as core::ffi::c_double >= 1.0f64 {
            *out_param_lin_gain = 1.0f32;
            *out_param_lin_slope = 0.0f32;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn impd_compressor_io_sigmoid(
    mut split_drc_characteristic: *mut ia_split_drc_characteristic_struct,
    mut in_db_level: FLOAT32,
    mut out_db_gain: *mut FLOAT32,
) -> WORD32 {
    let mut tmp: FLOAT32 = 0.;
    let mut in_out_ratio: FLOAT32 = (*split_drc_characteristic).in_out_ratio;
    let mut gainDbLimit: FLOAT32 = (*split_drc_characteristic).gain;
    let mut exp: FLOAT32 = (*split_drc_characteristic).exp;
    tmp = (DRC_INPUT_LOUDNESS_TARGET - in_db_level) * in_out_ratio;
    if exp < 1000.0f32 {
        let mut x: FLOAT32 = tmp / gainDbLimit;
        if x < 0.0f32 {
            return 2 as WORD32;
        }
        *out_db_gain = (tmp as core::ffi::c_double
            / pow(
                1.0f64 + pow(x as core::ffi::c_double, exp as core::ffi::c_double),
                (1.0f32 / exp) as core::ffi::c_double,
            )) as FLOAT32;
    } else {
        *out_db_gain = tmp;
    }
    if (*split_drc_characteristic).flip_sign == 1 as core::ffi::c_int {
        *out_db_gain = -*out_db_gain;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_compressor_io_sigmoid_inv(
    mut split_drc_characteristic: *mut ia_split_drc_characteristic_struct,
    mut loc_db_gain: FLOAT32,
    mut in_level: *mut FLOAT32,
) -> WORD32 {
    let mut in_out_ratio: FLOAT32 = (*split_drc_characteristic).in_out_ratio;
    let mut gainDbLimit: FLOAT32 = (*split_drc_characteristic).gain;
    let mut exp: FLOAT32 = (*split_drc_characteristic).exp;
    let mut tmp: FLOAT32 = loc_db_gain;
    if (*split_drc_characteristic).flip_sign == 1 as core::ffi::c_int {
        tmp = -loc_db_gain;
    }
    if exp < 1000.0f32 {
        let mut x: FLOAT32 = tmp / gainDbLimit;
        if x < 0.0f32 {
            return 2 as WORD32;
        }
        tmp = (tmp as core::ffi::c_double
            / pow(
                1.0f64 - pow(x as core::ffi::c_double, exp as core::ffi::c_double),
                (1.0f32 / exp) as core::ffi::c_double,
            )) as FLOAT32;
    }
    *in_level = DRC_INPUT_LOUDNESS_TARGET - tmp / in_out_ratio;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_compressor_io_nodes_lt(
    mut split_drc_characteristic: *mut ia_split_drc_characteristic_struct,
    mut in_db_level: FLOAT32,
    mut out_db_gain: *mut FLOAT32,
) -> WORD32 {
    let mut n: WORD32 = 0;
    let mut w: FLOAT32 = 0.;
    let mut node_level: *mut FLOAT32 = ((*split_drc_characteristic).node_level)
        .as_mut_ptr();
    let mut node_gain: *mut FLOAT32 = ((*split_drc_characteristic).node_gain)
        .as_mut_ptr();
    if in_db_level > DRC_INPUT_LOUDNESS_TARGET {
        return 2 as WORD32;
    }
    n = 1 as core::ffi::c_int as WORD32;
    while n <= (*split_drc_characteristic).characteristic_node_count {
        if in_db_level
            <= *node_level
                .offset((n as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
            && in_db_level > *node_level.offset(n as isize)
        {
            w = (*node_level.offset(n as isize) - in_db_level)
                / (*node_level.offset(n as isize)
                    - *node_level
                        .offset(
                            (n as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        ));
            *out_db_gain = ((w
                * *node_gain
                    .offset((n as core::ffi::c_int - 1 as core::ffi::c_int) as isize))
                as core::ffi::c_double
                + (1.0f64 - w as core::ffi::c_double)
                    * *node_gain.offset(n as isize) as core::ffi::c_double) as FLOAT32;
        }
        n += 1;
    }
    *out_db_gain = *node_gain
        .offset((*split_drc_characteristic).characteristic_node_count as isize);
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_compressor_io_nodes_rt(
    mut split_drc_characteristic: *mut ia_split_drc_characteristic_struct,
    mut in_db_level: FLOAT32,
    mut out_db_gain: *mut FLOAT32,
) -> WORD32 {
    let mut n: WORD32 = 0;
    let mut w: FLOAT32 = 0.;
    let mut node_level: *mut FLOAT32 = ((*split_drc_characteristic).node_level)
        .as_mut_ptr();
    let mut node_gain: *mut FLOAT32 = ((*split_drc_characteristic).node_gain)
        .as_mut_ptr();
    if in_db_level < DRC_INPUT_LOUDNESS_TARGET {
        return 2 as WORD32;
    }
    n = 1 as core::ffi::c_int as WORD32;
    while n <= (*split_drc_characteristic).characteristic_node_count {
        if in_db_level
            >= *node_level
                .offset((n as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
            && in_db_level < *node_level.offset(n as isize)
        {
            w = (*node_level.offset(n as isize) - in_db_level)
                / (*node_level.offset(n as isize)
                    - *node_level
                        .offset(
                            (n as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        ));
            *out_db_gain = ((w
                * *node_gain
                    .offset((n as core::ffi::c_int - 1 as core::ffi::c_int) as isize))
                as core::ffi::c_double
                + (1.0f64 - w as core::ffi::c_double)
                    * *node_gain.offset(n as isize) as core::ffi::c_double) as FLOAT32;
        }
        n += 1;
    }
    *out_db_gain = *node_gain
        .offset((*split_drc_characteristic).characteristic_node_count as isize);
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_compressor_io_nodes_inverse(
    mut split_drc_characteristic: *mut ia_split_drc_characteristic_struct,
    mut loc_db_gain: FLOAT32,
    mut in_level: *mut FLOAT32,
) -> VOID {
    let mut n: WORD32 = 0;
    let mut w: FLOAT32 = 0.;
    let mut node_level: *mut FLOAT32 = ((*split_drc_characteristic).node_level)
        .as_mut_ptr();
    let mut node_gain: *mut FLOAT32 = ((*split_drc_characteristic).node_gain)
        .as_mut_ptr();
    let mut node_count: WORD32 = (*split_drc_characteristic).characteristic_node_count;
    if *node_gain.offset(1 as core::ffi::c_int as isize) < 0.0f32 {
        if loc_db_gain <= *node_gain.offset(node_count as isize) {
            *in_level = *node_level.offset(node_count as isize);
        } else if loc_db_gain >= 0.0f32 {
            *in_level = DRC_INPUT_LOUDNESS_TARGET as FLOAT32;
        } else {
            n = 1 as core::ffi::c_int as WORD32;
            while n <= node_count {
                if loc_db_gain
                    <= *node_gain
                        .offset((n as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                    && loc_db_gain > *node_gain.offset(n as isize)
                {
                    w = (*node_gain.offset(n as isize) - loc_db_gain)
                        / (*node_gain.offset(n as isize)
                            - *node_gain
                                .offset(
                                    (n as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                                ));
                    *in_level = ((w
                        * *node_level
                            .offset(
                                (n as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            )) as core::ffi::c_double
                        + (1.0f64 - w as core::ffi::c_double)
                            * *node_level.offset(n as isize) as core::ffi::c_double)
                        as FLOAT32;
                }
                n += 1;
            }
        }
    } else if loc_db_gain >= *node_gain.offset(node_count as isize) {
        *in_level = *node_level.offset(node_count as isize);
    } else if loc_db_gain <= 0.0f32 {
        *in_level = DRC_INPUT_LOUDNESS_TARGET as FLOAT32;
    } else {
        n = 1 as core::ffi::c_int as WORD32;
        while n <= node_count {
            if loc_db_gain
                >= *node_gain
                    .offset((n as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
                && loc_db_gain < *node_gain.offset(n as isize)
            {
                w = (*node_gain.offset(n as isize) - loc_db_gain)
                    / (*node_gain.offset(n as isize)
                        - *node_gain
                            .offset(
                                (n as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ));
                *in_level = ((w
                    * *node_level
                        .offset(
                            (n as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                        )) as core::ffi::c_double
                    + (1.0f64 - w as core::ffi::c_double)
                        * *node_level.offset(n as isize) as core::ffi::c_double)
                    as FLOAT32;
            }
            n += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn impd_map_gain(
    mut split_drc_characteristic_source: *mut ia_split_drc_characteristic_struct,
    mut split_drc_characteristic_target: *mut ia_split_drc_characteristic_struct,
    mut gain_in_db: FLOAT32,
    mut gain_out_db: *mut FLOAT32,
) -> WORD32 {
    let mut in_level: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
    let mut err: WORD32 = 0 as WORD32;
    match (*split_drc_characteristic_source).characteristic_format {
        CHARACTERISTIC_SIGMOID => {
            err = impd_compressor_io_sigmoid_inv(
                split_drc_characteristic_source,
                gain_in_db,
                &mut in_level,
            );
            if err != 0 {
                return err;
            }
        }
        CHARACTERISTIC_NODES => {
            impd_compressor_io_nodes_inverse(
                split_drc_characteristic_source,
                gain_in_db,
                &mut in_level,
            );
        }
        CHARACTERISTIC_PASS_THRU => {
            in_level = gain_in_db;
        }
        _ => return 2 as WORD32,
    }
    match (*split_drc_characteristic_target).characteristic_format {
        CHARACTERISTIC_SIGMOID => {
            err = impd_compressor_io_sigmoid(
                split_drc_characteristic_target,
                in_level,
                gain_out_db,
            );
            if err != 0 {
                return err;
            }
        }
        CHARACTERISTIC_NODES => {
            if in_level < DRC_INPUT_LOUDNESS_TARGET {
                err = impd_compressor_io_nodes_lt(
                    split_drc_characteristic_target,
                    in_level,
                    gain_out_db,
                );
                if err != 0 {
                    return err;
                }
            } else {
                err = impd_compressor_io_nodes_rt(
                    split_drc_characteristic_target,
                    in_level,
                    gain_out_db,
                );
                if err != 0 {
                    return err;
                }
            }
        }
        CHARACTERISTIC_PASS_THRU => {
            *gain_out_db = in_level;
        }
        _ => {}
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_conv_to_linear_domain(
    mut interp_params_str: *mut ia_interp_params_struct,
    mut drc_band: WORD32,
    mut in_param_db_gain: FLOAT32,
    mut in_param_db_slope: FLOAT32,
    mut out_param_lin_gain: *mut FLOAT32,
    mut out_param_lin_slope: *mut FLOAT32,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut loc_db_gain: FLOAT32 = in_param_db_gain;
    let mut gain_ratio: FLOAT32 = 1.0f32;
    let mut mapped_db_gain: FLOAT32 = 0.;
    let mut pstr_gain_modifiers: *mut ia_gain_modifiers_struct = (*interp_params_str)
        .pstr_gain_modifiers;
    if (*interp_params_str).gain_modification_flag != 0 {
        let mut split_drc_characteristic_source: *mut ia_split_drc_characteristic_struct = 0
            as *mut ia_split_drc_characteristic_struct;
        let mut slopeIsNegative: WORD32 = 0;
        if (*interp_params_str).drc_characteristic_present != 0 {
            if !((*interp_params_str).drc_source_characteristic_cicp_format != 0) {
                slopeIsNegative = 0 as core::ffi::c_int as WORD32;
                split_drc_characteristic_source = (*interp_params_str)
                    .split_source_characteristic_left;
                if (*split_drc_characteristic_source).characteristic_format
                    == 0 as core::ffi::c_int
                {
                    slopeIsNegative = 1 as core::ffi::c_int as WORD32;
                } else if (*split_drc_characteristic_source)
                    .node_gain[1 as core::ffi::c_int as usize] > 0.0f32
                {
                    slopeIsNegative = 1 as core::ffi::c_int as WORD32;
                }
                if loc_db_gain == 0.0f32 {
                    if (*pstr_gain_modifiers)
                        .target_characteristic_left_present[drc_band as usize]
                        == 1 as core::ffi::c_int
                        && (*(*interp_params_str).split_target_characteristic_left)
                            .characteristic_format == CHARACTERISTIC_PASS_THRU
                        || (*pstr_gain_modifiers)
                            .target_characteristic_right_present[drc_band as usize]
                            == 1 as core::ffi::c_int
                            && (*(*interp_params_str).split_target_characteristic_right)
                                .characteristic_format == CHARACTERISTIC_PASS_THRU
                    {
                        mapped_db_gain = DRC_INPUT_LOUDNESS_TARGET as FLOAT32;
                        loc_db_gain = DRC_INPUT_LOUDNESS_TARGET as FLOAT32;
                    }
                } else if loc_db_gain > 0.0f32
                    && slopeIsNegative == 1 as core::ffi::c_int
                    || loc_db_gain < 0.0f32 && slopeIsNegative == 0 as core::ffi::c_int
                {
                    if (*pstr_gain_modifiers)
                        .target_characteristic_left_present[drc_band as usize]
                        == 1 as core::ffi::c_int
                    {
                        err = impd_map_gain(
                            split_drc_characteristic_source,
                            (*interp_params_str).split_target_characteristic_left,
                            loc_db_gain,
                            &mut mapped_db_gain,
                        );
                        if err != 0 {
                            return err;
                        }
                        gain_ratio = mapped_db_gain / loc_db_gain;
                    }
                } else if loc_db_gain < 0.0f32
                    && slopeIsNegative == 1 as core::ffi::c_int
                    || loc_db_gain > 0.0f32 && slopeIsNegative == 0 as core::ffi::c_int
                {
                    if (*pstr_gain_modifiers)
                        .target_characteristic_right_present[drc_band as usize]
                        == 1 as core::ffi::c_int
                    {
                        split_drc_characteristic_source = (*interp_params_str)
                            .split_source_characteristic_right;
                        err = impd_map_gain(
                            split_drc_characteristic_source,
                            (*interp_params_str).split_target_characteristic_right,
                            loc_db_gain,
                            &mut mapped_db_gain,
                        );
                        if err != 0 {
                            return err;
                        }
                        gain_ratio = mapped_db_gain / loc_db_gain;
                    }
                }
            }
        }
        if loc_db_gain < 0.0f32 {
            gain_ratio *= (*interp_params_str).compress;
        } else {
            gain_ratio *= (*interp_params_str).boost;
        }
    }
    if (*pstr_gain_modifiers).gain_scaling_flag[drc_band as usize]
        == 1 as core::ffi::c_int
    {
        if (loc_db_gain as core::ffi::c_double) < 0.0f64 {
            gain_ratio *= (*pstr_gain_modifiers).attn_scaling[drc_band as usize];
        } else {
            gain_ratio *= (*pstr_gain_modifiers).ampl_scaling[drc_band as usize];
        }
    }
    if (*(*interp_params_str).pstr_ducking_modifiers).ducking_scaling_flag
        == 1 as core::ffi::c_int
        && (*interp_params_str).ducking_flag == 1 as core::ffi::c_int
    {
        gain_ratio *= (*(*interp_params_str).pstr_ducking_modifiers).ducking_scaling;
    }
    if (*interp_params_str).interpolation_loud_eq == 1 as core::ffi::c_int {
        *out_param_lin_gain = gain_ratio * loc_db_gain
            + (*pstr_gain_modifiers).gain_offset[drc_band as usize];
        *out_param_lin_slope = 0.0f32 as FLOAT32;
    } else {
        *out_param_lin_gain = pow(
            2.0f64,
            (gain_ratio as core::ffi::c_float * loc_db_gain as core::ffi::c_float
                / 6.0f32) as core::ffi::c_double,
        ) as FLOAT32;
        *out_param_lin_slope = SLOPE_FACTOR_DB_TO_LINEAR * gain_ratio
            * *out_param_lin_gain * in_param_db_slope;
        if (*pstr_gain_modifiers).gain_offset_flag[drc_band as usize]
            == 1 as core::ffi::c_int
        {
            *out_param_lin_gain
                *= pow(
                    2.0f64,
                    ((*pstr_gain_modifiers).gain_offset[drc_band as usize] / 6.0f32)
                        as core::ffi::c_double,
                ) as FLOAT32;
        }
        if (*interp_params_str).limiter_peak_target_present == 1 as core::ffi::c_int
            && (*interp_params_str).clipping_flag == 1 as core::ffi::c_int
        {
            *out_param_lin_gain
                *= pow(
                    2.0f64,
                    (if 0.0f64
                        > (-(*interp_params_str).limiter_peak_target
                            - (*interp_params_str).loudness_normalization_gain_db)
                            as core::ffi::c_double
                    {
                        0.0f64
                    } else {
                        (-(*interp_params_str).limiter_peak_target
                            - (*interp_params_str).loudness_normalization_gain_db)
                            as core::ffi::c_double
                    }) / 6.0f64,
                ) as FLOAT32;
            if *out_param_lin_gain as core::ffi::c_double >= 1.0f64 {
                *out_param_lin_gain = 1.0f32;
                *out_param_lin_slope = 0.0f32;
            }
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_interpolate_drc_gain(
    mut interp_params_str: *mut ia_interp_params_struct,
    mut drc_band: WORD32,
    mut gain_step_tdomain: WORD32,
    mut gain0: FLOAT32,
    mut gain1: FLOAT32,
    mut slope0: FLOAT32,
    mut slope1: FLOAT32,
    mut result: *mut FLOAT32,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut n: WORD32 = 0;
    let mut k1: FLOAT32 = 0.;
    let mut k2: FLOAT32 = 0.;
    let mut a: FLOAT32 = 0.;
    let mut b: FLOAT32 = 0.;
    let mut c: FLOAT32 = 0.;
    let mut d: FLOAT32 = 0.;
    let mut slope_t1: FLOAT32 = 0.;
    let mut slope_t2: FLOAT32 = 0.;
    let mut gain_t1: FLOAT32 = 0.;
    let mut gain_t2: FLOAT32 = 0.;
    let mut cubic_interpolation: WORD32 = 1 as WORD32;
    let mut node_inser: WORD32 = 0;
    let mut node_inser_float: FLOAT32 = 0.;
    if gain_step_tdomain <= 0 as core::ffi::c_int {
        return 2 as WORD32;
    }
    err = impd_conv_to_linear_domain(
        interp_params_str,
        drc_band,
        gain0,
        slope0,
        &mut gain_t1,
        &mut slope_t1,
    );
    if err != 0 {
        return err;
    }
    err = impd_conv_to_linear_domain(
        interp_params_str,
        drc_band,
        gain1,
        slope1,
        &mut gain_t2,
        &mut slope_t2,
    );
    if err != 0 {
        return err;
    }
    if (*interp_params_str).gain_interpolation_type == GAIN_INTERPOLATION_TYPE_SPLINE {
        slope_t1 = slope_t1 / (*interp_params_str).delta_tmin as FLOAT32;
        slope_t2 = slope_t2 / (*interp_params_str).delta_tmin as FLOAT32;
        if fabs(slope_t1 as core::ffi::c_double) as FLOAT32
            > fabs(slope_t2 as core::ffi::c_double) as FLOAT32
        {
            node_inser_float = 2.0f32
                * (gain_t2 - gain_t1 - slope_t2 * gain_step_tdomain as FLOAT32)
                / (slope_t1 - slope_t2);
            node_inser = (0.5f32 + node_inser_float) as WORD32;
            if node_inser >= 0 as core::ffi::c_int && node_inser < gain_step_tdomain {
                cubic_interpolation = 0 as core::ffi::c_int as WORD32;
                *result.offset(0 as core::ffi::c_int as isize) = gain_t1;
                *result.offset(gain_step_tdomain as isize) = gain_t2;
                a = 0.5f32 * (slope_t2 - slope_t1) / node_inser_float;
                b = slope_t1;
                c = gain_t1;
                n = 1 as core::ffi::c_int as WORD32;
                while n < node_inser {
                    let mut t: FLOAT32 = n as FLOAT32;
                    *result.offset(n as isize) = (a * t + b) * t + c;
                    *result.offset(n as isize) = (if 0.0f32 > *result.offset(n as isize)
                    {
                        0.0f32
                    } else {
                        *result.offset(n as isize)
                    }) as FLOAT32;
                    n += 1;
                }
                a = slope_t2;
                b = gain_t2;
                while n < gain_step_tdomain {
                    let mut t_0: FLOAT32 = (n - gain_step_tdomain) as FLOAT32;
                    *result.offset(n as isize) = a * t_0 + b;
                    n += 1;
                }
            }
        } else if (fabs(slope_t1 as core::ffi::c_double) as FLOAT32)
            < fabs(slope_t2 as core::ffi::c_double) as FLOAT32
        {
            node_inser_float = 2.0f32
                * (gain_t1 - gain_t2 + slope_t1 * gain_step_tdomain as FLOAT32)
                / (slope_t1 - slope_t2);
            node_inser_float = gain_step_tdomain as FLOAT32 - node_inser_float;
            node_inser = (0.5f32 + node_inser_float) as WORD32;
            if node_inser >= 0 as core::ffi::c_int && node_inser < gain_step_tdomain {
                cubic_interpolation = 0 as core::ffi::c_int as WORD32;
                *result.offset(0 as core::ffi::c_int as isize) = gain_t1;
                *result.offset(gain_step_tdomain as isize) = gain_t2;
                a = slope_t1;
                b = gain_t1;
                n = 1 as core::ffi::c_int as WORD32;
                while n < node_inser {
                    let mut t_1: FLOAT32 = n as FLOAT32;
                    *result.offset(n as isize) = a * t_1 + b;
                    n += 1;
                }
                a = (slope_t2 - slope_t1)
                    / (2.0f32 * (gain_step_tdomain as FLOAT32 - node_inser_float));
                b = -slope_t2;
                c = gain_t2;
                while n < gain_step_tdomain {
                    let mut t_2: FLOAT32 = (gain_step_tdomain - n) as FLOAT32;
                    *result.offset(n as isize) = (a * t_2 + b) * t_2 + c;
                    *result.offset(n as isize) = (if 0.0f32 > *result.offset(n as isize)
                    {
                        0.0f32
                    } else {
                        *result.offset(n as isize)
                    }) as FLOAT32;
                    n += 1;
                }
            }
        }
        if cubic_interpolation == 1 as core::ffi::c_int {
            let mut gain_step_inv: FLOAT32 = 1.0f32 / gain_step_tdomain as FLOAT32;
            let mut gain_step_inv2: FLOAT32 = gain_step_inv * gain_step_inv;
            k1 = (gain_t2 - gain_t1) * gain_step_inv2;
            k2 = slope_t2 + slope_t1;
            a = gain_step_inv * (gain_step_inv * k2 - 2.0f32 * k1);
            b = 3.0f32 * k1 - gain_step_inv * (k2 + slope_t1);
            c = slope_t1;
            d = gain_t1;
            *result.offset(0 as core::ffi::c_int as isize) = gain_t1;
            *result.offset(gain_step_tdomain as isize) = gain_t2;
            n = 1 as core::ffi::c_int as WORD32;
            while n < gain_step_tdomain {
                let mut t_3: FLOAT32 = n as FLOAT32;
                *result.offset(n as isize) = ((a * t_3 + b) * t_3 + c) * t_3 + d;
                *result.offset(n as isize) = (if 0.0f32 > *result.offset(n as isize) {
                    0.0f32
                } else {
                    *result.offset(n as isize)
                }) as FLOAT32;
                n += 1;
            }
        }
    } else {
        a = (gain_t2 - gain_t1) / gain_step_tdomain as FLOAT32;
        b = gain_t1;
        *result.offset(0 as core::ffi::c_int as isize) = gain_t1;
        *result.offset(gain_step_tdomain as isize) = gain_t2;
        n = 1 as core::ffi::c_int as WORD32;
        while n < gain_step_tdomain {
            let mut t_4: FLOAT32 = n as FLOAT32;
            *result.offset(n as isize) = a * t_4 + b;
            n += 1;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_advance_buf(
    mut drc_frame_size: WORD32,
    mut pstr_gain_buf: *mut ia_gain_buffer_struct,
) -> VOID {
    let mut n: WORD32 = 0;
    let mut buf_interpolation: *mut ia_interp_buf_struct = 0
        as *mut ia_interp_buf_struct;
    n = 0 as core::ffi::c_int as WORD32;
    while n < (*pstr_gain_buf).buf_interpolation_count {
        buf_interpolation = &mut *((*pstr_gain_buf).buf_interpolation).offset(n as isize)
            as *mut ia_interp_buf_struct;
        (*buf_interpolation).prev_node = (*buf_interpolation).str_node;
        (*buf_interpolation).prev_node.time -= drc_frame_size;
        memmove(
            ((*buf_interpolation).lpcm_gains).as_mut_ptr() as *mut core::ffi::c_void,
            ((*buf_interpolation).lpcm_gains)
                .as_mut_ptr()
                .offset(drc_frame_size as isize) as *const core::ffi::c_void,
            (::core::mem::size_of::<FLOAT32>() as size_t)
                .wrapping_mul(
                    (drc_frame_size as core::ffi::c_int + MAX_SIGNAL_DELAY) as size_t,
                ),
        );
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn impd_concatenate_segments(
    mut drc_frame_size: WORD32,
    mut drc_band: WORD32,
    mut interp_params_str: *mut ia_interp_params_struct,
    mut str_spline_nodes: *mut ia_spline_nodes_struct,
    mut buf_interpolation: *mut ia_interp_buf_struct,
    mut sel_drc_index: WORD32,
    mut is_config_changed: WORD32,
    mut loudness_changed: WORD32,
) -> WORD32 {
    let mut time_prev: WORD32 = 0;
    let mut duration: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut err: WORD32 = 0 as WORD32;
    let mut loc_db_gain: FLOAT32 = 0.0f32;
    let mut prev_db_gain: FLOAT32 = 0.;
    let mut slope: FLOAT32 = 0.0f32;
    let mut slope_prev: FLOAT32 = 0.;
    time_prev = (*buf_interpolation).prev_node.time;
    prev_db_gain = (*buf_interpolation).prev_node.loc_db_gain;
    slope_prev = (*buf_interpolation).prev_node.slope;
    n = 0 as core::ffi::c_int as WORD32;
    while n < (*str_spline_nodes).num_nodes {
        duration = (*str_spline_nodes).str_node[n as usize].time - time_prev;
        loc_db_gain = (*str_spline_nodes).str_node[n as usize].loc_db_gain;
        if loudness_changed != 0 {
            if sel_drc_index == 0 as core::ffi::c_int
                && is_config_changed == 1 as core::ffi::c_int
            {
                loc_db_gain = (*str_spline_nodes).str_node[n as usize].loc_db_gain
                    + (*interp_params_str).loudness_normalization_gain_db;
                if prev_db_gain == 0 as core::ffi::c_int as FLOAT32 {
                    prev_db_gain = (*buf_interpolation).prev_node.loc_db_gain
                        + (*interp_params_str).loudness_normalization_gain_db;
                }
            }
        }
        slope = (*str_spline_nodes).str_node[n as usize].slope;
        err = impd_interpolate_drc_gain(
            interp_params_str,
            drc_band,
            duration,
            prev_db_gain,
            loc_db_gain,
            slope_prev,
            slope,
            ((*buf_interpolation).lpcm_gains)
                .as_mut_ptr()
                .offset(MAX_SIGNAL_DELAY as isize)
                .offset(drc_frame_size as isize)
                .offset(time_prev as isize),
        );
        if err != 0 {
            return err;
        }
        time_prev = (*str_spline_nodes).str_node[n as usize].time;
        prev_db_gain = loc_db_gain;
        slope_prev = slope;
        n += 1;
    }
    (*buf_interpolation).str_node.loc_db_gain = loc_db_gain;
    (*buf_interpolation).str_node.slope = slope;
    (*buf_interpolation).str_node.time = time_prev;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_get_drc_gain(
    mut p_drc_gain_dec_structs: *mut ia_drc_gain_dec_struct,
    mut pstr_drc_config: *mut ia_drc_config,
    mut pstr_drc_gain: *mut ia_drc_gain_struct,
    mut compress: FLOAT32,
    mut boost: FLOAT32,
    mut characteristic_index: WORD32,
    mut loudness_normalization_gain_db: FLOAT32,
    mut sel_drc_index: WORD32,
    mut drc_gain_buffers: *mut ia_drc_gain_buffers_struct,
) -> WORD32 {
    let mut ia_drc_params_struct: *mut ia_drc_params_struct = &mut (*p_drc_gain_dec_structs)
        .ia_drc_params_struct;
    let mut drc_instructions_index: WORD32 = (*ia_drc_params_struct)
        .sel_drc_array[sel_drc_index as usize]
        .drc_instructions_index;
    if drc_instructions_index >= 0 as core::ffi::c_int {
        let mut b: WORD32 = 0;
        let mut g: WORD32 = 0;
        let mut gain_element_index: WORD32 = 0;
        let mut err: WORD32 = 0 as WORD32;
        let mut parametric_drc_instance_index: WORD32 = 0 as WORD32;
        let mut interp_params_str: ia_interp_params_struct = {
            let mut init = ia_interp_params_struct {
                gain_interpolation_type: 0 as WORD32,
                gain_modification_flag: 0,
                ducking_flag: 0,
                clipping_flag: 0,
                pstr_ducking_modifiers: 0 as *mut ia_ducking_modifiers_struct,
                pstr_gain_modifiers: 0 as *mut ia_gain_modifiers_struct,
                drc_characteristic_present: 0,
                drc_source_characteristic_cicp_format: 0,
                source_drc_characteristic: 0,
                split_source_characteristic_left: 0
                    as *mut ia_split_drc_characteristic_struct,
                split_source_characteristic_right: 0
                    as *mut ia_split_drc_characteristic_struct,
                drc_target_characteristic_cicp_format: 0,
                target_drc_characteristic: 0,
                split_target_characteristic_left: 0
                    as *mut ia_split_drc_characteristic_struct,
                split_target_characteristic_right: 0
                    as *mut ia_split_drc_characteristic_struct,
                interpolation_loud_eq: 0,
                limiter_peak_target_present: 0,
                limiter_peak_target: 0.,
                loudness_normalization_gain_db: 0.,
                delta_tmin: 0,
                characteristic_index: 0,
                compress: 0.,
                boost: 0.,
            };
            init
        };
        let mut str_drc_instruction_str: *mut ia_drc_instructions_struct = &mut *((*pstr_drc_config)
            .str_drc_instruction_str)
            .as_mut_ptr()
            .offset(drc_instructions_index as isize) as *mut ia_drc_instructions_struct;
        let mut drc_set_effect: WORD32 = (*str_drc_instruction_str).drc_set_effect;
        let mut num_drc_ch_groups: WORD32 = (*str_drc_instruction_str).num_drc_ch_groups;
        let mut str_p_loc_drc_coefficients_uni_drc: *mut ia_uni_drc_coeffs_struct = 0
            as *mut ia_uni_drc_coeffs_struct;
        let mut drc_coeff_idx: WORD32 = (*ia_drc_params_struct)
            .sel_drc_array[sel_drc_index as usize]
            .drc_coeff_idx;
        if drc_coeff_idx >= 0 as core::ffi::c_int {
            str_p_loc_drc_coefficients_uni_drc = &mut *((*pstr_drc_config)
                .str_p_loc_drc_coefficients_uni_drc)
                .as_mut_ptr()
                .offset(drc_coeff_idx as isize) as *mut ia_uni_drc_coeffs_struct;
            interp_params_str.interpolation_loud_eq = 0 as core::ffi::c_int as WORD32;
        } else {
            return 2 as WORD32
        }
        interp_params_str.loudness_normalization_gain_db = loudness_normalization_gain_db;
        interp_params_str.characteristic_index = characteristic_index;
        interp_params_str.compress = compress;
        interp_params_str.boost = boost;
        interp_params_str.limiter_peak_target_present = (*str_drc_instruction_str)
            .limiter_peak_target_present;
        interp_params_str.limiter_peak_target = (*str_drc_instruction_str)
            .limiter_peak_target;
        if drc_set_effect as core::ffi::c_int
            & (EFFECT_BIT_DUCK_OTHER | EFFECT_BIT_DUCK_SELF) == 0 as core::ffi::c_int
            && drc_set_effect != EFFECT_BIT_FADE && drc_set_effect != EFFECT_BIT_CLIPPING
        {
            interp_params_str.gain_modification_flag = 1 as core::ffi::c_int as WORD32;
        } else {
            interp_params_str.gain_modification_flag = 0 as core::ffi::c_int as WORD32;
        }
        if drc_set_effect as core::ffi::c_int
            & (EFFECT_BIT_DUCK_OTHER | EFFECT_BIT_DUCK_SELF) != 0
        {
            interp_params_str.ducking_flag = 1 as core::ffi::c_int as WORD32;
        } else {
            interp_params_str.ducking_flag = 0 as core::ffi::c_int as WORD32;
        }
        if drc_set_effect == EFFECT_BIT_CLIPPING {
            interp_params_str.clipping_flag = 1 as core::ffi::c_int as WORD32;
        } else {
            interp_params_str.clipping_flag = 0 as core::ffi::c_int as WORD32;
        }
        impd_advance_buf(
            (*ia_drc_params_struct).drc_frame_size,
            &mut *((*drc_gain_buffers).pstr_gain_buf)
                .as_mut_ptr()
                .offset(sel_drc_index as isize),
        );
        gain_element_index = 0 as core::ffi::c_int as WORD32;
        g = 0 as core::ffi::c_int as WORD32;
        while g < num_drc_ch_groups {
            let mut gainSet: WORD32 = 0 as WORD32;
            let mut num_drc_bands: WORD32 = 0 as WORD32;
            interp_params_str.gain_interpolation_type = (*str_drc_instruction_str)
                .gain_interpolation_type_for_channel_group[g as usize];
            interp_params_str.delta_tmin = (*str_drc_instruction_str)
                .time_delta_min_for_channel_group[g as usize];
            interp_params_str.pstr_ducking_modifiers = &mut *((*str_drc_instruction_str)
                .str_ducking_modifiers_for_channel_group)
                .as_mut_ptr()
                .offset(g as isize) as *mut ia_ducking_modifiers_struct;
            interp_params_str.pstr_gain_modifiers = &mut *((*str_drc_instruction_str)
                .str_gain_modifiers_of_ch_group)
                .as_mut_ptr()
                .offset(g as isize) as *mut ia_gain_modifiers_struct;
            if (*str_drc_instruction_str).ch_group_parametric_drc_flag[g as usize]
                == 0 as core::ffi::c_int
            {
                gainSet = (*str_drc_instruction_str)
                    .gain_set_index_for_channel_group[g as usize];
                num_drc_bands = (*str_drc_instruction_str)
                    .band_count_of_ch_group[g as usize];
                b = 0 as core::ffi::c_int as WORD32;
                while b < num_drc_bands {
                    let mut gain_params: *mut ia_gain_params_struct = &mut *((*((*str_p_loc_drc_coefficients_uni_drc)
                        .gain_set_params)
                        .as_mut_ptr()
                        .offset(gainSet as isize))
                        .gain_params)
                        .as_mut_ptr()
                        .offset(b as isize) as *mut ia_gain_params_struct;
                    let mut seq: WORD32 = (*gain_params).gain_seq_idx;
                    interp_params_str.drc_characteristic_present = (*gain_params)
                        .drc_characteristic_present;
                    interp_params_str.drc_source_characteristic_cicp_format = (*gain_params)
                        .drc_characteristic_format_is_cicp;
                    interp_params_str.source_drc_characteristic = (*gain_params)
                        .drc_characteristic;
                    interp_params_str.split_source_characteristic_left = &mut *((*str_p_loc_drc_coefficients_uni_drc)
                        .str_split_characteristic_left)
                        .as_mut_ptr()
                        .offset((*gain_params).drc_characteristic_left_index as isize)
                        as *mut ia_split_drc_characteristic_struct;
                    interp_params_str.split_source_characteristic_right = &mut *((*str_p_loc_drc_coefficients_uni_drc)
                        .str_split_characteristic_right)
                        .as_mut_ptr()
                        .offset((*gain_params).drc_characteristic_right_index as isize)
                        as *mut ia_split_drc_characteristic_struct;
                    interp_params_str.split_target_characteristic_left = &mut *((*str_p_loc_drc_coefficients_uni_drc)
                        .str_split_characteristic_left)
                        .as_mut_ptr()
                        .offset(
                            *((*interp_params_str.pstr_gain_modifiers)
                                .target_characteristic_left_index)
                                .as_mut_ptr()
                                .offset(b as isize) as isize,
                        ) as *mut ia_split_drc_characteristic_struct;
                    interp_params_str.split_target_characteristic_right = &mut *((*str_p_loc_drc_coefficients_uni_drc)
                        .str_split_characteristic_right)
                        .as_mut_ptr()
                        .offset(
                            *((*interp_params_str.pstr_gain_modifiers)
                                .target_characteristic_right_index)
                                .as_mut_ptr()
                                .offset(b as isize) as isize,
                        ) as *mut ia_split_drc_characteristic_struct;
                    err = impd_concatenate_segments(
                        (*ia_drc_params_struct).drc_frame_size,
                        b,
                        &mut interp_params_str,
                        &mut *((*((*pstr_drc_gain).drc_gain_sequence)
                            .as_mut_ptr()
                            .offset(seq as isize))
                            .str_spline_nodes)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize),
                        &mut *((*((*drc_gain_buffers).pstr_gain_buf)
                            .as_mut_ptr()
                            .offset(sel_drc_index as isize))
                            .buf_interpolation)
                            .offset(gain_element_index as isize),
                        sel_drc_index,
                        (*pstr_drc_config).is_config_changed,
                        (*pstr_drc_config).ln_gain_changed,
                    );
                    if err != 0 {
                        return err;
                    }
                    gain_element_index += 1;
                    b += 1;
                }
            } else {
                if (*ia_drc_params_struct).sub_band_domain_mode
                    == SUBBAND_DOMAIN_MODE_OFF
                    && !((*p_drc_gain_dec_structs)
                        .parametricdrc_params
                        .str_parametric_drc_instance_params[parametric_drc_instance_index
                            as usize]
                        .parametric_drc_type == PARAM_DRC_TYPE_LIM)
                {
                    err = impd_parametric_drc_instance_process(
                        (*p_drc_gain_dec_structs).audio_in_out_buf.audio_in_out_buf
                            as *mut *mut FLOAT32,
                        0 as *mut *mut FLOAT32,
                        0 as *mut *mut FLOAT32,
                        &mut (*p_drc_gain_dec_structs).parametricdrc_params,
                        &mut *((*p_drc_gain_dec_structs)
                            .parametricdrc_params
                            .str_parametric_drc_instance_params)
                            .as_mut_ptr()
                            .offset(parametric_drc_instance_index as isize),
                    );
                    if err != 0 {
                        return err;
                    }
                    err = impd_concatenate_segments(
                        (*ia_drc_params_struct).drc_frame_size,
                        0 as WORD32,
                        &mut interp_params_str,
                        &mut (*((*p_drc_gain_dec_structs)
                            .parametricdrc_params
                            .str_parametric_drc_instance_params)
                            .as_mut_ptr()
                            .offset(parametric_drc_instance_index as isize))
                            .str_spline_nodes,
                        &mut *((*((*drc_gain_buffers).pstr_gain_buf)
                            .as_mut_ptr()
                            .offset(sel_drc_index as isize))
                            .buf_interpolation)
                            .offset(gain_element_index as isize),
                        sel_drc_index,
                        (*pstr_drc_config).is_config_changed,
                        (*pstr_drc_config).ln_gain_changed,
                    );
                    if err != 0 {
                        return err;
                    }
                } else if (*ia_drc_params_struct).sub_band_domain_mode
                    == SUBBAND_DOMAIN_MODE_OFF
                    && (*p_drc_gain_dec_structs)
                        .parametricdrc_params
                        .str_parametric_drc_instance_params[parametric_drc_instance_index
                            as usize]
                        .parametric_drc_type == PARAM_DRC_TYPE_LIM
                {
                    let mut lpcm_gains: *mut FLOAT32 = ((*((*drc_gain_buffers)
                        .pstr_gain_buf[sel_drc_index as usize]
                        .buf_interpolation)
                        .offset(gain_element_index as isize))
                        .lpcm_gains)
                        .as_mut_ptr()
                        .offset(MAX_SIGNAL_DELAY as isize);
                    impd_parametric_lim_type_drc_process(
                        (*p_drc_gain_dec_structs).audio_in_out_buf.audio_in_out_buf
                            as *mut *mut FLOAT32,
                        loudness_normalization_gain_db,
                        &mut (*((*p_drc_gain_dec_structs)
                            .parametricdrc_params
                            .str_parametric_drc_instance_params)
                            .as_mut_ptr()
                            .offset(parametric_drc_instance_index as isize))
                            .str_parametric_drc_type_lim_params,
                        lpcm_gains,
                    );
                } else if (*ia_drc_params_struct).sub_band_domain_mode
                    != SUBBAND_DOMAIN_MODE_OFF
                    && !((*p_drc_gain_dec_structs)
                        .parametricdrc_params
                        .str_parametric_drc_instance_params[parametric_drc_instance_index
                            as usize]
                        .parametric_drc_type == PARAM_DRC_TYPE_LIM)
                {
                    err = impd_parametric_drc_instance_process(
                        0 as *mut *mut FLOAT32,
                        (*p_drc_gain_dec_structs).audio_in_out_buf.audio_real_buff
                            as *mut *mut FLOAT32,
                        (*p_drc_gain_dec_structs).audio_in_out_buf.audio_imag_buff
                            as *mut *mut FLOAT32,
                        &mut (*p_drc_gain_dec_structs).parametricdrc_params,
                        &mut *((*p_drc_gain_dec_structs)
                            .parametricdrc_params
                            .str_parametric_drc_instance_params)
                            .as_mut_ptr()
                            .offset(parametric_drc_instance_index as isize),
                    );
                    if err != 0 {
                        return err;
                    }
                    err = impd_concatenate_segments(
                        (*ia_drc_params_struct).drc_frame_size,
                        0 as WORD32,
                        &mut interp_params_str,
                        &mut (*((*p_drc_gain_dec_structs)
                            .parametricdrc_params
                            .str_parametric_drc_instance_params)
                            .as_mut_ptr()
                            .offset(parametric_drc_instance_index as isize))
                            .str_spline_nodes,
                        &mut *((*((*drc_gain_buffers).pstr_gain_buf)
                            .as_mut_ptr()
                            .offset(sel_drc_index as isize))
                            .buf_interpolation)
                            .offset(gain_element_index as isize),
                        sel_drc_index,
                        (*pstr_drc_config).is_config_changed,
                        (*pstr_drc_config).ln_gain_changed,
                    );
                    if err != 0 {
                        return err;
                    }
                } else {
                    return 2 as WORD32
                }
                gain_element_index += 1;
                parametric_drc_instance_index += 1;
            }
            g += 1;
        }
    }
    return 0 as WORD32;
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
