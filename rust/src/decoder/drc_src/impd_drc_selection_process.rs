extern "C" {
    fn pow(__x: core::ffi::c_double, __y: core::ffi::c_double) -> core::ffi::c_double;
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memcmp(
        __s1: *const core::ffi::c_void,
        __s2: *const core::ffi::c_void,
        __n: size_t,
    ) -> core::ffi::c_int;
    fn impd_find_drc_instructions_uni_drc(
        drc_config: *mut ia_drc_config,
        drc_set_id_requested: WORD32,
        str_drc_instruction_str: *mut *mut ia_drc_instructions_struct,
    ) -> WORD32;
    fn impd_get_fading_drc_set(
        pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
    ) -> WORD32;
    fn impd_get_ducking_drc_set(
        pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
    ) -> WORD32;
    fn impd_get_selected_drc_set(
        pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
        drc_set_id_selected: WORD32,
    ) -> WORD32;
    fn impd_get_dependent_drc_set(
        pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
    ) -> WORD32;
    fn impd_select_drc_set(
        pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
        drc_set_id_selected: *mut WORD32,
        eq_set_id_selected: *mut WORD32,
        loud_eq_id_sel: *mut WORD32,
    ) -> WORD32;
    fn impd_drc_sel_proc_init_dflt(
        pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
    ) -> WORD32;
    fn impd_drc_sel_proc_init_sel_proc_params(
        pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
        pstr_drc_sel_proc_params_struct: *mut ia_drc_sel_proc_params_struct,
    ) -> WORD32;
    fn impd_drc_sel_proc_init_interface_params(
        pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
        pstr_drc_interface: *mut ia_drc_interface_struct,
    ) -> VOID;
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
pub const SUB_DRC_COUNT: core::ffi::c_int = 4 as core::ffi::c_int;
pub const EQ_INSTRUCTIONS_COUNT_MAX: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MAX_NUM_COMPRESSION_EQ: core::ffi::c_int = 16 as core::ffi::c_int;
pub const UNEXPECTED_ERROR: core::ffi::c_int = 2 as core::ffi::c_int;
pub const EXTERNAL_ERROR: core::ffi::c_int = 4 as core::ffi::c_int;
pub const ID_FOR_ANY_DOWNMIX: core::ffi::c_int = 0x7f as core::ffi::c_int;
pub const ID_FOR_ANY_DRC: core::ffi::c_int = 0x3f as core::ffi::c_int;
pub const ID_FOR_ANY_EQ: core::ffi::c_int = 0x3f as core::ffi::c_int;
pub const LOCATION_MP4_INSTREAM_UNIDRC: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const LOCATION_SELECTED: core::ffi::c_int = LOCATION_MP4_INSTREAM_UNIDRC;
pub const SUBBAND_DOMAIN_MODE_OFF: core::ffi::c_int = 0 as core::ffi::c_int;
pub const EQ_PURPOSE_DEFAULT: core::ffi::c_int = (1 as core::ffi::c_int)
    << 0 as core::ffi::c_int;
pub const EFFECT_BIT_DUCK_OTHER: core::ffi::c_int = 0x400 as core::ffi::c_int;
pub const EFFECT_BIT_DUCK_SELF: core::ffi::c_int = 0x800 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn impd_drc_uni_selction_proc_init(
    mut pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
    mut pstr_drc_sel_proc_params_struct: *mut ia_drc_sel_proc_params_struct,
    mut pstr_drc_interface: *mut ia_drc_interface_struct,
    mut subband_domain_mode: WORD32,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    if pstr_drc_uni_sel_proc.is_null() {
        return 1 as WORD32;
    }
    if (*pstr_drc_uni_sel_proc).first_frame == 1 as core::ffi::c_int {
        err = impd_drc_sel_proc_init_dflt(pstr_drc_uni_sel_proc);
        if err != 0 {
            return err;
        }
    }
    err = impd_drc_sel_proc_init_sel_proc_params(
        pstr_drc_uni_sel_proc,
        pstr_drc_sel_proc_params_struct,
    );
    if err != 0 {
        return err;
    }
    let mut i: WORD32 = 0;
    (*pstr_drc_uni_sel_proc).drc_set_id_valid_flag[0 as core::ffi::c_int as usize] = 1
        as core::ffi::c_int as WORD32;
    i = 1 as core::ffi::c_int as WORD32;
    while i < DRC_INSTRUCTIONS_COUNT_MAX {
        (*pstr_drc_uni_sel_proc).drc_set_id_valid_flag[i as usize] = 0
            as core::ffi::c_int as WORD32;
        i += 1;
    }
    (*pstr_drc_uni_sel_proc).eq_set_id_valid_flag[0 as core::ffi::c_int as usize] = 1
        as core::ffi::c_int as WORD32;
    i = 1 as core::ffi::c_int as WORD32;
    while i < EQ_INSTRUCTIONS_COUNT_MAX {
        (*pstr_drc_uni_sel_proc).eq_set_id_valid_flag[i as usize] = 0 as core::ffi::c_int
            as WORD32;
        i += 1;
    }
    impd_drc_sel_proc_init_interface_params(pstr_drc_uni_sel_proc, pstr_drc_interface);
    (*pstr_drc_uni_sel_proc).subband_domain_mode = subband_domain_mode;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_drc_uni_sel_proc_process(
    mut pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
    mut pstr_drc_config: *mut ia_drc_config,
    mut pstr_loudness_info: *mut ia_drc_loudness_info_set_struct,
    mut hia_drc_sel_proc_output_struct: *mut ia_drc_sel_proc_output_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut err: WORD32 = 0;
    let mut drc_set_id_selected: WORD32 = 0;
    let mut activeDrcSetIndex: WORD32 = 0;
    let mut eq_set_id_selected: WORD32 = 0;
    let mut loudEqSetIdSelected: WORD32 = 0;
    if !pstr_drc_config.is_null() {
        if memcmp(
            &mut (*pstr_drc_uni_sel_proc).drc_config as *mut ia_drc_config
                as *const core::ffi::c_void,
            pstr_drc_config as *const core::ffi::c_void,
            ::core::mem::size_of::<ia_drc_config>() as size_t,
        ) != 0
        {
            (*pstr_drc_uni_sel_proc).drc_config = *pstr_drc_config;
            (*pstr_drc_uni_sel_proc).drc_config_flag = 1 as core::ffi::c_int as WORD32;
            if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.base_channel_count
                != (*pstr_drc_uni_sel_proc).drc_config.channel_layout.base_channel_count
            {
                (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.base_channel_count = (*pstr_drc_uni_sel_proc)
                    .drc_config
                    .channel_layout
                    .base_channel_count;
            }
            if (*pstr_drc_uni_sel_proc)
                .drc_config
                .channel_layout
                .layout_signaling_present == 1 as core::ffi::c_int
                && (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.base_layout
                    != (*pstr_drc_uni_sel_proc).drc_config.channel_layout.defined_layout
            {
                (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.base_layout = (*pstr_drc_uni_sel_proc)
                    .drc_config
                    .channel_layout
                    .defined_layout;
            }
        } else {
            (*pstr_drc_uni_sel_proc).drc_config_flag = 0 as core::ffi::c_int as WORD32;
        }
    }
    if !pstr_loudness_info.is_null() {
        if memcmp(
            &mut (*pstr_drc_uni_sel_proc).loudness_info_set
                as *mut ia_drc_loudness_info_set_struct as *const core::ffi::c_void,
            pstr_loudness_info as *const core::ffi::c_void,
            ::core::mem::size_of::<ia_drc_loudness_info_set_struct>() as size_t,
        ) != 0
        {
            (*pstr_drc_uni_sel_proc).loudness_info_set = *pstr_loudness_info;
            (*pstr_drc_uni_sel_proc).loudness_info_set_flag = 1 as core::ffi::c_int
                as WORD32;
        } else {
            (*pstr_drc_uni_sel_proc).loudness_info_set_flag = 0 as core::ffi::c_int
                as WORD32;
        }
    }
    if (*pstr_drc_uni_sel_proc).drc_config_flag != 0
        && (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.target_config_request_type
            != 0 as core::ffi::c_int
        || (*pstr_drc_uni_sel_proc).sel_proc_request_flag != 0
            && (*pstr_drc_uni_sel_proc)
                .uni_drc_sel_proc_params
                .target_config_request_type != 0 as core::ffi::c_int
        || (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.target_config_request_type
            == 0 as core::ffi::c_int
            && (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.num_downmix_id_requests
                == 0 as core::ffi::c_int
    {
        err = impd_map_target_config_req_downmix_id(
            pstr_drc_uni_sel_proc,
            &mut (*pstr_drc_uni_sel_proc).drc_config,
        );
        if err != 0 {
            return err;
        }
    }
    if (*pstr_drc_uni_sel_proc).drc_config_flag != 0
        || (*pstr_drc_uni_sel_proc).loudness_info_set_flag != 0
        || (*pstr_drc_uni_sel_proc).sel_proc_request_flag != 0
    {
        let mut repeat_selection: WORD32 = 1 as WORD32;
        err = impd_manage_drc_complexity(pstr_drc_uni_sel_proc, pstr_drc_config);
        if err != 0 {
            return err;
        }
        err = impd_manage_eq_complexity(pstr_drc_uni_sel_proc, pstr_drc_config);
        if err != 0 {
            return err;
        }
        while repeat_selection == 1 as core::ffi::c_int {
            err = impd_select_drc_set(
                pstr_drc_uni_sel_proc,
                &mut drc_set_id_selected,
                &mut eq_set_id_selected,
                &mut loudEqSetIdSelected,
            );
            if err != 0 {
                return err;
            }
            err = impd_get_selected_drc_set(pstr_drc_uni_sel_proc, drc_set_id_selected);
            if err != 0 {
                return err;
            }
            err = impd_get_dependent_drc_set(pstr_drc_uni_sel_proc);
            if err != 0 {
                return err;
            }
            err = impd_get_fading_drc_set(pstr_drc_uni_sel_proc);
            if err != 0 {
                return err;
            }
            err = impd_get_ducking_drc_set(pstr_drc_uni_sel_proc);
            if err != 0 {
                return err;
            }
            (*pstr_drc_uni_sel_proc).eq_inst_index[0 as core::ffi::c_int as usize] = -(1
                as core::ffi::c_int) as WORD32;
            (*pstr_drc_uni_sel_proc).eq_inst_index[1 as core::ffi::c_int as usize] = -(1
                as core::ffi::c_int) as WORD32;
            err = impd_get_selected_eq_set(pstr_drc_uni_sel_proc, eq_set_id_selected);
            if err != 0 {
                return err;
            }
            err = impd_get_dependent_eq_set(pstr_drc_uni_sel_proc);
            if err != 0 {
                return err;
            }
            err = impd_get_selected_loud_eq_set(
                pstr_drc_uni_sel_proc,
                loudEqSetIdSelected,
            );
            if err != 0 {
                return err;
            }
            activeDrcSetIndex = 0 as core::ffi::c_int as WORD32;
            i = (SUB_DRC_COUNT - 1 as core::ffi::c_int) as WORD32;
            while i >= 0 as core::ffi::c_int {
                let mut drc_instructions_index: WORD32 = (*pstr_drc_uni_sel_proc)
                    .drc_instructions_index[i as usize];
                let mut str_drc_instruction_str: *mut ia_drc_instructions_struct = 0
                    as *mut ia_drc_instructions_struct;
                if !(drc_instructions_index < 0 as core::ffi::c_int) {
                    str_drc_instruction_str = &mut *((*pstr_drc_uni_sel_proc)
                        .drc_config
                        .str_drc_instruction_str)
                        .as_mut_ptr()
                        .offset(drc_instructions_index as isize)
                        as *mut ia_drc_instructions_struct;
                    if (*str_drc_instruction_str).drc_set_id > 0 as core::ffi::c_int {
                        (*pstr_drc_uni_sel_proc)
                            .uni_drc_sel_proc_output
                            .sel_drc_set_ids[activeDrcSetIndex as usize] = (*str_drc_instruction_str)
                            .drc_set_id;
                        if i == 3 as core::ffi::c_int
                            && (*str_drc_instruction_str).drc_set_effect
                                as core::ffi::c_int
                                & (EFFECT_BIT_DUCK_SELF | EFFECT_BIT_DUCK_OTHER) != 0
                        {
                            (*pstr_drc_uni_sel_proc)
                                .uni_drc_sel_proc_output
                                .sel_downmix_ids[activeDrcSetIndex as usize] = 0
                                as core::ffi::c_int as WORD32;
                        } else if (*str_drc_instruction_str).drc_apply_to_dwnmix
                            == 1 as core::ffi::c_int
                        {
                            (*pstr_drc_uni_sel_proc)
                                .uni_drc_sel_proc_output
                                .sel_downmix_ids[activeDrcSetIndex as usize] = (*str_drc_instruction_str)
                                .downmix_id[0 as core::ffi::c_int as usize];
                        } else {
                            (*pstr_drc_uni_sel_proc)
                                .uni_drc_sel_proc_output
                                .sel_downmix_ids[activeDrcSetIndex as usize] = 0
                                as core::ffi::c_int as WORD32;
                        }
                        activeDrcSetIndex += 1;
                    }
                }
                i -= 1;
            }
            if activeDrcSetIndex <= 3 as core::ffi::c_int {
                (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.num_sel_drc_sets = activeDrcSetIndex;
            } else {
                (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.num_sel_drc_sets = -(1
                    as core::ffi::c_int) as WORD32;
                return 2 as WORD32;
            }
            impd_sel_downmix_matrix(
                pstr_drc_uni_sel_proc,
                &mut (*pstr_drc_uni_sel_proc).drc_config,
            );
            err = impd_manage_complexity(
                pstr_drc_uni_sel_proc,
                pstr_drc_config,
                &mut repeat_selection,
            );
            if err != 0 {
                return err;
            }
        }
        (*pstr_drc_uni_sel_proc).sel_proc_request_flag = 0 as core::ffi::c_int as WORD32;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.boost = (*pstr_drc_uni_sel_proc)
            .uni_drc_sel_proc_params
            .boost;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.compress = (*pstr_drc_uni_sel_proc)
            .uni_drc_sel_proc_params
            .compress;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.drc_characteristic_target = (*pstr_drc_uni_sel_proc)
            .uni_drc_sel_proc_params
            .drc_characteristic_target;
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.loudness_normalization_gain_db
            += (*pstr_drc_uni_sel_proc)
                .uni_drc_sel_proc_params
                .loudness_norm_gain_modification_db;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < 2 as core::ffi::c_int {
        if (*pstr_drc_uni_sel_proc).eq_inst_index[i as usize] >= 0 as core::ffi::c_int {
            (*pstr_drc_uni_sel_proc)
                .uni_drc_sel_proc_output
                .sel_eq_set_ids[i as usize] = (*pstr_drc_uni_sel_proc)
                .drc_config
                .str_drc_config_ext
                .str_eq_instructions[(*pstr_drc_uni_sel_proc).eq_inst_index[i as usize]
                    as usize]
                .eq_set_id;
        }
        i += 1;
    }
    if (*pstr_drc_uni_sel_proc).loud_eq_inst_index_sel >= 0 as core::ffi::c_int {
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.sel_loud_eq_id = (*pstr_drc_uni_sel_proc)
            .drc_config
            .str_drc_config_ext
            .loud_eq_instructions[(*pstr_drc_uni_sel_proc).loud_eq_inst_index_sel
                as usize]
            .loud_eq_set_id;
    } else {
        (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.sel_loud_eq_id = 0
            as core::ffi::c_int as WORD32;
    }
    *hia_drc_sel_proc_output_struct = (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_map_target_config_req_downmix_id(
    mut pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
    mut pstr_drc_config: *mut ia_drc_config,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut dwnmix_instructions_count: WORD32 = 0;
    let mut target_ch_count_prelim: WORD32 = (*pstr_drc_uni_sel_proc)
        .uni_drc_sel_proc_params
        .base_channel_count;
    (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.num_downmix_id_requests = 0
        as core::ffi::c_int as WORD32;
    match (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.target_config_request_type {
        0 => {
            if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.num_downmix_id_requests
                == 0 as core::ffi::c_int
            {
                (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .requested_dwnmix_id[0 as core::ffi::c_int as usize] = 0
                    as core::ffi::c_int as WORD32;
                (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .num_downmix_id_requests = 1 as core::ffi::c_int as WORD32;
            }
        }
        1 => {
            if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.requested_target_layout
                == (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.base_layout
            {
                (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .requested_dwnmix_id[0 as core::ffi::c_int as usize] = 0
                    as core::ffi::c_int as WORD32;
                (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .num_downmix_id_requests = 1 as core::ffi::c_int as WORD32;
            }
            if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.num_downmix_id_requests
                == 0 as core::ffi::c_int
            {
                dwnmix_instructions_count = (*pstr_drc_uni_sel_proc)
                    .drc_config
                    .dwnmix_instructions_count;
                i = 0 as core::ffi::c_int as WORD32;
                while i < dwnmix_instructions_count {
                    let mut dwnmix_instructions: *mut ia_downmix_instructions_struct = &mut *((*pstr_drc_config)
                        .dwnmix_instructions)
                        .as_mut_ptr()
                        .offset(i as isize) as *mut ia_downmix_instructions_struct;
                    if (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .requested_target_layout == (*dwnmix_instructions).target_layout
                    {
                        (*pstr_drc_uni_sel_proc)
                            .uni_drc_sel_proc_params
                            .requested_dwnmix_id[(*pstr_drc_uni_sel_proc)
                            .uni_drc_sel_proc_params
                            .num_downmix_id_requests as usize] = (*dwnmix_instructions)
                            .downmix_id;
                        (*pstr_drc_uni_sel_proc)
                            .uni_drc_sel_proc_params
                            .num_downmix_id_requests += 1 as core::ffi::c_int;
                        target_ch_count_prelim = (*dwnmix_instructions)
                            .target_channel_count;
                    }
                    i += 1;
                }
            }
            if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.num_downmix_id_requests
                == 0 as core::ffi::c_int
            {
                (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .requested_dwnmix_id[0 as core::ffi::c_int as usize] = 0
                    as core::ffi::c_int as WORD32;
                (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .num_downmix_id_requests = 1 as core::ffi::c_int as WORD32;
            }
        }
        2 => {
            if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.requested_target_ch_count
                == (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.base_channel_count
            {
                (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .requested_dwnmix_id[0 as core::ffi::c_int as usize] = 0
                    as core::ffi::c_int as WORD32;
                (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .num_downmix_id_requests = 1 as core::ffi::c_int as WORD32;
            }
            if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.num_downmix_id_requests
                == 0 as core::ffi::c_int
            {
                dwnmix_instructions_count = (*pstr_drc_uni_sel_proc)
                    .drc_config
                    .dwnmix_instructions_count;
                i = 0 as core::ffi::c_int as WORD32;
                while i < dwnmix_instructions_count {
                    let mut dwnmix_instructions_0: *mut ia_downmix_instructions_struct = &mut *((*pstr_drc_config)
                        .dwnmix_instructions)
                        .as_mut_ptr()
                        .offset(i as isize) as *mut ia_downmix_instructions_struct;
                    if (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_params
                        .requested_target_ch_count
                        == (*dwnmix_instructions_0).target_channel_count
                    {
                        (*pstr_drc_uni_sel_proc)
                            .uni_drc_sel_proc_params
                            .requested_dwnmix_id[(*pstr_drc_uni_sel_proc)
                            .uni_drc_sel_proc_params
                            .num_downmix_id_requests as usize] = (*dwnmix_instructions_0)
                            .downmix_id;
                        (*pstr_drc_uni_sel_proc)
                            .uni_drc_sel_proc_params
                            .num_downmix_id_requests += 1 as core::ffi::c_int;
                        target_ch_count_prelim = (*dwnmix_instructions_0)
                            .target_channel_count;
                    }
                    i += 1;
                }
            }
            if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.num_downmix_id_requests
                == 0 as core::ffi::c_int
            {
                (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .requested_dwnmix_id[0 as core::ffi::c_int as usize] = 0
                    as core::ffi::c_int as WORD32;
                (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_params
                    .num_downmix_id_requests = 1 as core::ffi::c_int as WORD32;
            }
        }
        _ => return UNEXPECTED_ERROR,
    }
    (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_params.target_ch_count_prelim = target_ch_count_prelim;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_sel_downmix_matrix(
    mut pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
    mut pstr_drc_config: *mut ia_drc_config,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut n: WORD32 = 0;
    (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.base_channel_count = (*pstr_drc_config)
        .channel_layout
        .base_channel_count;
    (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.target_channel_count = (*pstr_drc_config)
        .channel_layout
        .base_channel_count;
    (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.target_layout = -(1
        as core::ffi::c_int) as WORD32;
    (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.downmix_matrix_present = 0
        as core::ffi::c_int as WORD32;
    (*pstr_drc_uni_sel_proc).downmix_inst_index_sel = -(1 as core::ffi::c_int) as WORD32;
    if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.active_downmix_id
        != 0 as core::ffi::c_int
    {
        n = 0 as core::ffi::c_int as WORD32;
        while n < (*pstr_drc_config).dwnmix_instructions_count {
            let mut dwnmix_instructions: *mut ia_downmix_instructions_struct = &mut *((*pstr_drc_config)
                .dwnmix_instructions)
                .as_mut_ptr()
                .offset(n as isize) as *mut ia_downmix_instructions_struct;
            if (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.active_downmix_id
                == (*dwnmix_instructions).downmix_id
            {
                (*pstr_drc_uni_sel_proc).downmix_inst_index_sel = n;
                (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.target_channel_count = (*dwnmix_instructions)
                    .target_channel_count;
                (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.target_layout = (*dwnmix_instructions)
                    .target_layout;
                if (*dwnmix_instructions).downmix_coefficients_present != 0 {
                    i = 0 as core::ffi::c_int as WORD32;
                    while i
                        < (*pstr_drc_uni_sel_proc)
                            .uni_drc_sel_proc_output
                            .base_channel_count
                    {
                        j = 0 as core::ffi::c_int as WORD32;
                        while j
                            < (*pstr_drc_uni_sel_proc)
                                .uni_drc_sel_proc_output
                                .target_channel_count
                        {
                            (*pstr_drc_uni_sel_proc)
                                .uni_drc_sel_proc_output
                                .downmix_matrix[i as usize][j as usize] = (*dwnmix_instructions)
                                .downmix_coefficient[(i
                                + j
                                    * (*pstr_drc_uni_sel_proc)
                                        .uni_drc_sel_proc_output
                                        .base_channel_count) as usize];
                            j += 1;
                        }
                        i += 1;
                    }
                    (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_output
                        .downmix_matrix_present = 1 as core::ffi::c_int as WORD32;
                }
                break;
            } else {
                n += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn impd_get_selected_eq_set(
    mut pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
    mut eq_set_id_selected: WORD32,
) -> WORD32 {
    let mut n: WORD32 = 0;
    (*pstr_drc_uni_sel_proc).eq_inst_index_sel = -(1 as core::ffi::c_int) as WORD32;
    if eq_set_id_selected > 0 as core::ffi::c_int {
        n = 0 as core::ffi::c_int as WORD32;
        while n
            < (*pstr_drc_uni_sel_proc)
                .drc_config
                .str_drc_config_ext
                .eq_instructions_count
        {
            if (*pstr_drc_uni_sel_proc)
                .drc_config
                .str_drc_config_ext
                .str_eq_instructions[n as usize]
                .eq_set_id == eq_set_id_selected
            {
                break;
            }
            n += 1;
        }
        if n
            == (*pstr_drc_uni_sel_proc)
                .drc_config
                .str_drc_config_ext
                .eq_instructions_count
        {
            return 4 as WORD32;
        }
        (*pstr_drc_uni_sel_proc).eq_inst_index_sel = n;
        if (*pstr_drc_uni_sel_proc)
            .drc_config
            .str_drc_config_ext
            .str_eq_instructions[n as usize]
            .eq_apply_to_downmix == 1 as core::ffi::c_int
        {
            (*pstr_drc_uni_sel_proc).eq_inst_index[1 as core::ffi::c_int as usize] = (*pstr_drc_uni_sel_proc)
                .eq_inst_index_sel;
        } else {
            (*pstr_drc_uni_sel_proc).eq_inst_index[0 as core::ffi::c_int as usize] = (*pstr_drc_uni_sel_proc)
                .eq_inst_index_sel;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_get_dependent_eq_set(
    mut pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
) -> WORD32 {
    let mut str_eq_instructions: *mut ia_eq_instructions_struct = 0
        as *mut ia_eq_instructions_struct;
    if (*pstr_drc_uni_sel_proc).eq_inst_index_sel >= 0 as core::ffi::c_int {
        str_eq_instructions = &mut *((*pstr_drc_uni_sel_proc)
            .drc_config
            .str_drc_config_ext
            .str_eq_instructions)
            .as_mut_ptr()
            .offset((*pstr_drc_uni_sel_proc).eq_inst_index_sel as isize)
            as *mut ia_eq_instructions_struct;
        if (*str_eq_instructions).depends_on_eq_set_present == 1 as core::ffi::c_int {
            let mut n: WORD32 = 0;
            let mut dependsOnEqSetID: WORD32 = (*str_eq_instructions).depends_on_eq_set;
            n = 0 as core::ffi::c_int as WORD32;
            while n
                < (*pstr_drc_uni_sel_proc)
                    .drc_config
                    .str_drc_config_ext
                    .eq_instructions_count
            {
                if (*pstr_drc_uni_sel_proc)
                    .drc_config
                    .str_drc_config_ext
                    .str_eq_instructions[n as usize]
                    .eq_set_id == dependsOnEqSetID
                {
                    break;
                }
                n += 1;
            }
            if n
                == (*pstr_drc_uni_sel_proc)
                    .drc_config
                    .str_drc_config_ext
                    .eq_instructions_count
            {
                return 2 as WORD32;
            }
            if (*pstr_drc_uni_sel_proc)
                .drc_config
                .str_drc_config_ext
                .str_eq_instructions[n as usize]
                .eq_apply_to_downmix == 1 as core::ffi::c_int
            {
                (*pstr_drc_uni_sel_proc).eq_inst_index[1 as core::ffi::c_int as usize] = n;
            } else {
                (*pstr_drc_uni_sel_proc).eq_inst_index[0 as core::ffi::c_int as usize] = n;
            }
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_get_selected_loud_eq_set(
    mut pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
    mut loudEqSetIdSelected: WORD32,
) -> WORD32 {
    let mut n: WORD32 = 0;
    (*pstr_drc_uni_sel_proc).loud_eq_inst_index_sel = -(1 as core::ffi::c_int) as WORD32;
    if loudEqSetIdSelected > 0 as core::ffi::c_int {
        n = 0 as core::ffi::c_int as WORD32;
        while n
            < (*pstr_drc_uni_sel_proc)
                .drc_config
                .str_drc_config_ext
                .loud_eq_instructions_count
        {
            if (*pstr_drc_uni_sel_proc)
                .drc_config
                .str_drc_config_ext
                .loud_eq_instructions[n as usize]
                .loud_eq_set_id == loudEqSetIdSelected
            {
                break;
            }
            n += 1;
        }
        if n
            == (*pstr_drc_uni_sel_proc)
                .drc_config
                .str_drc_config_ext
                .loud_eq_instructions_count
        {
            return 4 as WORD32;
        }
        (*pstr_drc_uni_sel_proc).loud_eq_inst_index_sel = n;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_select_loud_eq(
    mut pstr_drc_config: *mut ia_drc_config,
    mut requested_dwnmix_id: WORD32,
    mut drc_set_id_requested: WORD32,
    mut eq_set_id_requested: WORD32,
    mut loud_eq_id_sel: *mut WORD32,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut c: WORD32 = 0;
    let mut d: WORD32 = 0;
    let mut e: WORD32 = 0;
    *loud_eq_id_sel = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*pstr_drc_config).str_drc_config_ext.loud_eq_instructions_count {
        let mut loud_eq_instructions: *mut ia_loud_eq_instructions_struct = &mut *((*pstr_drc_config)
            .str_drc_config_ext
            .loud_eq_instructions)
            .as_mut_ptr()
            .offset(i as isize) as *mut ia_loud_eq_instructions_struct;
        if (*loud_eq_instructions).drc_location == LOCATION_SELECTED {
            d = 0 as core::ffi::c_int as WORD32;
            while d < (*loud_eq_instructions).dwnmix_id_count {
                if (*loud_eq_instructions).downmix_id[d as usize] == requested_dwnmix_id
                    || (*loud_eq_instructions).downmix_id[d as usize]
                        == ID_FOR_ANY_DOWNMIX
                {
                    c = 0 as core::ffi::c_int as WORD32;
                    while c < (*loud_eq_instructions).drc_set_id_count {
                        if (*loud_eq_instructions).drc_set_id[c as usize]
                            == drc_set_id_requested
                            || (*loud_eq_instructions).drc_set_id[c as usize]
                                == ID_FOR_ANY_DRC
                        {
                            e = 0 as core::ffi::c_int as WORD32;
                            while e < (*loud_eq_instructions).eq_set_id_count {
                                if (*loud_eq_instructions).eq_set_id[e as usize]
                                    == eq_set_id_requested
                                    || (*loud_eq_instructions).eq_set_id[e as usize]
                                        == ID_FOR_ANY_EQ
                                {
                                    *loud_eq_id_sel = (*loud_eq_instructions).loud_eq_set_id;
                                }
                                e += 1;
                            }
                        }
                        c += 1;
                    }
                }
                d += 1;
            }
        }
        i += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_match_eq_set(
    mut drc_config: *mut ia_drc_config,
    mut downmix_id: WORD32,
    mut drc_set_id: WORD32,
    mut eq_set_id_valid_flag: *mut WORD32,
    mut matching_eq_set_count: *mut WORD32,
    mut matching_eq_set_idx: *mut WORD32,
) -> WORD32 {
    let mut str_eq_instructions: *mut ia_eq_instructions_struct = 0
        as *mut ia_eq_instructions_struct;
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut n: WORD32 = 0;
    *matching_eq_set_count = 0 as core::ffi::c_int as WORD32;
    let mut current_block_14: u64;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*drc_config).str_drc_config_ext.eq_instructions_count {
        str_eq_instructions = &mut *((*drc_config)
            .str_drc_config_ext
            .str_eq_instructions)
            .as_mut_ptr()
            .offset(i as isize) as *mut ia_eq_instructions_struct;
        if (*str_eq_instructions).depends_on_eq_set_present == 0 as core::ffi::c_int {
            if (*str_eq_instructions).no_independent_eq_use == 1 as core::ffi::c_int {
                current_block_14 = 16559507199688588974;
            } else {
                current_block_14 = 11875828834189669668;
            }
        } else {
            current_block_14 = 11875828834189669668;
        }
        match current_block_14 {
            11875828834189669668 => {
                if !(*eq_set_id_valid_flag
                    .offset((*str_eq_instructions).eq_set_id as isize)
                    == 0 as core::ffi::c_int)
                {
                    k = 0 as core::ffi::c_int as WORD32;
                    while k < (*str_eq_instructions).dwnmix_id_count {
                        if (*str_eq_instructions).downmix_id[k as usize]
                            == ID_FOR_ANY_DOWNMIX
                            || downmix_id
                                == (*str_eq_instructions).downmix_id[k as usize]
                        {
                            n = 0 as core::ffi::c_int as WORD32;
                            while n < (*str_eq_instructions).drc_set_id_count {
                                if (*str_eq_instructions).drc_set_id[n as usize]
                                    == ID_FOR_ANY_DRC
                                    || drc_set_id
                                        == (*str_eq_instructions).drc_set_id[n as usize]
                                {
                                    *matching_eq_set_idx
                                        .offset(*matching_eq_set_count as isize) = i;
                                    *matching_eq_set_count += 1;
                                }
                                n += 1;
                            }
                        }
                        k += 1;
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_match_eq_set_purpose(
    mut drc_config: *mut ia_drc_config,
    mut eq_set_purpose_requested: WORD32,
    mut eq_set_id_valid_flag: *mut WORD32,
    mut selection_candidate_count: *mut WORD32,
    mut selection_candidate_info: *mut ia_selection_candidate_info_struct,
    mut selection_candidate_info_step_2: *mut ia_selection_candidate_info_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut match_found_flag: WORD32 = 0;
    let mut loop_cnt: WORD32 = 0 as WORD32;
    let mut str_eq_instructions: *mut ia_eq_instructions_struct = 0
        as *mut ia_eq_instructions_struct;
    match_found_flag = 0 as core::ffi::c_int as WORD32;
    k = 0 as core::ffi::c_int as WORD32;
    while k == 0 as core::ffi::c_int && loop_cnt < 2 as core::ffi::c_int {
        j = 0 as core::ffi::c_int as WORD32;
        while j < *selection_candidate_count {
            let mut eq_set_id_requested: WORD32 = (*selection_candidate_info
                .offset(j as isize))
                .eq_set_id;
            let mut current_block_7: u64;
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*drc_config).str_drc_config_ext.eq_instructions_count {
                str_eq_instructions = &mut *((*drc_config)
                    .str_drc_config_ext
                    .str_eq_instructions)
                    .as_mut_ptr()
                    .offset(i as isize) as *mut ia_eq_instructions_struct;
                if (*str_eq_instructions).depends_on_eq_set_present
                    == 0 as core::ffi::c_int
                {
                    if *eq_set_id_valid_flag
                        .offset((*str_eq_instructions).eq_set_id as isize)
                        == 0 as core::ffi::c_int
                    {
                        current_block_7 = 6483416627284290920;
                    } else {
                        current_block_7 = 13109137661213826276;
                    }
                } else {
                    current_block_7 = 13109137661213826276;
                }
                match current_block_7 {
                    13109137661213826276 => {
                        if !(*eq_set_id_valid_flag
                            .offset((*str_eq_instructions).eq_set_id as isize)
                            == 0 as core::ffi::c_int)
                        {
                            if (*str_eq_instructions).eq_set_id == eq_set_id_requested
                                && (*str_eq_instructions).eq_set_purpose
                                    & eq_set_purpose_requested != 0
                            {
                                match_found_flag = 1 as core::ffi::c_int as WORD32;
                            }
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
            if match_found_flag > 0 as core::ffi::c_int {
                memcpy(
                    &mut *selection_candidate_info_step_2.offset(k as isize)
                        as *mut ia_selection_candidate_info_struct
                        as *mut core::ffi::c_void,
                    &mut *selection_candidate_info.offset(j as isize)
                        as *mut ia_selection_candidate_info_struct
                        as *const core::ffi::c_void,
                    ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                        as size_t,
                );
                k += 1;
            }
            j += 1;
        }
        eq_set_purpose_requested = EQ_PURPOSE_DEFAULT as WORD32;
        loop_cnt += 1;
    }
    if k > 0 as core::ffi::c_int {
        memcpy(
            &mut *selection_candidate_info.offset(0 as core::ffi::c_int as isize)
                as *mut ia_selection_candidate_info_struct as *mut core::ffi::c_void,
            &mut *selection_candidate_info_step_2.offset(0 as core::ffi::c_int as isize)
                as *mut ia_selection_candidate_info_struct as *const core::ffi::c_void,
            (k as size_t)
                .wrapping_mul(
                    ::core::mem::size_of::<ia_selection_candidate_info_struct>()
                        as size_t,
                ),
        );
        *selection_candidate_count = k;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_find_eq_set_no_compression(
    mut pstr_drc_config: *mut ia_drc_config,
    mut requested_dwnmix_id: WORD32,
    mut num_compression_eq_count: *mut WORD32,
    mut num_compression_eq_id: *mut WORD32,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut d: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut c: WORD32 = 0;
    k = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*pstr_drc_config).str_drc_config_ext.eq_instructions_count {
        let mut str_eq_instructions: *mut ia_eq_instructions_struct = &mut *((*pstr_drc_config)
            .str_drc_config_ext
            .str_eq_instructions)
            .as_mut_ptr()
            .offset(i as isize) as *mut ia_eq_instructions_struct;
        d = 0 as core::ffi::c_int as WORD32;
        while d < (*str_eq_instructions).dwnmix_id_count {
            if requested_dwnmix_id == (*str_eq_instructions).downmix_id[d as usize] {
                c = 0 as core::ffi::c_int as WORD32;
                while c < (*str_eq_instructions).drc_set_id_count {
                    if (*str_eq_instructions).drc_set_id[c as usize] == ID_FOR_ANY_DRC
                        || (*str_eq_instructions).drc_set_id[c as usize]
                            == 0 as core::ffi::c_int
                    {
                        if k >= MAX_NUM_COMPRESSION_EQ {
                            return UNEXPECTED_ERROR;
                        }
                        *num_compression_eq_id.offset(k as isize) = (*str_eq_instructions)
                            .eq_set_id;
                        k += 1;
                    }
                    c += 1;
                }
            }
            d += 1;
        }
        i += 1;
    }
    *num_compression_eq_count = k;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_select_drc_coeff3(
    mut drc_config: *mut ia_drc_config,
    mut str_p_loc_drc_coefficients_uni_drc: *mut *mut ia_uni_drc_coeffs_struct,
) -> VOID {
    let mut n: WORD32 = 0;
    let mut cV1: WORD32 = -(1 as WORD32);
    let mut cV0: WORD32 = -(1 as WORD32);
    n = 0 as core::ffi::c_int as WORD32;
    while n < (*drc_config).drc_coefficients_drc_count {
        if (*drc_config).str_p_loc_drc_coefficients_uni_drc[n as usize].drc_location
            == 1 as core::ffi::c_int
        {
            if (*drc_config).str_p_loc_drc_coefficients_uni_drc[n as usize].version
                == 0 as core::ffi::c_int
            {
                cV0 = n;
            } else {
                cV1 = n;
            }
        }
        n += 1;
    }
    if cV1 >= 0 as core::ffi::c_int {
        *str_p_loc_drc_coefficients_uni_drc = &mut *((*drc_config)
            .str_p_loc_drc_coefficients_uni_drc)
            .as_mut_ptr()
            .offset(cV1 as isize) as *mut ia_uni_drc_coeffs_struct;
    } else if cV0 >= 0 as core::ffi::c_int {
        *str_p_loc_drc_coefficients_uni_drc = &mut *((*drc_config)
            .str_p_loc_drc_coefficients_uni_drc)
            .as_mut_ptr()
            .offset(cV0 as isize) as *mut ia_uni_drc_coeffs_struct;
    } else {
        *str_p_loc_drc_coefficients_uni_drc = 0 as *mut ia_uni_drc_coeffs_struct;
    };
}
#[no_mangle]
pub unsafe extern "C" fn impd_manage_drc_complexity(
    mut pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
    mut pstr_drc_config: *mut ia_drc_config,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut err: WORD32 = 0;
    let mut channel_count: WORD32 = 0;
    let mut numBandsTooLarge: WORD32 = 0 as WORD32;
    let mut complexityDrcPrelim: FLOAT32 = 0.;
    let mut str_p_loc_drc_coefficients_uni_drc: *mut ia_uni_drc_coeffs_struct = 0
        as *mut ia_uni_drc_coeffs_struct;
    let mut complexitySupportedTotal: FLOAT32 = pow(
        2.0f64,
        (*pstr_drc_uni_sel_proc).compl_level_supported_total as core::ffi::c_double,
    ) as FLOAT32;
    let mut str_drc_instruction_str: *mut ia_drc_instructions_struct = 0
        as *mut ia_drc_instructions_struct;
    let mut drc_inst_uni_drc_dependent: *mut ia_drc_instructions_struct = 0
        as *mut ia_drc_instructions_struct;
    let mut uni_drc_sel_proc_output: *mut ia_drc_sel_proc_output_struct = &mut (*pstr_drc_uni_sel_proc)
        .uni_drc_sel_proc_output;
    let mut pstr_drc_sel_proc_params_struct: *mut ia_drc_sel_proc_params_struct = &mut (*pstr_drc_uni_sel_proc)
        .uni_drc_sel_proc_params;
    impd_select_drc_coeff3(pstr_drc_config, &mut str_p_loc_drc_coefficients_uni_drc);
    if str_p_loc_drc_coefficients_uni_drc.is_null() {
        return UNEXPECTED_ERROR;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*pstr_drc_config).drc_instructions_uni_drc_count {
        str_drc_instruction_str = &mut *((*pstr_drc_config).str_drc_instruction_str)
            .as_mut_ptr()
            .offset(i as isize) as *mut ia_drc_instructions_struct;
        if !((*str_drc_instruction_str).no_independent_use != 0) {
            numBandsTooLarge = 0 as core::ffi::c_int as WORD32;
            if (*str_drc_instruction_str).drc_apply_to_dwnmix == 1 as core::ffi::c_int {
                channel_count = (*uni_drc_sel_proc_output).target_channel_count;
            } else {
                channel_count = (*uni_drc_sel_proc_output).base_channel_count;
            }
            if (*pstr_drc_uni_sel_proc).subband_domain_mode == SUBBAND_DOMAIN_MODE_OFF {
                j = 0 as core::ffi::c_int as WORD32;
                while j < (*str_drc_instruction_str).num_drc_ch_groups {
                    let mut gain_set_params: *mut ia_gain_set_params_struct = &mut *((*str_p_loc_drc_coefficients_uni_drc)
                        .gain_set_params)
                        .as_mut_ptr()
                        .offset(
                            *((*str_drc_instruction_str)
                                .gain_set_index_for_channel_group)
                                .as_mut_ptr()
                                .offset(j as isize) as isize,
                        ) as *mut ia_gain_set_params_struct;
                    if (*gain_set_params).band_count
                        > (*pstr_drc_sel_proc_params_struct).num_bands_supported
                    {
                        numBandsTooLarge = 1 as core::ffi::c_int as WORD32;
                    } else {
                        (*gain_set_params).band_count > 4 as core::ffi::c_int;
                    }
                    j += 1;
                }
            }
            complexityDrcPrelim = (channel_count as core::ffi::c_int
                * ((1 as core::ffi::c_int)
                    << (*str_drc_instruction_str).drc_set_complexity_level)) as FLOAT32;
            if (*str_drc_instruction_str).depends_on_drc_set > 0 as core::ffi::c_int {
                err = impd_find_drc_instructions_uni_drc(
                    pstr_drc_config,
                    (*str_drc_instruction_str).depends_on_drc_set,
                    &mut drc_inst_uni_drc_dependent,
                );
                if err != 0 {
                    return err;
                }
                if (*drc_inst_uni_drc_dependent).drc_apply_to_dwnmix
                    == 1 as core::ffi::c_int
                {
                    channel_count = (*uni_drc_sel_proc_output).target_channel_count;
                } else {
                    channel_count = (*uni_drc_sel_proc_output).base_channel_count;
                }
                if (*pstr_drc_uni_sel_proc).subband_domain_mode
                    == SUBBAND_DOMAIN_MODE_OFF
                {
                    j = 0 as core::ffi::c_int as WORD32;
                    while j < (*str_drc_instruction_str).num_drc_ch_groups {
                        let mut gain_set_params_0: *mut ia_gain_set_params_struct = &mut *((*str_p_loc_drc_coefficients_uni_drc)
                            .gain_set_params)
                            .as_mut_ptr()
                            .offset(
                                *((*drc_inst_uni_drc_dependent)
                                    .gain_set_index_for_channel_group)
                                    .as_mut_ptr()
                                    .offset(j as isize) as isize,
                            ) as *mut ia_gain_set_params_struct;
                        if (*gain_set_params_0).band_count
                            > (*pstr_drc_sel_proc_params_struct).num_bands_supported
                        {
                            numBandsTooLarge = 1 as core::ffi::c_int as WORD32;
                        } else {
                            (*gain_set_params_0).band_count > 4 as core::ffi::c_int;
                        }
                        j += 1;
                    }
                }
                complexityDrcPrelim
                    += (channel_count as core::ffi::c_int
                        * ((1 as core::ffi::c_int)
                            << (*drc_inst_uni_drc_dependent).drc_set_complexity_level))
                        as FLOAT32;
            }
            complexityDrcPrelim
                *= (*pstr_drc_config).sampling_rate as core::ffi::c_float / 48000.0f32;
            if complexityDrcPrelim <= complexitySupportedTotal
                && numBandsTooLarge == 0 as core::ffi::c_int
            {
                (*pstr_drc_uni_sel_proc)
                    .drc_set_id_valid_flag[(*str_drc_instruction_str).drc_set_id
                    as usize] = 1 as core::ffi::c_int as WORD32;
            }
        }
        i += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_manage_eq_complexity(
    mut pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
    mut pstr_drc_config: *mut ia_drc_config,
) -> WORD32 {
    let mut k: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut m: WORD32 = 0;
    let mut err: WORD32 = 0;
    let mut eqComplexityPrimary: WORD32 = 0 as WORD32;
    let mut eqComplexityDependent: WORD32 = 0 as WORD32;
    let mut eqChannelCountPrimary: WORD32 = 0 as WORD32;
    let mut eqChannelCountDependent: WORD32 = 0 as WORD32;
    let mut complexityTotalEq: FLOAT32 = 0.;
    let mut drc_config: *mut ia_drc_config = &mut (*pstr_drc_uni_sel_proc).drc_config;
    let mut pstr_drc_sel_proc_params_struct: *mut ia_drc_sel_proc_params_struct = &mut (*pstr_drc_uni_sel_proc)
        .uni_drc_sel_proc_params;
    let mut complexitySupportedTotal: FLOAT32 = pow(
        2.0f64,
        (*pstr_drc_uni_sel_proc).compl_level_supported_total as core::ffi::c_double,
    ) as FLOAT32;
    n = 0 as core::ffi::c_int as WORD32;
    while n < (*drc_config).str_drc_config_ext.eq_instructions_count {
        let mut str_eq_instructions: *mut ia_eq_instructions_struct = &mut *((*pstr_drc_config)
            .str_drc_config_ext
            .str_eq_instructions)
            .as_mut_ptr()
            .offset(n as isize) as *mut ia_eq_instructions_struct;
        eqChannelCountPrimary = (*pstr_drc_sel_proc_params_struct).base_channel_count;
        eqChannelCountDependent = (*pstr_drc_sel_proc_params_struct).base_channel_count;
        eqComplexityPrimary = ((1 as core::ffi::c_int)
            << (*str_eq_instructions).eq_set_complexity_level) as WORD32;
        if (*pstr_drc_uni_sel_proc).subband_domain_mode == SUBBAND_DOMAIN_MODE_OFF {
            if (*str_eq_instructions).td_filter_cascade_present == 0 as core::ffi::c_int
            {
                eqComplexityPrimary = 0 as core::ffi::c_int as WORD32;
            }
        } else if (*str_eq_instructions).td_filter_cascade_present
            == 1 as core::ffi::c_int
        {
            eqComplexityPrimary = 2.5f32 as WORD32;
        }
        if (*str_eq_instructions).eq_apply_to_downmix == 1 as core::ffi::c_int {
            if (*str_eq_instructions).downmix_id[0 as core::ffi::c_int as usize]
                == ID_FOR_ANY_DOWNMIX
            {
                eqChannelCountPrimary = (*pstr_drc_sel_proc_params_struct)
                    .target_ch_count_prelim;
            } else {
                k = 0 as core::ffi::c_int as WORD32;
                while k < (*pstr_drc_config).dwnmix_instructions_count {
                    m = 0 as core::ffi::c_int as WORD32;
                    while m < (*str_eq_instructions).dwnmix_id_count {
                        if (*pstr_drc_config).dwnmix_instructions[k as usize].downmix_id
                            == (*str_eq_instructions).downmix_id[m as usize]
                        {
                            if eqChannelCountPrimary
                                > (*pstr_drc_config)
                                    .dwnmix_instructions[k as usize]
                                    .target_channel_count
                            {
                                eqChannelCountPrimary = (*pstr_drc_config)
                                    .dwnmix_instructions[k as usize]
                                    .target_channel_count;
                            }
                        }
                        m += 1;
                    }
                    k += 1;
                }
            }
        }
        complexityTotalEq = (eqChannelCountPrimary * eqComplexityPrimary) as FLOAT32;
        if (*str_eq_instructions).depends_on_eq_set_present > 0 as core::ffi::c_int {
            let mut eq_instructionsDependent: *mut ia_eq_instructions_struct = 0
                as *mut ia_eq_instructions_struct;
            err = impd_find_eq_instructions(
                drc_config,
                (*str_eq_instructions).depends_on_eq_set,
                &mut eq_instructionsDependent,
            );
            if err != 0 {
                return err;
            }
            eqComplexityDependent = ((1 as core::ffi::c_int)
                << (*eq_instructionsDependent).eq_set_complexity_level) as WORD32;
            if (*pstr_drc_uni_sel_proc).subband_domain_mode == SUBBAND_DOMAIN_MODE_OFF {
                if (*str_eq_instructions).td_filter_cascade_present
                    == 0 as core::ffi::c_int
                {
                    eqComplexityDependent = 0 as core::ffi::c_int as WORD32;
                }
            } else if (*str_eq_instructions).td_filter_cascade_present
                == 1 as core::ffi::c_int
            {
                eqComplexityDependent = 2.5f32 as WORD32;
            }
            if (*eq_instructionsDependent).eq_apply_to_downmix == 1 as core::ffi::c_int {
                if (*eq_instructionsDependent).downmix_id[0 as core::ffi::c_int as usize]
                    == ID_FOR_ANY_DOWNMIX
                {
                    eqChannelCountDependent = (*pstr_drc_sel_proc_params_struct)
                        .target_ch_count_prelim;
                } else {
                    k = 0 as core::ffi::c_int as WORD32;
                    while k < (*pstr_drc_config).dwnmix_instructions_count {
                        m = 0 as core::ffi::c_int as WORD32;
                        while m < (*str_eq_instructions).dwnmix_id_count {
                            if (*pstr_drc_config)
                                .dwnmix_instructions[k as usize]
                                .downmix_id
                                == (*eq_instructionsDependent).downmix_id[m as usize]
                            {
                                if eqChannelCountDependent
                                    > (*pstr_drc_config)
                                        .dwnmix_instructions[k as usize]
                                        .target_channel_count
                                {
                                    eqChannelCountDependent = (*pstr_drc_config)
                                        .dwnmix_instructions[k as usize]
                                        .target_channel_count;
                                }
                            }
                            m += 1;
                        }
                        k += 1;
                    }
                }
            }
            complexityTotalEq
                += (eqChannelCountDependent * eqComplexityDependent) as FLOAT32;
        }
        (*pstr_drc_uni_sel_proc)
            .eq_set_id_valid_flag[(*str_eq_instructions).eq_set_id as usize] = 0
            as core::ffi::c_int as WORD32;
        complexityTotalEq
            *= (*pstr_drc_config).sampling_rate as core::ffi::c_float / 48000.0f32;
        if complexityTotalEq <= complexitySupportedTotal {
            (*pstr_drc_uni_sel_proc)
                .eq_set_id_valid_flag[(*str_eq_instructions).eq_set_id as usize] = 1
                as core::ffi::c_int as WORD32;
        }
        n += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_manage_complexity(
    mut pstr_drc_uni_sel_proc: *mut ia_drc_sel_pro_struct,
    mut pstr_drc_config: *mut ia_drc_config,
    mut repeat_selection: *mut WORD32,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut p: WORD32 = 0;
    let mut err: WORD32 = 0;
    let mut channel_count: WORD32 = 0;
    let mut numBandsTooLarge: WORD32 = 0 as WORD32;
    let mut drcRequiresEq: WORD32 = 0;
    let mut complexityEq: FLOAT32 = 0.;
    let mut complexityDrcTotal: FLOAT32 = 0.0f32;
    let mut complexityEqTotal: FLOAT32 = 0.0f32;
    let mut freqNorm: FLOAT32 = (*pstr_drc_config).sampling_rate as FLOAT32 / 48000.0f32;
    let mut str_p_loc_drc_coefficients_uni_drc: *mut ia_uni_drc_coeffs_struct = 0
        as *mut ia_uni_drc_coeffs_struct;
    let mut str_drc_instruction_str: *mut ia_drc_instructions_struct = &mut *((*pstr_drc_config)
        .str_drc_instruction_str)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut ia_drc_instructions_struct;
    let mut uni_drc_sel_proc_output: *mut ia_drc_sel_proc_output_struct = &mut (*pstr_drc_uni_sel_proc)
        .uni_drc_sel_proc_output;
    let mut pstr_drc_sel_proc_params_struct: *mut ia_drc_sel_proc_params_struct = &mut (*pstr_drc_uni_sel_proc)
        .uni_drc_sel_proc_params;
    let mut complexitySupportedTotal: FLOAT32 = pow(
        2.0f64,
        (*pstr_drc_uni_sel_proc).compl_level_supported_total as core::ffi::c_double,
    ) as FLOAT32;
    impd_select_drc_coeff3(pstr_drc_config, &mut str_p_loc_drc_coefficients_uni_drc);
    if str_p_loc_drc_coefficients_uni_drc.is_null() {
        return UNEXPECTED_ERROR;
    }
    p = 0 as core::ffi::c_int as WORD32;
    while p < 4 as core::ffi::c_int {
        if !((*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output.sel_drc_set_ids[p as usize]
            <= 0 as core::ffi::c_int)
        {
            err = impd_find_drc_instructions_uni_drc(
                pstr_drc_config,
                (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_output
                    .sel_drc_set_ids[p as usize],
                &mut str_drc_instruction_str,
            );
            if err != 0 {
                return err;
            }
            if (*str_drc_instruction_str).drc_apply_to_dwnmix == 1 as core::ffi::c_int {
                channel_count = (*uni_drc_sel_proc_output).target_channel_count;
            } else {
                channel_count = (*uni_drc_sel_proc_output).base_channel_count;
            }
            if (*pstr_drc_uni_sel_proc).subband_domain_mode == SUBBAND_DOMAIN_MODE_OFF {
                j = 0 as core::ffi::c_int as WORD32;
                while j < (*str_drc_instruction_str).num_drc_ch_groups {
                    let mut gain_set_params: *mut ia_gain_set_params_struct = &mut *((*str_p_loc_drc_coefficients_uni_drc)
                        .gain_set_params)
                        .as_mut_ptr()
                        .offset(
                            *((*str_drc_instruction_str)
                                .gain_set_index_for_channel_group)
                                .as_mut_ptr()
                                .offset(j as isize) as isize,
                        ) as *mut ia_gain_set_params_struct;
                    if (*gain_set_params).band_count
                        > (*pstr_drc_sel_proc_params_struct).num_bands_supported
                    {
                        if p < 2 as core::ffi::c_int {
                            numBandsTooLarge = 1 as core::ffi::c_int as WORD32;
                        } else {
                            (*pstr_drc_uni_sel_proc)
                                .drc_set_id_valid_flag[(*str_drc_instruction_str).drc_set_id
                                as usize] = 0 as core::ffi::c_int as WORD32;
                            (*pstr_drc_uni_sel_proc)
                                .uni_drc_sel_proc_output
                                .sel_drc_set_ids[p as usize] = 0 as core::ffi::c_int
                                as WORD32;
                        }
                    } else {
                        (*gain_set_params).band_count > 4 as core::ffi::c_int;
                    }
                    j += 1;
                }
            }
            complexityDrcTotal
                += (channel_count as core::ffi::c_int
                    * ((1 as core::ffi::c_int)
                        << (*str_drc_instruction_str).drc_set_complexity_level))
                    as FLOAT32;
        }
        p += 1;
    }
    if (*uni_drc_sel_proc_output).active_downmix_id > 0 as core::ffi::c_int {
        let mut complexityPerCoeff: FLOAT32 = 0.;
        let mut dwnmix_instructions: *mut ia_downmix_instructions_struct = 0
            as *mut ia_downmix_instructions_struct;
        if (*pstr_drc_uni_sel_proc).subband_domain_mode == SUBBAND_DOMAIN_MODE_OFF {
            complexityPerCoeff = 1.0f32 as FLOAT32;
        } else {
            complexityPerCoeff = 2.0f32 as FLOAT32;
        }
        err = impd_find_downmix(
            pstr_drc_config,
            (*uni_drc_sel_proc_output).active_downmix_id,
            &mut dwnmix_instructions,
        );
        if err != 0 {
            return err;
        }
        if (*dwnmix_instructions).downmix_coefficients_present == 1 as core::ffi::c_int {
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*uni_drc_sel_proc_output).base_channel_count {
                j = 0 as core::ffi::c_int as WORD32;
                while j < (*uni_drc_sel_proc_output).target_channel_count {
                    if (*uni_drc_sel_proc_output).downmix_matrix[i as usize][j as usize]
                        != 0.0f32
                    {
                        complexityDrcTotal += complexityPerCoeff;
                    }
                    j += 1;
                }
                i += 1;
            }
        }
    }
    p = 0 as core::ffi::c_int as WORD32;
    while p < 2 as core::ffi::c_int {
        if (*pstr_drc_uni_sel_proc).eq_inst_index[p as usize] >= 0 as core::ffi::c_int {
            let mut str_eq_instructions: *mut ia_eq_instructions_struct = &mut *((*pstr_drc_config)
                .str_drc_config_ext
                .str_eq_instructions)
                .as_mut_ptr()
                .offset(
                    *((*pstr_drc_uni_sel_proc).eq_inst_index)
                        .as_mut_ptr()
                        .offset(p as isize) as isize,
                ) as *mut ia_eq_instructions_struct;
            if p == 0 as core::ffi::c_int {
                channel_count = (*uni_drc_sel_proc_output).base_channel_count;
            } else {
                channel_count = (*uni_drc_sel_proc_output).target_channel_count;
            }
            complexityEq = ((1 as core::ffi::c_int)
                << (*str_eq_instructions).eq_set_complexity_level) as FLOAT32;
            if (*pstr_drc_uni_sel_proc).subband_domain_mode == SUBBAND_DOMAIN_MODE_OFF {
                if (*str_eq_instructions).td_filter_cascade_present
                    == 0 as core::ffi::c_int
                {
                    complexityEq = 0.0f32;
                }
            } else if (*str_eq_instructions).td_filter_cascade_present
                == 1 as core::ffi::c_int
            {
                complexityEq = 2.5f32;
            }
            complexityEqTotal += channel_count as FLOAT32 * complexityEq;
        }
        p += 1;
    }
    complexityDrcTotal *= freqNorm;
    complexityEqTotal *= freqNorm;
    if complexityDrcTotal > complexitySupportedTotal
        || complexityEqTotal > complexitySupportedTotal
    {
        return UNEXPECTED_ERROR;
    }
    if numBandsTooLarge == 1 as core::ffi::c_int {
        if (*pstr_drc_uni_sel_proc)
            .uni_drc_sel_proc_output
            .sel_drc_set_ids[0 as core::ffi::c_int as usize] > 0 as core::ffi::c_int
        {
            err = impd_find_drc_instructions_uni_drc(
                pstr_drc_config,
                (*pstr_drc_uni_sel_proc)
                    .uni_drc_sel_proc_output
                    .sel_drc_set_ids[0 as core::ffi::c_int as usize],
                &mut str_drc_instruction_str,
            );
            if err != 0 {
                return err;
            }
            (*pstr_drc_uni_sel_proc)
                .drc_set_id_valid_flag[(*str_drc_instruction_str).drc_set_id as usize] = 0
                as core::ffi::c_int as WORD32;
        }
        *repeat_selection = 1 as core::ffi::c_int as WORD32;
    } else if complexityDrcTotal + complexityEqTotal <= complexitySupportedTotal {
        *repeat_selection = 0 as core::ffi::c_int as WORD32;
    } else {
        drcRequiresEq = 0 as core::ffi::c_int as WORD32;
        p = 0 as core::ffi::c_int as WORD32;
        while p < 2 as core::ffi::c_int {
            if !((*pstr_drc_uni_sel_proc)
                .uni_drc_sel_proc_output
                .sel_drc_set_ids[p as usize] <= 0 as core::ffi::c_int)
            {
                err = impd_find_drc_instructions_uni_drc(
                    pstr_drc_config,
                    (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_output
                        .sel_drc_set_ids[p as usize],
                    &mut str_drc_instruction_str,
                );
                if err != 0 {
                    return err;
                }
                if (*str_drc_instruction_str).requires_eq == 1 as core::ffi::c_int {
                    drcRequiresEq = 1 as core::ffi::c_int as WORD32;
                }
            }
            p += 1;
        }
        if drcRequiresEq == 0 as core::ffi::c_int
            && complexityDrcTotal <= complexitySupportedTotal
        {
            p = 0 as core::ffi::c_int as WORD32;
            while p < 2 as core::ffi::c_int {
                (*pstr_drc_uni_sel_proc).eq_inst_index[p as usize] = 0
                    as core::ffi::c_int as WORD32;
                p += 1;
            }
            *repeat_selection = 0 as core::ffi::c_int as WORD32;
        } else {
            if (*pstr_drc_uni_sel_proc)
                .uni_drc_sel_proc_output
                .sel_drc_set_ids[0 as core::ffi::c_int as usize] > 0 as core::ffi::c_int
            {
                err = impd_find_drc_instructions_uni_drc(
                    pstr_drc_config,
                    (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_output
                        .sel_drc_set_ids[0 as core::ffi::c_int as usize],
                    &mut str_drc_instruction_str,
                );
                if err != 0 {
                    return err;
                }
                (*pstr_drc_uni_sel_proc)
                    .drc_set_id_valid_flag[(*str_drc_instruction_str).drc_set_id
                    as usize] = 0 as core::ffi::c_int as WORD32;
            } else {
                p = 2 as core::ffi::c_int as WORD32;
                while p < 4 as core::ffi::c_int {
                    (*pstr_drc_uni_sel_proc)
                        .drc_set_id_valid_flag[(*str_drc_instruction_str).drc_set_id
                        as usize] = 0 as core::ffi::c_int as WORD32;
                    (*pstr_drc_uni_sel_proc)
                        .uni_drc_sel_proc_output
                        .sel_drc_set_ids[p as usize] = 0 as core::ffi::c_int as WORD32;
                    p += 1;
                }
            }
            *repeat_selection = 1 as core::ffi::c_int as WORD32;
        }
    }
    if *repeat_selection == 1 as core::ffi::c_int {
        memset(
            &mut (*pstr_drc_uni_sel_proc).uni_drc_sel_proc_output
                as *mut ia_drc_sel_proc_output_struct as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<ia_drc_sel_proc_output_struct>() as size_t,
        );
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_find_loud_eq_instructions_idx_for_id(
    mut drc_config: *mut ia_drc_config,
    mut loud_eq_set_id_requested: WORD32,
    mut instructions_idx: *mut WORD32,
) -> WORD32 {
    let mut i: WORD32 = 0;
    if loud_eq_set_id_requested > 0 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*drc_config).str_drc_config_ext.loud_eq_instructions_count {
            if (*drc_config)
                .str_drc_config_ext
                .loud_eq_instructions[i as usize]
                .loud_eq_set_id == loud_eq_set_id_requested
            {
                break;
            }
            i += 1;
        }
        if i == (*drc_config).str_drc_config_ext.loud_eq_instructions_count {
            return 2 as WORD32;
        }
        *instructions_idx = i;
    } else {
        *instructions_idx = -(1 as core::ffi::c_int) as WORD32;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_find_eq_instructions(
    mut drc_config: *mut ia_drc_config,
    mut eq_set_id_requested: WORD32,
    mut str_eq_instructions: *mut *mut ia_eq_instructions_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*drc_config).str_drc_config_ext.eq_instructions_count {
        if eq_set_id_requested
            == (*drc_config).str_drc_config_ext.str_eq_instructions[i as usize].eq_set_id
        {
            break;
        }
        i += 1;
    }
    if i == (*drc_config).str_drc_config_ext.eq_instructions_count {
        return 2 as WORD32;
    }
    *str_eq_instructions = &mut *((*drc_config).str_drc_config_ext.str_eq_instructions)
        .as_mut_ptr()
        .offset(i as isize) as *mut ia_eq_instructions_struct;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_find_downmix(
    mut drc_config: *mut ia_drc_config,
    mut requested_dwnmix_id: WORD32,
    mut dwnmix_instructions: *mut *mut ia_downmix_instructions_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*drc_config).dwnmix_instructions_count {
        if requested_dwnmix_id
            == (*drc_config).dwnmix_instructions[i as usize].downmix_id
        {
            break;
        }
        i += 1;
    }
    if i == (*drc_config).dwnmix_instructions_count {
        return 2 as WORD32;
    }
    *dwnmix_instructions = &mut *((*drc_config).dwnmix_instructions)
        .as_mut_ptr()
        .offset(i as isize) as *mut ia_downmix_instructions_struct;
    return 0 as WORD32;
}
