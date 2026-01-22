extern "C" {
    fn log10(__x: core::ffi::c_double) -> core::ffi::c_double;
    static measurement_system_default_tbl: [WORD32; 0];
    static measurement_system_bs1770_3_tbl: [WORD32; 0];
    static measurement_system_user_tbl: [WORD32; 0];
    static measurement_system_expert_tbl: [WORD32; 0];
    static measurement_system_rms_a_tbl: [WORD32; 0];
    static measurement_system_rms_b_tbl: [WORD32; 0];
    static measurement_system_rms_c_tbl: [WORD32; 0];
    static measurement_system_rms_d_tbl: [WORD32; 0];
    static measurement_system_rms_e_tbl: [WORD32; 0];
    static measurement_method_prog_loudness_tbl: [WORD32; 0];
    static measurement_method_peak_loudness_tbl: [WORD32; 0];
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
pub const MAX_NUM_COMPRESSION_EQ: core::ffi::c_int = 16 as core::ffi::c_int;
pub const UNEXPECTED_ERROR: core::ffi::c_int = 2 as core::ffi::c_int;
pub const UNDEFINED_LOUDNESS_VALUE: core::ffi::c_float = 1000.0f32;
pub const ID_FOR_BASE_LAYOUT: core::ffi::c_int = 0 as core::ffi::c_int;
pub const ID_FOR_ANY_DOWNMIX: core::ffi::c_int = 0x7f as core::ffi::c_int;
pub const ID_FOR_NO_DRC: core::ffi::c_int = 0 as core::ffi::c_int;
pub const ID_FOR_ANY_DRC: core::ffi::c_int = 0x3f as core::ffi::c_int;
pub const METHOD_DEFINITION_PROGRAM_LOUDNESS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const METHOD_DEFINITION_ANCHOR_LOUDNESS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const METHOD_DEFINITION_MAX_OF_LOUDNESS_RANGE: core::ffi::c_int = 3
    as core::ffi::c_int;
pub const METHOD_DEFINITION_MOMENTARY_LOUDNESS_MAX: core::ffi::c_int = 4
    as core::ffi::c_int;
pub const METHOD_DEFINITION_SHORT_TERM_LOUDNESS_MAX: core::ffi::c_int = 5
    as core::ffi::c_int;
pub const METHOD_DEFINITION_MIXING_LEVEL: core::ffi::c_int = 7 as core::ffi::c_int;
pub const MEASUREMENT_SYSTEM_BS_1770_4: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MEASUREMENT_SYSTEM_BS_1770_4_PRE_PROCESSING: core::ffi::c_int = 3
    as core::ffi::c_int;
pub const USER_METHOD_DEFINITION_DEFAULT: core::ffi::c_int = 0;
pub const USER_METHOD_DEFINITION_PROGRAM_LOUDNESS: core::ffi::c_int = 1;
pub const USER_METHOD_DEFINITION_ANCHOR_LOUDNESS: core::ffi::c_int = 2;
pub const USER_MEASUREMENT_SYSTEM_DEFAULT: core::ffi::c_int = 0;
pub const USER_MEASUREMENT_SYSTEM_BS_1770_4: core::ffi::c_int = 1;
pub const USER_MEASUREMENT_SYSTEM_USER: core::ffi::c_int = 2;
pub const USER_MEASUREMENT_SYSTEM_EXPERT_PANEL: core::ffi::c_int = 3;
pub const USER_MEASUREMENT_SYSTEM_RESERVED_A: core::ffi::c_int = 4;
pub const USER_MEASUREMENT_SYSTEM_RESERVED_B: core::ffi::c_int = 5;
pub const USER_MEASUREMENT_SYSTEM_RESERVED_C: core::ffi::c_int = 6;
pub const USER_MEASUREMENT_SYSTEM_RESERVED_D: core::ffi::c_int = 7;
pub const USER_MEASUREMENT_SYSTEM_RESERVED_E: core::ffi::c_int = 8;
pub const USER_LOUDNESS_PREPROCESSING_DEFAULT: core::ffi::c_int = 0;
pub const USER_LOUDNESS_PREPROCESSING_OFF: core::ffi::c_int = 1;
pub const USER_LOUDNESS_PREPROCESSING_HIGHPASS: core::ffi::c_int = 2;
pub const SHORT_TERM_LOUDNESS_TO_AVG: core::ffi::c_int = 0;
pub const MOMENTARY_LOUDNESS_TO_AVG: core::ffi::c_int = 1;
pub const TOP_OF_LOUDNESS_RANGE_TO_AVG: core::ffi::c_int = 2;
#[no_mangle]
pub unsafe extern "C" fn impd_signal_peak_level_info(
    mut pstr_drc_config: *mut ia_drc_config,
    mut pstr_loudness_info: *mut ia_drc_loudness_info_set_struct,
    mut str_drc_instruction_str: *mut ia_drc_instructions_struct,
    mut requested_dwnmix_id: WORD32,
    mut album_mode: WORD32,
    mut num_compression_eq_count: WORD32,
    mut num_compression_eq_id: *mut WORD32,
    mut peak_info_count: *mut WORD32,
    mut eq_set_id: *mut WORD32,
    mut signal_peak_level: *mut FLOAT32,
    mut explicit_peak_information_present: *mut WORD32,
) -> VOID {
    let mut c: WORD32 = 0;
    let mut d: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut base_channel_count: WORD32 = 0;
    let mut mode: WORD32 = 0;
    let mut pre_lim_count: WORD32 = 0;
    let mut peak_count: WORD32 = 0 as WORD32;
    let mut loudness_info_count: WORD32 = 0 as WORD32;
    let mut loudness_info: *mut ia_loudness_info_struct = 0
        as *mut ia_loudness_info_struct;
    let mut sum: FLOAT32 = 0.;
    let mut max_sum: FLOAT32 = 0.;
    let mut drc_set_id_requested: WORD32 = (*str_drc_instruction_str).drc_set_id;
    let mut loudness_drc_set_id_requested: WORD32 = 0;
    let mut match_found_flag: WORD32 = 0 as WORD32;
    let mut signal_peak_level_tmp: FLOAT32 = 0.;
    *eq_set_id.offset(0 as core::ffi::c_int as isize) = 0 as core::ffi::c_int as WORD32;
    *signal_peak_level.offset(0 as core::ffi::c_int as isize) = 0.0f32 as FLOAT32;
    *explicit_peak_information_present.offset(0 as core::ffi::c_int as isize) = 0
        as core::ffi::c_int as WORD32;
    k = 0 as core::ffi::c_int as WORD32;
    if drc_set_id_requested < 0 as core::ffi::c_int {
        k = 0 as core::ffi::c_int as WORD32;
        while k < num_compression_eq_count {
            *eq_set_id.offset(k as isize) = *num_compression_eq_id.offset(k as isize);
            *signal_peak_level.offset(k as isize) = 0.0f32 as FLOAT32;
            *explicit_peak_information_present.offset(k as isize) = 0 as core::ffi::c_int
                as WORD32;
            k += 1;
        }
    }
    *eq_set_id.offset(k as isize) = 0 as core::ffi::c_int as WORD32;
    *signal_peak_level.offset(k as isize) = 0.0f32 as FLOAT32;
    *explicit_peak_information_present.offset(k as isize) = 0 as core::ffi::c_int
        as WORD32;
    k += 1;
    pre_lim_count = k;
    if drc_set_id_requested < 0 as core::ffi::c_int {
        loudness_drc_set_id_requested = 0 as core::ffi::c_int as WORD32;
    } else {
        loudness_drc_set_id_requested = drc_set_id_requested;
    }
    if album_mode == 1 as core::ffi::c_int {
        mode = 1 as core::ffi::c_int as WORD32;
        loudness_info_count = (*pstr_loudness_info).loudness_info_album_count;
    } else {
        mode = 0 as core::ffi::c_int as WORD32;
        loudness_info_count = (*pstr_loudness_info).loudness_info_count;
    }
    n = 0 as core::ffi::c_int as WORD32;
    while n < loudness_info_count {
        if mode == 1 as core::ffi::c_int {
            loudness_info = &mut *((*pstr_loudness_info).str_loudness_info_album)
                .as_mut_ptr()
                .offset(n as isize) as *mut ia_loudness_info_struct;
        } else {
            loudness_info = &mut *((*pstr_loudness_info).loudness_info)
                .as_mut_ptr()
                .offset(n as isize) as *mut ia_loudness_info_struct;
        }
        if loudness_drc_set_id_requested == (*loudness_info).drc_set_id
            && requested_dwnmix_id == (*loudness_info).downmix_id
        {
            if (*loudness_info).true_peak_level_present != 0 {
                *eq_set_id.offset(peak_count as isize) = (*loudness_info).eq_set_id;
                *signal_peak_level.offset(peak_count as isize) = (*loudness_info)
                    .true_peak_level;
                *explicit_peak_information_present.offset(peak_count as isize) = 1
                    as core::ffi::c_int as WORD32;
                match_found_flag = 1 as core::ffi::c_int as WORD32;
                peak_count += 1;
            }
            if match_found_flag == 0 as core::ffi::c_int {
                if (*loudness_info).sample_peak_level_present != 0 {
                    *eq_set_id.offset(peak_count as isize) = (*loudness_info).eq_set_id;
                    *signal_peak_level.offset(peak_count as isize) = (*loudness_info)
                        .sample_peak_level;
                    *explicit_peak_information_present.offset(peak_count as isize) = 1
                        as core::ffi::c_int as WORD32;
                    match_found_flag = 1 as core::ffi::c_int as WORD32;
                    peak_count += 1;
                }
            }
        }
        n += 1;
    }
    if match_found_flag == 0 as core::ffi::c_int {
        n = 0 as core::ffi::c_int as WORD32;
        while n < loudness_info_count {
            if mode == 1 as core::ffi::c_int {
                loudness_info = &mut *((*pstr_loudness_info).str_loudness_info_album)
                    .as_mut_ptr()
                    .offset(n as isize) as *mut ia_loudness_info_struct;
            } else {
                loudness_info = &mut *((*pstr_loudness_info).loudness_info)
                    .as_mut_ptr()
                    .offset(n as isize) as *mut ia_loudness_info_struct;
            }
            if ID_FOR_ANY_DRC == (*loudness_info).drc_set_id
                && requested_dwnmix_id == (*loudness_info).downmix_id
            {
                if (*loudness_info).true_peak_level_present != 0 {
                    *eq_set_id.offset(peak_count as isize) = (*loudness_info).eq_set_id;
                    *signal_peak_level.offset(peak_count as isize) = (*loudness_info)
                        .true_peak_level;
                    *explicit_peak_information_present.offset(peak_count as isize) = 1
                        as core::ffi::c_int as WORD32;
                    match_found_flag = 1 as core::ffi::c_int as WORD32;
                    peak_count += 1;
                }
                if match_found_flag == 0 as core::ffi::c_int {
                    if (*loudness_info).sample_peak_level_present != 0 {
                        *eq_set_id.offset(peak_count as isize) = (*loudness_info)
                            .eq_set_id;
                        *signal_peak_level.offset(peak_count as isize) = (*loudness_info)
                            .sample_peak_level;
                        *explicit_peak_information_present.offset(peak_count as isize) = 1
                            as core::ffi::c_int as WORD32;
                        match_found_flag = 1 as core::ffi::c_int as WORD32;
                        peak_count += 1;
                    }
                }
            }
            n += 1;
        }
    }
    if match_found_flag == 0 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*str_drc_instruction_str).dwnmix_id_count {
            if requested_dwnmix_id
                == (*str_drc_instruction_str).downmix_id[0 as core::ffi::c_int as usize]
                || ID_FOR_ANY_DOWNMIX
                    == (*str_drc_instruction_str)
                        .downmix_id[0 as core::ffi::c_int as usize]
            {
                if (*str_drc_instruction_str).limiter_peak_target_present != 0 {
                    if (*str_drc_instruction_str).requires_eq == 1 as core::ffi::c_int {
                        d = 0 as core::ffi::c_int as WORD32;
                        while d
                            < (*pstr_drc_config).str_drc_config_ext.eq_instructions_count
                        {
                            let mut eq_instructions: *mut ia_eq_instructions_struct = &mut *((*pstr_drc_config)
                                .str_drc_config_ext
                                .str_eq_instructions)
                                .as_mut_ptr()
                                .offset(d as isize) as *mut ia_eq_instructions_struct;
                            c = 0 as core::ffi::c_int as WORD32;
                            while c < (*eq_instructions).drc_set_id_count {
                                if (*eq_instructions).drc_set_id[c as usize]
                                    == loudness_drc_set_id_requested
                                    || (*eq_instructions).drc_set_id[c as usize]
                                        == ID_FOR_ANY_DRC
                                {
                                    i = 0 as core::ffi::c_int as WORD32;
                                    while i < (*eq_instructions).dwnmix_id_count {
                                        if (*eq_instructions).downmix_id[i as usize]
                                            == requested_dwnmix_id
                                            || (*eq_instructions).downmix_id[i as usize]
                                                == ID_FOR_ANY_DOWNMIX
                                        {
                                            *eq_set_id.offset(peak_count as isize) = (*eq_instructions)
                                                .eq_set_id;
                                            *signal_peak_level.offset(peak_count as isize) = (*str_drc_instruction_str)
                                                .limiter_peak_target;
                                            *explicit_peak_information_present
                                                .offset(peak_count as isize) = 1 as core::ffi::c_int
                                                as WORD32;
                                            match_found_flag = 1 as core::ffi::c_int as WORD32;
                                            peak_count += 1;
                                        }
                                        i += 1;
                                    }
                                }
                                c += 1;
                            }
                            d += 1;
                        }
                    } else {
                        *eq_set_id.offset(peak_count as isize) = 0 as core::ffi::c_int
                            as WORD32;
                        *signal_peak_level.offset(peak_count as isize) = (*str_drc_instruction_str)
                            .limiter_peak_target;
                        *explicit_peak_information_present.offset(peak_count as isize) = 1
                            as core::ffi::c_int as WORD32;
                        match_found_flag = 1 as core::ffi::c_int as WORD32;
                        peak_count += 1;
                    }
                }
            }
            i += 1;
        }
    }
    if match_found_flag == 0 as core::ffi::c_int {
        i = 1 as core::ffi::c_int as WORD32;
        while i < (*str_drc_instruction_str).dwnmix_id_count {
            if requested_dwnmix_id == (*str_drc_instruction_str).downmix_id[i as usize] {
                if (*str_drc_instruction_str).limiter_peak_target_present != 0 {
                    *eq_set_id.offset(peak_count as isize) = 0 as core::ffi::c_int
                        as WORD32;
                    *signal_peak_level.offset(peak_count as isize) = (*str_drc_instruction_str)
                        .limiter_peak_target;
                    *explicit_peak_information_present.offset(peak_count as isize) = 1
                        as core::ffi::c_int as WORD32;
                    match_found_flag = 1 as core::ffi::c_int as WORD32;
                    peak_count += 1;
                }
            }
            i += 1;
        }
    }
    if match_found_flag == 0 as core::ffi::c_int {
        if requested_dwnmix_id != ID_FOR_BASE_LAYOUT {
            signal_peak_level_tmp = 0.0f32 as FLOAT32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*pstr_drc_config).dwnmix_instructions_count {
                if (*pstr_drc_config).dwnmix_instructions[i as usize].downmix_id
                    == requested_dwnmix_id
                {
                    if (*pstr_drc_config)
                        .dwnmix_instructions[i as usize]
                        .downmix_coefficients_present != 0
                    {
                        base_channel_count = (*pstr_drc_config)
                            .channel_layout
                            .base_channel_count;
                        max_sum = 0.0f32 as FLOAT32;
                        c = 0 as core::ffi::c_int as WORD32;
                        while c
                            < (*pstr_drc_config)
                                .dwnmix_instructions[i as usize]
                                .target_channel_count
                        {
                            sum = 0.0f32 as FLOAT32;
                            d = 0 as core::ffi::c_int as WORD32;
                            while d < base_channel_count {
                                sum
                                    += (*pstr_drc_config)
                                        .dwnmix_instructions[i as usize]
                                        .downmix_coefficient[(c * base_channel_count + d) as usize];
                                d += 1;
                            }
                            if max_sum < sum {
                                max_sum = sum;
                            }
                            c += 1;
                        }
                        signal_peak_level_tmp = 20.0f32
                            * log10(max_sum as core::ffi::c_double) as FLOAT32;
                    }
                    break;
                } else {
                    i += 1;
                }
            }
            n = 0 as core::ffi::c_int as WORD32;
            while n < loudness_info_count {
                if mode == 1 as core::ffi::c_int {
                    loudness_info = &mut *((*pstr_loudness_info).str_loudness_info_album)
                        .as_mut_ptr()
                        .offset(n as isize) as *mut ia_loudness_info_struct;
                } else {
                    loudness_info = &mut *((*pstr_loudness_info).loudness_info)
                        .as_mut_ptr()
                        .offset(n as isize) as *mut ia_loudness_info_struct;
                }
                if loudness_drc_set_id_requested == (*loudness_info).drc_set_id
                    && ID_FOR_BASE_LAYOUT == (*loudness_info).downmix_id
                {
                    if (*loudness_info).true_peak_level_present != 0 {
                        *eq_set_id.offset(peak_count as isize) = (*loudness_info)
                            .eq_set_id;
                        *signal_peak_level.offset(peak_count as isize) = (*loudness_info)
                            .true_peak_level + signal_peak_level_tmp;
                        *explicit_peak_information_present.offset(peak_count as isize) = 0
                            as core::ffi::c_int as WORD32;
                        match_found_flag = 1 as core::ffi::c_int as WORD32;
                        peak_count += 1;
                    }
                    if match_found_flag == 0 as core::ffi::c_int {
                        if (*loudness_info).sample_peak_level_present != 0 {
                            *eq_set_id.offset(peak_count as isize) = (*loudness_info)
                                .eq_set_id;
                            *signal_peak_level.offset(peak_count as isize) = (*loudness_info)
                                .sample_peak_level + signal_peak_level_tmp;
                            *explicit_peak_information_present
                                .offset(peak_count as isize) = 0 as core::ffi::c_int
                                as WORD32;
                            match_found_flag = 1 as core::ffi::c_int as WORD32;
                            peak_count += 1;
                        }
                    }
                }
                n += 1;
            }
            if match_found_flag == 0 as core::ffi::c_int {
                n = 0 as core::ffi::c_int as WORD32;
                while n < loudness_info_count {
                    if mode == 1 as core::ffi::c_int {
                        loudness_info = &mut *((*pstr_loudness_info)
                            .str_loudness_info_album)
                            .as_mut_ptr()
                            .offset(n as isize) as *mut ia_loudness_info_struct;
                    } else {
                        loudness_info = &mut *((*pstr_loudness_info).loudness_info)
                            .as_mut_ptr()
                            .offset(n as isize) as *mut ia_loudness_info_struct;
                    }
                    if ID_FOR_ANY_DRC == (*loudness_info).drc_set_id
                        && ID_FOR_BASE_LAYOUT == (*loudness_info).downmix_id
                    {
                        if (*loudness_info).true_peak_level_present != 0 {
                            *eq_set_id.offset(peak_count as isize) = (*loudness_info)
                                .eq_set_id;
                            *signal_peak_level.offset(peak_count as isize) = (*loudness_info)
                                .true_peak_level + signal_peak_level_tmp;
                            *explicit_peak_information_present
                                .offset(peak_count as isize) = 0 as core::ffi::c_int
                                as WORD32;
                            match_found_flag = 1 as core::ffi::c_int as WORD32;
                            peak_count += 1;
                        }
                        if match_found_flag == 0 as core::ffi::c_int {
                            if (*loudness_info).sample_peak_level_present != 0 {
                                *eq_set_id.offset(peak_count as isize) = (*loudness_info)
                                    .eq_set_id;
                                *signal_peak_level.offset(peak_count as isize) = (*loudness_info)
                                    .sample_peak_level + signal_peak_level_tmp;
                                *explicit_peak_information_present
                                    .offset(peak_count as isize) = 0 as core::ffi::c_int
                                    as WORD32;
                                match_found_flag = 1 as core::ffi::c_int as WORD32;
                                peak_count += 1;
                            }
                        }
                    }
                    n += 1;
                }
            }
            if match_found_flag == 0 as core::ffi::c_int {
                let mut drc_instructions_drc_tmp: *mut ia_drc_instructions_struct = 0
                    as *mut ia_drc_instructions_struct;
                n = 0 as core::ffi::c_int as WORD32;
                while n < (*pstr_drc_config).drc_instructions_count_plus {
                    drc_instructions_drc_tmp = &mut *((*pstr_drc_config)
                        .str_drc_instruction_str)
                        .as_mut_ptr()
                        .offset(n as isize) as *mut ia_drc_instructions_struct;
                    if loudness_drc_set_id_requested
                        == (*drc_instructions_drc_tmp).drc_set_id
                    {
                        if ID_FOR_BASE_LAYOUT
                            == (*drc_instructions_drc_tmp)
                                .downmix_id[0 as core::ffi::c_int as usize]
                        {
                            if (*drc_instructions_drc_tmp).limiter_peak_target_present
                                != 0
                            {
                                *eq_set_id.offset(peak_count as isize) = -(1
                                    as core::ffi::c_int) as WORD32;
                                *signal_peak_level.offset(peak_count as isize) = (*drc_instructions_drc_tmp)
                                    .limiter_peak_target + signal_peak_level_tmp;
                                *explicit_peak_information_present
                                    .offset(peak_count as isize) = 0 as core::ffi::c_int
                                    as WORD32;
                                match_found_flag = 1 as core::ffi::c_int as WORD32;
                                peak_count += 1;
                            }
                        }
                    }
                    n += 1;
                }
            }
        }
    }
    if peak_count > 0 as core::ffi::c_int {
        *peak_info_count = peak_count;
    } else {
        *peak_info_count = pre_lim_count;
    };
}
#[no_mangle]
pub unsafe extern "C" fn impd_extract_loudness_peak_to_average_info(
    mut loudness_info: *mut ia_loudness_info_struct,
    mut dyn_range_measurement_type: WORD32,
    mut loudness_peak_2_avg_value_present: *mut WORD32,
    mut loudness_peak_2_avg_value: *mut FLOAT32,
) -> WORD32 {
    let mut k: WORD32 = 0;
    let mut program_loudness_present: WORD32 = 0 as WORD32;
    let mut peak_loudness_present: WORD32 = 0 as WORD32;
    let mut match_measure_program_loudness: WORD32 = 0 as WORD32;
    let mut match_measure_peak_loudness: WORD32 = 0 as WORD32;
    let mut program_loudness: FLOAT32 = 0.0f32;
    let mut peak_loudness: FLOAT32 = 0.0f32;
    let mut loudness_measure: *mut ia_loudness_measure_struct = 0
        as *mut ia_loudness_measure_struct;
    k = 0 as core::ffi::c_int as WORD32;
    while k < (*loudness_info).measurement_count {
        loudness_measure = &mut *((*loudness_info).loudness_measure)
            .as_mut_ptr()
            .offset(k as isize) as *mut ia_loudness_measure_struct;
        if (*loudness_measure).method_def == METHOD_DEFINITION_PROGRAM_LOUDNESS {
            if match_measure_program_loudness
                < *measurement_method_prog_loudness_tbl
                    .as_ptr()
                    .offset((*loudness_measure).measurement_system as isize)
            {
                program_loudness = (*loudness_measure).method_val;
                program_loudness_present = 1 as core::ffi::c_int as WORD32;
                match_measure_program_loudness = *measurement_method_prog_loudness_tbl
                    .as_ptr()
                    .offset((*loudness_measure).measurement_system as isize);
            }
        }
        match dyn_range_measurement_type {
            SHORT_TERM_LOUDNESS_TO_AVG => {
                if (*loudness_measure).method_def
                    == METHOD_DEFINITION_SHORT_TERM_LOUDNESS_MAX
                {
                    if match_measure_peak_loudness
                        < *measurement_method_peak_loudness_tbl
                            .as_ptr()
                            .offset((*loudness_measure).measurement_system as isize)
                    {
                        peak_loudness = (*loudness_measure).method_val;
                        peak_loudness_present = 1 as core::ffi::c_int as WORD32;
                        match_measure_peak_loudness = *measurement_method_peak_loudness_tbl
                            .as_ptr()
                            .offset((*loudness_measure).measurement_system as isize);
                    }
                }
            }
            MOMENTARY_LOUDNESS_TO_AVG => {
                if (*loudness_measure).method_def
                    == METHOD_DEFINITION_MOMENTARY_LOUDNESS_MAX
                {
                    if match_measure_peak_loudness
                        < *measurement_method_peak_loudness_tbl
                            .as_ptr()
                            .offset((*loudness_measure).measurement_system as isize)
                    {
                        peak_loudness = (*loudness_measure).method_val;
                        peak_loudness_present = 1 as core::ffi::c_int as WORD32;
                        match_measure_peak_loudness = *measurement_method_peak_loudness_tbl
                            .as_ptr()
                            .offset((*loudness_measure).measurement_system as isize);
                    }
                }
            }
            TOP_OF_LOUDNESS_RANGE_TO_AVG => {
                if (*loudness_measure).method_def
                    == METHOD_DEFINITION_MAX_OF_LOUDNESS_RANGE
                {
                    if match_measure_peak_loudness
                        < *measurement_method_peak_loudness_tbl
                            .as_ptr()
                            .offset((*loudness_measure).measurement_system as isize)
                    {
                        peak_loudness = (*loudness_measure).method_val;
                        peak_loudness_present = 1 as core::ffi::c_int as WORD32;
                        match_measure_peak_loudness = *measurement_method_peak_loudness_tbl
                            .as_ptr()
                            .offset((*loudness_measure).measurement_system as isize);
                    }
                }
            }
            _ => return 2 as WORD32,
        }
        k += 1;
    }
    if program_loudness_present == 1 as core::ffi::c_int
        && peak_loudness_present == 1 as core::ffi::c_int
    {
        *loudness_peak_2_avg_value = peak_loudness - program_loudness;
        *loudness_peak_2_avg_value_present = 1 as core::ffi::c_int as WORD32;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_loudness_peak_to_average_info(
    mut pstr_loudness_info: *mut ia_drc_loudness_info_set_struct,
    mut str_drc_instruction_str: *mut ia_drc_instructions_struct,
    mut requested_dwnmix_id: WORD32,
    mut dyn_range_measurement_type: WORD32,
    mut album_mode: WORD32,
    mut loudness_peak_2_avg_value_present: *mut WORD32,
    mut loudness_peak_2_avg_value: *mut FLOAT32,
) -> WORD32 {
    let mut n: WORD32 = 0;
    let mut err: WORD32 = 0;
    let mut drc_set_id: WORD32 = if 0 as core::ffi::c_int
        > (*str_drc_instruction_str).drc_set_id
    {
        0 as WORD32
    } else {
        (*str_drc_instruction_str).drc_set_id
    };
    *loudness_peak_2_avg_value_present = 0 as core::ffi::c_int as WORD32;
    if album_mode == 1 as core::ffi::c_int {
        n = 0 as core::ffi::c_int as WORD32;
        while n < (*pstr_loudness_info).loudness_info_album_count {
            let mut loudness_info: *mut ia_loudness_info_struct = &mut *((*pstr_loudness_info)
                .str_loudness_info_album)
                .as_mut_ptr()
                .offset(n as isize) as *mut ia_loudness_info_struct;
            if drc_set_id == (*loudness_info).drc_set_id {
                if requested_dwnmix_id == (*loudness_info).downmix_id {
                    err = impd_extract_loudness_peak_to_average_info(
                        loudness_info,
                        dyn_range_measurement_type,
                        loudness_peak_2_avg_value_present,
                        loudness_peak_2_avg_value,
                    );
                    if err != 0 {
                        return err;
                    }
                }
            }
            n += 1;
        }
    }
    if *loudness_peak_2_avg_value_present == 0 as core::ffi::c_int {
        n = 0 as core::ffi::c_int as WORD32;
        while n < (*pstr_loudness_info).loudness_info_count {
            let mut loudness_info_0: *mut ia_loudness_info_struct = &mut *((*pstr_loudness_info)
                .loudness_info)
                .as_mut_ptr()
                .offset(n as isize) as *mut ia_loudness_info_struct;
            if drc_set_id == (*loudness_info_0).drc_set_id {
                if requested_dwnmix_id == (*loudness_info_0).downmix_id {
                    err = impd_extract_loudness_peak_to_average_info(
                        loudness_info_0,
                        dyn_range_measurement_type,
                        loudness_peak_2_avg_value_present,
                        loudness_peak_2_avg_value,
                    );
                    if err != 0 {
                        return err;
                    }
                }
            }
            n += 1;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_overall_loudness_present(
    mut loudness_info: *mut ia_loudness_info_struct,
    mut loudness_info_present: *mut WORD32,
) -> VOID {
    let mut m: WORD32 = 0;
    *loudness_info_present = 0 as core::ffi::c_int as WORD32;
    m = 0 as core::ffi::c_int as WORD32;
    while m < (*loudness_info).measurement_count {
        if (*loudness_info).loudness_measure[m as usize].method_def
            == METHOD_DEFINITION_PROGRAM_LOUDNESS
            || (*loudness_info).loudness_measure[m as usize].method_def
                == METHOD_DEFINITION_ANCHOR_LOUDNESS
        {
            *loudness_info_present = 1 as core::ffi::c_int as WORD32;
        }
        m += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn impd_check_loud_info(
    mut loudness_info_count: WORD32,
    mut loudness_info: *mut ia_loudness_info_struct,
    mut requested_dwnmix_id: WORD32,
    mut drc_set_id_requested: WORD32,
    mut info_count: *mut WORD32,
    mut loudness_info_matching: *mut *mut ia_loudness_info_struct,
) -> WORD32 {
    let mut n: WORD32 = 0;
    let mut loudness_info_present: WORD32 = 0;
    n = 0 as core::ffi::c_int as WORD32;
    while n < loudness_info_count {
        if requested_dwnmix_id == (*loudness_info.offset(n as isize)).downmix_id {
            if drc_set_id_requested == (*loudness_info.offset(n as isize)).drc_set_id {
                impd_overall_loudness_present(
                    &mut *loudness_info.offset(n as isize),
                    &mut loudness_info_present,
                );
                if loudness_info_present != 0 {
                    let ref mut fresh0 = *loudness_info_matching
                        .offset(*info_count as isize);
                    *fresh0 = &mut *loudness_info.offset(n as isize)
                        as *mut ia_loudness_info_struct;
                    *info_count += 1;
                }
            }
        }
        n += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_check_loud_payload(
    mut loudness_info_count: WORD32,
    mut loudness_info: *mut ia_loudness_info_struct,
    mut requested_dwnmix_id: WORD32,
    mut drc_set_id_requested: WORD32,
    mut info_count: *mut WORD32,
    mut loudness_info_matching: *mut *mut ia_loudness_info_struct,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    err = impd_check_loud_info(
        loudness_info_count,
        loudness_info,
        requested_dwnmix_id,
        drc_set_id_requested,
        info_count,
        loudness_info_matching,
    );
    if !(err != 0 || *info_count != 0) {
        err = impd_check_loud_info(
            loudness_info_count,
            loudness_info,
            ID_FOR_ANY_DOWNMIX,
            drc_set_id_requested,
            info_count,
            loudness_info_matching,
        );
        if !(err != 0 || *info_count != 0) {
            err = impd_check_loud_info(
                loudness_info_count,
                loudness_info,
                requested_dwnmix_id,
                ID_FOR_ANY_DRC,
                info_count,
                loudness_info_matching,
            );
            if !(err != 0 || *info_count != 0) {
                err = impd_check_loud_info(
                    loudness_info_count,
                    loudness_info,
                    requested_dwnmix_id,
                    ID_FOR_NO_DRC,
                    info_count,
                    loudness_info_matching,
                );
                if !(err != 0 || *info_count != 0) {
                    err = impd_check_loud_info(
                        loudness_info_count,
                        loudness_info,
                        ID_FOR_ANY_DOWNMIX,
                        ID_FOR_ANY_DRC,
                        info_count,
                        loudness_info_matching,
                    );
                    if !(err != 0 || *info_count != 0) {
                        err = impd_check_loud_info(
                            loudness_info_count,
                            loudness_info,
                            ID_FOR_ANY_DOWNMIX,
                            ID_FOR_NO_DRC,
                            info_count,
                            loudness_info_matching,
                        );
                        if !(err != 0 || *info_count != 0) {
                            err = impd_check_loud_info(
                                loudness_info_count,
                                loudness_info,
                                ID_FOR_BASE_LAYOUT,
                                drc_set_id_requested,
                                info_count,
                                loudness_info_matching,
                            );
                            if !(err != 0 || *info_count != 0) {
                                err = impd_check_loud_info(
                                    loudness_info_count,
                                    loudness_info,
                                    ID_FOR_BASE_LAYOUT,
                                    ID_FOR_ANY_DRC,
                                    info_count,
                                    loudness_info_matching,
                                );
                                if !(err != 0 || *info_count != 0) {
                                    err = impd_check_loud_info(
                                        loudness_info_count,
                                        loudness_info,
                                        ID_FOR_BASE_LAYOUT,
                                        ID_FOR_NO_DRC,
                                        info_count,
                                        loudness_info_matching,
                                    );
                                    err != 0 || *info_count != 0;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn impd_find_overall_loudness_info(
    mut pstr_drc_sel_proc_params_struct: *mut ia_drc_sel_proc_params_struct,
    mut pstr_loudness_info: *mut ia_drc_loudness_info_set_struct,
    mut requested_dwnmix_id: WORD32,
    mut drc_set_id_requested: WORD32,
    mut overall_loudness_info_present: *mut WORD32,
    mut info_count: *mut WORD32,
    mut loudness_info_matching: *mut *mut ia_loudness_info_struct,
) -> WORD32 {
    let mut err: WORD32 = 0;
    let mut loudness_drc_set_id_requested: WORD32 = 0;
    *info_count = 0 as core::ffi::c_int as WORD32;
    if drc_set_id_requested < 0 as core::ffi::c_int {
        loudness_drc_set_id_requested = ID_FOR_NO_DRC as WORD32;
    } else {
        loudness_drc_set_id_requested = drc_set_id_requested;
    }
    if (*pstr_drc_sel_proc_params_struct).album_mode == 1 as core::ffi::c_int {
        err = impd_check_loud_payload(
            (*pstr_loudness_info).loudness_info_album_count,
            ((*pstr_loudness_info).str_loudness_info_album).as_mut_ptr(),
            requested_dwnmix_id,
            loudness_drc_set_id_requested,
            info_count,
            loudness_info_matching,
        );
        if err != 0 {
            return err;
        }
    }
    if *info_count == 0 as core::ffi::c_int {
        err = impd_check_loud_payload(
            (*pstr_loudness_info).loudness_info_count,
            ((*pstr_loudness_info).loudness_info).as_mut_ptr(),
            requested_dwnmix_id,
            loudness_drc_set_id_requested,
            info_count,
            loudness_info_matching,
        );
        if err != 0 {
            return err;
        }
    }
    *overall_loudness_info_present = (*info_count > 0 as core::ffi::c_int)
        as core::ffi::c_int as WORD32;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_high_pass_loudness_adjust_info(
    mut loudness_info: *mut ia_loudness_info_struct,
    mut loudness_hp_adjust_present: *mut WORD32,
    mut loudness_hp_adjust: *mut FLOAT32,
) -> WORD32 {
    let mut m: WORD32 = 0;
    let mut k: WORD32 = 0;
    *loudness_hp_adjust_present = 0 as core::ffi::c_int as WORD32;
    *loudness_hp_adjust = 0.0f32 as FLOAT32;
    m = 0 as core::ffi::c_int as WORD32;
    while m < (*loudness_info).measurement_count {
        if (*loudness_info).loudness_measure[m as usize].measurement_system
            == MEASUREMENT_SYSTEM_BS_1770_4_PRE_PROCESSING
        {
            k = 0 as core::ffi::c_int as WORD32;
            while k < (*loudness_info).measurement_count {
                if (*loudness_info).loudness_measure[k as usize].measurement_system
                    == MEASUREMENT_SYSTEM_BS_1770_4
                {
                    if (*loudness_info).loudness_measure[m as usize].method_def
                        == (*loudness_info).loudness_measure[k as usize].method_def
                    {
                        *loudness_hp_adjust_present = 1 as core::ffi::c_int as WORD32;
                        *loudness_hp_adjust = (*loudness_info)
                            .loudness_measure[m as usize]
                            .method_val
                            - (*loudness_info).loudness_measure[k as usize].method_val;
                    }
                }
                k += 1;
            }
        }
        m += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_find_high_pass_loudness_adjust(
    mut pstr_loudness_info: *mut ia_drc_loudness_info_set_struct,
    mut requested_dwnmix_id: WORD32,
    mut drc_set_id_requested: WORD32,
    mut album_mode: WORD32,
    mut device_cutoff_freq: FLOAT32,
    mut loudness_hp_adjust_present: *mut WORD32,
    mut loudness_hp_adjust: *mut FLOAT32,
) -> WORD32 {
    let mut n: WORD32 = 0;
    let mut err: WORD32 = 0;
    let mut loudness_drc_set_id_requested: WORD32 = 0;
    if drc_set_id_requested < 0 as core::ffi::c_int {
        loudness_drc_set_id_requested = 0 as core::ffi::c_int as WORD32;
    } else {
        loudness_drc_set_id_requested = drc_set_id_requested;
    }
    *loudness_hp_adjust_present = 0 as core::ffi::c_int as WORD32;
    if album_mode == 1 as core::ffi::c_int {
        n = 0 as core::ffi::c_int as WORD32;
        while n < (*pstr_loudness_info).loudness_info_album_count {
            if requested_dwnmix_id
                == (*pstr_loudness_info).str_loudness_info_album[n as usize].downmix_id
                || ID_FOR_ANY_DOWNMIX
                    == (*pstr_loudness_info)
                        .str_loudness_info_album[n as usize]
                        .downmix_id
            {
                if loudness_drc_set_id_requested
                    == (*pstr_loudness_info)
                        .str_loudness_info_album[n as usize]
                        .drc_set_id
                {
                    err = impd_high_pass_loudness_adjust_info(
                        &mut *((*pstr_loudness_info).loudness_info)
                            .as_mut_ptr()
                            .offset(n as isize),
                        loudness_hp_adjust_present,
                        loudness_hp_adjust,
                    );
                    if err != 0 {
                        return err;
                    }
                }
            }
            n += 1;
        }
    }
    if *loudness_hp_adjust_present == 0 as core::ffi::c_int {
        n = 0 as core::ffi::c_int as WORD32;
        while n < (*pstr_loudness_info).loudness_info_count {
            if requested_dwnmix_id
                == (*pstr_loudness_info).loudness_info[n as usize].downmix_id
                || ID_FOR_ANY_DOWNMIX
                    == (*pstr_loudness_info).loudness_info[n as usize].downmix_id
            {
                if loudness_drc_set_id_requested
                    == (*pstr_loudness_info).loudness_info[n as usize].drc_set_id
                {
                    err = impd_high_pass_loudness_adjust_info(
                        &mut *((*pstr_loudness_info).loudness_info)
                            .as_mut_ptr()
                            .offset(n as isize),
                        loudness_hp_adjust_present,
                        loudness_hp_adjust,
                    );
                    if err != 0 {
                        return err;
                    }
                }
            }
            n += 1;
        }
    }
    if *loudness_hp_adjust_present == 0 as core::ffi::c_int {
        n = 0 as core::ffi::c_int as WORD32;
        while n < (*pstr_loudness_info).loudness_info_count {
            if ID_FOR_BASE_LAYOUT
                == (*pstr_loudness_info).loudness_info[n as usize].downmix_id
            {
                if loudness_drc_set_id_requested
                    == (*pstr_loudness_info).loudness_info[n as usize].drc_set_id
                {
                    err = impd_high_pass_loudness_adjust_info(
                        &mut *((*pstr_loudness_info).loudness_info)
                            .as_mut_ptr()
                            .offset(n as isize),
                        loudness_hp_adjust_present,
                        loudness_hp_adjust,
                    );
                    if err != 0 {
                        return err;
                    }
                }
            }
            n += 1;
        }
    }
    if *loudness_hp_adjust_present == 0 as core::ffi::c_int {
        n = 0 as core::ffi::c_int as WORD32;
        while n < (*pstr_loudness_info).loudness_info_count {
            if ID_FOR_BASE_LAYOUT
                == (*pstr_loudness_info).loudness_info[n as usize].downmix_id
            {
                if 0 as core::ffi::c_int
                    == (*pstr_loudness_info).loudness_info[n as usize].drc_set_id
                {
                    err = impd_high_pass_loudness_adjust_info(
                        &mut *((*pstr_loudness_info).loudness_info)
                            .as_mut_ptr()
                            .offset(n as isize),
                        loudness_hp_adjust_present,
                        loudness_hp_adjust,
                    );
                    if err != 0 {
                        return err;
                    }
                }
            }
            n += 1;
        }
    }
    if *loudness_hp_adjust_present == 0 as core::ffi::c_int {
        *loudness_hp_adjust = 0.0f32 as FLOAT32;
    } else {
        *loudness_hp_adjust
            *= ((if 20.0f32
                > (if 500.0f32 < device_cutoff_freq {
                    500.0f32
                } else {
                    device_cutoff_freq as core::ffi::c_float
                })
            {
                20.0f32
            } else {
                (if 500.0f32 < device_cutoff_freq {
                    500.0f32
                } else {
                    device_cutoff_freq as core::ffi::c_float
                })
            }) - 20.0f32) / (500.0f32 - 20.0f32);
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_init_loudness_control(
    mut pstr_drc_sel_proc_params_struct: *mut ia_drc_sel_proc_params_struct,
    mut pstr_loudness_info: *mut ia_drc_loudness_info_set_struct,
    mut requested_dwnmix_id: WORD32,
    mut drc_set_id_requested: WORD32,
    mut num_compression_eq_count: WORD32,
    mut num_compression_eq_id: *mut WORD32,
    mut loudness_info_count: *mut WORD32,
    mut eq_set_id: *mut WORD32,
    mut loudness_normalization_gain_db: *mut FLOAT32,
    mut loudness: *mut FLOAT32,
) -> WORD32 {
    let mut err: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut info_count: WORD32 = 0 as WORD32;
    let mut pre_lim_count: WORD32 = 0;
    let mut loudness_hp_adjust_present: WORD32 = 0;
    let mut overall_loudness_info_present: WORD32 = 0;
    let mut pre_proc_adjust: FLOAT32 = 0.;
    k = 0 as core::ffi::c_int as WORD32;
    if drc_set_id_requested < 0 as core::ffi::c_int {
        k = 0 as core::ffi::c_int as WORD32;
        while k < num_compression_eq_count {
            *eq_set_id.offset(k as isize) = *num_compression_eq_id.offset(k as isize);
            *loudness.offset(k as isize) = UNDEFINED_LOUDNESS_VALUE as FLOAT32;
            *loudness_normalization_gain_db.offset(k as isize) = 0.0f32 as FLOAT32;
            k += 1;
        }
    }
    if k >= MAX_NUM_COMPRESSION_EQ {
        return UNEXPECTED_ERROR;
    }
    *eq_set_id.offset(k as isize) = 0 as core::ffi::c_int as WORD32;
    *loudness.offset(k as isize) = UNDEFINED_LOUDNESS_VALUE as FLOAT32;
    *loudness_normalization_gain_db.offset(k as isize) = 0.0f32 as FLOAT32;
    k += 1;
    pre_lim_count = k;
    if (*pstr_drc_sel_proc_params_struct).loudness_normalization_on
        == 1 as core::ffi::c_int
    {
        let mut n: WORD32 = 0;
        let mut loudness_info: [*mut ia_loudness_info_struct; 16] = [0
            as *mut ia_loudness_info_struct; 16];
        err = impd_find_overall_loudness_info(
            pstr_drc_sel_proc_params_struct,
            pstr_loudness_info,
            requested_dwnmix_id,
            drc_set_id_requested,
            &mut overall_loudness_info_present,
            &mut info_count,
            loudness_info.as_mut_ptr(),
        );
        if err != 0 {
            return err;
        }
        if overall_loudness_info_present == 1 as core::ffi::c_int {
            let mut requested_method_definition: WORD32 = METHOD_DEFINITION_PROGRAM_LOUDNESS;
            let mut other_method_definition: WORD32 = METHOD_DEFINITION_PROGRAM_LOUDNESS;
            let mut requested_preprocessing: WORD32 = 0 as WORD32;
            let mut system_bonus: *const WORD32 = measurement_system_default_tbl
                .as_ptr();
            let mut match_measure: WORD32 = 0;
            let mut method_val: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
            match (*pstr_drc_sel_proc_params_struct).loudness_measurement_method {
                USER_METHOD_DEFINITION_DEFAULT
                | USER_METHOD_DEFINITION_PROGRAM_LOUDNESS => {
                    requested_method_definition = METHOD_DEFINITION_PROGRAM_LOUDNESS
                        as WORD32;
                    other_method_definition = METHOD_DEFINITION_ANCHOR_LOUDNESS
                        as WORD32;
                }
                USER_METHOD_DEFINITION_ANCHOR_LOUDNESS => {
                    requested_method_definition = METHOD_DEFINITION_ANCHOR_LOUDNESS
                        as WORD32;
                    other_method_definition = METHOD_DEFINITION_PROGRAM_LOUDNESS
                        as WORD32;
                }
                _ => return 2 as WORD32,
            }
            match (*pstr_drc_sel_proc_params_struct).loudness_measurement_system {
                USER_MEASUREMENT_SYSTEM_DEFAULT | USER_MEASUREMENT_SYSTEM_BS_1770_4 => {
                    system_bonus = measurement_system_bs1770_3_tbl.as_ptr();
                }
                USER_MEASUREMENT_SYSTEM_USER => {
                    system_bonus = measurement_system_user_tbl.as_ptr();
                }
                USER_MEASUREMENT_SYSTEM_EXPERT_PANEL => {
                    system_bonus = measurement_system_expert_tbl.as_ptr();
                }
                USER_MEASUREMENT_SYSTEM_RESERVED_A => {
                    system_bonus = measurement_system_rms_a_tbl.as_ptr();
                }
                USER_MEASUREMENT_SYSTEM_RESERVED_B => {
                    system_bonus = measurement_system_rms_b_tbl.as_ptr();
                }
                USER_MEASUREMENT_SYSTEM_RESERVED_C => {
                    system_bonus = measurement_system_rms_c_tbl.as_ptr();
                }
                USER_MEASUREMENT_SYSTEM_RESERVED_D => {
                    system_bonus = measurement_system_rms_d_tbl.as_ptr();
                }
                USER_MEASUREMENT_SYSTEM_RESERVED_E => {
                    system_bonus = measurement_system_rms_e_tbl.as_ptr();
                }
                _ => return 2 as WORD32,
            }
            match (*pstr_drc_sel_proc_params_struct).loudness_measurement_pre_proc {
                USER_LOUDNESS_PREPROCESSING_DEFAULT
                | USER_LOUDNESS_PREPROCESSING_OFF => {
                    requested_preprocessing = 0 as core::ffi::c_int as WORD32;
                }
                USER_LOUDNESS_PREPROCESSING_HIGHPASS => {
                    requested_preprocessing = 1 as core::ffi::c_int as WORD32;
                }
                _ => return 2 as WORD32,
            }
            k = 0 as core::ffi::c_int as WORD32;
            while k < info_count {
                match_measure = -(1 as core::ffi::c_int) as WORD32;
                n = 0 as core::ffi::c_int as WORD32;
                while n < (*loudness_info[k as usize]).measurement_count {
                    let mut loudness_measure: *mut ia_loudness_measure_struct = &mut *((**loudness_info
                        .as_mut_ptr()
                        .offset(k as isize))
                        .loudness_measure)
                        .as_mut_ptr()
                        .offset(n as isize) as *mut ia_loudness_measure_struct;
                    if match_measure
                        < *system_bonus
                            .offset((*loudness_measure).measurement_system as isize)
                        && requested_method_definition == (*loudness_measure).method_def
                    {
                        method_val = (*loudness_measure).method_val;
                        match_measure = *system_bonus
                            .offset((*loudness_measure).measurement_system as isize);
                    }
                    n += 1;
                }
                if match_measure == -(1 as core::ffi::c_int) {
                    n = 0 as core::ffi::c_int as WORD32;
                    while n < (*loudness_info[k as usize]).measurement_count {
                        let mut loudness_measure_0: *mut ia_loudness_measure_struct = &mut *((**loudness_info
                            .as_mut_ptr()
                            .offset(k as isize))
                            .loudness_measure)
                            .as_mut_ptr()
                            .offset(n as isize) as *mut ia_loudness_measure_struct;
                        if match_measure
                            < *system_bonus
                                .offset((*loudness_measure_0).measurement_system as isize)
                            && other_method_definition
                                == (*loudness_measure_0).method_def
                        {
                            method_val = (*loudness_measure_0).method_val;
                            match_measure = *system_bonus
                                .offset((*loudness_measure_0).measurement_system as isize);
                        }
                        n += 1;
                    }
                }
                if requested_preprocessing == 1 as core::ffi::c_int {
                    err = impd_find_high_pass_loudness_adjust(
                        pstr_loudness_info,
                        requested_dwnmix_id,
                        drc_set_id_requested,
                        (*pstr_drc_sel_proc_params_struct).album_mode,
                        (*pstr_drc_sel_proc_params_struct).device_cut_off_frequency
                            as FLOAT32,
                        &mut loudness_hp_adjust_present,
                        &mut pre_proc_adjust,
                    );
                    if err != 0 {
                        return err;
                    }
                    if loudness_hp_adjust_present == 0 as core::ffi::c_int {
                        pre_proc_adjust = -2.0f32 as FLOAT32;
                    }
                    method_val += pre_proc_adjust;
                }
                *eq_set_id.offset(k as isize) = 0 as core::ffi::c_int as WORD32;
                *loudness_normalization_gain_db.offset(k as isize) = (*pstr_drc_sel_proc_params_struct)
                    .target_loudness - method_val;
                *loudness.offset(k as isize) = method_val;
                k += 1;
            }
        }
    }
    if info_count > 0 as core::ffi::c_int {
        *loudness_info_count = info_count;
    } else {
        *loudness_info_count = pre_lim_count;
    }
    return 0 as WORD32;
}
pub const MIXING_LEVEL_DEFAULT: core::ffi::c_float = 85.0f32;
#[no_mangle]
pub unsafe extern "C" fn impd_mixing_level_info(
    mut pstr_drc_sel_proc_params_struct: *mut ia_drc_sel_proc_params_struct,
    mut pstr_loudness_info: *mut ia_drc_loudness_info_set_struct,
    mut requested_dwnmix_id: WORD32,
    mut drc_set_id_requested: WORD32,
    mut eq_set_id_requested: WORD32,
    mut mixing_level: *mut FLOAT32,
) -> WORD32 {
    let mut n: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut info_count: WORD32 = 0;
    let mut album_mode: WORD32 = (*pstr_drc_sel_proc_params_struct).album_mode;
    let mut loudness_drc_set_id_requested: WORD32 = 0;
    let mut loudness_info: *mut ia_loudness_info_struct = 0
        as *mut ia_loudness_info_struct;
    *mixing_level = MIXING_LEVEL_DEFAULT as FLOAT32;
    if drc_set_id_requested < 0 as core::ffi::c_int {
        loudness_drc_set_id_requested = 0 as core::ffi::c_int as WORD32;
    } else {
        loudness_drc_set_id_requested = drc_set_id_requested;
    }
    if album_mode == 1 as core::ffi::c_int {
        info_count = (*pstr_loudness_info).loudness_info_album_count;
        loudness_info = ((*pstr_loudness_info).str_loudness_info_album).as_mut_ptr();
    } else {
        info_count = (*pstr_loudness_info).loudness_info_count;
        loudness_info = ((*pstr_loudness_info).loudness_info).as_mut_ptr();
    }
    n = 0 as core::ffi::c_int as WORD32;
    while n < info_count {
        if requested_dwnmix_id == (*loudness_info.offset(n as isize)).downmix_id
            || ID_FOR_ANY_DOWNMIX == (*loudness_info.offset(n as isize)).downmix_id
        {
            if loudness_drc_set_id_requested
                == (*loudness_info.offset(n as isize)).drc_set_id
            {
                if eq_set_id_requested == (*loudness_info.offset(n as isize)).eq_set_id {
                    k = 0 as core::ffi::c_int as WORD32;
                    while k < (*loudness_info.offset(n as isize)).measurement_count {
                        if (*loudness_info.offset(n as isize))
                            .loudness_measure[k as usize]
                            .method_def == METHOD_DEFINITION_MIXING_LEVEL
                        {
                            *mixing_level = (*loudness_info.offset(n as isize))
                                .loudness_measure[k as usize]
                                .method_val;
                            break;
                        } else {
                            k += 1;
                        }
                    }
                }
            }
        }
        n += 1;
    }
    return 0 as WORD32;
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
