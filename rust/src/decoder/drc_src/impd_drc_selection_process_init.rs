extern "C" {
    fn memcmp(
        __s1: *const core::ffi::c_void,
        __s2: *const core::ffi::c_void,
        __n: size_t,
    ) -> core::ffi::c_int;
}
pub type size_t = usize;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
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
pub struct ia_drc_sel_proc_output_struct {
    pub output_peak_level_db: FLOAT32,
    pub loudness_normalization_gain_db: FLOAT32,
    pub output_loudness: FLOAT32,
    pub sel_drc_set_ids: [WORD32; 4],
    pub sel_downmix_ids: [WORD32; 4],
    pub num_sel_drc_sets: WORD32,
    pub active_downmix_id: WORD32,
    pub base_channel_count: WORD32,
    pub target_channel_count: WORD32,
    pub target_layout: WORD32,
    pub downmix_matrix_present: WORD32,
    pub downmix_matrix: [[FLOAT32; 128]; 128],
    pub boost: FLOAT32,
    pub compress: FLOAT32,
    pub drc_characteristic_target: WORD32,
    pub mixing_level: FLOAT32,
    pub sel_eq_set_ids: [WORD32; 2],
    pub sel_loud_eq_id: WORD32,
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
pub struct ia_loudness_measure_struct {
    pub method_def: WORD32,
    pub method_val: FLOAT32,
    pub measurement_system: WORD32,
    pub reliability: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_loudness_info_struct {
    pub drc_set_id: WORD32,
    pub eq_set_id: WORD32,
    pub downmix_id: WORD32,
    pub sample_peak_level_present: WORD32,
    pub sample_peak_level: FLOAT32,
    pub true_peak_level_present: WORD32,
    pub true_peak_level: FLOAT32,
    pub true_peak_level_measurement_system: WORD32,
    pub true_peak_level_reliability: WORD32,
    pub measurement_count: WORD32,
    pub anchor_loudness_present: WORD32,
    pub expert_loudness_present: WORD32,
    pub loudness_measure: [ia_loudness_measure_struct; 16],
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
pub struct ia_loudness_info_set_ext_struct {
    pub loudness_info_set_ext_type: [WORD32; 8],
    pub ext_bit_size: [WORD32; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_loudness_info_set_struct {
    pub loudness_info_album_count: WORD32,
    pub loudness_info_count: WORD32,
    pub loudness_info_set_ext_present: WORD32,
    pub str_loudness_info_album: [ia_loudness_info_struct; 36],
    pub loudness_info: [ia_loudness_info_struct; 36],
    pub str_loudness_info_set_ext: ia_loudness_info_set_ext_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_loudness_eq_parameter_interface_struct {
    pub loudness_eq_request_flag: WORD32,
    pub loudness_eq_request: WORD32,
    pub sensitivity_flag: WORD32,
    pub sensitivity: FLOAT32,
    pub playback_gain_flag: WORD32,
    pub playback_gain: FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_equalization_ctrl_interface_struct {
    pub eq_set_purpose_request: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_specific_interface_extension_struct {
    pub ext_size_bits: WORD32,
    pub ext_bit_size: WORD32,
    pub uni_drc_interface_ext_type: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_uni_interface_ext_struct {
    pub interface_ext_count: WORD32,
    pub specific_interface_ext: [ia_specific_interface_extension_struct; 8],
    pub loudness_leveling_on: WORD32,
    pub loudness_eq_parameter_interface_flag: WORD32,
    pub eq_ctrl_interface_flag: WORD32,
    pub loudness_eq_parameter_interface: ia_loudness_eq_parameter_interface_struct,
    pub eq_ctrl_interface: ia_equalization_ctrl_interface_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_parameter_interface_struct {
    pub change_compress: WORD32,
    pub change_boost: WORD32,
    pub compress: FLOAT32,
    pub boost: FLOAT32,
    pub change_drc_characteristic_target: WORD32,
    pub drc_characteristic_target: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_dyn_rng_ctrl_interface_struct {
    pub dynamic_range_control_on: WORD32,
    pub num_drc_feature_requests: WORD32,
    pub drc_feature_req_type: [WORD32; 7],
    pub requested_num_drc_effects: [WORD32; 7],
    pub desired_num_drc_effects_of_requested: [WORD32; 7],
    pub requested_drc_effect_type: [[WORD32; 15]; 7],
    pub requested_dyn_rng_measurement_type: [WORD32; 7],
    pub requested_dyn_range_is_single_val_flag: [WORD32; 7],
    pub requested_dyn_range_value: [FLOAT32; 7],
    pub requested_dyn_range_min_val: [FLOAT32; 7],
    pub requested_dyn_range_max_val: [FLOAT32; 7],
    pub requested_drc_characteristic: [WORD32; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_loudness_norm_parameter_interface_struct {
    pub album_mode: WORD32,
    pub peak_limiter: WORD32,
    pub change_loudness_deviation_max: WORD32,
    pub loudness_deviation_max: WORD32,
    pub change_loudness_measur_method: WORD32,
    pub loudness_measurement_method: WORD32,
    pub change_loudness_measur_system: WORD32,
    pub loudness_measurement_system: WORD32,
    pub change_loudness_measur_pre_proc: WORD32,
    pub loudness_measurement_pre_proc: WORD32,
    pub change_device_cut_off_freq: WORD32,
    pub device_cut_off_frequency: WORD32,
    pub change_loudness_norm_gain_db_max: WORD32,
    pub loudness_norm_gain_db_max: FLOAT32,
    pub change_loudness_norm_gain_modification_db: WORD32,
    pub loudness_norm_gain_modification_db: FLOAT32,
    pub change_output_peak_level_max: WORD32,
    pub output_peak_level_max: FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_loudness_norm_ctrl_interface_struct {
    pub loudness_normalization_on: WORD32,
    pub target_loudness: FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_system_interface_struct {
    pub target_config_request_type: WORD32,
    pub num_downmix_id_requests: WORD32,
    pub requested_dwnmix_id: [WORD32; 15],
    pub requested_target_layout: WORD32,
    pub requested_target_ch_count: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_uni_interface_signat_struct {
    pub interface_signat_type: WORD32,
    pub interface_signat_data_len: WORD32,
    pub interface_signat_data: [UWORD32; 2048],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_interface_struct {
    pub interface_signat_flag: WORD32,
    pub system_interface_flag: WORD32,
    pub loudness_norm_ctrl_interface_flag: WORD32,
    pub loudness_norm_parameter_interface_flag: WORD32,
    pub drc_interface_flag: WORD32,
    pub drc_parameter_interface_flag: WORD32,
    pub drc_uni_interface_ext_flag: WORD32,
    pub drc_uni_interface_signature: ia_drc_uni_interface_signat_struct,
    pub system_interface: ia_system_interface_struct,
    pub loudness_norm_ctrl_interface: ia_loudness_norm_ctrl_interface_struct,
    pub loudness_norm_param_interface: ia_loudness_norm_parameter_interface_struct,
    pub drc_ctrl_interface: ia_dyn_rng_ctrl_interface_struct,
    pub drc_parameter_interface: ia_drc_parameter_interface_struct,
    pub drc_uni_interface_ext: ia_drc_uni_interface_ext_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_sel_proc_params_struct {
    pub base_channel_count: WORD32,
    pub base_layout: WORD32,
    pub target_config_request_type: WORD32,
    pub num_downmix_id_requests: WORD32,
    pub requested_dwnmix_id: [WORD32; 15],
    pub requested_target_layout: WORD32,
    pub requested_target_ch_count: WORD32,
    pub target_ch_count_prelim: WORD32,
    pub loudness_normalization_on: WORD32,
    pub target_loudness: FLOAT32,
    pub album_mode: WORD32,
    pub peak_limiter: WORD32,
    pub loudness_deviation_max: WORD32,
    pub loudness_measurement_method: WORD32,
    pub loudness_measurement_system: WORD32,
    pub loudness_measurement_pre_proc: WORD32,
    pub device_cut_off_frequency: WORD32,
    pub loudness_norm_gain_db_max: FLOAT32,
    pub loudness_norm_gain_modification_db: FLOAT32,
    pub output_peak_level_max: FLOAT32,
    pub num_bands_supported: WORD32,
    pub dynamic_range_control_on: WORD32,
    pub num_drc_feature_requests: WORD32,
    pub drc_feature_req_type: [WORD32; 7],
    pub requested_num_drc_effects: [WORD32; 7],
    pub desired_num_drc_effects_of_requested: [WORD32; 7],
    pub requested_drc_effect_type: [[WORD32; 15]; 7],
    pub requested_dyn_range_measur_type: [WORD32; 7],
    pub requested_dyn_range_range_flag: [WORD32; 7],
    pub requested_dyn_range_value: [FLOAT32; 7],
    pub requested_dyn_range_min_val: [FLOAT32; 7],
    pub requested_dyn_range_max_val: [FLOAT32; 7],
    pub requested_drc_characteristic: [WORD32; 7],
    pub loudness_eq_request: WORD32,
    pub sensitivity: FLOAT32,
    pub playback_gain: FLOAT32,
    pub eq_set_purpose_request: WORD32,
    pub loudness_leveling_on: WORD32,
    pub boost: FLOAT32,
    pub compress: FLOAT32,
    pub drc_characteristic_target: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_sel_pro_struct {
    pub first_frame: WORD32,
    pub drc_config_flag: WORD32,
    pub loudness_info_set_flag: WORD32,
    pub sel_proc_request_flag: WORD32,
    pub subband_domain_mode: WORD32,
    pub eq_inst_index: [WORD32; 2],
    pub drc_instructions_index: [WORD32; 4],
    pub uni_drc_sel_proc_params: ia_drc_sel_proc_params_struct,
    pub drc_config: ia_drc_config,
    pub loudness_info_set: ia_drc_loudness_info_set_struct,
    pub drc_inst_index_sel: WORD32,
    pub drc_coef_index_sel: WORD32,
    pub downmix_inst_index_sel: WORD32,
    pub drc_set_id_valid_flag: [WORD32; 36],
    pub eq_set_id_valid_flag: [WORD32; 8],
    pub eq_inst_index_sel: WORD32,
    pub loud_eq_inst_index_sel: WORD32,
    pub compl_level_supported_total: FLOAT32,
    pub uni_drc_sel_proc_output: ia_drc_sel_proc_output_struct,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const UNDEFINED_LOUDNESS_VALUE: core::ffi::c_float = 1000.0f32;
pub const LOUD_EQ_REQUEST_OFF: core::ffi::c_int = 0 as core::ffi::c_int;
pub const EQ_PURPOSE_EQ_OFF: core::ffi::c_int = 0 as core::ffi::c_int;
pub const USER_METHOD_DEFINITION_DEFAULT: core::ffi::c_int = 0 as core::ffi::c_int;
pub const USER_MEASUREMENT_SYSTEM_DEFAULT: core::ffi::c_int = 0 as core::ffi::c_int;
pub const USER_LOUDNESS_PREPROCESSING_DEFAULT: core::ffi::c_int = 0 as core::ffi::c_int;
pub const LOUDNESS_DEVIATION_MAX_DEFAULT: core::ffi::c_int = 63 as core::ffi::c_int;
pub const LOUDNESS_NORMALIZATION_GAIN_MAX_DEFAULT: core::ffi::c_int = 1000
    as core::ffi::c_int;
pub const COMPLEXITY_LEVEL_SUPPORTED_TOTAL: core::ffi::c_float = 20.0f32;
#[no_mangle]
pub unsafe extern "C" fn impd_drc_sel_proc_init_dflt(
    mut pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
) -> WORD32 {
    if !pstr_drc_uni_sel_proc.is_null() {
        (*pstr_drc_uni_sel_proc).first_frame = 0 as core::ffi::c_int as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.base_channel_count = -(1
            as core::ffi::c_int) as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.base_layout = -(1
            as core::ffi::c_int) as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.target_config_request_type = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.num_downmix_id_requests = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.album_mode = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.peak_limiter = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.loudness_normalization_on = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.target_loudness = -24.0f32
            as FLOAT32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.loudness_deviation_max = LOUDNESS_DEVIATION_MAX_DEFAULT
            as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.loudness_measurement_method = USER_METHOD_DEFINITION_DEFAULT
            as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.loudness_measurement_system = USER_MEASUREMENT_SYSTEM_DEFAULT
            as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.loudness_measurement_pre_proc = USER_LOUDNESS_PREPROCESSING_DEFAULT
            as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.device_cut_off_frequency = 500
            as core::ffi::c_int as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.loudness_norm_gain_db_max = LOUDNESS_NORMALIZATION_GAIN_MAX_DEFAULT
            as FLOAT32;
        (*pstr_drc_uni_sel_proc)
            .uni_drc_sel_proc_params
            .loudness_norm_gain_modification_db = 0.0f32 as FLOAT32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.output_peak_level_max = 0.0f32
            as FLOAT32;
        if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.peak_limiter
            == 1 as core::ffi::c_int
        {
            (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.output_peak_level_max = 6.0f32
                as FLOAT32;
        }
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.dynamic_range_control_on = 1
            as core::ffi::c_int as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.num_bands_supported = 4
            as core::ffi::c_int as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.num_drc_feature_requests = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.boost = 1.0f32 as FLOAT32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.compress = 1.0f32 as FLOAT32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.drc_characteristic_target = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.loudness_eq_request = LOUD_EQ_REQUEST_OFF
            as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.eq_set_purpose_request = EQ_PURPOSE_EQ_OFF
            as WORD32;
        (*pstr_drc_uni_sel_proc).compl_level_supported_total = COMPLEXITY_LEVEL_SUPPORTED_TOTAL
            as FLOAT32;
        (*pstr_drc_uni_sel_proc).drc_inst_index_sel = -(1 as core::ffi::c_int) as WORD32;
        (*pstr_drc_uni_sel_proc).drc_coef_index_sel = -(1 as core::ffi::c_int) as WORD32;
        (*pstr_drc_uni_sel_proc).downmix_inst_index_sel = -(1 as core::ffi::c_int)
            as WORD32;
        (*pstr_drc_uni_sel_proc)
            .drc_instructions_index[0 as core::ffi::c_int as usize] = -(1
            as core::ffi::c_int) as WORD32;
        (*pstr_drc_uni_sel_proc)
            .drc_instructions_index[1 as core::ffi::c_int as usize] = -(1
            as core::ffi::c_int) as WORD32;
        (*pstr_drc_uni_sel_proc)
            .drc_instructions_index[2 as core::ffi::c_int as usize] = -(1
            as core::ffi::c_int) as WORD32;
        (*pstr_drc_uni_sel_proc)
            .drc_instructions_index[3 as core::ffi::c_int as usize] = -(1
            as core::ffi::c_int) as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.output_peak_level_db = 0
            as core::ffi::c_int as FLOAT32;
        (*pstr_drc_uni_sel_proc)
            .uni_drc_sel_proc_output
            .loudness_normalization_gain_db = 0 as core::ffi::c_int as FLOAT32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.output_loudness = UNDEFINED_LOUDNESS_VALUE
            as FLOAT32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.num_sel_drc_sets = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.active_downmix_id = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.base_channel_count = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.target_channel_count = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.downmix_matrix_present = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_uni_sel_proc).eq_inst_index[0 as core::ffi::c_int as usize] = -(1
            as core::ffi::c_int) as WORD32;
        (*pstr_drc_uni_sel_proc).eq_inst_index[1 as core::ffi::c_int as usize] = -(1
            as core::ffi::c_int) as WORD32;
        (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1 as core::ffi::c_int as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.loudness_leveling_on = 1
            as core::ffi::c_int as WORD32;
    } else {
        return 1 as WORD32
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_drc_sel_proc_init_sel_proc_params(
    mut pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
    mut pstr_drc_sel_proc_params_struct: *mut ia_drc_sel_proc_params_struct,
) -> WORD32 {
    if !pstr_drc_uni_sel_proc.is_null() && !pstr_drc_sel_proc_params_struct.is_null() {
        if memcmp(
            &mut (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params
                as *mut ia_drc_sel_proc_params_struct as *const core::ffi::c_void,
            pstr_drc_sel_proc_params_struct as *const core::ffi::c_void,
            ::core::mem::size_of::<ia_drc_sel_proc_params_struct>() as size_t,
        ) != 0
        {
            (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params = *pstr_drc_sel_proc_params_struct;
            (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1 as core::ffi::c_int
                as WORD32;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_drc_sel_proc_init_interface_params(
    mut pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
    mut pstr_drc_interface: *mut ia_drc_interface_struct,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    if !pstr_drc_uni_sel_proc.is_null() && !pstr_drc_interface.is_null() {
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.loudness_leveling_on = (*pstr_drc_interface)
            .drc_uni_interface_ext
            .loudness_leveling_on;
        if (*pstr_drc_interface).system_interface_flag != 0 {
            if (*pstr_drc_uni_sel_proc)
                .uni_drc_sel_proc_params
                .target_config_request_type
                != (*pstr_drc_interface).system_interface.target_config_request_type
            {
                (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .target_config_request_type = (*pstr_drc_interface)
                    .system_interface
                    .target_config_request_type;
                (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1 as core::ffi::c_int
                    as WORD32;
            }
            match (*pstr_drc_uni_sel_proc)
                .uni_drc_sel_proc_params
                .target_config_request_type
            {
                0 => {
                    if (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .num_downmix_id_requests
                        != (*pstr_drc_interface).system_interface.num_downmix_id_requests
                    {
                        (*pstr_drc_uni_sel_proc)
                            .uni_drc_sel_proc_params
                            .num_downmix_id_requests = (*pstr_drc_interface)
                            .system_interface
                            .num_downmix_id_requests;
                        (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                            as core::ffi::c_int as WORD32;
                    }
                    i = 0 as core::ffi::c_int as WORD32;
                    while i
                        < (*pstr_drc_uni_sel_proc)
                            .uni_drc_sel_proc_params
                            .num_downmix_id_requests
                    {
                        if (*pstr_drc_uni_sel_proc)
                            .uni_drc_sel_proc_params
                            .requested_dwnmix_id[i as usize]
                            != (*pstr_drc_interface)
                                .system_interface
                                .requested_dwnmix_id[i as usize]
                        {
                            (*pstr_drc_uni_sel_proc)
                                .uni_drc_sel_proc_params
                                .requested_dwnmix_id[i as usize] = (*pstr_drc_interface)
                                .system_interface
                                .requested_dwnmix_id[i as usize];
                            (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                                as core::ffi::c_int as WORD32;
                        }
                        i += 1;
                    }
                }
                1 => {
                    if (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .requested_target_layout
                        != (*pstr_drc_interface).system_interface.requested_target_layout
                    {
                        (*pstr_drc_uni_sel_proc)
                            .uni_drc_sel_proc_params
                            .requested_target_layout = (*pstr_drc_interface)
                            .system_interface
                            .requested_target_layout;
                        (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                            as core::ffi::c_int as WORD32;
                    }
                }
                2 => {
                    if (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .requested_target_ch_count
                        != (*pstr_drc_interface)
                            .system_interface
                            .requested_target_ch_count
                    {
                        (*pstr_drc_uni_sel_proc)
                            .uni_drc_sel_proc_params
                            .requested_target_ch_count = (*pstr_drc_interface)
                            .system_interface
                            .requested_target_ch_count;
                        (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                            as core::ffi::c_int as WORD32;
                    }
                }
                _ => {}
            }
        }
        if (*pstr_drc_interface).loudness_norm_ctrl_interface_flag != 0 {
            if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.loudness_normalization_on
                != (*pstr_drc_interface)
                    .loudness_norm_ctrl_interface
                    .loudness_normalization_on
            {
                (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .loudness_normalization_on = (*pstr_drc_interface)
                    .loudness_norm_ctrl_interface
                    .loudness_normalization_on;
                (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1 as core::ffi::c_int
                    as WORD32;
            }
            if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.loudness_normalization_on
                != 0
            {
                if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.target_loudness
                    != (*pstr_drc_interface).loudness_norm_ctrl_interface.target_loudness
                {
                    (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.target_loudness = (*pstr_drc_interface)
                        .loudness_norm_ctrl_interface
                        .target_loudness;
                    (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                        as core::ffi::c_int as WORD32;
                }
            }
        }
        if (*pstr_drc_interface).loudness_norm_parameter_interface_flag != 0 {
            if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.album_mode
                != (*pstr_drc_interface).loudness_norm_param_interface.album_mode
            {
                (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.album_mode = (*pstr_drc_interface)
                    .loudness_norm_param_interface
                    .album_mode;
                (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1 as core::ffi::c_int
                    as WORD32;
            }
            if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.peak_limiter
                != (*pstr_drc_interface).loudness_norm_param_interface.peak_limiter
            {
                (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.peak_limiter = (*pstr_drc_interface)
                    .loudness_norm_param_interface
                    .peak_limiter;
                (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1 as core::ffi::c_int
                    as WORD32;
            }
            if (*pstr_drc_interface)
                .loudness_norm_param_interface
                .change_loudness_deviation_max != 0
            {
                if (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .loudness_deviation_max
                    != (*pstr_drc_interface)
                        .loudness_norm_param_interface
                        .loudness_deviation_max
                {
                    (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .loudness_deviation_max = (*pstr_drc_interface)
                        .loudness_norm_param_interface
                        .loudness_deviation_max;
                    (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                        as core::ffi::c_int as WORD32;
                }
            }
            if (*pstr_drc_interface)
                .loudness_norm_param_interface
                .change_loudness_measur_method != 0
            {
                if (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .loudness_measurement_method
                    != (*pstr_drc_interface)
                        .loudness_norm_param_interface
                        .loudness_measurement_method
                {
                    (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .loudness_measurement_method = (*pstr_drc_interface)
                        .loudness_norm_param_interface
                        .loudness_measurement_method;
                    (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                        as core::ffi::c_int as WORD32;
                }
            }
            if (*pstr_drc_interface)
                .loudness_norm_param_interface
                .change_loudness_measur_pre_proc != 0
            {
                if (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .loudness_measurement_pre_proc
                    != (*pstr_drc_interface)
                        .loudness_norm_param_interface
                        .loudness_measurement_pre_proc
                {
                    (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .loudness_measurement_pre_proc = (*pstr_drc_interface)
                        .loudness_norm_param_interface
                        .loudness_measurement_pre_proc;
                    (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                        as core::ffi::c_int as WORD32;
                }
            }
            if (*pstr_drc_interface)
                .loudness_norm_param_interface
                .change_loudness_measur_system != 0
            {
                if (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .loudness_measurement_system
                    != (*pstr_drc_interface)
                        .loudness_norm_param_interface
                        .loudness_measurement_system
                {
                    (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .loudness_measurement_system = (*pstr_drc_interface)
                        .loudness_norm_param_interface
                        .loudness_measurement_system;
                    (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                        as core::ffi::c_int as WORD32;
                }
            }
            if (*pstr_drc_interface)
                .loudness_norm_param_interface
                .change_device_cut_off_freq != 0
            {
                if (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .device_cut_off_frequency
                    != (*pstr_drc_interface)
                        .loudness_norm_param_interface
                        .device_cut_off_frequency
                {
                    (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .device_cut_off_frequency = (*pstr_drc_interface)
                        .loudness_norm_param_interface
                        .device_cut_off_frequency;
                    (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                        as core::ffi::c_int as WORD32;
                }
            }
            if (*pstr_drc_interface)
                .loudness_norm_param_interface
                .change_loudness_norm_gain_db_max != 0
            {
                if (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .loudness_norm_gain_db_max
                    != (*pstr_drc_interface)
                        .loudness_norm_param_interface
                        .loudness_norm_gain_db_max
                {
                    (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .loudness_norm_gain_db_max = (*pstr_drc_interface)
                        .loudness_norm_param_interface
                        .loudness_norm_gain_db_max;
                    (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                        as core::ffi::c_int as WORD32;
                }
            }
            if (*pstr_drc_interface)
                .loudness_norm_param_interface
                .change_loudness_norm_gain_modification_db != 0
            {
                if (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .loudness_norm_gain_modification_db
                    != (*pstr_drc_interface)
                        .loudness_norm_param_interface
                        .loudness_norm_gain_modification_db
                {
                    (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .loudness_norm_gain_modification_db = (*pstr_drc_interface)
                        .loudness_norm_param_interface
                        .loudness_norm_gain_modification_db;
                    (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                        as core::ffi::c_int as WORD32;
                }
            }
            if (*pstr_drc_interface)
                .loudness_norm_param_interface
                .change_output_peak_level_max != 0
            {
                if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.output_peak_level_max
                    != (*pstr_drc_interface)
                        .loudness_norm_param_interface
                        .output_peak_level_max
                {
                    (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .output_peak_level_max = (*pstr_drc_interface)
                        .loudness_norm_param_interface
                        .output_peak_level_max;
                    (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                        as core::ffi::c_int as WORD32;
                }
            }
        }
        if (*pstr_drc_interface).drc_interface_flag != 0 {
            if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.dynamic_range_control_on
                != (*pstr_drc_interface).drc_ctrl_interface.dynamic_range_control_on
            {
                (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .dynamic_range_control_on = (*pstr_drc_interface)
                    .drc_ctrl_interface
                    .dynamic_range_control_on;
                (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1 as core::ffi::c_int
                    as WORD32;
                if (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .dynamic_range_control_on == 0
                {
                    (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .num_drc_feature_requests = 0 as core::ffi::c_int as WORD32;
                }
            }
            if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.dynamic_range_control_on
                != 0
            {
                if (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .num_drc_feature_requests
                    != (*pstr_drc_interface).drc_ctrl_interface.num_drc_feature_requests
                {
                    (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .num_drc_feature_requests = (*pstr_drc_interface)
                        .drc_ctrl_interface
                        .num_drc_feature_requests;
                    (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                        as core::ffi::c_int as WORD32;
                }
                i = 0 as core::ffi::c_int as WORD32;
                while i
                    < (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .num_drc_feature_requests
                {
                    if (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .drc_feature_req_type[i as usize]
                        != (*pstr_drc_interface)
                            .drc_ctrl_interface
                            .drc_feature_req_type[i as usize]
                    {
                        (*pstr_drc_uni_sel_proc)
                            .uni_drc_sel_proc_params
                            .drc_feature_req_type[i as usize] = (*pstr_drc_interface)
                            .drc_ctrl_interface
                            .drc_feature_req_type[i as usize];
                        (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                            as core::ffi::c_int as WORD32;
                    }
                    match (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .drc_feature_req_type[i as usize]
                    {
                        0 => {
                            if (*pstr_drc_uni_sel_proc)
                                .uni_drc_sel_proc_params
                                .requested_num_drc_effects[i as usize]
                                != (*pstr_drc_interface)
                                    .drc_ctrl_interface
                                    .requested_num_drc_effects[i as usize]
                            {
                                (*pstr_drc_uni_sel_proc)
                                    .uni_drc_sel_proc_params
                                    .requested_num_drc_effects[i as usize] = (*pstr_drc_interface)
                                    .drc_ctrl_interface
                                    .requested_num_drc_effects[i as usize];
                                (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                                    as core::ffi::c_int as WORD32;
                            }
                            if (*pstr_drc_uni_sel_proc)
                                .uni_drc_sel_proc_params
                                .desired_num_drc_effects_of_requested[i as usize]
                                != (*pstr_drc_interface)
                                    .drc_ctrl_interface
                                    .desired_num_drc_effects_of_requested[i as usize]
                            {
                                (*pstr_drc_uni_sel_proc)
                                    .uni_drc_sel_proc_params
                                    .desired_num_drc_effects_of_requested[i as usize] = (*pstr_drc_interface)
                                    .drc_ctrl_interface
                                    .desired_num_drc_effects_of_requested[i as usize];
                                (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                                    as core::ffi::c_int as WORD32;
                            }
                            j = 0 as core::ffi::c_int as WORD32;
                            while j
                                < (*pstr_drc_uni_sel_proc)
                                    .uni_drc_sel_proc_params
                                    .requested_num_drc_effects[i as usize]
                            {
                                if (*pstr_drc_uni_sel_proc)
                                    .uni_drc_sel_proc_params
                                    .requested_drc_effect_type[i as usize][j as usize]
                                    != (*pstr_drc_interface)
                                        .drc_ctrl_interface
                                        .requested_drc_effect_type[i as usize][j as usize]
                                {
                                    (*pstr_drc_uni_sel_proc)
                                        .uni_drc_sel_proc_params
                                        .requested_drc_effect_type[i as usize][j as usize] = (*pstr_drc_interface)
                                        .drc_ctrl_interface
                                        .requested_drc_effect_type[i as usize][j as usize];
                                    (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                                        as core::ffi::c_int as WORD32;
                                }
                                j += 1;
                            }
                        }
                        1 => {
                            if (*pstr_drc_uni_sel_proc)
                                .uni_drc_sel_proc_params
                                .requested_dyn_range_measur_type[i as usize]
                                != (*pstr_drc_interface)
                                    .drc_ctrl_interface
                                    .requested_dyn_rng_measurement_type[i as usize]
                            {
                                (*pstr_drc_uni_sel_proc)
                                    .uni_drc_sel_proc_params
                                    .requested_dyn_range_measur_type[i as usize] = (*pstr_drc_interface)
                                    .drc_ctrl_interface
                                    .requested_dyn_rng_measurement_type[i as usize];
                                (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                                    as core::ffi::c_int as WORD32;
                            }
                            if (*pstr_drc_uni_sel_proc)
                                .uni_drc_sel_proc_params
                                .requested_dyn_range_range_flag[i as usize]
                                != (*pstr_drc_interface)
                                    .drc_ctrl_interface
                                    .requested_dyn_range_is_single_val_flag[i as usize]
                            {
                                (*pstr_drc_uni_sel_proc)
                                    .uni_drc_sel_proc_params
                                    .requested_dyn_range_range_flag[i as usize] = (*pstr_drc_interface)
                                    .drc_ctrl_interface
                                    .requested_dyn_range_is_single_val_flag[i as usize];
                                (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                                    as core::ffi::c_int as WORD32;
                            }
                            if (*pstr_drc_uni_sel_proc)
                                .uni_drc_sel_proc_params
                                .requested_dyn_range_range_flag[i as usize]
                                == 0 as core::ffi::c_int
                            {
                                if (*pstr_drc_uni_sel_proc)
                                    .uni_drc_sel_proc_params
                                    .requested_dyn_range_value[i as usize]
                                    != (*pstr_drc_interface)
                                        .drc_ctrl_interface
                                        .requested_dyn_range_value[i as usize]
                                {
                                    (*pstr_drc_uni_sel_proc)
                                        .uni_drc_sel_proc_params
                                        .requested_dyn_range_value[i as usize] = (*pstr_drc_interface)
                                        .drc_ctrl_interface
                                        .requested_dyn_range_value[i as usize];
                                    (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                                        as core::ffi::c_int as WORD32;
                                }
                            } else {
                                if (*pstr_drc_uni_sel_proc)
                                    .uni_drc_sel_proc_params
                                    .requested_dyn_range_min_val[i as usize]
                                    != (*pstr_drc_interface)
                                        .drc_ctrl_interface
                                        .requested_dyn_range_min_val[i as usize]
                                {
                                    (*pstr_drc_uni_sel_proc)
                                        .uni_drc_sel_proc_params
                                        .requested_dyn_range_min_val[i as usize] = (*pstr_drc_interface)
                                        .drc_ctrl_interface
                                        .requested_dyn_range_min_val[i as usize];
                                    (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                                        as core::ffi::c_int as WORD32;
                                }
                                if (*pstr_drc_uni_sel_proc)
                                    .uni_drc_sel_proc_params
                                    .requested_dyn_range_max_val[i as usize]
                                    != (*pstr_drc_interface)
                                        .drc_ctrl_interface
                                        .requested_dyn_range_max_val[i as usize]
                                {
                                    (*pstr_drc_uni_sel_proc)
                                        .uni_drc_sel_proc_params
                                        .requested_dyn_range_max_val[i as usize] = (*pstr_drc_interface)
                                        .drc_ctrl_interface
                                        .requested_dyn_range_max_val[i as usize];
                                    (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                                        as core::ffi::c_int as WORD32;
                                }
                            }
                        }
                        2 => {
                            if (*pstr_drc_uni_sel_proc)
                                .uni_drc_sel_proc_params
                                .requested_drc_characteristic[i as usize]
                                != (*pstr_drc_interface)
                                    .drc_ctrl_interface
                                    .requested_drc_characteristic[i as usize]
                            {
                                (*pstr_drc_uni_sel_proc)
                                    .uni_drc_sel_proc_params
                                    .requested_drc_characteristic[i as usize] = (*pstr_drc_interface)
                                    .drc_ctrl_interface
                                    .requested_drc_characteristic[i as usize];
                                (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                                    as core::ffi::c_int as WORD32;
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }
            }
        }
        if (*pstr_drc_interface).drc_parameter_interface_flag != 0 {
            if (*pstr_drc_interface).drc_parameter_interface.change_compress != 0 {
                if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.compress
                    != (*pstr_drc_interface).drc_parameter_interface.compress
                {
                    (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.compress = (*pstr_drc_interface)
                        .drc_parameter_interface
                        .compress;
                    (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                        as core::ffi::c_int as WORD32;
                }
            }
            if (*pstr_drc_interface).drc_parameter_interface.change_boost != 0 {
                if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.boost
                    != (*pstr_drc_interface).drc_parameter_interface.boost
                {
                    (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.boost = (*pstr_drc_interface)
                        .drc_parameter_interface
                        .boost;
                    (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                        as core::ffi::c_int as WORD32;
                }
            }
            if (*pstr_drc_interface)
                .drc_parameter_interface
                .change_drc_characteristic_target != 0
            {
                if (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .drc_characteristic_target
                    != (*pstr_drc_interface)
                        .drc_parameter_interface
                        .drc_characteristic_target
                {
                    (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .drc_characteristic_target = (*pstr_drc_interface)
                        .drc_parameter_interface
                        .drc_characteristic_target;
                    (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 1
                        as core::ffi::c_int as WORD32;
                }
            }
        }
        if (*pstr_drc_interface).drc_uni_interface_ext_flag != 0 {
            let mut drc_uni_interface_ext: *mut ia_drc_uni_interface_ext_struct = &mut (*pstr_drc_interface)
                .drc_uni_interface_ext;
            if (*drc_uni_interface_ext).loudness_eq_parameter_interface_flag != 0 {
                let mut loudness_eq_parameter_interface: *mut ia_loudness_eq_parameter_interface_struct = &mut (*drc_uni_interface_ext)
                    .loudness_eq_parameter_interface;
                if (*loudness_eq_parameter_interface).loudness_eq_request_flag != 0 {
                    (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .loudness_eq_request = (*loudness_eq_parameter_interface)
                        .loudness_eq_request;
                    (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.sensitivity = (*loudness_eq_parameter_interface)
                        .sensitivity;
                    (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.playback_gain = (*loudness_eq_parameter_interface)
                        .playback_gain;
                }
            }
            if (*drc_uni_interface_ext).eq_ctrl_interface_flag != 0 {
                (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .eq_set_purpose_request = (*drc_uni_interface_ext)
                    .eq_ctrl_interface
                    .eq_set_purpose_request;
            }
        }
    }
}
