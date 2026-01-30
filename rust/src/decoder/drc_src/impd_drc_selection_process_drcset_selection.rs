extern "C" {
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn fabs(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn impd_find_eq_set_no_compression(
        pstr_drc_config: *mut ia_drc_config,
        requested_dwnmix_id: WORD32,
        no_compression_eq_cnt: *mut WORD32,
        no_compression_eq_id: *mut WORD32,
    ) -> WORD32;
    fn impd_match_eq_set(
        drc_config: *mut ia_drc_config,
        downmix_id: WORD32,
        drc_set_id: WORD32,
        eq_set_id_valid_flag: *mut WORD32,
        matching_eq_set_cnt: *mut WORD32,
        matching_eq_set_idx: *mut WORD32,
    ) -> WORD32;
    fn impd_select_loud_eq(
        pstr_drc_config: *mut ia_drc_config,
        requested_dwnmix_id: WORD32,
        drc_set_id_requested: WORD32,
        eq_set_id_requested: WORD32,
        loud_eq_id_selected: *mut WORD32,
    ) -> WORD32;
    fn impd_select_drc_coeff3(
        drc_config: *mut ia_drc_config,
        str_p_loc_drc_coefficients_uni_drc: *mut *mut ia_uni_drc_coeffs_struct,
    ) -> VOID;
    fn impd_match_eq_set_purpose(
        drc_config: *mut ia_drc_config,
        eq_set_purpose_requested: WORD32,
        eq_set_id_valid_flag: *mut WORD32,
        selection_candidate_count: *mut WORD32,
        selection_candidate_info: *mut ia_selection_candidate_info_struct,
        selection_candidate_info_step_2: *mut ia_selection_candidate_info_struct,
    ) -> WORD32;
    fn impd_mixing_level_info(
        pstr_drc_sel_proc_params_struct: *mut ia_drc_sel_proc_params_struct,
        pstr_loudness_info: *mut ia_drc_loudness_info_set_struct,
        requested_dwnmix_id: WORD32,
        drc_set_id_requested: WORD32,
        eq_set_id_requested: WORD32,
        mixing_level: *mut FLOAT32,
    ) -> WORD32;
    fn impd_signal_peak_level_info(
        pstr_drc_config: *mut ia_drc_config,
        pstr_loudness_info: *mut ia_drc_loudness_info_set_struct,
        str_drc_instruction_str: *mut ia_drc_instructions_struct,
        requested_dwnmix_id: WORD32,
        album_mode: WORD32,
        num_compression_eq_count: WORD32,
        num_compression_eq_id: *mut WORD32,
        peak_info_count: *mut WORD32,
        eq_set_id: *mut WORD32,
        signal_peak_level: *mut FLOAT32,
        explicit_peak_information_present: *mut WORD32,
    ) -> VOID;
    fn impd_loudness_peak_to_average_info(
        pstr_loudness_info: *mut ia_drc_loudness_info_set_struct,
        str_drc_instruction_str: *mut ia_drc_instructions_struct,
        requested_dwnmix_id: WORD32,
        dyn_range_measurement_type: WORD32,
        album_mode: WORD32,
        loudness_peak_2_avg_value_present: *mut WORD32,
        loudness_peak_2_avg_value: *mut FLOAT32,
    ) -> WORD32;
    fn impd_init_loudness_control(
        pstr_drc_sel_proc_params_struct: *mut ia_drc_sel_proc_params_struct,
        pstr_loudness_info: *mut ia_drc_loudness_info_set_struct,
        requested_dwnmix_id: WORD32,
        drc_set_id_requested: WORD32,
        num_compression_eq_count: WORD32,
        num_compression_eq_id: *mut WORD32,
        loudness_info_count: *mut WORD32,
        eq_set_id: *mut WORD32,
        loudness_normalization_gain_db: *mut FLOAT32,
        loudness: *mut FLOAT32,
    ) -> WORD32;
    static drc_characteristic_order_default: [[WORD32; 3]; 0];
}
pub type size_t = usize;
pub type WORD32 = core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_selection_candidate_info_struct {
    pub drc_instructions_index: WORD32,
    pub downmix_id_request_index: WORD32,
    pub eq_set_id: WORD32,
    pub output_peak_level: FLOAT32,
    pub loudness_norm_db_gain_adjusted: FLOAT32,
    pub output_loudness: FLOAT32,
    pub mixing_level: FLOAT32,
    pub selection_flags: WORD32,
}
pub const DOWNMIX_INSTRUCTION_COUNT_MAX: core::ffi::c_int = 16 as core::ffi::c_int;
pub const DRC_INSTRUCTIONS_COUNT_MAX: core::ffi::c_int = DOWNMIX_INSTRUCTION_COUNT_MAX
    + 20 as core::ffi::c_int;
pub const SELECTION_CANDIDATE_COUNT_MAX: core::ffi::c_int = 32 as core::ffi::c_int;
pub const UNEXPECTED_ERROR: core::ffi::c_int = 2 as core::ffi::c_int;
pub const EXTERNAL_ERROR: core::ffi::c_int = 4 as core::ffi::c_int;
pub const UNDEFINED_LOUDNESS_VALUE: core::ffi::c_float = 1000.0f32;
pub const ID_FOR_BASE_LAYOUT: core::ffi::c_int = 0 as core::ffi::c_int;
pub const ID_FOR_ANY_DOWNMIX: core::ffi::c_int = 0x7f as core::ffi::c_int;
pub const LOCATION_MP4_INSTREAM_UNIDRC: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const LOCATION_SELECTED: core::ffi::c_int = LOCATION_MP4_INSTREAM_UNIDRC;
pub const MAX_LOUDNESS_INFO_COUNT: core::ffi::c_int = 16 as core::ffi::c_int;
pub const EQ_PURPOSE_EQ_OFF: core::ffi::c_int = 0 as core::ffi::c_int;
pub const EFFECT_BIT_NONE: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const EFFECT_BIT_NIGHT: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const EFFECT_BIT_NOISY: core::ffi::c_int = 0x2 as core::ffi::c_int;
pub const EFFECT_BIT_LIMITED: core::ffi::c_int = 0x4 as core::ffi::c_int;
pub const EFFECT_BIT_LOWLEVEL: core::ffi::c_int = 0x8 as core::ffi::c_int;
pub const EFFECT_BIT_DIALOG: core::ffi::c_int = 0x10 as core::ffi::c_int;
pub const EFFECT_BIT_GENERAL_COMPR: core::ffi::c_int = 0x20 as core::ffi::c_int;
pub const EFFECT_BIT_EXPAND: core::ffi::c_int = 0x40 as core::ffi::c_int;
pub const EFFECT_BIT_ARTISTIC: core::ffi::c_int = 0x80 as core::ffi::c_int;
pub const EFFECT_BIT_CLIPPING: core::ffi::c_int = 0x100 as core::ffi::c_int;
pub const EFFECT_BIT_FADE: core::ffi::c_int = 0x200 as core::ffi::c_int;
pub const EFFECT_BIT_DUCK_OTHER: core::ffi::c_int = 0x400 as core::ffi::c_int;
pub const EFFECT_BIT_DUCK_SELF: core::ffi::c_int = 0x800 as core::ffi::c_int;
pub const EFFECT_TYPE_REQUESTED_NONE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const EFFECT_TYPE_REQUESTED_NIGHT: core::ffi::c_int = 1;
pub const EFFECT_TYPE_REQUESTED_NOISY: core::ffi::c_int = 2;
pub const EFFECT_TYPE_REQUESTED_LIMITED: core::ffi::c_int = 3;
pub const EFFECT_TYPE_REQUESTED_LOWLEVEL: core::ffi::c_int = 4;
pub const EFFECT_TYPE_REQUESTED_DIALOG: core::ffi::c_int = 5;
pub const EFFECT_TYPE_REQUESTED_GENERAL_COMPR: core::ffi::c_int = 6;
pub const EFFECT_TYPE_REQUESTED_EXPAND: core::ffi::c_int = 7;
pub const EFFECT_TYPE_REQUESTED_ARTISTIC: core::ffi::c_int = 8;
pub const MATCH_EFFECT_TYPE: core::ffi::c_int = 0;
pub const MATCH_DYNAMIC_RANGE: core::ffi::c_int = 1;
pub const MATCH_DRC_CHARACTERISTIC: core::ffi::c_int = 2;
static mut effect_types_request_table: [WORD32; 8] = [
    EFFECT_BIT_NIGHT,
    EFFECT_BIT_NOISY,
    EFFECT_BIT_LIMITED,
    EFFECT_BIT_LOWLEVEL,
    EFFECT_BIT_DIALOG,
    EFFECT_BIT_GENERAL_COMPR,
    EFFECT_BIT_EXPAND,
    EFFECT_BIT_ARTISTIC,
];
#[no_mangle]
pub unsafe extern "C" fn impd_validate_requested_drc_feature(
    mut pstr_drc_sel_proc_params_struct: *mut ia_drc_sel_proc_params_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*pstr_drc_sel_proc_params_struct).num_drc_feature_requests {
        match (*pstr_drc_sel_proc_params_struct).drc_feature_req_type[i as usize] {
            MATCH_EFFECT_TYPE => {
                j = 0 as core::ffi::c_int as WORD32;
                while j
                    < (*pstr_drc_sel_proc_params_struct)
                        .desired_num_drc_effects_of_requested[i as usize]
                {
                    if (*pstr_drc_sel_proc_params_struct)
                        .requested_drc_effect_type[i as usize][j as usize]
                        == EFFECT_TYPE_REQUESTED_NONE
                    {
                        if (*pstr_drc_sel_proc_params_struct)
                            .desired_num_drc_effects_of_requested[i as usize]
                            > 1 as core::ffi::c_int
                        {
                            return 2 as WORD32;
                        }
                    }
                    j += 1;
                }
            }
            MATCH_DYNAMIC_RANGE | MATCH_DRC_CHARACTERISTIC => {}
            _ => return 2 as WORD32,
        }
        i += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_find_drc_instructions_uni_drc(
    mut drc_config: *mut ia_drc_config,
    mut drc_set_id_requested: WORD32,
    mut str_drc_instruction_str: *mut *mut ia_drc_instructions_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*drc_config).drc_instructions_uni_drc_count {
        if drc_set_id_requested
            == (*drc_config).str_drc_instruction_str[i as usize].drc_set_id
        {
            break;
        }
        i += 1;
    }
    if i == (*drc_config).drc_instructions_uni_drc_count {
        return 2 as WORD32;
    }
    *str_drc_instruction_str = &mut *((*drc_config).str_drc_instruction_str)
        .as_mut_ptr()
        .offset(i as isize) as *mut ia_drc_instructions_struct;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_map_requested_effect_bit_idx(
    mut requested_effect_type: WORD32,
    mut effect_bit_idx: *mut WORD32,
) -> WORD32 {
    match requested_effect_type {
        EFFECT_TYPE_REQUESTED_NONE => {
            *effect_bit_idx = EFFECT_BIT_NONE as WORD32;
        }
        EFFECT_TYPE_REQUESTED_NIGHT => {
            *effect_bit_idx = EFFECT_BIT_NIGHT as WORD32;
        }
        EFFECT_TYPE_REQUESTED_NOISY => {
            *effect_bit_idx = EFFECT_BIT_NOISY as WORD32;
        }
        EFFECT_TYPE_REQUESTED_LIMITED => {
            *effect_bit_idx = EFFECT_BIT_LIMITED as WORD32;
        }
        EFFECT_TYPE_REQUESTED_LOWLEVEL => {
            *effect_bit_idx = EFFECT_BIT_LOWLEVEL as WORD32;
        }
        EFFECT_TYPE_REQUESTED_DIALOG => {
            *effect_bit_idx = EFFECT_BIT_DIALOG as WORD32;
        }
        EFFECT_TYPE_REQUESTED_GENERAL_COMPR => {
            *effect_bit_idx = EFFECT_BIT_GENERAL_COMPR as WORD32;
        }
        EFFECT_TYPE_REQUESTED_EXPAND => {
            *effect_bit_idx = EFFECT_BIT_EXPAND as WORD32;
        }
        EFFECT_TYPE_REQUESTED_ARTISTIC => {
            *effect_bit_idx = EFFECT_BIT_ARTISTIC as WORD32;
        }
        _ => return 2 as WORD32,
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_get_fading_drc_set(
    mut pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
) -> WORD32 {
    (*pstr_drc_uni_sel_proc).drc_instructions_index[2 as core::ffi::c_int as usize] = -(1
        as core::ffi::c_int) as WORD32;
    if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.album_mode
        == 0 as core::ffi::c_int
    {
        let mut n: WORD32 = 0;
        let mut str_drc_instruction_str: *mut ia_drc_instructions_struct = 0
            as *mut ia_drc_instructions_struct;
        n = 0 as core::ffi::c_int as WORD32;
        while n < (*pstr_drc_uni_sel_proc).drc_config.drc_instructions_uni_drc_count {
            str_drc_instruction_str = &mut *((*pstr_drc_uni_sel_proc)
                .drc_config
                .str_drc_instruction_str)
                .as_mut_ptr()
                .offset(n as isize) as *mut ia_drc_instructions_struct;
            if (*str_drc_instruction_str).drc_set_effect as core::ffi::c_int
                & EFFECT_BIT_FADE != 0
            {
                if (*str_drc_instruction_str).downmix_id[0 as core::ffi::c_int as usize]
                    == ID_FOR_ANY_DOWNMIX
                {
                    (*pstr_drc_uni_sel_proc)
                        .drc_instructions_index[2 as core::ffi::c_int as usize] = n;
                } else {
                    return 2 as WORD32
                }
            }
            n += 1;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_get_ducking_drc_set(
    mut pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
) -> WORD32 {
    let mut drc_instructions_index: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut str_drc_instruction_str: *mut ia_drc_instructions_struct = 0
        as *mut ia_drc_instructions_struct;
    let mut requested_dwnmix_id: WORD32 = (*pstr_drc_uni_sel_proc)
        .uni_drc_sel_proc_output
        .active_downmix_id;
    (*pstr_drc_uni_sel_proc).drc_instructions_index[3 as core::ffi::c_int as usize] = -(1
        as core::ffi::c_int) as WORD32;
    drc_instructions_index = -(1 as core::ffi::c_int) as WORD32;
    str_drc_instruction_str = 0 as *mut ia_drc_instructions_struct;
    n = 0 as core::ffi::c_int as WORD32;
    while n < (*pstr_drc_uni_sel_proc).drc_config.drc_instructions_uni_drc_count {
        str_drc_instruction_str = &mut *((*pstr_drc_uni_sel_proc)
            .drc_config
            .str_drc_instruction_str)
            .as_mut_ptr()
            .offset(n as isize) as *mut ia_drc_instructions_struct;
        if (*str_drc_instruction_str).drc_set_effect as core::ffi::c_int
            & (EFFECT_BIT_DUCK_OTHER | EFFECT_BIT_DUCK_SELF) != 0
        {
            k = 0 as core::ffi::c_int as WORD32;
            while k < (*str_drc_instruction_str).dwnmix_id_count {
                if (*str_drc_instruction_str).downmix_id[k as usize]
                    == requested_dwnmix_id
                {
                    if drc_instructions_index != -(1 as core::ffi::c_int) {
                        break;
                    }
                    if !((*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .loudness_leveling_on == 0 as core::ffi::c_int
                        && (*str_drc_instruction_str).leveling_present
                            == 1 as core::ffi::c_int)
                    {
                        drc_instructions_index = n;
                    }
                }
                k += 1;
            }
        }
        n += 1;
    }
    if drc_instructions_index == -(1 as core::ffi::c_int) {
        n = 0 as core::ffi::c_int as WORD32;
        while n < (*pstr_drc_uni_sel_proc).drc_config.drc_instructions_uni_drc_count {
            str_drc_instruction_str = &mut *((*pstr_drc_uni_sel_proc)
                .drc_config
                .str_drc_instruction_str)
                .as_mut_ptr()
                .offset(n as isize) as *mut ia_drc_instructions_struct;
            if (*str_drc_instruction_str).drc_set_effect as core::ffi::c_int
                & (EFFECT_BIT_DUCK_OTHER | EFFECT_BIT_DUCK_SELF) != 0
            {
                k = 0 as core::ffi::c_int as WORD32;
                while k < (*str_drc_instruction_str).dwnmix_id_count {
                    if (*str_drc_instruction_str).downmix_id[k as usize]
                        == ID_FOR_BASE_LAYOUT
                    {
                        if drc_instructions_index != -(1 as core::ffi::c_int) {
                            break;
                        }
                        if !((*pstr_drc_uni_sel_proc)
                            .uni_drc_sel_proc_params
                            .loudness_leveling_on == 0 as core::ffi::c_int
                            && (*str_drc_instruction_str).leveling_present
                                == 1 as core::ffi::c_int)
                        {
                            drc_instructions_index = n;
                        }
                    }
                    k += 1;
                }
            }
            n += 1;
        }
    }
    if drc_instructions_index > -(1 as core::ffi::c_int) {
        (*pstr_drc_uni_sel_proc)
            .drc_instructions_index[2 as core::ffi::c_int as usize] = -(1
            as core::ffi::c_int) as WORD32;
        (*pstr_drc_uni_sel_proc)
            .drc_instructions_index[3 as core::ffi::c_int as usize] = drc_instructions_index;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_get_selected_drc_set(
    mut pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
    mut drc_set_id_selected: WORD32,
) -> WORD32 {
    let mut n: WORD32 = 0;
    n = 0 as core::ffi::c_int as WORD32;
    while n < (*pstr_drc_uni_sel_proc).drc_config.drc_instructions_count_plus {
        if (*pstr_drc_uni_sel_proc)
            .drc_config
            .str_drc_instruction_str[n as usize]
            .drc_set_id == drc_set_id_selected
        {
            break;
        }
        n += 1;
    }
    if n == (*pstr_drc_uni_sel_proc).drc_config.drc_instructions_count_plus {
        return 4 as WORD32;
    }
    (*pstr_drc_uni_sel_proc).drc_inst_index_sel = n;
    (*pstr_drc_uni_sel_proc).drc_instructions_index[0 as core::ffi::c_int as usize] = (*pstr_drc_uni_sel_proc)
        .drc_inst_index_sel;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_get_dependent_drc_set(
    mut pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
) -> WORD32 {
    let mut str_drc_instruction_str: *mut ia_drc_instructions_struct = 0
        as *mut ia_drc_instructions_struct;
    str_drc_instruction_str = &mut *((*pstr_drc_uni_sel_proc)
        .drc_config
        .str_drc_instruction_str)
        .as_mut_ptr()
        .offset((*pstr_drc_uni_sel_proc).drc_inst_index_sel as isize)
        as *mut ia_drc_instructions_struct;
    if (*str_drc_instruction_str).depends_on_drc_set_present == 1 as core::ffi::c_int {
        let mut n: WORD32 = 0;
        let mut drc_dependent_set_id: WORD32 = (*str_drc_instruction_str)
            .depends_on_drc_set;
        n = 0 as core::ffi::c_int as WORD32;
        while n < (*pstr_drc_uni_sel_proc).drc_config.drc_instructions_count_plus {
            if (*pstr_drc_uni_sel_proc)
                .drc_config
                .str_drc_instruction_str[n as usize]
                .drc_set_id == drc_dependent_set_id
            {
                break;
            }
            n += 1;
        }
        if n == (*pstr_drc_uni_sel_proc).drc_config.drc_instructions_count_plus {
            return 2 as WORD32;
        }
        (*pstr_drc_uni_sel_proc)
            .drc_instructions_index[1 as core::ffi::c_int as usize] = n;
    } else {
        (*pstr_drc_uni_sel_proc)
            .drc_instructions_index[1 as core::ffi::c_int as usize] = -(1
            as core::ffi::c_int) as WORD32;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_get_dependent_drc_instructions(
    mut drc_config: *const ia_drc_config,
    mut str_drc_instruction_str: *const ia_drc_instructions_struct,
    mut drc_instructions_dependent: *mut *mut ia_drc_instructions_struct,
) -> WORD32 {
    let mut j: WORD32 = 0;
    let mut dependent_drc: *mut ia_drc_instructions_struct = 0
        as *mut ia_drc_instructions_struct;
    j = 0 as core::ffi::c_int as WORD32;
    while j < (*drc_config).drc_instructions_uni_drc_count {
        dependent_drc = &*((*drc_config).str_drc_instruction_str)
            .as_ptr()
            .offset(j as isize) as *const ia_drc_instructions_struct
            as *mut ia_drc_instructions_struct;
        if (*dependent_drc).drc_set_id == (*str_drc_instruction_str).depends_on_drc_set {
            break;
        }
        j += 1;
    }
    if j == (*drc_config).drc_instructions_uni_drc_count {
        return 2 as WORD32;
    }
    if (*dependent_drc).depends_on_drc_set_present == 1 as core::ffi::c_int {
        return 2 as WORD32;
    }
    *drc_instructions_dependent = dependent_drc;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_select_drcs_without_compr_effects(
    mut pstr_drc_config: *mut ia_drc_config,
    mut match_found_flag: *mut WORD32,
    mut selection_candidate_count: *mut WORD32,
    mut selection_candidate_info: *mut ia_selection_candidate_info_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut selection_candidate_step_2_count: WORD32 = 0 as WORD32;
    let mut selection_candidate_info_step_2: [ia_selection_candidate_info_struct; 32] = [ia_selection_candidate_info_struct {
        drc_instructions_index: 0,
        downmix_id_request_index: 0,
        eq_set_id: 0,
        output_peak_level: 0.,
        loudness_norm_db_gain_adjusted: 0.,
        output_loudness: 0.,
        mixing_level: 0.,
        selection_flags: 0,
    }; 32];
    let mut effect_types_request_table_size: WORD32 = 0;
    let mut match_0: WORD32 = 0;
    let mut str_drc_instruction_str: *mut ia_drc_instructions_struct = 0
        as *mut ia_drc_instructions_struct;
    effect_types_request_table_size = (::core::mem::size_of::<[WORD32; 8]>() as usize)
        .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as WORD32;
    k = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < *selection_candidate_count {
        str_drc_instruction_str = &mut *((*pstr_drc_config).str_drc_instruction_str)
            .as_mut_ptr()
            .offset(
                (*selection_candidate_info.offset(i as isize)).drc_instructions_index
                    as isize,
            ) as *mut ia_drc_instructions_struct;
        match_0 = 1 as core::ffi::c_int as WORD32;
        n = 0 as core::ffi::c_int as WORD32;
        while n < effect_types_request_table_size {
            if (*str_drc_instruction_str).drc_set_effect
                & effect_types_request_table[n as usize] != 0 as core::ffi::c_int
            {
                match_0 = 0 as core::ffi::c_int as WORD32;
            }
            n += 1;
        }
        if match_0 == 1 as core::ffi::c_int {
            if k >= SELECTION_CANDIDATE_COUNT_MAX {
                return UNEXPECTED_ERROR;
            }
            memcpy(
                &mut *selection_candidate_info_step_2.as_mut_ptr().offset(k as isize)
                    as *mut ia_selection_candidate_info_struct as *mut core::ffi::c_void,
                &mut *selection_candidate_info.offset(i as isize)
                    as *mut ia_selection_candidate_info_struct
                    as *const core::ffi::c_void,
                ::core::mem::size_of::<ia_selection_candidate_info_struct>() as size_t,
            );
            k += 1;
        }
        i += 1;
    }
    selection_candidate_step_2_count = k;
    if selection_candidate_step_2_count > 0 as core::ffi::c_int {
        *match_found_flag = 1 as core::ffi::c_int as WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < selection_candidate_step_2_count {
            memcpy(
                &mut *selection_candidate_info.offset(i as isize)
                    as *mut ia_selection_candidate_info_struct as *mut core::ffi::c_void,
                &mut *selection_candidate_info_step_2.as_mut_ptr().offset(i as isize)
                    as *mut ia_selection_candidate_info_struct
                    as *const core::ffi::c_void,
                ::core::mem::size_of::<ia_selection_candidate_info_struct>() as size_t,
            );
            *selection_candidate_count = selection_candidate_step_2_count;
            i += 1;
        }
    } else {
        *match_found_flag = 0 as core::ffi::c_int as WORD32;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_match_effect_type_attempt(
    mut pstr_drc_config: *mut ia_drc_config,
    mut requested_effect_type: WORD32,
    mut state_requested: WORD32,
    mut match_found_flag: *mut WORD32,
    mut selection_candidate_count: *mut WORD32,
    mut selection_candidate_info: *mut ia_selection_candidate_info_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut err: WORD32 = 0;
    let mut selection_candidate_step_2_count: WORD32 = 0 as WORD32;
    let mut selection_candidate_info_step_2: [ia_selection_candidate_info_struct; 32] = [ia_selection_candidate_info_struct {
        drc_instructions_index: 0,
        downmix_id_request_index: 0,
        eq_set_id: 0,
        output_peak_level: 0.,
        loudness_norm_db_gain_adjusted: 0.,
        output_loudness: 0.,
        mixing_level: 0.,
        selection_flags: 0,
    }; 32];
    let mut str_drc_instruction_str: *mut ia_drc_instructions_struct = 0
        as *mut ia_drc_instructions_struct;
    let mut drc_instructions_dependent: *mut ia_drc_instructions_struct = 0
        as *mut ia_drc_instructions_struct;
    let mut effect_bit_idx: WORD32 = 0;
    err = impd_map_requested_effect_bit_idx(requested_effect_type, &mut effect_bit_idx);
    if err != 0 {
        return err;
    }
    if effect_bit_idx == EFFECT_BIT_NONE {
        err = impd_select_drcs_without_compr_effects(
            pstr_drc_config,
            match_found_flag,
            selection_candidate_count,
            selection_candidate_info,
        );
        if err != 0 {
            return err;
        }
    } else {
        k = 0 as core::ffi::c_int as WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < *selection_candidate_count {
            str_drc_instruction_str = &mut *((*pstr_drc_config).str_drc_instruction_str)
                .as_mut_ptr()
                .offset(
                    (*selection_candidate_info.offset(i as isize)).drc_instructions_index
                        as isize,
                ) as *mut ia_drc_instructions_struct;
            if (*str_drc_instruction_str).depends_on_drc_set_present
                == 1 as core::ffi::c_int
            {
                err = impd_get_dependent_drc_instructions(
                    pstr_drc_config,
                    str_drc_instruction_str,
                    &mut drc_instructions_dependent,
                );
                if err != 0 {
                    return err;
                }
                if state_requested == 1 as core::ffi::c_int {
                    if (*str_drc_instruction_str).drc_set_effect & effect_bit_idx
                        != 0 as core::ffi::c_int
                        || (*drc_instructions_dependent).drc_set_effect & effect_bit_idx
                            != 0 as core::ffi::c_int
                    {
                        if k >= SELECTION_CANDIDATE_COUNT_MAX {
                            return UNEXPECTED_ERROR;
                        }
                        memcpy(
                            &mut *selection_candidate_info_step_2
                                .as_mut_ptr()
                                .offset(k as isize)
                                as *mut ia_selection_candidate_info_struct
                                as *mut core::ffi::c_void,
                            &mut *selection_candidate_info.offset(i as isize)
                                as *mut ia_selection_candidate_info_struct
                                as *const core::ffi::c_void,
                            ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                                as size_t,
                        );
                        k += 1;
                    }
                } else if (*str_drc_instruction_str).drc_set_effect & effect_bit_idx
                    == 0 as core::ffi::c_int
                    && (*drc_instructions_dependent).drc_set_effect & effect_bit_idx
                        == 0 as core::ffi::c_int
                {
                    if k >= SELECTION_CANDIDATE_COUNT_MAX {
                        return UNEXPECTED_ERROR;
                    }
                    memcpy(
                        &mut *selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(k as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *mut core::ffi::c_void,
                        &mut *selection_candidate_info.offset(i as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *const core::ffi::c_void,
                        ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                            as size_t,
                    );
                    k += 1;
                }
            } else if state_requested == 1 as core::ffi::c_int {
                if (*str_drc_instruction_str).drc_set_effect & effect_bit_idx
                    != 0 as core::ffi::c_int
                {
                    if k >= SELECTION_CANDIDATE_COUNT_MAX {
                        return UNEXPECTED_ERROR;
                    }
                    memcpy(
                        &mut *selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(k as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *mut core::ffi::c_void,
                        &mut *selection_candidate_info.offset(i as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *const core::ffi::c_void,
                        ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                            as size_t,
                    );
                    k += 1;
                }
            } else if (*str_drc_instruction_str).drc_set_effect & effect_bit_idx
                == 0 as core::ffi::c_int
            {
                if k >= SELECTION_CANDIDATE_COUNT_MAX {
                    return UNEXPECTED_ERROR;
                }
                memcpy(
                    &mut *selection_candidate_info_step_2.as_mut_ptr().offset(k as isize)
                        as *mut ia_selection_candidate_info_struct
                        as *mut core::ffi::c_void,
                    &mut *selection_candidate_info.offset(i as isize)
                        as *mut ia_selection_candidate_info_struct
                        as *const core::ffi::c_void,
                    ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                        as size_t,
                );
                k += 1;
            }
            i += 1;
        }
        selection_candidate_step_2_count = k;
        if selection_candidate_step_2_count > 0 as core::ffi::c_int {
            *match_found_flag = 1 as core::ffi::c_int as WORD32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < selection_candidate_step_2_count {
                *selection_candidate_count = selection_candidate_step_2_count;
                memcpy(
                    &mut *selection_candidate_info.offset(i as isize)
                        as *mut ia_selection_candidate_info_struct
                        as *mut core::ffi::c_void,
                    &mut *selection_candidate_info_step_2.as_mut_ptr().offset(i as isize)
                        as *mut ia_selection_candidate_info_struct
                        as *const core::ffi::c_void,
                    ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                        as size_t,
                );
                i += 1;
            }
        } else {
            *match_found_flag = 0 as core::ffi::c_int as WORD32;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_match_effect_types(
    mut pstr_drc_config: *mut ia_drc_config,
    mut effect_type_requested_total_count: WORD32,
    mut effect_type_requested_desired_count: WORD32,
    mut requested_effect_type: *mut WORD32,
    mut selection_candidate_count: *mut WORD32,
    mut selection_candidate_info: *mut ia_selection_candidate_info_struct,
) -> WORD32 {
    let mut k: WORD32 = 0;
    let mut err: WORD32 = 0;
    let mut match_found_flag: WORD32 = 0 as WORD32;
    let mut state_requested: WORD32 = 0;
    let mut desired_effect_type_found: WORD32 = 0;
    desired_effect_type_found = 0 as core::ffi::c_int as WORD32;
    k = 0 as core::ffi::c_int as WORD32;
    while k < effect_type_requested_desired_count {
        state_requested = 1 as core::ffi::c_int as WORD32;
        err = impd_match_effect_type_attempt(
            pstr_drc_config,
            *requested_effect_type.offset(k as isize),
            state_requested,
            &mut match_found_flag,
            selection_candidate_count,
            selection_candidate_info,
        );
        if err != 0 {
            return err;
        }
        if match_found_flag != 0 {
            desired_effect_type_found = 1 as core::ffi::c_int as WORD32;
        }
        k += 1;
    }
    if desired_effect_type_found == 0 as core::ffi::c_int {
        while k < effect_type_requested_total_count
            && match_found_flag == 0 as core::ffi::c_int
        {
            state_requested = 1 as core::ffi::c_int as WORD32;
            err = impd_match_effect_type_attempt(
                pstr_drc_config,
                *requested_effect_type.offset(k as isize),
                state_requested,
                &mut match_found_flag,
                selection_candidate_count,
                selection_candidate_info,
            );
            if err != 0 {
                return err;
            }
            k += 1;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_match_dynamic_range(
    mut pstr_drc_config: *mut ia_drc_config,
    mut pstr_loudness_info: *mut ia_drc_loudness_info_set_struct,
    mut pstr_drc_sel_proc_params_struct: *mut ia_drc_sel_proc_params_struct,
    mut num_drc_requests: WORD32,
    mut selection_candidate_count: *mut WORD32,
    mut selection_candidate_info: *mut ia_selection_candidate_info_struct,
) -> WORD32 {
    let mut str_drc_instruction_str: *mut ia_drc_instructions_struct = 0
        as *mut ia_drc_instructions_struct;
    let mut err: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut lp_avg_present_val: WORD32 = 0;
    let mut lp_avg_val: FLOAT32 = 0.;
    let mut deviation_min: FLOAT32 = 1000.0f32;
    let mut selected: [WORD32; 36] = [0; 36];
    let mut dynamic_range_measurement_type: WORD32 = (*pstr_drc_sel_proc_params_struct)
        .requested_dyn_range_measur_type[num_drc_requests as usize];
    let mut requested_dyn_range_range_flag: WORD32 = (*pstr_drc_sel_proc_params_struct)
        .requested_dyn_range_range_flag[num_drc_requests as usize];
    let mut dynamic_range_requested: FLOAT32 = (*pstr_drc_sel_proc_params_struct)
        .requested_dyn_range_value[num_drc_requests as usize];
    let mut dynamic_range_min_requested: FLOAT32 = (*pstr_drc_sel_proc_params_struct)
        .requested_dyn_range_min_val[num_drc_requests as usize];
    let mut dynamic_range_max_requested: FLOAT32 = (*pstr_drc_sel_proc_params_struct)
        .requested_dyn_range_max_val[num_drc_requests as usize];
    let mut requested_dwnmix_id: *mut WORD32 = ((*pstr_drc_sel_proc_params_struct)
        .requested_dwnmix_id)
        .as_mut_ptr();
    let mut album_mode: WORD32 = (*pstr_drc_sel_proc_params_struct).album_mode;
    k = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < *selection_candidate_count {
        str_drc_instruction_str = &mut *((*pstr_drc_config).str_drc_instruction_str)
            .as_mut_ptr()
            .offset(
                (*selection_candidate_info.offset(i as isize)).drc_instructions_index
                    as isize,
            ) as *mut ia_drc_instructions_struct;
        err = impd_loudness_peak_to_average_info(
            pstr_loudness_info,
            str_drc_instruction_str,
            *requested_dwnmix_id
                .offset(
                    (*selection_candidate_info.offset(i as isize))
                        .downmix_id_request_index as isize,
                ),
            dynamic_range_measurement_type,
            album_mode,
            &mut lp_avg_present_val,
            &mut lp_avg_val,
        );
        if err != 0 {
            return err;
        }
        if lp_avg_present_val == 1 as core::ffi::c_int {
            if requested_dyn_range_range_flag == 1 as core::ffi::c_int {
                if lp_avg_val >= dynamic_range_min_requested
                    && lp_avg_val <= dynamic_range_max_requested
                {
                    if k >= DRC_INSTRUCTIONS_COUNT_MAX {
                        return UNEXPECTED_ERROR;
                    }
                    selected[k as usize] = i;
                    k += 1;
                }
            } else {
                let mut deviation: FLOAT32 = fabs(
                    (dynamic_range_requested - lp_avg_val) as core::ffi::c_double,
                ) as FLOAT32;
                if deviation_min >= deviation {
                    if deviation_min > deviation {
                        deviation_min = deviation;
                        k = 0 as core::ffi::c_int as WORD32;
                    }
                    if k >= DRC_INSTRUCTIONS_COUNT_MAX {
                        return UNEXPECTED_ERROR;
                    }
                    selected[k as usize] = i;
                    k += 1;
                }
            }
        }
        i += 1;
    }
    if k > 0 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < k {
            memcpy(
                &mut *selection_candidate_info.offset(i as isize)
                    as *mut ia_selection_candidate_info_struct as *mut core::ffi::c_void,
                &mut *selection_candidate_info
                    .offset(*selected.as_mut_ptr().offset(i as isize) as isize)
                    as *mut ia_selection_candidate_info_struct
                    as *const core::ffi::c_void,
                ::core::mem::size_of::<ia_selection_candidate_info_struct>() as size_t,
            );
            i += 1;
        }
        *selection_candidate_count = k;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_match_drc_characteristic_attempt(
    mut pstr_drc_config: *mut ia_drc_config,
    mut requested_drc_characteristic: WORD32,
    mut match_found_flag: *mut WORD32,
    mut selection_candidate_count: *mut WORD32,
    mut selection_candidate_info: *mut ia_selection_candidate_info_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut b: WORD32 = 0;
    let mut m: WORD32 = 0;
    let mut ref_count: WORD32 = 0;
    let mut drc_characteristic: WORD32 = 0;
    let mut match_count: FLOAT32 = 0.;
    let mut drc_characteristic_request_1: WORD32 = 0;
    let mut drc_characteristic_request_2: WORD32 = 0;
    let mut drc_characteristic_request_3: WORD32 = 0;
    let mut str_drc_instruction_str: *mut ia_drc_instructions_struct = 0
        as *mut ia_drc_instructions_struct;
    let mut str_p_loc_drc_coefficients_uni_drc: *mut ia_uni_drc_coeffs_struct = 0
        as *mut ia_uni_drc_coeffs_struct;
    let mut gain_set_params: *mut ia_gain_set_params_struct = 0
        as *mut ia_gain_set_params_struct;
    *match_found_flag = 0 as core::ffi::c_int as WORD32;
    if requested_drc_characteristic < 1 as core::ffi::c_int {
        return 2 as WORD32;
    }
    if requested_drc_characteristic < 12 as core::ffi::c_int {
        drc_characteristic_request_1 = (*drc_characteristic_order_default
            .as_ptr()
            .offset(
                (requested_drc_characteristic as core::ffi::c_int
                    - 1 as core::ffi::c_int) as isize,
            ))[0 as core::ffi::c_int as usize];
        drc_characteristic_request_2 = (*drc_characteristic_order_default
            .as_ptr()
            .offset(
                (requested_drc_characteristic as core::ffi::c_int
                    - 1 as core::ffi::c_int) as isize,
            ))[1 as core::ffi::c_int as usize];
        drc_characteristic_request_3 = (*drc_characteristic_order_default
            .as_ptr()
            .offset(
                (requested_drc_characteristic as core::ffi::c_int
                    - 1 as core::ffi::c_int) as isize,
            ))[2 as core::ffi::c_int as usize];
    } else {
        drc_characteristic_request_1 = requested_drc_characteristic;
        drc_characteristic_request_2 = -(1 as core::ffi::c_int) as WORD32;
        drc_characteristic_request_3 = -(1 as core::ffi::c_int) as WORD32;
    }
    if (*pstr_drc_config).drc_coefficients_drc_count != 0 {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*pstr_drc_config).drc_coefficients_drc_count {
            str_p_loc_drc_coefficients_uni_drc = &mut *((*pstr_drc_config)
                .str_p_loc_drc_coefficients_uni_drc)
                .as_mut_ptr()
                .offset(i as isize) as *mut ia_uni_drc_coeffs_struct;
            if (*str_p_loc_drc_coefficients_uni_drc).drc_location == LOCATION_SELECTED {
                break;
            }
            i += 1;
        }
        if i == (*pstr_drc_config).drc_coefficients_drc_count {
            return 2 as WORD32;
        }
    }
    n = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < *selection_candidate_count {
        ref_count = 0 as core::ffi::c_int as WORD32;
        match_count = 0 as core::ffi::c_int as FLOAT32;
        str_drc_instruction_str = &mut *((*pstr_drc_config).str_drc_instruction_str)
            .as_mut_ptr()
            .offset(
                (*selection_candidate_info.offset(i as isize)).drc_instructions_index
                    as isize,
            ) as *mut ia_drc_instructions_struct;
        k = 0 as core::ffi::c_int as WORD32;
        while k < (*str_drc_instruction_str).num_drc_ch_groups {
            gain_set_params = &mut *((*str_p_loc_drc_coefficients_uni_drc)
                .gain_set_params)
                .as_mut_ptr()
                .offset(
                    *((*str_drc_instruction_str).gain_set_index_for_channel_group)
                        .as_mut_ptr()
                        .offset(k as isize) as isize,
                ) as *mut ia_gain_set_params_struct;
            b = 0 as core::ffi::c_int as WORD32;
            while b < (*gain_set_params).band_count {
                ref_count += 1;
                drc_characteristic = (*gain_set_params)
                    .gain_params[b as usize]
                    .drc_characteristic;
                if drc_characteristic == drc_characteristic_request_1 {
                    match_count += 1.0f32;
                } else if drc_characteristic == drc_characteristic_request_2 {
                    match_count += 0.75f32;
                } else if drc_characteristic == drc_characteristic_request_3 {
                    match_count += 0.5f32;
                }
                b += 1;
            }
            k += 1;
        }
        if (*str_drc_instruction_str).depends_on_drc_set_present == 1 as core::ffi::c_int
        {
            let mut depends_on_drc_set: WORD32 = (*str_drc_instruction_str)
                .depends_on_drc_set;
            m = 0 as core::ffi::c_int as WORD32;
            while m < (*pstr_drc_config).drc_instructions_uni_drc_count {
                if (*pstr_drc_config).str_drc_instruction_str[m as usize].drc_set_id
                    == depends_on_drc_set
                {
                    break;
                }
                m += 1;
            }
            if m == (*pstr_drc_config).drc_instructions_uni_drc_count {
                return 2 as WORD32;
            }
            str_drc_instruction_str = &mut *((*pstr_drc_config).str_drc_instruction_str)
                .as_mut_ptr()
                .offset(m as isize) as *mut ia_drc_instructions_struct;
            if (*str_drc_instruction_str).drc_set_effect as core::ffi::c_int
                & (EFFECT_BIT_FADE | EFFECT_BIT_DUCK_OTHER | EFFECT_BIT_DUCK_SELF)
                == 0 as core::ffi::c_int
            {
                if (*str_drc_instruction_str).drc_set_effect != EFFECT_BIT_CLIPPING {
                    k = 0 as core::ffi::c_int as WORD32;
                    while k < (*str_drc_instruction_str).num_drc_ch_groups {
                        gain_set_params = &mut *((*str_p_loc_drc_coefficients_uni_drc)
                            .gain_set_params)
                            .as_mut_ptr()
                            .offset(
                                *((*str_drc_instruction_str)
                                    .gain_set_index_for_channel_group)
                                    .as_mut_ptr()
                                    .offset(k as isize) as isize,
                            ) as *mut ia_gain_set_params_struct;
                        b = 0 as core::ffi::c_int as WORD32;
                        while b < (*gain_set_params).band_count {
                            ref_count += 1;
                            drc_characteristic = (*gain_set_params)
                                .gain_params[b as usize]
                                .drc_characteristic;
                            if drc_characteristic == drc_characteristic_request_1 {
                                match_count += 1.0f32;
                            } else if drc_characteristic == drc_characteristic_request_2
                            {
                                match_count += 0.75f32;
                            } else if drc_characteristic == drc_characteristic_request_3
                            {
                                match_count = (match_count as core::ffi::c_double + 0.5f64)
                                    as FLOAT32;
                            }
                            b += 1;
                        }
                        k += 1;
                    }
                }
            }
        }
        if ref_count > 0 as core::ffi::c_int
            && match_count > 0.5f32 * ref_count as core::ffi::c_float
        {
            if n >= SELECTION_CANDIDATE_COUNT_MAX {
                return UNEXPECTED_ERROR;
            }
            memcpy(
                &mut *selection_candidate_info.offset(n as isize)
                    as *mut ia_selection_candidate_info_struct as *mut core::ffi::c_void,
                &mut *selection_candidate_info.offset(i as isize)
                    as *mut ia_selection_candidate_info_struct
                    as *const core::ffi::c_void,
                ::core::mem::size_of::<ia_selection_candidate_info_struct>() as size_t,
            );
            n += 1;
        }
        i += 1;
    }
    if n > 0 as core::ffi::c_int {
        *selection_candidate_count = n;
        *match_found_flag = 1 as core::ffi::c_int as WORD32;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_match_drc_characteristic(
    mut pstr_drc_config: *mut ia_drc_config,
    mut requested_drc_characteristic: WORD32,
    mut selection_candidate_count: *mut WORD32,
    mut selection_candidate_info: *mut ia_selection_candidate_info_struct,
) -> WORD32 {
    let mut k: WORD32 = 0;
    let mut err: WORD32 = 0;
    let mut match_found_flag: WORD32 = 0 as WORD32;
    let mut drc_characteristic_order: *const WORD32 = (*drc_characteristic_order_default
        .as_ptr()
        .offset(
            (requested_drc_characteristic as core::ffi::c_int - 1 as core::ffi::c_int)
                as isize,
        ))
        .as_ptr();
    let drc_characteristic_order_count: WORD32 = (::core::mem::size_of::<[WORD32; 3]>()
        as usize)
        .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as WORD32;
    k = 0 as core::ffi::c_int as WORD32;
    while k < drc_characteristic_order_count && match_found_flag == 0 as core::ffi::c_int
        && *drc_characteristic_order.offset(k as isize) > 0 as core::ffi::c_int
    {
        err = impd_match_drc_characteristic_attempt(
            pstr_drc_config,
            *drc_characteristic_order.offset(k as isize),
            &mut match_found_flag,
            selection_candidate_count,
            selection_candidate_info,
        );
        if err != 0 {
            return err;
        }
        k += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_drc_set_preselection(
    mut pstr_drc_sel_proc_params_struct: *mut ia_drc_sel_proc_params_struct,
    mut pstr_drc_config: *mut ia_drc_config,
    mut pstr_loudness_info: *mut ia_drc_loudness_info_set_struct,
    mut restrict_to_drc_with_album_loudness: WORD32,
    mut pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
    mut selection_candidate_count: *mut WORD32,
    mut selection_candidate_info: *mut ia_selection_candidate_info_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut l: WORD32 = 0;
    let mut d: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut err: WORD32 = 0;
    let mut downmix_id_match: WORD32 = 0 as WORD32;
    let mut selection_candidate_step_2_count: WORD32 = 0;
    let mut selection_candidate_info_step_2: [ia_selection_candidate_info_struct; 32] = [ia_selection_candidate_info_struct {
        drc_instructions_index: 0,
        downmix_id_request_index: 0,
        eq_set_id: 0,
        output_peak_level: 0.,
        loudness_norm_db_gain_adjusted: 0.,
        output_loudness: 0.,
        mixing_level: 0.,
        selection_flags: 0,
    }; 32];
    let mut num_downmix_id_requests: WORD32 = (*pstr_drc_sel_proc_params_struct)
        .num_downmix_id_requests;
    let mut requested_dwnmix_id: *mut WORD32 = ((*pstr_drc_sel_proc_params_struct)
        .requested_dwnmix_id)
        .as_mut_ptr();
    let mut output_peak_level_max: FLOAT32 = (*pstr_drc_sel_proc_params_struct)
        .output_peak_level_max;
    let mut loudness_deviation_max: WORD32 = (*pstr_drc_sel_proc_params_struct)
        .loudness_deviation_max;
    let mut drc_set_id_valid_flag: *mut WORD32 = ((*pstr_drc_uni_sel_proc)
        .drc_set_id_valid_flag)
        .as_mut_ptr();
    let mut eq_set_id_valid_flag: *mut WORD32 = ((*pstr_drc_uni_sel_proc)
        .eq_set_id_valid_flag)
        .as_mut_ptr();
    let mut output_peak_level_min: FLOAT32 = 1000.0f32;
    let mut adjustment: FLOAT32 = 0.;
    let mut loudness_drc_set_id_requested: WORD32 = 0;
    let mut num_compression_eq_count: WORD32 = 0 as WORD32;
    let mut num_compression_eq_id: [WORD32; 16] = [0; 16];
    let mut loudness_info_count: WORD32 = 0 as WORD32;
    let mut eq_set_id_loudness: [WORD32; 16] = [0; 16];
    let mut loudness_normalization_gain_db: [FLOAT32; 16] = [0.; 16];
    let mut loudness: [FLOAT32; 16] = [0.; 16];
    let mut peak_info_count: WORD32 = 0;
    let mut eq_set_id_Peak: [WORD32; 16] = [0; 16];
    let mut signal_peak_level: [FLOAT32; 16] = [0.; 16];
    let mut explicit_peak_information_present: [WORD32; 16] = [0
        as core::ffi::c_int; 16];
    let mut str_p_loc_drc_coefficients_uni_drc: *mut ia_uni_drc_coeffs_struct = 0
        as *mut ia_uni_drc_coeffs_struct;
    let mut str_drc_instruction_str: *mut ia_drc_instructions_struct = 0
        as *mut ia_drc_instructions_struct;
    impd_select_drc_coeff3(pstr_drc_config, &mut str_p_loc_drc_coefficients_uni_drc);
    if str_p_loc_drc_coefficients_uni_drc.is_null() {
        return UNEXPECTED_ERROR;
    }
    k = 0 as core::ffi::c_int as WORD32;
    d = 0 as core::ffi::c_int as WORD32;
    while d < num_downmix_id_requests {
        err = impd_find_eq_set_no_compression(
            pstr_drc_config,
            *requested_dwnmix_id.offset(d as isize),
            &mut num_compression_eq_count,
            num_compression_eq_id.as_mut_ptr(),
        );
        if err != 0 {
            return err;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*pstr_drc_config).drc_instructions_count_plus {
            downmix_id_match = 0 as core::ffi::c_int as WORD32;
            str_drc_instruction_str = &mut *((*pstr_drc_config).str_drc_instruction_str)
                .as_mut_ptr()
                .offset(i as isize) as *mut ia_drc_instructions_struct;
            j = 0 as core::ffi::c_int as WORD32;
            while j < (*str_drc_instruction_str).dwnmix_id_count {
                if (*str_drc_instruction_str).downmix_id[j as usize]
                    == *requested_dwnmix_id.offset(d as isize)
                    || (*str_drc_instruction_str).downmix_id[j as usize]
                        == ID_FOR_BASE_LAYOUT
                        && (*str_drc_instruction_str).drc_set_id > 0 as core::ffi::c_int
                    || (*str_drc_instruction_str).downmix_id[j as usize]
                        == ID_FOR_ANY_DOWNMIX
                {
                    downmix_id_match = 1 as core::ffi::c_int as WORD32;
                }
                j += 1;
            }
            if downmix_id_match == 1 as core::ffi::c_int {
                if (*pstr_drc_sel_proc_params_struct).dynamic_range_control_on
                    == 1 as core::ffi::c_int
                {
                    if (*str_drc_instruction_str).drc_set_effect != EFFECT_BIT_FADE
                        && (*str_drc_instruction_str).drc_set_effect
                            != EFFECT_BIT_DUCK_OTHER
                        && (*str_drc_instruction_str).drc_set_effect
                            != EFFECT_BIT_DUCK_SELF
                        && ((*str_drc_instruction_str).drc_set_effect
                            != 0 as core::ffi::c_int
                            || (*str_drc_instruction_str).drc_set_id
                                < 0 as core::ffi::c_int)
                        && ((*str_drc_instruction_str).depends_on_drc_set_present
                            == 0 as core::ffi::c_int
                            && (*str_drc_instruction_str).no_independent_use
                                == 0 as core::ffi::c_int
                            || (*str_drc_instruction_str).depends_on_drc_set_present
                                == 1 as core::ffi::c_int)
                    {
                        let mut drc_is_permitted: WORD32 = 1 as WORD32;
                        if (*str_drc_instruction_str).drc_set_id > 0 as core::ffi::c_int
                        {
                            drc_is_permitted = *drc_set_id_valid_flag
                                .offset((*str_drc_instruction_str).drc_set_id as isize);
                        }
                        if drc_is_permitted == 1 as core::ffi::c_int {
                            err = impd_init_loudness_control(
                                pstr_drc_sel_proc_params_struct,
                                pstr_loudness_info,
                                *requested_dwnmix_id.offset(d as isize),
                                (*str_drc_instruction_str).drc_set_id,
                                num_compression_eq_count,
                                num_compression_eq_id.as_mut_ptr(),
                                &mut loudness_info_count,
                                eq_set_id_loudness.as_mut_ptr(),
                                loudness_normalization_gain_db.as_mut_ptr(),
                                loudness.as_mut_ptr(),
                            );
                            if err != 0 {
                                return err;
                            }
                            if loudness_info_count > MAX_LOUDNESS_INFO_COUNT {
                                return UNEXPECTED_ERROR;
                            }
                            impd_signal_peak_level_info(
                                pstr_drc_config,
                                pstr_loudness_info,
                                str_drc_instruction_str,
                                *requested_dwnmix_id.offset(d as isize),
                                (*pstr_drc_sel_proc_params_struct).album_mode,
                                num_compression_eq_count,
                                num_compression_eq_id.as_mut_ptr(),
                                &mut peak_info_count,
                                eq_set_id_Peak.as_mut_ptr(),
                                signal_peak_level.as_mut_ptr(),
                                explicit_peak_information_present.as_mut_ptr(),
                            );
                            l = 0 as core::ffi::c_int as WORD32;
                            while l < loudness_info_count {
                                let mut match_found_flag: WORD32 = 0 as WORD32;
                                let mut p: WORD32 = 0;
                                if k >= SELECTION_CANDIDATE_COUNT_MAX {
                                    return UNEXPECTED_ERROR;
                                }
                                (*selection_candidate_info.offset(k as isize))
                                    .loudness_norm_db_gain_adjusted = loudness_normalization_gain_db[l
                                    as usize];
                                (*selection_candidate_info.offset(k as isize))
                                    .loudness_norm_db_gain_adjusted = if (*selection_candidate_info
                                    .offset(k as isize))
                                    .loudness_norm_db_gain_adjusted
                                    < (*pstr_drc_sel_proc_params_struct)
                                        .loudness_norm_gain_db_max
                                {
                                    (*selection_candidate_info.offset(k as isize))
                                        .loudness_norm_db_gain_adjusted
                                } else {
                                    (*pstr_drc_sel_proc_params_struct).loudness_norm_gain_db_max
                                };
                                if loudness[l as usize] != UNDEFINED_LOUDNESS_VALUE {
                                    (*selection_candidate_info.offset(k as isize))
                                        .output_loudness = loudness[l as usize]
                                        + (*selection_candidate_info.offset(k as isize))
                                            .loudness_norm_db_gain_adjusted;
                                } else {
                                    (*selection_candidate_info.offset(k as isize))
                                        .output_loudness = UNDEFINED_LOUDNESS_VALUE as FLOAT32;
                                }
                                p = 0 as core::ffi::c_int as WORD32;
                                while p < peak_info_count {
                                    if eq_set_id_Peak[p as usize]
                                        == eq_set_id_loudness[l as usize]
                                    {
                                        if *eq_set_id_valid_flag
                                            .offset(eq_set_id_Peak[p as usize] as isize)
                                            == 1 as core::ffi::c_int
                                        {
                                            match_found_flag = 1 as core::ffi::c_int as WORD32;
                                            break;
                                        }
                                    }
                                    p += 1;
                                }
                                if match_found_flag == 1 as core::ffi::c_int {
                                    (*selection_candidate_info.offset(k as isize))
                                        .output_peak_level = signal_peak_level[p as usize]
                                        + (*selection_candidate_info.offset(k as isize))
                                            .loudness_norm_db_gain_adjusted;
                                } else {
                                    (*selection_candidate_info.offset(k as isize))
                                        .output_peak_level = (*selection_candidate_info
                                        .offset(k as isize))
                                        .loudness_norm_db_gain_adjusted;
                                }
                                if !((*str_drc_instruction_str).requires_eq
                                    == 1 as core::ffi::c_int
                                    && *eq_set_id_valid_flag
                                        .offset(eq_set_id_loudness[l as usize] as isize)
                                        == 0 as core::ffi::c_int)
                                {
                                    (*selection_candidate_info.offset(k as isize))
                                        .drc_instructions_index = i;
                                    (*selection_candidate_info.offset(k as isize))
                                        .downmix_id_request_index = d;
                                    (*selection_candidate_info.offset(k as isize)).eq_set_id = eq_set_id_loudness[l
                                        as usize];
                                    if explicit_peak_information_present[p as usize]
                                        == 1 as core::ffi::c_int
                                    {
                                        (*selection_candidate_info.offset(k as isize))
                                            .selection_flags = SELECTION_FLAG_EXPLICIT_PEAK_INFO_PRESENT
                                            as WORD32;
                                    } else {
                                        (*selection_candidate_info.offset(k as isize))
                                            .selection_flags = 0 as core::ffi::c_int as WORD32;
                                    }
                                    impd_mixing_level_info(
                                        pstr_drc_sel_proc_params_struct,
                                        pstr_loudness_info,
                                        *requested_dwnmix_id.offset(d as isize),
                                        (*str_drc_instruction_str).drc_set_id,
                                        eq_set_id_loudness[l as usize],
                                        &mut (*selection_candidate_info.offset(k as isize))
                                            .mixing_level,
                                    );
                                    if (*str_drc_instruction_str)
                                        .drc_set_target_loudness_present != 0
                                        && ((*pstr_drc_sel_proc_params_struct)
                                            .loudness_normalization_on != 0
                                            && (*str_drc_instruction_str)
                                                .drc_set_target_loudness_value_upper as FLOAT32
                                                >= (*pstr_drc_sel_proc_params_struct).target_loudness
                                            && ((*str_drc_instruction_str)
                                                .drc_set_target_loudness_value_lower as FLOAT32)
                                                < (*pstr_drc_sel_proc_params_struct).target_loudness
                                            || (*pstr_drc_sel_proc_params_struct)
                                                .loudness_normalization_on == 0)
                                    {
                                        let ref mut fresh0 = (*selection_candidate_info
                                            .offset(k as isize))
                                            .selection_flags;
                                        *fresh0 |= SELECTION_FLAG_DRC_TARGET_LOUDNESS_MATCH;
                                        if explicit_peak_information_present[p as usize] == 0 {
                                            if (*pstr_drc_sel_proc_params_struct)
                                                .loudness_normalization_on != 0
                                            {
                                                (*selection_candidate_info.offset(k as isize))
                                                    .output_peak_level = (*pstr_drc_sel_proc_params_struct)
                                                    .target_loudness
                                                    - (*str_drc_instruction_str)
                                                        .drc_set_target_loudness_value_upper as FLOAT32;
                                            } else {
                                                (*selection_candidate_info.offset(k as isize))
                                                    .output_peak_level = 0.0f32 as FLOAT32;
                                            }
                                        }
                                    }
                                    if (*selection_candidate_info.offset(k as isize))
                                        .selection_flags as core::ffi::c_int
                                        & (SELECTION_FLAG_DRC_TARGET_LOUDNESS_MATCH
                                            | SELECTION_FLAG_EXPLICIT_PEAK_INFO_PRESENT) != 0
                                        || (*str_drc_instruction_str)
                                            .drc_set_target_loudness_present == 0
                                    {
                                        k += 1;
                                    }
                                }
                                l += 1;
                            }
                        }
                    }
                } else if (*str_drc_instruction_str).drc_set_id < 0 as core::ffi::c_int {
                    err = impd_init_loudness_control(
                        pstr_drc_sel_proc_params_struct,
                        pstr_loudness_info,
                        *requested_dwnmix_id.offset(d as isize),
                        (*str_drc_instruction_str).drc_set_id,
                        num_compression_eq_count,
                        num_compression_eq_id.as_mut_ptr(),
                        &mut loudness_info_count,
                        eq_set_id_loudness.as_mut_ptr(),
                        loudness_normalization_gain_db.as_mut_ptr(),
                        loudness.as_mut_ptr(),
                    );
                    if err != 0 {
                        return err;
                    }
                    impd_signal_peak_level_info(
                        pstr_drc_config,
                        pstr_loudness_info,
                        str_drc_instruction_str,
                        *requested_dwnmix_id.offset(d as isize),
                        (*pstr_drc_sel_proc_params_struct).album_mode,
                        num_compression_eq_count,
                        num_compression_eq_id.as_mut_ptr(),
                        &mut peak_info_count,
                        eq_set_id_Peak.as_mut_ptr(),
                        signal_peak_level.as_mut_ptr(),
                        explicit_peak_information_present.as_mut_ptr(),
                    );
                    l = 0 as core::ffi::c_int as WORD32;
                    while l < loudness_info_count {
                        let mut match_found_flag_0: WORD32 = 0 as WORD32;
                        let mut p_0: WORD32 = 0;
                        if k >= SELECTION_CANDIDATE_COUNT_MAX {
                            return UNEXPECTED_ERROR;
                        }
                        p_0 = 0 as core::ffi::c_int as WORD32;
                        while p_0 < peak_info_count {
                            if eq_set_id_Peak[p_0 as usize]
                                == eq_set_id_loudness[l as usize]
                            {
                                if *eq_set_id_valid_flag
                                    .offset(eq_set_id_Peak[p_0 as usize] as isize)
                                    == 1 as core::ffi::c_int
                                {
                                    match_found_flag_0 = 1 as core::ffi::c_int as WORD32;
                                    break;
                                }
                            }
                            p_0 += 1;
                        }
                        if match_found_flag_0 == 1 as core::ffi::c_int {
                            adjustment = (if 0.0f32
                                > signal_peak_level[p_0 as usize]
                                    + loudness_normalization_gain_db[l as usize]
                                    - (*pstr_drc_sel_proc_params_struct).output_peak_level_max
                            {
                                0.0f32
                            } else {
                                signal_peak_level[p_0 as usize]
                                    + loudness_normalization_gain_db[l as usize]
                                    - (*pstr_drc_sel_proc_params_struct).output_peak_level_max
                                        as core::ffi::c_float
                            }) as FLOAT32;
                            adjustment = (if adjustment
                                < (if 0.0f32 > loudness_deviation_max as core::ffi::c_float
                                {
                                    0.0f32
                                } else {
                                    loudness_deviation_max as core::ffi::c_float
                                })
                            {
                                adjustment as core::ffi::c_float
                            } else if 0.0f32
                                > loudness_deviation_max as core::ffi::c_float
                            {
                                0.0f32
                            } else {
                                loudness_deviation_max as core::ffi::c_float
                            }) as FLOAT32;
                            (*selection_candidate_info.offset(k as isize))
                                .loudness_norm_db_gain_adjusted = loudness_normalization_gain_db[l
                                as usize] - adjustment;
                            (*selection_candidate_info.offset(k as isize))
                                .loudness_norm_db_gain_adjusted = if (*selection_candidate_info
                                .offset(k as isize))
                                .loudness_norm_db_gain_adjusted
                                < (*pstr_drc_sel_proc_params_struct)
                                    .loudness_norm_gain_db_max
                            {
                                (*selection_candidate_info.offset(k as isize))
                                    .loudness_norm_db_gain_adjusted
                            } else {
                                (*pstr_drc_sel_proc_params_struct).loudness_norm_gain_db_max
                            };
                            (*selection_candidate_info.offset(k as isize))
                                .output_peak_level = signal_peak_level[p_0 as usize]
                                + (*selection_candidate_info.offset(k as isize))
                                    .loudness_norm_db_gain_adjusted;
                            if loudness[l as usize] != UNDEFINED_LOUDNESS_VALUE {
                                (*selection_candidate_info.offset(k as isize))
                                    .output_loudness = loudness[l as usize]
                                    + (*selection_candidate_info.offset(k as isize))
                                        .loudness_norm_db_gain_adjusted;
                            } else {
                                (*selection_candidate_info.offset(k as isize))
                                    .output_loudness = UNDEFINED_LOUDNESS_VALUE as FLOAT32;
                            }
                            (*selection_candidate_info.offset(k as isize))
                                .drc_instructions_index = i;
                            (*selection_candidate_info.offset(k as isize))
                                .downmix_id_request_index = d;
                            (*selection_candidate_info.offset(k as isize)).eq_set_id = eq_set_id_loudness[l
                                as usize];
                            if explicit_peak_information_present[p_0 as usize]
                                == 1 as core::ffi::c_int
                            {
                                (*selection_candidate_info.offset(k as isize))
                                    .selection_flags = SELECTION_FLAG_EXPLICIT_PEAK_INFO_PRESENT
                                    as WORD32;
                            } else {
                                (*selection_candidate_info.offset(k as isize))
                                    .selection_flags = 0 as core::ffi::c_int as WORD32;
                            }
                            impd_mixing_level_info(
                                pstr_drc_sel_proc_params_struct,
                                pstr_loudness_info,
                                *requested_dwnmix_id.offset(d as isize),
                                (*str_drc_instruction_str).drc_set_id,
                                eq_set_id_loudness[l as usize],
                                &mut (*selection_candidate_info.offset(k as isize))
                                    .mixing_level,
                            );
                            k += 1;
                        }
                        l += 1;
                    }
                }
            }
            i += 1;
        }
        d += 1;
    }
    if k > SELECTION_CANDIDATE_COUNT_MAX {
        return UNEXPECTED_ERROR;
    }
    *selection_candidate_count = k;
    if (*pstr_drc_sel_proc_params_struct).dynamic_range_control_on
        == 1 as core::ffi::c_int
    {
        n = 0 as core::ffi::c_int as WORD32;
        k = 0 as core::ffi::c_int as WORD32;
        while k < *selection_candidate_count {
            str_drc_instruction_str = &mut *((*pstr_drc_config).str_drc_instruction_str)
                .as_mut_ptr()
                .offset(
                    (*selection_candidate_info.offset(k as isize)).drc_instructions_index
                        as isize,
                ) as *mut ia_drc_instructions_struct;
            if (*pstr_drc_sel_proc_params_struct).eq_set_purpose_request
                != EQ_PURPOSE_EQ_OFF
            {
                let mut matching_eq_set_count: WORD32 = 0 as WORD32;
                let mut matching_eq_instrucions_index: [WORD32; 64] = [0; 64];
                err = impd_match_eq_set(
                    pstr_drc_config,
                    *requested_dwnmix_id
                        .offset(
                            (*selection_candidate_info.offset(k as isize))
                                .downmix_id_request_index as isize,
                        ),
                    (*str_drc_instruction_str).drc_set_id,
                    eq_set_id_valid_flag,
                    &mut matching_eq_set_count,
                    matching_eq_instrucions_index.as_mut_ptr(),
                );
                if err != 0 {
                    return err;
                }
                j = 0 as core::ffi::c_int as WORD32;
                while j < matching_eq_set_count {
                    if n >= SELECTION_CANDIDATE_COUNT_MAX {
                        return UNEXPECTED_ERROR;
                    }
                    memcpy(
                        &mut *selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(n as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *mut core::ffi::c_void,
                        &mut *selection_candidate_info.offset(k as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *const core::ffi::c_void,
                        ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                            as size_t,
                    );
                    selection_candidate_info_step_2[n as usize].eq_set_id = (*pstr_drc_config)
                        .str_drc_config_ext
                        .str_eq_instructions[matching_eq_instrucions_index[j as usize]
                            as usize]
                        .eq_set_id;
                    n += 1;
                    j += 1;
                }
            }
            if (*str_drc_instruction_str).requires_eq == 0 as core::ffi::c_int {
                if n >= SELECTION_CANDIDATE_COUNT_MAX {
                    return UNEXPECTED_ERROR;
                }
                memcpy(
                    &mut *selection_candidate_info_step_2.as_mut_ptr().offset(n as isize)
                        as *mut ia_selection_candidate_info_struct
                        as *mut core::ffi::c_void,
                    &mut *selection_candidate_info.offset(k as isize)
                        as *mut ia_selection_candidate_info_struct
                        as *const core::ffi::c_void,
                    ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                        as size_t,
                );
                selection_candidate_info_step_2[n as usize].eq_set_id = 0
                    as core::ffi::c_int as WORD32;
                n += 1;
            }
            k += 1;
        }
        if n > SELECTION_CANDIDATE_COUNT_MAX {
            return UNEXPECTED_ERROR;
        }
        memcpy(
            selection_candidate_info as *mut core::ffi::c_void,
            selection_candidate_info_step_2.as_mut_ptr() as *const core::ffi::c_void,
            (n as size_t)
                .wrapping_mul(
                    ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                        as size_t,
                ),
        );
        *selection_candidate_count = n;
        n = 0 as core::ffi::c_int as WORD32;
        k = 0 as core::ffi::c_int as WORD32;
        while k < *selection_candidate_count {
            if (*selection_candidate_info.offset(k as isize)).selection_flags
                as core::ffi::c_int & SELECTION_FLAG_DRC_TARGET_LOUDNESS_MATCH != 0
                && (*selection_candidate_info.offset(k as isize)).selection_flags
                    as core::ffi::c_int & SELECTION_FLAG_EXPLICIT_PEAK_INFO_PRESENT == 0
            {
                memcpy(
                    &mut *selection_candidate_info_step_2.as_mut_ptr().offset(n as isize)
                        as *mut ia_selection_candidate_info_struct
                        as *mut core::ffi::c_void,
                    &mut *selection_candidate_info.offset(k as isize)
                        as *mut ia_selection_candidate_info_struct
                        as *const core::ffi::c_void,
                    ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                        as size_t,
                );
                n += 1;
            } else {
                if (*selection_candidate_info.offset(k as isize)).output_peak_level
                    <= output_peak_level_max
                {
                    memcpy(
                        &mut *selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(n as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *mut core::ffi::c_void,
                        &mut *selection_candidate_info.offset(k as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *const core::ffi::c_void,
                        ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                            as size_t,
                    );
                    n += 1;
                }
                if (*selection_candidate_info.offset(k as isize)).output_peak_level
                    < output_peak_level_min
                {
                    output_peak_level_min = (*selection_candidate_info
                        .offset(k as isize))
                        .output_peak_level;
                }
            }
            k += 1;
        }
        selection_candidate_step_2_count = n;
        if selection_candidate_step_2_count == 0 as core::ffi::c_int {
            n = 0 as core::ffi::c_int as WORD32;
            k = 0 as core::ffi::c_int as WORD32;
            while k < *selection_candidate_count {
                if (*selection_candidate_info.offset(k as isize)).selection_flags
                    as core::ffi::c_int & SELECTION_FLAG_DRC_TARGET_LOUDNESS_MATCH != 0
                    && (*selection_candidate_info.offset(k as isize)).selection_flags
                        as core::ffi::c_int & SELECTION_FLAG_EXPLICIT_PEAK_INFO_PRESENT
                        != 0
                {
                    memcpy(
                        &mut *selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(n as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *mut core::ffi::c_void,
                        &mut *selection_candidate_info.offset(k as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *const core::ffi::c_void,
                        ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                            as size_t,
                    );
                    n += 1;
                }
                k += 1;
            }
            selection_candidate_step_2_count = n;
        }
        if selection_candidate_step_2_count == 0 as core::ffi::c_int {
            n = 0 as core::ffi::c_int as WORD32;
            k = 0 as core::ffi::c_int as WORD32;
            while k < *selection_candidate_count {
                if selection_candidate_info_step_2[k as usize].output_peak_level
                    < output_peak_level_min as core::ffi::c_float + 1.0f32
                {
                    memcpy(
                        &mut *selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(n as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *mut core::ffi::c_void,
                        &mut *selection_candidate_info.offset(k as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *const core::ffi::c_void,
                        ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                            as size_t,
                    );
                    adjustment = (if 0.0f32
                        > selection_candidate_info_step_2[n as usize].output_peak_level
                            - output_peak_level_max
                    {
                        0.0f32
                    } else {
                        selection_candidate_info_step_2[n as usize].output_peak_level
                            as core::ffi::c_float
                            - output_peak_level_max as core::ffi::c_float
                    }) as FLOAT32;
                    adjustment = (if adjustment
                        < (if 0.0f32 > loudness_deviation_max as core::ffi::c_float {
                            0.0f32
                        } else {
                            loudness_deviation_max as core::ffi::c_float
                        })
                    {
                        adjustment as core::ffi::c_float
                    } else if 0.0f32 > loudness_deviation_max as core::ffi::c_float {
                        0.0f32
                    } else {
                        loudness_deviation_max as core::ffi::c_float
                    }) as FLOAT32;
                    selection_candidate_info_step_2[n as usize]
                        .loudness_norm_db_gain_adjusted -= adjustment;
                    selection_candidate_info_step_2[n as usize].output_peak_level
                        -= adjustment;
                    selection_candidate_info_step_2[n as usize].output_loudness
                        -= adjustment;
                    n += 1;
                }
                k += 1;
            }
            selection_candidate_step_2_count = n;
        }
        n = 0 as core::ffi::c_int as WORD32;
        while n < selection_candidate_step_2_count {
            memcpy(
                &mut *selection_candidate_info.offset(n as isize)
                    as *mut ia_selection_candidate_info_struct as *mut core::ffi::c_void,
                &mut *selection_candidate_info_step_2.as_mut_ptr().offset(n as isize)
                    as *mut ia_selection_candidate_info_struct
                    as *const core::ffi::c_void,
                ::core::mem::size_of::<ia_selection_candidate_info_struct>() as size_t,
            );
            n += 1;
        }
        *selection_candidate_count = selection_candidate_step_2_count;
    }
    if restrict_to_drc_with_album_loudness == 1 as core::ffi::c_int {
        j = 0 as core::ffi::c_int as WORD32;
        k = 0 as core::ffi::c_int as WORD32;
        while k < *selection_candidate_count {
            loudness_drc_set_id_requested = (if 0 as core::ffi::c_int
                > (*pstr_drc_config)
                    .str_drc_instruction_str[(*selection_candidate_info
                        .offset(k as isize))
                        .drc_instructions_index as usize]
                    .drc_set_id
            {
                0 as core::ffi::c_int
            } else {
                (*pstr_drc_config)
                    .str_drc_instruction_str[(*selection_candidate_info
                        .offset(k as isize))
                        .drc_instructions_index as usize]
                    .drc_set_id as core::ffi::c_int
            }) as WORD32;
            n = 0 as core::ffi::c_int as WORD32;
            while n < (*pstr_loudness_info).loudness_info_album_count {
                if loudness_drc_set_id_requested
                    == (*pstr_loudness_info)
                        .str_loudness_info_album[n as usize]
                        .drc_set_id
                {
                    if j >= SELECTION_CANDIDATE_COUNT_MAX {
                        return UNEXPECTED_ERROR;
                    }
                    memcpy(
                        &mut *selection_candidate_info.offset(j as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *mut core::ffi::c_void,
                        &mut *selection_candidate_info.offset(k as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *const core::ffi::c_void,
                        ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                            as size_t,
                    );
                    j += 1;
                    break;
                } else {
                    n += 1;
                }
            }
            k += 1;
        }
        *selection_candidate_count = j;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_drc_set_final_selection(
    mut pstr_drc_config: *mut ia_drc_config,
    mut pstr_drc_sel_proc_params_struct: *mut ia_drc_sel_proc_params_struct,
    mut selection_candidate_count: *mut WORD32,
    mut selection_candidate_info: *mut ia_selection_candidate_info_struct,
    mut eq_set_id_valid_flag: *mut WORD32,
) -> WORD32 {
    let mut k: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut err: WORD32 = 0;
    let mut selection_candidate_step_2_count: WORD32 = 0;
    let mut selection_candidate_info_step_2: [ia_selection_candidate_info_struct; 32] = [ia_selection_candidate_info_struct {
        drc_instructions_index: 0,
        downmix_id_request_index: 0,
        eq_set_id: 0,
        output_peak_level: 0.,
        loudness_norm_db_gain_adjusted: 0.,
        output_loudness: 0.,
        mixing_level: 0.,
        selection_flags: 0,
    }; 32];
    let mut drc_set_id_max: WORD32 = 0;
    let mut output_level_max: FLOAT32 = 0.;
    let mut output_level_min: FLOAT32 = 0.;
    let mut effect_count: WORD32 = 0;
    let mut effect_count_min: WORD32 = 0;
    let mut effect_types_request_table_size: WORD32 = 0;
    let mut drc_set_target_loudness_val_upper_min: WORD32 = 0;
    let mut str_drc_instruction_str: *mut ia_drc_instructions_struct = 0
        as *mut ia_drc_instructions_struct;
    let mut drc_instructions_dependent: *mut ia_drc_instructions_struct = 0
        as *mut ia_drc_instructions_struct;
    if (*pstr_drc_sel_proc_params_struct).eq_set_purpose_request > 0 as core::ffi::c_int
    {
        let mut eq_purpose_requested: WORD32 = (*pstr_drc_sel_proc_params_struct)
            .eq_set_purpose_request;
        err = impd_match_eq_set_purpose(
            pstr_drc_config,
            eq_purpose_requested,
            eq_set_id_valid_flag,
            selection_candidate_count,
            selection_candidate_info,
            selection_candidate_info_step_2.as_mut_ptr(),
        );
        if err != 0 {
            return err;
        }
    }
    output_level_min = 10000.0f32 as FLOAT32;
    k = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < *selection_candidate_count {
        if output_level_min
            >= (*selection_candidate_info.offset(i as isize)).output_peak_level
        {
            if output_level_min
                > (*selection_candidate_info.offset(i as isize)).output_peak_level
            {
                output_level_min = (*selection_candidate_info.offset(i as isize))
                    .output_peak_level;
                k = 0 as core::ffi::c_int as WORD32;
            }
            memcpy(
                &mut *selection_candidate_info_step_2.as_mut_ptr().offset(k as isize)
                    as *mut ia_selection_candidate_info_struct as *mut core::ffi::c_void,
                &mut *selection_candidate_info.offset(i as isize)
                    as *mut ia_selection_candidate_info_struct
                    as *const core::ffi::c_void,
                ::core::mem::size_of::<ia_selection_candidate_info_struct>() as size_t,
            );
            k += 1;
        }
        i += 1;
    }
    selection_candidate_step_2_count = k;
    if output_level_min <= 0.0f32 {
        selection_candidate_step_2_count = *selection_candidate_count;
        k = 0 as core::ffi::c_int as WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < selection_candidate_step_2_count {
            if (*selection_candidate_info.offset(i as isize)).output_peak_level <= 0.0f32
            {
                memcpy(
                    &mut *selection_candidate_info_step_2.as_mut_ptr().offset(k as isize)
                        as *mut ia_selection_candidate_info_struct
                        as *mut core::ffi::c_void,
                    &mut *selection_candidate_info.offset(i as isize)
                        as *mut ia_selection_candidate_info_struct
                        as *const core::ffi::c_void,
                    ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                        as size_t,
                );
                k += 1;
            }
            i += 1;
        }
        selection_candidate_step_2_count = k;
        k = 0 as core::ffi::c_int as WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < selection_candidate_step_2_count {
            str_drc_instruction_str = &mut *((*pstr_drc_config).str_drc_instruction_str)
                .as_mut_ptr()
                .offset(
                    (*selection_candidate_info_step_2.as_mut_ptr().offset(i as isize))
                        .drc_instructions_index as isize,
                ) as *mut ia_drc_instructions_struct;
            n = 0 as core::ffi::c_int as WORD32;
            while n < (*str_drc_instruction_str).dwnmix_id_count {
                if (*pstr_drc_sel_proc_params_struct)
                    .requested_dwnmix_id[selection_candidate_info_step_2[i as usize]
                    .downmix_id_request_index as usize]
                    == (*str_drc_instruction_str).downmix_id[n as usize]
                {
                    if k >= SELECTION_CANDIDATE_COUNT_MAX {
                        return UNEXPECTED_ERROR;
                    }
                    memcpy(
                        &mut *selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(k as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *mut core::ffi::c_void,
                        &mut *selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(i as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *const core::ffi::c_void,
                        ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                            as size_t,
                    );
                    k += 1;
                }
                n += 1;
            }
            i += 1;
        }
        if k > 0 as core::ffi::c_int {
            selection_candidate_step_2_count = k;
        }
        effect_types_request_table_size = (::core::mem::size_of::<[WORD32; 8]>()
            as usize)
            .wrapping_div(::core::mem::size_of::<WORD32>() as usize) as WORD32;
        effect_count_min = 100 as core::ffi::c_int as WORD32;
        k = 0 as core::ffi::c_int as WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < selection_candidate_step_2_count {
            str_drc_instruction_str = &mut *((*pstr_drc_config).str_drc_instruction_str)
                .as_mut_ptr()
                .offset(
                    (*selection_candidate_info_step_2.as_mut_ptr().offset(i as isize))
                        .drc_instructions_index as isize,
                ) as *mut ia_drc_instructions_struct;
            effect_count = 0 as core::ffi::c_int as WORD32;
            if (*str_drc_instruction_str).depends_on_drc_set_present
                == 1 as core::ffi::c_int
            {
                err = impd_get_dependent_drc_instructions(
                    pstr_drc_config,
                    str_drc_instruction_str,
                    &mut drc_instructions_dependent,
                );
                if err != 0 {
                    return err;
                }
                n = 0 as core::ffi::c_int as WORD32;
                while n < effect_types_request_table_size {
                    if effect_types_request_table[n as usize] != EFFECT_BIT_GENERAL_COMPR
                    {
                        if (*str_drc_instruction_str).drc_set_effect
                            & effect_types_request_table[n as usize]
                            != 0 as core::ffi::c_int
                            || (*drc_instructions_dependent).drc_set_effect
                                & effect_types_request_table[n as usize]
                                != 0 as core::ffi::c_int
                        {
                            effect_count += 1;
                        }
                    }
                    n += 1;
                }
            } else {
                n = 0 as core::ffi::c_int as WORD32;
                while n < effect_types_request_table_size {
                    if effect_types_request_table[n as usize] != EFFECT_BIT_GENERAL_COMPR
                    {
                        if (*str_drc_instruction_str).drc_set_effect
                            & effect_types_request_table[n as usize]
                            != 0 as core::ffi::c_int
                        {
                            effect_count += 1;
                        }
                    }
                    n += 1;
                }
            }
            if effect_count_min >= effect_count {
                if effect_count_min > effect_count {
                    effect_count_min = effect_count;
                    k = 0 as core::ffi::c_int as WORD32;
                }
                memcpy(
                    &mut *selection_candidate_info_step_2.as_mut_ptr().offset(k as isize)
                        as *mut ia_selection_candidate_info_struct
                        as *mut core::ffi::c_void,
                    &mut *selection_candidate_info_step_2.as_mut_ptr().offset(i as isize)
                        as *mut ia_selection_candidate_info_struct
                        as *const core::ffi::c_void,
                    ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                        as size_t,
                );
                k += 1;
            }
            i += 1;
        }
        selection_candidate_step_2_count = k;
        drc_set_target_loudness_val_upper_min = 100 as core::ffi::c_int as WORD32;
        k = 0 as core::ffi::c_int as WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < selection_candidate_step_2_count {
            if selection_candidate_info_step_2[i as usize].selection_flags
                as core::ffi::c_int & SELECTION_FLAG_DRC_TARGET_LOUDNESS_MATCH != 0
            {
                k += 1;
            }
            i += 1;
        }
        if k != 0 as core::ffi::c_int && k != selection_candidate_step_2_count {
            k = 0 as core::ffi::c_int as WORD32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < selection_candidate_step_2_count {
                if selection_candidate_info_step_2[i as usize].selection_flags
                    as core::ffi::c_int & SELECTION_FLAG_DRC_TARGET_LOUDNESS_MATCH == 0
                {
                    memcpy(
                        &mut *selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(k as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *mut core::ffi::c_void,
                        &mut *selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(i as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *const core::ffi::c_void,
                        ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                            as size_t,
                    );
                    k += 1;
                }
                i += 1;
            }
            selection_candidate_step_2_count = k;
        } else if k == selection_candidate_step_2_count {
            k = 0 as core::ffi::c_int as WORD32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < selection_candidate_step_2_count {
                str_drc_instruction_str = &mut *((*pstr_drc_config)
                    .str_drc_instruction_str)
                    .as_mut_ptr()
                    .offset(
                        (*selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(i as isize))
                            .drc_instructions_index as isize,
                    ) as *mut ia_drc_instructions_struct;
                if (*str_drc_instruction_str).drc_set_target_loudness_present
                    != 1 as core::ffi::c_int
                {
                    return UNEXPECTED_ERROR;
                }
                if drc_set_target_loudness_val_upper_min
                    >= (*str_drc_instruction_str).drc_set_target_loudness_value_upper
                {
                    if drc_set_target_loudness_val_upper_min
                        > (*str_drc_instruction_str).drc_set_target_loudness_value_upper
                    {
                        drc_set_target_loudness_val_upper_min = (*str_drc_instruction_str)
                            .drc_set_target_loudness_value_upper;
                        k = 0 as core::ffi::c_int as WORD32;
                    }
                    memcpy(
                        &mut *selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(k as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *mut core::ffi::c_void,
                        &mut *selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(i as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *const core::ffi::c_void,
                        ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                            as size_t,
                    );
                    k += 1;
                }
                i += 1;
            }
            selection_candidate_step_2_count = k;
        }
        k = 0 as core::ffi::c_int as WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < selection_candidate_step_2_count {
            str_drc_instruction_str = &mut *((*pstr_drc_config).str_drc_instruction_str)
                .as_mut_ptr()
                .offset(
                    (*selection_candidate_info_step_2.as_mut_ptr().offset(i as isize))
                        .drc_instructions_index as isize,
                ) as *mut ia_drc_instructions_struct;
            if (*str_drc_instruction_str).drc_set_target_loudness_present != 0
                && (*pstr_drc_sel_proc_params_struct).loudness_normalization_on != 0
                && (*str_drc_instruction_str).drc_set_target_loudness_value_upper
                    as FLOAT32 >= (*pstr_drc_sel_proc_params_struct).target_loudness
                && ((*str_drc_instruction_str).drc_set_target_loudness_value_lower
                    as FLOAT32) < (*pstr_drc_sel_proc_params_struct).target_loudness
            {
                k += 1;
            }
            i += 1;
        }
        if k != 0 as core::ffi::c_int && k != selection_candidate_step_2_count {
            k = 0 as core::ffi::c_int as WORD32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < selection_candidate_step_2_count {
                str_drc_instruction_str = &mut *((*pstr_drc_config)
                    .str_drc_instruction_str)
                    .as_mut_ptr()
                    .offset(
                        (*selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(i as isize))
                            .drc_instructions_index as isize,
                    ) as *mut ia_drc_instructions_struct;
                if (*str_drc_instruction_str).drc_set_target_loudness_present != 0
                    && (*pstr_drc_sel_proc_params_struct).loudness_normalization_on != 0
                    && (*str_drc_instruction_str).drc_set_target_loudness_value_upper
                        as FLOAT32 >= (*pstr_drc_sel_proc_params_struct).target_loudness
                    && ((*str_drc_instruction_str).drc_set_target_loudness_value_lower
                        as FLOAT32) < (*pstr_drc_sel_proc_params_struct).target_loudness
                {
                    memcpy(
                        &mut *selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(k as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *mut core::ffi::c_void,
                        &mut *selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(i as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *const core::ffi::c_void,
                        ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                            as size_t,
                    );
                    k += 1;
                }
                i += 1;
            }
            selection_candidate_step_2_count = k;
            drc_set_target_loudness_val_upper_min = 100 as core::ffi::c_int as WORD32;
            k = 0 as core::ffi::c_int as WORD32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < selection_candidate_step_2_count {
                str_drc_instruction_str = &mut *((*pstr_drc_config)
                    .str_drc_instruction_str)
                    .as_mut_ptr()
                    .offset(
                        (*selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(i as isize))
                            .drc_instructions_index as isize,
                    ) as *mut ia_drc_instructions_struct;
                if (*str_drc_instruction_str).drc_set_target_loudness_present
                    != 1 as core::ffi::c_int
                {
                    return UNEXPECTED_ERROR;
                }
                if drc_set_target_loudness_val_upper_min
                    >= (*str_drc_instruction_str).drc_set_target_loudness_value_upper
                {
                    if drc_set_target_loudness_val_upper_min
                        > (*str_drc_instruction_str).drc_set_target_loudness_value_upper
                    {
                        drc_set_target_loudness_val_upper_min = (*str_drc_instruction_str)
                            .drc_set_target_loudness_value_upper;
                        k = 0 as core::ffi::c_int as WORD32;
                    }
                    memcpy(
                        &mut *selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(k as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *mut core::ffi::c_void,
                        &mut *selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(i as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *const core::ffi::c_void,
                        ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                            as size_t,
                    );
                    k += 1;
                }
                i += 1;
            }
            selection_candidate_step_2_count = k;
        } else if k == selection_candidate_step_2_count {
            drc_set_target_loudness_val_upper_min = 100 as core::ffi::c_int as WORD32;
            k = 0 as core::ffi::c_int as WORD32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < selection_candidate_step_2_count {
                str_drc_instruction_str = &mut *((*pstr_drc_config)
                    .str_drc_instruction_str)
                    .as_mut_ptr()
                    .offset(
                        (*selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(i as isize))
                            .drc_instructions_index as isize,
                    ) as *mut ia_drc_instructions_struct;
                if (*str_drc_instruction_str).drc_set_target_loudness_present
                    != 1 as core::ffi::c_int
                {
                    return UNEXPECTED_ERROR;
                }
                if drc_set_target_loudness_val_upper_min
                    >= (*str_drc_instruction_str).drc_set_target_loudness_value_upper
                {
                    if drc_set_target_loudness_val_upper_min
                        > (*str_drc_instruction_str).drc_set_target_loudness_value_upper
                    {
                        drc_set_target_loudness_val_upper_min = (*str_drc_instruction_str)
                            .drc_set_target_loudness_value_upper;
                        k = 0 as core::ffi::c_int as WORD32;
                    }
                    memcpy(
                        &mut *selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(k as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *mut core::ffi::c_void,
                        &mut *selection_candidate_info_step_2
                            .as_mut_ptr()
                            .offset(i as isize)
                            as *mut ia_selection_candidate_info_struct
                            as *const core::ffi::c_void,
                        ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                            as size_t,
                    );
                    k += 1;
                }
                i += 1;
            }
            selection_candidate_step_2_count = k;
        }
        k = 0 as core::ffi::c_int as WORD32;
        output_level_max = -1000.0f32 as FLOAT32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < selection_candidate_step_2_count {
            if selection_candidate_info_step_2[i as usize].output_peak_level <= 0.0f32
                && output_level_max
                    <= selection_candidate_info_step_2[i as usize].output_peak_level
            {
                if output_level_max
                    < selection_candidate_info_step_2[i as usize].output_peak_level
                {
                    output_level_max = selection_candidate_info_step_2[i as usize]
                        .output_peak_level;
                    k = 0 as core::ffi::c_int as WORD32;
                }
                memcpy(
                    &mut *selection_candidate_info_step_2.as_mut_ptr().offset(k as isize)
                        as *mut ia_selection_candidate_info_struct
                        as *mut core::ffi::c_void,
                    &mut *selection_candidate_info_step_2.as_mut_ptr().offset(i as isize)
                        as *mut ia_selection_candidate_info_struct
                        as *const core::ffi::c_void,
                    ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                        as size_t,
                );
                k += 1;
                output_level_max = selection_candidate_info_step_2[i as usize]
                    .output_peak_level;
            }
            i += 1;
        }
        selection_candidate_step_2_count = k;
    }
    drc_set_id_max = -(1000 as core::ffi::c_int) as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < selection_candidate_step_2_count {
        str_drc_instruction_str = &mut *((*pstr_drc_config).str_drc_instruction_str)
            .as_mut_ptr()
            .offset(
                (*selection_candidate_info_step_2.as_mut_ptr().offset(i as isize))
                    .drc_instructions_index as isize,
            ) as *mut ia_drc_instructions_struct;
        if drc_set_id_max < (*str_drc_instruction_str).drc_set_id {
            drc_set_id_max = (*str_drc_instruction_str).drc_set_id;
            memcpy(
                &mut *selection_candidate_info_step_2
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize)
                    as *mut ia_selection_candidate_info_struct as *mut core::ffi::c_void,
                &mut *selection_candidate_info_step_2.as_mut_ptr().offset(i as isize)
                    as *mut ia_selection_candidate_info_struct
                    as *const core::ffi::c_void,
                ::core::mem::size_of::<ia_selection_candidate_info_struct>() as size_t,
            );
        }
        i += 1;
    }
    memcpy(
        &mut *selection_candidate_info.offset(0 as core::ffi::c_int as isize)
            as *mut ia_selection_candidate_info_struct as *mut core::ffi::c_void,
        &mut *selection_candidate_info_step_2
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize)
            as *mut ia_selection_candidate_info_struct as *const core::ffi::c_void,
        ::core::mem::size_of::<ia_selection_candidate_info_struct>() as size_t,
    );
    *selection_candidate_count = 1 as core::ffi::c_int as WORD32;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_select_drc_set(
    mut pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
    mut drc_set_id_selected: *mut WORD32,
    mut eq_set_id_selected: *mut WORD32,
    mut loud_eq_id_sel: *mut WORD32,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut err: WORD32 = 0;
    let mut pstr_drc_sel_proc_params_struct: *mut ia_drc_sel_proc_params_struct = &mut (*pstr_drc_uni_sel_proc)
        .uni_drc_sel_proc_params;
    let mut pstr_drc_config: *mut ia_drc_config = &mut (*pstr_drc_uni_sel_proc)
        .drc_config;
    let mut pstr_loudness_info: *mut ia_drc_loudness_info_set_struct = &mut (*pstr_drc_uni_sel_proc)
        .loudness_info_set;
    let mut selection_candidate_count: WORD32 = 0 as WORD32;
    let mut restrict_to_drc_with_album_loudness: WORD32 = 0 as WORD32;
    let mut selection_candidate_info: [ia_selection_candidate_info_struct; 32] = [ia_selection_candidate_info_struct {
        drc_instructions_index: 0,
        downmix_id_request_index: 0,
        eq_set_id: 0,
        output_peak_level: 0.,
        loudness_norm_db_gain_adjusted: 0.,
        output_loudness: 0.,
        mixing_level: 0.,
        selection_flags: 0,
    }; 32];
    if (*pstr_drc_sel_proc_params_struct).album_mode == 1 as core::ffi::c_int {
        restrict_to_drc_with_album_loudness = 1 as core::ffi::c_int as WORD32;
    }
    while selection_candidate_count == 0 {
        err = impd_drc_set_preselection(
            pstr_drc_sel_proc_params_struct,
            pstr_drc_config,
            pstr_loudness_info,
            restrict_to_drc_with_album_loudness,
            pstr_drc_uni_sel_proc,
            &mut selection_candidate_count,
            selection_candidate_info.as_mut_ptr(),
        );
        if err != 0 {
            return err;
        }
        if selection_candidate_count == 0 as core::ffi::c_int {
            if restrict_to_drc_with_album_loudness == 1 as core::ffi::c_int {
                restrict_to_drc_with_album_loudness = 0 as core::ffi::c_int as WORD32;
            } else {
                return 2 as WORD32
            }
        } else {
            err = impd_validate_requested_drc_feature(pstr_drc_sel_proc_params_struct);
            if err != 0 {
                return err;
            }
            if (*pstr_drc_sel_proc_params_struct).dynamic_range_control_on
                == 1 as core::ffi::c_int
            {
                if (*pstr_drc_sel_proc_params_struct).num_drc_feature_requests
                    > 0 as core::ffi::c_int
                {
                    i = 0 as core::ffi::c_int as WORD32;
                    while i < (*pstr_drc_sel_proc_params_struct).num_drc_feature_requests
                    {
                        match (*pstr_drc_sel_proc_params_struct)
                            .drc_feature_req_type[i as usize]
                        {
                            MATCH_EFFECT_TYPE => {
                                err = impd_match_effect_types(
                                    pstr_drc_config,
                                    (*pstr_drc_sel_proc_params_struct)
                                        .requested_num_drc_effects[i as usize],
                                    (*pstr_drc_sel_proc_params_struct)
                                        .desired_num_drc_effects_of_requested[i as usize],
                                    ((*pstr_drc_sel_proc_params_struct)
                                        .requested_drc_effect_type[i as usize])
                                        .as_mut_ptr(),
                                    &mut selection_candidate_count,
                                    selection_candidate_info.as_mut_ptr(),
                                );
                                if err != 0 {
                                    return err;
                                }
                            }
                            MATCH_DYNAMIC_RANGE => {
                                err = impd_match_dynamic_range(
                                    pstr_drc_config,
                                    pstr_loudness_info,
                                    pstr_drc_sel_proc_params_struct,
                                    i,
                                    &mut selection_candidate_count,
                                    selection_candidate_info.as_mut_ptr(),
                                );
                                if err != 0 {
                                    return err;
                                }
                            }
                            MATCH_DRC_CHARACTERISTIC => {
                                err = impd_match_drc_characteristic(
                                    pstr_drc_config,
                                    (*pstr_drc_sel_proc_params_struct)
                                        .requested_drc_characteristic[i as usize],
                                    &mut selection_candidate_count,
                                    selection_candidate_info.as_mut_ptr(),
                                );
                                if err != 0 {
                                    return err;
                                }
                            }
                            _ => return 2 as WORD32,
                        }
                        i += 1;
                    }
                } else {
                    let mut match_found_flag: WORD32 = 0 as WORD32;
                    err = impd_select_drcs_without_compr_effects(
                        pstr_drc_config,
                        &mut match_found_flag,
                        &mut selection_candidate_count,
                        selection_candidate_info.as_mut_ptr(),
                    );
                    if err != 0 {
                        return err;
                    }
                    if match_found_flag == 0 as core::ffi::c_int {
                        let mut requested_num_drc_effects: WORD32 = 5 as WORD32;
                        let mut desired_num_drc_effects_of_requested: WORD32 = 1
                            as WORD32;
                        let mut requested_drc_effect_type: [WORD32; 5] = [
                            EFFECT_TYPE_REQUESTED_GENERAL_COMPR,
                            EFFECT_TYPE_REQUESTED_NIGHT,
                            EFFECT_TYPE_REQUESTED_NOISY,
                            EFFECT_TYPE_REQUESTED_LIMITED,
                            EFFECT_TYPE_REQUESTED_LOWLEVEL,
                        ];
                        err = impd_match_effect_types(
                            pstr_drc_config,
                            requested_num_drc_effects,
                            desired_num_drc_effects_of_requested,
                            requested_drc_effect_type.as_mut_ptr(),
                            &mut selection_candidate_count,
                            selection_candidate_info.as_mut_ptr(),
                        );
                        if err != 0 {
                            return err;
                        }
                    }
                }
                if selection_candidate_count > 0 as core::ffi::c_int {
                    err = impd_drc_set_final_selection(
                        pstr_drc_config,
                        pstr_drc_sel_proc_params_struct,
                        &mut selection_candidate_count,
                        selection_candidate_info.as_mut_ptr(),
                        ((*pstr_drc_uni_sel_proc).eq_set_id_valid_flag).as_mut_ptr(),
                    );
                    if err != 0 {
                        return err;
                    }
                } else {
                    selection_candidate_count = 0 as core::ffi::c_int as WORD32;
                    return 2 as WORD32;
                }
            }
            if selection_candidate_count == 0 as core::ffi::c_int {
                if restrict_to_drc_with_album_loudness == 1 as core::ffi::c_int {
                    restrict_to_drc_with_album_loudness = 0 as core::ffi::c_int
                        as WORD32;
                } else {
                    return 2 as WORD32
                }
            }
        }
    }
    *drc_set_id_selected = (*pstr_drc_config)
        .str_drc_instruction_str[selection_candidate_info[0 as core::ffi::c_int as usize]
            .drc_instructions_index as usize]
        .drc_set_id;
    *eq_set_id_selected = selection_candidate_info[0 as core::ffi::c_int as usize]
        .eq_set_id;
    err = impd_select_loud_eq(
        pstr_drc_config,
        (*pstr_drc_sel_proc_params_struct)
            .requested_dwnmix_id[selection_candidate_info[0 as core::ffi::c_int as usize]
            .downmix_id_request_index as usize],
        *drc_set_id_selected,
        *eq_set_id_selected,
        loud_eq_id_sel,
    );
    if err != 0 {
        return err;
    }
    if selection_candidate_count > 0 as core::ffi::c_int {
        (*pstr_drc_uni_sel_proc)
            .uni_drc_sel_proc_output
            .loudness_normalization_gain_db = selection_candidate_info[0
                as core::ffi::c_int as usize]
            .loudness_norm_db_gain_adjusted;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.output_peak_level_db = selection_candidate_info[0
                as core::ffi::c_int as usize]
            .output_peak_level;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.output_loudness = selection_candidate_info[0
                as core::ffi::c_int as usize]
            .output_loudness;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.active_downmix_id = (*pstr_drc_sel_proc_params_struct)
            .requested_dwnmix_id[selection_candidate_info[0 as core::ffi::c_int as usize]
            .downmix_id_request_index as usize];
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.mixing_level = selection_candidate_info[0
                as core::ffi::c_int as usize]
            .mixing_level;
    }
    return 0 as WORD32;
}
pub const SELECTION_FLAG_DRC_TARGET_LOUDNESS_MATCH: core::ffi::c_int = (1
    as core::ffi::c_int) << 0 as core::ffi::c_int;
pub const SELECTION_FLAG_EXPLICIT_PEAK_INFO_PRESENT: core::ffi::c_int = (1
    as core::ffi::c_int) << 1 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
