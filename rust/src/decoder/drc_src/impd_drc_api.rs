extern "C" {
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
    fn impd_drc_init(p_obj_drc: *mut ia_drc_api_struct) -> IA_ERRORCODE;
    fn impd_drc_set_default_config(p_obj_drc: *mut ia_drc_api_struct) -> IA_ERRORCODE;
    fn impd_drc_set_struct_pointer(p_obj_drc: *mut ia_drc_api_struct) -> IA_ERRORCODE;
    fn impd_process_time_domain(p_obj_drc: *mut ia_drc_api_struct) -> IA_ERRORCODE;
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type pWORD8 = *mut core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD32 = core::ffi::c_int;
pub type pWORD32 = *mut core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type pUWORD32 = *mut core::ffi::c_uint;
pub type FLOAT32 = core::ffi::c_float;
pub type FLOAT64 = core::ffi::c_double;
pub type pVOID = *mut core::ffi::c_void;
pub type LOOPIDX = core::ffi::c_int;
pub type IA_ERRORCODE = WORD32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mem_info_struct {
    pub ui_size: UWORD32,
    pub ui_alignment: UWORD32,
    pub ui_type: UWORD32,
    pub ui_placement: [UWORD32; 2],
    pub ui_priority: UWORD32,
    pub ui_placed: [UWORD32; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_bit_buf_struct {
    pub ptr_bit_buf_base: *mut UWORD8,
    pub ptr_bit_buf_end: *mut UWORD8,
    pub ptr_read_next: *mut UWORD8,
    pub bit_pos: WORD32,
    pub cnt_bits: WORD32,
    pub size: WORD32,
    pub error: WORD32,
}
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
pub struct ia_drc_params_bs_dec_struct {
    pub delta_tmin_default: WORD32,
    pub drc_frame_size: WORD32,
    pub num_gain_values_max_default: WORD32,
    pub delay_mode: WORD32,
    pub lfe_channel_map_count: WORD32,
    pub lfe_channel_map: [WORD32; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_bits_dec_struct {
    pub tables_default: ia_tables_struct,
    pub ia_drc_params_struct: ia_drc_params_bs_dec_struct,
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
pub struct ia_drc_qmf_filt_struct {
    pub ana_buff: *mut FLOAT64,
    pub syn_buff: *mut FLOAT64,
    pub ana_tab_real: [[FLOAT64; 128]; 64],
    pub ana_tab_imag: [[FLOAT64; 128]; 64],
    pub syn_tab_real: [[FLOAT64; 64]; 128],
    pub syn_tab_imag: [[FLOAT64; 64]; 128],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_peak_limiter_struct {
    pub attack_time: FLOAT32,
    pub release_time: FLOAT32,
    pub attack_constant: FLOAT32,
    pub release_constant: FLOAT32,
    pub limit_threshold: FLOAT32,
    pub num_channels: UWORD32,
    pub sample_rate: UWORD32,
    pub attack_time_samples: UWORD32,
    pub limiter_on: UWORD32,
    pub gain_modified: FLOAT32,
    pub pre_smoothed_gain: FLOAT64,
    pub delayed_input: *mut FLOAT32,
    pub delayed_input_index: UWORD32,
    pub max_buf: *mut FLOAT32,
    pub min_gain: FLOAT32,
    pub buffer: *mut FLOAT32,
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
pub struct ia_drc_config_struct {
    pub bitstream_file_format: WORD32,
    pub dec_type: WORD32,
    pub sub_band_domain_mode: WORD32,
    pub num_ch_in: WORD32,
    pub num_ch_out: WORD32,
    pub sampling_rate: WORD32,
    pub control_parameter_index: WORD32,
    pub delay_mode: WORD32,
    pub absorb_delay_on: WORD32,
    pub gain_delay_samples: WORD32,
    pub subband_domain_io_flag: WORD32,
    pub frame_size: WORD32,
    pub sub_band_down_sampling_factor: WORD32,
    pub sub_band_count: WORD32,
    pub peak_limiter: WORD32,
    pub interface_bitstream_present: WORD32,
    pub pcm_size: WORD32,
    pub parametric_drc_delay_gain_dec_instance: WORD32,
    pub parametric_drc_delay: WORD32,
    pub parametric_drc_delay_max: WORD32,
    pub eq_delay_gain_dec_instance: WORD32,
    pub eq_delay: WORD32,
    pub eq_delay_max: WORD32,
    pub delay_line_samples: WORD32,
    pub constant_delay_on: WORD32,
    pub audio_delay_samples: WORD32,
    pub effect_type: WORD32,
    pub target_loudness: WORD32,
    pub loud_norm_flag: WORD32,
    pub album_mode: WORD32,
    pub boost: FLOAT32,
    pub compress: FLOAT32,
    pub boost_set: UWORD8,
    pub compress_set: UWORD8,
    pub apply_crossfade: WORD32,
    pub is_config_changed: WORD32,
    pub ln_dbgain_prev: WORD32,
    pub loudness_leveling_flag: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bits_handler {
    pub bitstream_drc_config: *mut UWORD8,
    pub bitstream_loudness_info: *mut UWORD8,
    pub bitstream_unidrc_interface: *mut UWORD8,
    pub it_bit_buf: *mut UWORD8,
    pub num_bytes_bs_drc_config: WORD32,
    pub num_bytes_bs_loudness_info: WORD32,
    pub num_bits_read_bs_unidrc_interface: WORD32,
    pub num_bytes_bs_unidrc_interface: WORD32,
    pub num_bits_read_bs: WORD32,
    pub num_bytes_read_bs: WORD32,
    pub num_bytes_bs: WORD32,
    pub num_bytes_offset_bs: WORD32,
    pub num_total_bytes: WORD32,
    pub num_bits_offset_bs: WORD32,
    pub byte_index_bs: WORD32,
    pub num_byts_cur: WORD32,
    pub num_byts_cur_ic: WORD32,
    pub num_byts_cur_il: WORD32,
    pub num_byts_cur_in: WORD32,
    pub cpy_over: WORD32,
    pub cpy_over_ic: WORD32,
    pub cpy_over_il: WORD32,
    pub cpy_over_in: WORD32,
    pub gain_stream_flag: WORD32,
}
pub type ia_drc_bits_handler_struct = bits_handler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_payload_struct {
    pub pstr_bitstream_dec: *mut ia_drc_bits_dec_struct,
    pub pstr_gain_dec: [*mut ia_drc_gain_dec_struct; 2],
    pub pstr_selection_proc: *mut ia_drc_sel_pro_struct,
    pub pstr_drc_config: *mut ia_drc_config,
    pub pstr_loudness_info: *mut ia_drc_loudness_info_set_struct,
    pub pstr_drc_gain: *mut ia_drc_gain_struct,
    pub pstr_drc_interface: *mut ia_drc_interface_struct,
    pub pstr_peak_limiter: *mut ia_drc_peak_limiter_struct,
    pub pstr_qmf_filter: *mut ia_drc_qmf_filt_struct,
    pub pstr_drc_sel_proc_params: *mut ia_drc_sel_proc_params_struct,
    pub pstr_drc_sel_proc_output: *mut ia_drc_sel_proc_output_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_state_struct {
    pub ui_out_bytes: UWORD32,
    pub ui_in_bytes: UWORD32,
    pub ui_ir_bytes: UWORD32,
    pub total_num_out_samples: UWORD32,
    pub frame_no: UWORD32,
    pub out_size: UWORD32,
    pub ui_init_done: UWORD32,
    pub ui_exe_done: UWORD32,
    pub ui_ir_used: UWORD32,
    pub delay_in_output: WORD32,
    pub delay_adjust_samples: WORD32,
    pub persistent_ptr: pVOID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IA_PSM_API_Struct {
    pub p_state: *mut ia_drc_state_struct,
    pub str_config: ia_drc_config_struct,
    pub str_payload: ia_drc_payload_struct,
    pub str_bit_handler: ia_drc_bits_handler_struct,
    pub p_mem_info: *mut ia_mem_info_struct,
    pub pp_mem: *mut pVOID,
    pub str_bit_buf: ia_bit_buf_struct,
    pub pstr_bit_buf: *mut ia_bit_buf_struct,
    pub frame_count: WORD32,
}
pub type ia_drc_api_struct = IA_PSM_API_Struct;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const IA_FATAL_ERROR: core::ffi::c_uint = 0x80000000 as core::ffi::c_uint;
pub const IA_API_CMD_GET_LIB_ID_STRINGS: core::ffi::c_int = 1;
pub const IA_API_CMD_GET_API_SIZE: core::ffi::c_int = 2;
pub const IA_API_CMD_INIT: core::ffi::c_int = 3;
pub const IA_API_CMD_SET_CONFIG_PARAM: core::ffi::c_int = 4;
pub const IA_API_CMD_GET_CONFIG_PARAM: core::ffi::c_int = 5;
pub const IA_API_CMD_GET_MEMTABS_SIZE: core::ffi::c_int = 6;
pub const IA_API_CMD_SET_MEMTABS_PTR: core::ffi::c_int = 7;
pub const IA_API_CMD_GET_N_MEMTABS: core::ffi::c_int = 8;
pub const IA_API_CMD_EXECUTE: core::ffi::c_int = 9;
pub const IA_API_CMD_PUT_INPUT_QUERY: core::ffi::c_int = 10;
pub const IA_API_CMD_GET_CURIDX_INPUT_BUF: core::ffi::c_int = 11;
pub const IA_API_CMD_SET_INPUT_BYTES: core::ffi::c_int = 12;
pub const IA_API_CMD_GET_OUTPUT_BYTES: core::ffi::c_int = 13;
pub const IA_API_CMD_INPUT_OVER: core::ffi::c_int = 14;
pub const IA_API_CMD_GET_MEM_INFO_SIZE: core::ffi::c_int = 17;
pub const IA_API_CMD_GET_MEM_INFO_ALIGNMENT: core::ffi::c_int = 18;
pub const IA_API_CMD_GET_MEM_INFO_TYPE: core::ffi::c_int = 19;
pub const IA_API_CMD_GET_MEM_INFO_PLACEMENT: core::ffi::c_int = 20;
pub const IA_API_CMD_GET_MEM_INFO_PRIORITY: core::ffi::c_int = 21;
pub const IA_API_CMD_SET_MEM_PTR: core::ffi::c_int = 22;
pub const IA_API_CMD_SET_MEM_PLACEMENT: core::ffi::c_int = 24;
pub const IA_API_CMD_GET_N_TABLES: core::ffi::c_int = 25;
pub const IA_API_CMD_SET_INPUT_BYTES_BS: core::ffi::c_int = 38;
pub const IA_API_CMD_SET_INPUT_BYTES_IC_BS: core::ffi::c_int = 39;
pub const IA_API_CMD_SET_INPUT_BYTES_IL_BS: core::ffi::c_int = 41;
pub const IA_API_CMD_SET_INPUT_BYTES_IN_BS: core::ffi::c_int = 42;
pub const IA_CMD_TYPE_LIB_NAME: core::ffi::c_int = 256;
pub const IA_CMD_TYPE_LIB_VERSION: core::ffi::c_int = 512;
pub const IA_CMD_TYPE_API_VERSION: core::ffi::c_int = 768;
pub const IA_CMD_TYPE_INIT_API_PRE_CONFIG_PARAMS: core::ffi::c_int = 256;
pub const IA_CMD_TYPE_INIT_API_POST_CONFIG_PARAMS: core::ffi::c_int = 512;
pub const IA_CMD_TYPE_INIT_PROCESS: core::ffi::c_int = 768;
pub const IA_CMD_TYPE_INIT_DONE_QUERY: core::ffi::c_int = 1024;
pub const IA_CMD_TYPE_INIT_CPY_BSF_BUFF: core::ffi::c_int = 513;
pub const IA_CMD_TYPE_INIT_CPY_IC_BSF_BUFF: core::ffi::c_int = 514;
pub const IA_CMD_TYPE_INIT_CPY_IL_BSF_BUFF: core::ffi::c_int = 515;
pub const IA_CMD_TYPE_INIT_CPY_IN_BSF_BUFF: core::ffi::c_int = 517;
pub const IA_CMD_TYPE_INIT_CPY_BSF_BUFF_OVER_QUERY: core::ffi::c_int = 518;
pub const IA_CMD_TYPE_INIT_CPY_IC_BSF_BUFF_OVER_QUERY: core::ffi::c_int = 519;
pub const IA_CMD_TYPE_INIT_CPY_IL_BSF_BUFF_OVER_QUERY: core::ffi::c_int = 520;
pub const IA_CMD_TYPE_INIT_CPY_IN_BSF_BUFF_OVER_QUERY: core::ffi::c_int = 522;
pub const IA_CMD_TYPE_INIT_SET_BUFF_PTR: core::ffi::c_int = 523;
pub const IA_CMD_TYPE_DO_EXECUTE: core::ffi::c_int = 256;
pub const IA_CMD_TYPE_DONE_QUERY: core::ffi::c_int = 512;
pub const IA_MEMTYPE_PERSIST: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_MEMTYPE_SCRATCH: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const IA_MEMTYPE_INPUT: core::ffi::c_int = 0x2 as core::ffi::c_int;
pub const IA_MEMTYPE_OUTPUT: core::ffi::c_int = 0x3 as core::ffi::c_int;
pub const IA_MEMPRIORITY_ANYWHERE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const MAX_CHANNEL_COUNT: core::ffi::c_int = 8 as core::ffi::c_int;
pub const SEL_DRC_COUNT: core::ffi::c_int = 3 as core::ffi::c_int;
pub const NUM_ELE_IN_CPLX_NUM: core::ffi::c_int = 2 as core::ffi::c_int;
pub const NUM_GAIN_DEC_INSTANCES: core::ffi::c_int = 2 as core::ffi::c_int;
pub const IA_DRC_DEC_CONFIG_PARAM_PCM_WDSZ: core::ffi::c_int = 0;
pub const IA_DRC_DEC_CONFIG_PARAM_SAMP_FREQ: core::ffi::c_int = 1;
pub const IA_DRC_DEC_CONFIG_PARAM_NUM_CHANNELS: core::ffi::c_int = 2;
pub const IA_DRC_DEC_CONFIG_PARAM_BITS_FORMAT: core::ffi::c_int = 7;
pub const IA_DRC_DEC_CONFIG_PARAM_INT_PRESENT: core::ffi::c_int = 8;
pub const IA_DRC_DEC_CONFIG_PARAM_FRAME_SIZE: core::ffi::c_int = 14;
pub const IA_DRC_DEC_CONFIG_PROC_OUT_PTR: core::ffi::c_int = 15;
pub const IA_DRC_DEC_CONFIG_GAIN_STREAM_FLAG: core::ffi::c_int = 16;
pub const IA_DRC_DEC_CONFIG_DRC_EFFECT_TYPE: core::ffi::c_int = 17;
pub const IA_DRC_DEC_CONFIG_DRC_TARGET_LOUDNESS: core::ffi::c_int = 18;
pub const IA_DRC_DEC_CONFIG_DRC_LOUD_NORM: core::ffi::c_int = 19;
pub const IA_DRC_DEC_CONFIG_DRC_ALBUM_MODE: core::ffi::c_int = 20;
pub const IA_DRC_DEC_CONFIG_DRC_BOOST: core::ffi::c_int = 21;
pub const IA_DRC_DEC_CONFIG_DRC_CUT: core::ffi::c_int = 22;
pub const IA_DRC_DEC_CONFIG_PARAM_APPLY_CROSSFADE: core::ffi::c_int = 23;
pub const IA_DRC_DEC_CONFIG_PARAM_CONFIG_CHANGED: core::ffi::c_int = 24;
pub const IA_DRC_DEC_CONFIG_DRC_LOUDNESS_LEVELING: core::ffi::c_int = 25;
pub const IA_API_STR_LEN: core::ffi::c_int = 30 as core::ffi::c_int;
pub const LIBNAME: [WORD8; 7] = unsafe {
    ::core::mem::transmute::<[u8; 7], [WORD8; 7]>(*b"IA_DRC\0")
};
pub const IA_DRC_PERSIST_IDX: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_DRC_SCRATCH_IDX: core::ffi::c_int = 1 as core::ffi::c_int;
pub const IA_DRC_INPUT_IDX: core::ffi::c_int = 2 as core::ffi::c_int;
pub const IA_DRC_OUTPUT_IDX: core::ffi::c_int = 3 as core::ffi::c_int;
pub const DEC_TYPE_TD: core::ffi::c_int = 0 as core::ffi::c_int;
pub const DEC_TYPE_TD_QMF64: core::ffi::c_int = 1 as core::ffi::c_int;
pub const DEC_TYPE_QMF64: core::ffi::c_int = 2 as core::ffi::c_int;
pub const DEC_TYPE_STFT256: core::ffi::c_int = 3 as core::ffi::c_int;
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_DRC_DEC_API_FATAL_MEM_ALLOC: core::ffi::c_uint = 0xffff8002
    as core::ffi::c_uint;
pub const IA_DRC_DEC_CONFIG_NON_FATAL_INVALID_NUM_OF_CHANNELS: core::ffi::c_int = 0x800
    as core::ffi::c_int;
pub const IA_DRC_DEC_CONFIG_NON_FATAL_INVALID_SAMP_FREQ: core::ffi::c_int = 0x801
    as core::ffi::c_int;
pub const IA_DRC_DEC_CONFIG_NON_FATAL_INVALID_PCM_SIZE: core::ffi::c_int = 0x802
    as core::ffi::c_int;
pub const IA_DRC_DEC_CONFIG_NON_FATAL_INVALID_FRAME_SIZE: core::ffi::c_int = 0x803
    as core::ffi::c_int;
pub const NUM_DRC_TABLES: core::ffi::c_int = 4 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ia_drc_dec_api(
    mut p_ia_drc_dec_obj: pVOID,
    mut i_cmd: WORD32,
    mut i_idx: WORD32,
    mut pv_value: pVOID,
) -> IA_ERRORCODE {
    let mut p_obj_drc: *mut ia_drc_api_struct = p_ia_drc_dec_obj
        as *mut ia_drc_api_struct;
    let mut error_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut i: LOOPIDX = 0;
    let mut pui_value: pUWORD32 = pv_value as pUWORD32;
    let mut pus_value: pUWORD32 = pv_value as pUWORD32;
    let mut pb_value: pWORD8 = pv_value as pWORD8;
    let mut ps_value: *mut size_t = pv_value as *mut size_t;
    let mut pi_value: pWORD32 = pv_value as pWORD32;
    match i_cmd {
        IA_API_CMD_GET_MEM_INFO_SIZE
        | IA_API_CMD_GET_MEM_INFO_ALIGNMENT
        | IA_API_CMD_GET_MEM_INFO_TYPE
        | IA_API_CMD_GET_MEM_INFO_PLACEMENT
        | IA_API_CMD_GET_MEM_INFO_PRIORITY
        | IA_API_CMD_SET_MEM_PTR
        | IA_API_CMD_SET_MEM_PLACEMENT => {
            return impd_drc_mem_api(
                p_ia_drc_dec_obj as *mut ia_drc_api_struct,
                i_cmd,
                i_idx,
                pv_value,
            );
        }
        _ => {}
    }
    match i_cmd {
        IA_API_CMD_GET_LIB_ID_STRINGS => {
            match i_idx {
                IA_CMD_TYPE_LIB_NAME => {
                    let mut lib_name: [WORD8; 7] = LIBNAME;
                    i = 0 as core::ffi::c_int as LOOPIDX;
                    while i < IA_API_STR_LEN
                        && lib_name[(i as core::ffi::c_int - 1 as core::ffi::c_int)
                            as usize] as core::ffi::c_int != 0 as core::ffi::c_int
                    {
                        *pb_value.offset(i as isize) = lib_name[i as usize]
                            as core::ffi::c_schar;
                        i += 1;
                    }
                }
                IA_CMD_TYPE_LIB_VERSION => {}
                IA_CMD_TYPE_API_VERSION | _ => return -(1 as IA_ERRORCODE),
            }
        }
        IA_API_CMD_GET_API_SIZE => {
            *pui_value = ::core::mem::size_of::<ia_drc_api_struct>()
                as core::ffi::c_uint;
        }
        IA_API_CMD_INIT => {
            match i_idx {
                IA_CMD_TYPE_INIT_SET_BUFF_PTR => {
                    (*(*p_obj_drc).p_state).persistent_ptr = (*((*p_obj_drc).pp_mem)
                        .offset(IA_DRC_PERSIST_IDX as isize) as *mut UWORD8)
                        .offset(
                            ((::core::mem::size_of::<ia_drc_state_struct>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                                as isize,
                        ) as pVOID;
                    error_code = impd_drc_set_struct_pointer(p_obj_drc);
                    if error_code != 0 {
                        return error_code;
                    }
                }
                IA_CMD_TYPE_INIT_API_PRE_CONFIG_PARAMS => {
                    error_code = impd_drc_set_default_config(p_obj_drc);
                    if error_code != 0 {
                        return error_code;
                    }
                }
                IA_CMD_TYPE_INIT_API_POST_CONFIG_PARAMS => {
                    error_code = impd_drc_fill_mem_tables(p_obj_drc);
                    if error_code != 0 {
                        return error_code;
                    }
                }
                IA_CMD_TYPE_INIT_PROCESS => {
                    let mut Error: IA_ERRORCODE = 0 as IA_ERRORCODE;
                    if (*((*p_obj_drc).pp_mem).offset(IA_DRC_PERSIST_IDX as isize))
                        .is_null()
                    {
                        return -(1 as IA_ERRORCODE);
                    }
                    Error = impd_drc_init(p_obj_drc);
                    if Error != 0 {
                        return Error;
                    }
                    (*(*p_obj_drc).p_state).ui_init_done = 1 as UWORD32;
                    return Error;
                }
                IA_CMD_TYPE_INIT_DONE_QUERY => {
                    if (*(*p_obj_drc).p_state).ui_init_done == 1 as UWORD32 {
                        *pui_value = 1 as core::ffi::c_uint;
                    } else {
                        *pui_value = 0 as core::ffi::c_uint;
                    }
                }
                IA_CMD_TYPE_INIT_CPY_BSF_BUFF_OVER_QUERY => {
                    *pui_value = (*p_obj_drc).str_bit_handler.cpy_over
                        as core::ffi::c_uint;
                }
                IA_CMD_TYPE_INIT_CPY_IC_BSF_BUFF_OVER_QUERY => {
                    *pui_value = (*p_obj_drc).str_bit_handler.cpy_over_ic
                        as core::ffi::c_uint;
                }
                IA_CMD_TYPE_INIT_CPY_IL_BSF_BUFF_OVER_QUERY => {
                    *pui_value = (*p_obj_drc).str_bit_handler.cpy_over_il
                        as core::ffi::c_uint;
                }
                IA_CMD_TYPE_INIT_CPY_IN_BSF_BUFF_OVER_QUERY => {
                    *pui_value = (*p_obj_drc).str_bit_handler.cpy_over_in
                        as core::ffi::c_uint;
                }
                IA_CMD_TYPE_INIT_CPY_BSF_BUFF => {
                    memcpy(
                        ((*p_obj_drc).str_bit_handler.it_bit_buf)
                            .offset(
                                (*p_obj_drc).str_bit_handler.num_bytes_offset_bs as isize,
                            ) as *mut core::ffi::c_void,
                        *((*p_obj_drc).pp_mem).offset(2 as core::ffi::c_int as isize)
                            as *const core::ffi::c_void,
                        (*p_obj_drc).str_bit_handler.num_byts_cur as size_t,
                    );
                    (*p_obj_drc).str_bit_handler.num_bytes_bs = (*p_obj_drc)
                        .str_bit_handler
                        .num_bytes_bs + (*p_obj_drc).str_bit_handler.num_byts_cur;
                    (*p_obj_drc).str_bit_handler.num_bytes_offset_bs = (*p_obj_drc)
                        .str_bit_handler
                        .num_bytes_bs;
                    (*p_obj_drc).str_bit_handler.num_total_bytes = (*p_obj_drc)
                        .str_bit_handler
                        .num_bytes_bs;
                }
                IA_CMD_TYPE_INIT_CPY_IC_BSF_BUFF => {
                    memcpy(
                        (*p_obj_drc).str_bit_handler.bitstream_drc_config
                            as *mut core::ffi::c_void,
                        *((*p_obj_drc).pp_mem).offset(2 as core::ffi::c_int as isize)
                            as *const core::ffi::c_void,
                        (*p_obj_drc).str_bit_handler.num_byts_cur_ic as size_t,
                    );
                    (*p_obj_drc).str_bit_handler.num_bytes_bs_drc_config = (*p_obj_drc)
                        .str_bit_handler
                        .num_byts_cur_ic;
                }
                IA_CMD_TYPE_INIT_CPY_IL_BSF_BUFF => {
                    memcpy(
                        (*p_obj_drc).str_bit_handler.bitstream_loudness_info
                            as *mut core::ffi::c_void,
                        *((*p_obj_drc).pp_mem).offset(2 as core::ffi::c_int as isize)
                            as *const core::ffi::c_void,
                        (*p_obj_drc).str_bit_handler.num_byts_cur_il as size_t,
                    );
                    (*p_obj_drc).str_bit_handler.num_bytes_bs_loudness_info = (*p_obj_drc)
                        .str_bit_handler
                        .num_byts_cur_il;
                }
                IA_CMD_TYPE_INIT_CPY_IN_BSF_BUFF => {
                    memcpy(
                        (*p_obj_drc).str_bit_handler.bitstream_unidrc_interface
                            as *mut core::ffi::c_void,
                        *((*p_obj_drc).pp_mem).offset(2 as core::ffi::c_int as isize)
                            as *const core::ffi::c_void,
                        (*p_obj_drc).str_bit_handler.num_byts_cur_in as size_t,
                    );
                    (*p_obj_drc).str_bit_handler.num_bytes_bs_unidrc_interface = (*p_obj_drc)
                        .str_bit_handler
                        .num_byts_cur_in;
                }
                _ => return -(1 as IA_ERRORCODE),
            }
        }
        IA_API_CMD_GET_CONFIG_PARAM => {
            match i_idx {
                IA_DRC_DEC_CONFIG_PARAM_SAMP_FREQ => {
                    *pus_value = (*p_obj_drc).str_config.sampling_rate
                        as core::ffi::c_uint;
                }
                IA_DRC_DEC_CONFIG_PARAM_NUM_CHANNELS => {
                    *pus_value = (*p_obj_drc).str_config.num_ch_out as core::ffi::c_uint;
                }
                IA_DRC_DEC_CONFIG_PROC_OUT_PTR => {
                    *ps_value = (*p_obj_drc).str_payload.pstr_drc_sel_proc_output
                        as size_t;
                }
                IA_DRC_DEC_CONFIG_DRC_TARGET_LOUDNESS => {
                    *pi_value = (*((*((*(*p_obj_drc).str_payload.pstr_loudness_info)
                        .loudness_info)
                        .as_mut_ptr())
                        .loudness_measure)
                        .as_mut_ptr())
                        .method_val as WORD32 as core::ffi::c_int;
                    if *pi_value < -(1 as core::ffi::c_int) {
                        *pi_value = *pi_value * -(4 as core::ffi::c_int);
                    } else {
                        *pi_value = -(1 as core::ffi::c_int);
                    }
                }
                _ => {}
            }
        }
        IA_API_CMD_SET_CONFIG_PARAM => {
            match i_idx {
                IA_DRC_DEC_CONFIG_PARAM_SAMP_FREQ => {
                    if *pus_value == 0 as core::ffi::c_uint
                        || *pus_value > 96000 as core::ffi::c_uint
                    {
                        return IA_DRC_DEC_CONFIG_NON_FATAL_INVALID_SAMP_FREQ;
                    }
                    (*p_obj_drc).str_config.sampling_rate = *pus_value as WORD32;
                }
                IA_DRC_DEC_CONFIG_PARAM_APPLY_CROSSFADE => {
                    (*p_obj_drc).str_config.apply_crossfade = *pus_value as WORD32;
                }
                IA_DRC_DEC_CONFIG_PARAM_CONFIG_CHANGED => {
                    (*p_obj_drc).str_config.is_config_changed = *pus_value as WORD32;
                }
                IA_DRC_DEC_CONFIG_PARAM_NUM_CHANNELS => {
                    (*p_obj_drc).str_config.num_ch_in = *pus_value as WORD32;
                    if *pus_value < 1 as core::ffi::c_uint
                        || *pus_value > MAX_CHANNEL_COUNT as core::ffi::c_uint
                    {
                        return IA_DRC_DEC_CONFIG_NON_FATAL_INVALID_NUM_OF_CHANNELS;
                    }
                }
                IA_DRC_DEC_CONFIG_PARAM_PCM_WDSZ => {
                    match *pus_value {
                        16 | 24 | 32 => {}
                        _ => return IA_DRC_DEC_CONFIG_NON_FATAL_INVALID_PCM_SIZE,
                    }
                    (*p_obj_drc).str_config.pcm_size = *pus_value as WORD32;
                }
                IA_DRC_DEC_CONFIG_PARAM_FRAME_SIZE => {
                    if *pus_value < 1 as core::ffi::c_uint
                        || *pus_value > 4096 as core::ffi::c_uint
                    {
                        return IA_DRC_DEC_CONFIG_NON_FATAL_INVALID_FRAME_SIZE;
                    }
                    (*p_obj_drc).str_config.frame_size = *pus_value as WORD32;
                }
                IA_DRC_DEC_CONFIG_PARAM_BITS_FORMAT
                | IA_DRC_DEC_CONFIG_PARAM_INT_PRESENT
                | IA_DRC_DEC_CONFIG_GAIN_STREAM_FLAG => {}
                IA_DRC_DEC_CONFIG_DRC_EFFECT_TYPE => {
                    (*p_obj_drc).str_config.effect_type = *pus_value as WORD32;
                }
                IA_DRC_DEC_CONFIG_DRC_TARGET_LOUDNESS => {
                    (*p_obj_drc).str_config.target_loudness = *pus_value as WORD32;
                }
                IA_DRC_DEC_CONFIG_DRC_LOUD_NORM => {
                    (*p_obj_drc).str_config.loud_norm_flag = *pus_value as WORD32;
                }
                IA_DRC_DEC_CONFIG_DRC_ALBUM_MODE => {
                    (*p_obj_drc).str_config.album_mode = *pus_value as WORD32;
                }
                IA_DRC_DEC_CONFIG_DRC_BOOST => {
                    (*p_obj_drc).str_config.boost = (*pus_value as core::ffi::c_float
                        / 100.0f32) as FLOAT32;
                    (*p_obj_drc).str_config.boost_set = 1 as UWORD8;
                }
                IA_DRC_DEC_CONFIG_DRC_CUT => {
                    (*p_obj_drc).str_config.compress = (*pus_value as core::ffi::c_float
                        / 100.0f32) as FLOAT32;
                    (*p_obj_drc).str_config.compress_set = 1 as UWORD8;
                }
                IA_DRC_DEC_CONFIG_DRC_LOUDNESS_LEVELING => {
                    (*p_obj_drc).str_config.loudness_leveling_flag = *pus_value
                        as WORD32;
                }
                _ => return -(1 as IA_ERRORCODE),
            }
        }
        IA_API_CMD_GET_MEMTABS_SIZE => {
            *pui_value = (::core::mem::size_of::<ia_mem_info_struct>() as usize)
                .wrapping_add(::core::mem::size_of::<*mut pVOID>() as usize)
                .wrapping_mul(4 as usize) as core::ffi::c_uint;
        }
        IA_API_CMD_SET_MEMTABS_PTR => {
            if ps_value.is_null() {
                return IA_DRC_DEC_API_FATAL_MEM_ALLOC as IA_ERRORCODE;
            }
            memset(
                ps_value as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (::core::mem::size_of::<ia_mem_info_struct>() as size_t)
                    .wrapping_add(::core::mem::size_of::<*mut pVOID>() as size_t)
                    .wrapping_mul(4 as size_t),
            );
            (*p_obj_drc).p_mem_info = ps_value as *mut ia_mem_info_struct;
            (*p_obj_drc).pp_mem = ((*p_obj_drc).p_mem_info as size_t)
                .wrapping_add(
                    (NUM_DRC_TABLES as size_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<ia_mem_info_struct>() as size_t,
                        ),
                ) as pVOID as *mut pVOID;
        }
        IA_API_CMD_GET_N_MEMTABS => {
            *pui_value = NUM_DRC_TABLES as core::ffi::c_uint;
        }
        IA_API_CMD_GET_N_TABLES => {}
        IA_API_CMD_EXECUTE => {
            match i_idx {
                IA_CMD_TYPE_DO_EXECUTE => {
                    if (*(*p_obj_drc).p_state).ui_init_done == 0 {
                        error_code = IA_FATAL_ERROR as IA_ERRORCODE;
                    } else if (*p_obj_drc).str_config.dec_type == DEC_TYPE_TD {
                        error_code = impd_process_time_domain(p_obj_drc);
                    } else if (*p_obj_drc).str_config.dec_type == DEC_TYPE_QMF64 {
                        error_code = IA_FATAL_ERROR as IA_ERRORCODE;
                    } else if (*p_obj_drc).str_config.dec_type == DEC_TYPE_STFT256 {
                        error_code = IA_FATAL_ERROR as IA_ERRORCODE;
                    } else if (*p_obj_drc).str_config.dec_type == DEC_TYPE_TD_QMF64 {
                        error_code = IA_FATAL_ERROR as IA_ERRORCODE;
                    }
                    (*p_obj_drc).str_bit_handler.byte_index_bs = (*p_obj_drc)
                        .str_bit_handler
                        .num_total_bytes - (*p_obj_drc).str_bit_handler.num_bytes_bs;
                    (*p_obj_drc).str_bit_handler.num_bytes_offset_bs = 0
                        as core::ffi::c_int as WORD32;
                }
                IA_CMD_TYPE_DONE_QUERY => {
                    *pui_value = (*(*p_obj_drc).p_state).ui_exe_done
                        as core::ffi::c_uint;
                }
                _ => return -(1 as IA_ERRORCODE),
            }
        }
        IA_API_CMD_PUT_INPUT_QUERY => {
            *pui_value = 1 as core::ffi::c_uint;
        }
        IA_API_CMD_GET_CURIDX_INPUT_BUF => {
            let mut ui_in_buf_size: UWORD32 = (*((*p_obj_drc).p_mem_info)
                .offset(IA_DRC_INPUT_IDX as isize))
                .ui_size;
            let mut ui_in_bytes: UWORD32 = (*(*p_obj_drc).p_state).ui_in_bytes;
            *pui_value = (if ui_in_buf_size > ui_in_bytes {
                ui_in_bytes
            } else {
                ui_in_buf_size
            }) as core::ffi::c_uint;
        }
        IA_API_CMD_SET_INPUT_BYTES => {
            (*(*p_obj_drc).p_state).ui_in_bytes = *pui_value as UWORD32;
        }
        IA_API_CMD_GET_OUTPUT_BYTES => {
            *pui_value = (*(*p_obj_drc).p_state).ui_out_bytes as core::ffi::c_uint;
        }
        IA_API_CMD_INPUT_OVER => {
            (*(*p_obj_drc).p_state).ui_exe_done = 1 as UWORD32;
        }
        IA_API_CMD_SET_INPUT_BYTES_BS => {
            (*p_obj_drc).str_bit_handler.num_byts_cur = *pus_value as WORD32;
        }
        IA_API_CMD_SET_INPUT_BYTES_IC_BS => {
            (*p_obj_drc).str_bit_handler.num_byts_cur_ic = *pus_value as WORD32;
        }
        IA_API_CMD_SET_INPUT_BYTES_IL_BS => {
            (*p_obj_drc).str_bit_handler.num_byts_cur_il = *pus_value as WORD32;
        }
        IA_API_CMD_SET_INPUT_BYTES_IN_BS => {
            (*p_obj_drc).str_bit_handler.num_byts_cur_in = *pus_value as WORD32;
        }
        _ => return -(1 as IA_ERRORCODE),
    }
    return error_code;
}
#[no_mangle]
pub unsafe extern "C" fn impd_drc_mem_api(
    mut p_obj_drc: *mut ia_drc_api_struct,
    mut i_cmd: WORD32,
    mut i_idx: WORD32,
    mut pv_value: pVOID,
) -> IA_ERRORCODE {
    let mut pui_value: pUWORD32 = pv_value as pUWORD32;
    match i_cmd {
        IA_API_CMD_GET_MEM_INFO_SIZE => {
            *pui_value = (*((*p_obj_drc).p_mem_info).offset(i_idx as isize)).ui_size
                as core::ffi::c_uint;
        }
        IA_API_CMD_GET_MEM_INFO_ALIGNMENT => {
            *pui_value = (*((*p_obj_drc).p_mem_info).offset(i_idx as isize)).ui_alignment
                as core::ffi::c_uint;
        }
        IA_API_CMD_GET_MEM_INFO_TYPE => {
            *pui_value = (*((*p_obj_drc).p_mem_info).offset(i_idx as isize)).ui_type
                as core::ffi::c_uint;
        }
        IA_API_CMD_GET_MEM_INFO_PLACEMENT => {
            *pui_value = (*((*p_obj_drc).p_mem_info).offset(i_idx as isize))
                .ui_placement[0 as core::ffi::c_int as usize] as core::ffi::c_uint;
            *pui_value.offset(1 as core::ffi::c_int as isize) = (*((*p_obj_drc)
                .p_mem_info)
                .offset(i_idx as isize))
                .ui_placement[1 as core::ffi::c_int as usize] as core::ffi::c_uint;
        }
        IA_API_CMD_GET_MEM_INFO_PRIORITY => {
            *pui_value = (*((*p_obj_drc).p_mem_info).offset(i_idx as isize)).ui_priority
                as core::ffi::c_uint;
        }
        IA_API_CMD_SET_MEM_PTR => {
            if pv_value.is_null() {
                return -(1 as IA_ERRORCODE);
            }
            if (pv_value as size_t)
                .wrapping_rem(
                    (*((*p_obj_drc).p_mem_info).offset(i_idx as isize)).ui_alignment
                        as size_t,
                ) != 0 as size_t
            {
                return -(1 as IA_ERRORCODE);
            }
            let ref mut fresh0 = *((*p_obj_drc).pp_mem).offset(i_idx as isize);
            *fresh0 = pv_value;
            memset(
                *((*p_obj_drc).pp_mem).offset(i_idx as isize),
                0 as core::ffi::c_int,
                (*((*p_obj_drc).p_mem_info).offset(i_idx as isize)).ui_size as size_t,
            );
            if IA_MEMTYPE_PERSIST == i_idx {
                (*p_obj_drc).p_state = pv_value as *mut ia_drc_state_struct;
            }
        }
        IA_API_CMD_SET_MEM_PLACEMENT | _ => {}
    }
    return IA_NO_ERROR;
}
unsafe extern "C" fn impd_calc_scratch_size() -> WORD32 {
    return (((4096 as core::ffi::c_int * 8 as core::ffi::c_int * 2 as core::ffi::c_int)
        as usize)
        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize)
        .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize) as WORD32;
}
unsafe extern "C" fn impd_calc_pers_size() -> WORD32 {
    let mut size: WORD32 = 0 as WORD32;
    let mut analysis_buf_size: WORD32 = (((4096 as core::ffi::c_int
        * 8 as core::ffi::c_int) as usize)
        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize)
        .wrapping_mul(2 as usize)
        .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize) as WORD32;
    let mut synth_buf_size: WORD32 = (((4096 as core::ffi::c_int * 8 as core::ffi::c_int)
        as usize)
        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize)
        .wrapping_mul(2 as usize)
        .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize) as WORD32;
    let mut peak_lim_buf_size: WORD32 = (((4096 as core::ffi::c_int
        * 8 as core::ffi::c_int) as usize)
        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize)
        .wrapping_mul(2 as usize)
        .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize) as WORD32;
    let mut subband_buf_size: WORD32 = (NUM_ELE_IN_CPLX_NUM as usize)
        .wrapping_mul(
            (8 as usize)
                .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
        )
        .wrapping_add(
            ((NUM_ELE_IN_CPLX_NUM * MAX_CHANNEL_COUNT) as usize)
                .wrapping_mul(
                    ((4500 as core::ffi::c_int / 64 as core::ffi::c_int
                        + 4096 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ),
        ) as WORD32;
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            ((::core::mem::size_of::<ia_drc_state_struct>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            ((::core::mem::size_of::<ia_drc_bits_dec_struct>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            (2 as usize)
                .wrapping_mul(
                    (::core::mem::size_of::<ia_drc_gain_dec_struct>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            ((::core::mem::size_of::<ia_drc_loudness_info_set_struct>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            ((::core::mem::size_of::<ia_drc_gain_struct>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            ((::core::mem::size_of::<ia_drc_interface_struct>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            ((::core::mem::size_of::<ia_drc_config>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            ((::core::mem::size_of::<ia_drc_sel_pro_struct>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            ((::core::mem::size_of::<ia_drc_sel_proc_params_struct>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            ((::core::mem::size_of::<ia_drc_sel_proc_output_struct>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            ((::core::mem::size_of::<ia_drc_peak_limiter_struct>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            ((::core::mem::size_of::<ia_drc_qmf_filt_struct>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size += analysis_buf_size;
    size += synth_buf_size;
    size += peak_lim_buf_size;
    size
        += 768 as core::ffi::c_int * 3 as core::ffi::c_int
            + (8 as core::ffi::c_int - 1 as core::ffi::c_int)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int);
    size
        += 768 as core::ffi::c_int + (8 as core::ffi::c_int - 1 as core::ffi::c_int)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int);
    size
        += 768 as core::ffi::c_int + (8 as core::ffi::c_int - 1 as core::ffi::c_int)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int);
    size
        += 768 as core::ffi::c_int + (8 as core::ffi::c_int - 1 as core::ffi::c_int)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int);
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            ((NUM_GAIN_DEC_INSTANCES * SEL_DRC_COUNT) as usize)
                .wrapping_mul(
                    (::core::mem::size_of::<ia_interp_buf_struct>() as usize)
                        .wrapping_mul(15 as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            (NUM_GAIN_DEC_INSTANCES as usize)
                .wrapping_mul(
                    (::core::mem::size_of::<ia_eq_set_struct>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            (NUM_GAIN_DEC_INSTANCES as usize)
                .wrapping_mul(
                    (8 as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            ((NUM_GAIN_DEC_INSTANCES * MAX_CHANNEL_COUNT) as usize)
                .wrapping_mul(
                    (4096 as usize)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            (NUM_GAIN_DEC_INSTANCES as usize)
                .wrapping_mul(
                    (8 as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            (NUM_GAIN_DEC_INSTANCES as usize)
                .wrapping_mul(
                    (8 as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            ((NUM_GAIN_DEC_INSTANCES * MAX_CHANNEL_COUNT) as usize)
                .wrapping_mul(
                    (4096 as usize)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            ((NUM_GAIN_DEC_INSTANCES * NUM_ELE_IN_CPLX_NUM) as usize)
                .wrapping_mul(
                    (8 as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size += (NUM_GAIN_DEC_INSTANCES * subband_buf_size) as core::ffi::c_int;
    size = (size as core::ffi::c_ulong)
        .wrapping_add(
            ((NUM_GAIN_DEC_INSTANCES * MAX_CHANNEL_COUNT) as usize)
                .wrapping_mul(
                    (110 as usize)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn impd_drc_fill_mem_tables(
    mut p_obj_drc: *mut ia_drc_api_struct,
) -> IA_ERRORCODE {
    let mut p_mem_info: *mut ia_mem_info_struct = 0 as *mut ia_mem_info_struct;
    p_mem_info = &mut *((*p_obj_drc).p_mem_info).offset(IA_DRC_PERSIST_IDX as isize)
        as *mut ia_mem_info_struct;
    memset(
        p_mem_info as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<ia_mem_info_struct>() as size_t,
    );
    (*p_mem_info).ui_size = impd_calc_pers_size() as UWORD32;
    (*p_mem_info).ui_alignment = 8 as UWORD32;
    (*p_mem_info).ui_type = IA_MEMTYPE_PERSIST as UWORD32;
    (*p_mem_info).ui_placement[0 as core::ffi::c_int as usize] = 0 as UWORD32;
    (*p_mem_info).ui_placement[1 as core::ffi::c_int as usize] = 0 as UWORD32;
    (*p_mem_info).ui_priority = IA_MEMPRIORITY_ANYWHERE as UWORD32;
    (*p_mem_info).ui_placed[0 as core::ffi::c_int as usize] = 0 as UWORD32;
    (*p_mem_info).ui_placed[1 as core::ffi::c_int as usize] = 0 as UWORD32;
    p_mem_info = &mut *((*p_obj_drc).p_mem_info).offset(IA_DRC_INPUT_IDX as isize)
        as *mut ia_mem_info_struct;
    memset(
        p_mem_info as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<ia_mem_info_struct>() as size_t,
    );
    (*p_mem_info).ui_size = ((*p_obj_drc).str_config.frame_size
        * ((*p_obj_drc).str_config.pcm_size >> 3 as core::ffi::c_int)
        * (*p_obj_drc).str_config.num_ch_in) as UWORD32;
    (*p_mem_info).ui_alignment = 4 as UWORD32;
    (*p_mem_info).ui_type = IA_MEMTYPE_INPUT as UWORD32;
    (*p_mem_info).ui_placement[0 as core::ffi::c_int as usize] = 0 as UWORD32;
    (*p_mem_info).ui_placement[1 as core::ffi::c_int as usize] = 0 as UWORD32;
    (*p_mem_info).ui_priority = IA_MEMPRIORITY_ANYWHERE as UWORD32;
    (*p_mem_info).ui_placed[0 as core::ffi::c_int as usize] = 0 as UWORD32;
    (*p_mem_info).ui_placed[1 as core::ffi::c_int as usize] = 0 as UWORD32;
    p_mem_info = &mut *((*p_obj_drc).p_mem_info).offset(IA_DRC_OUTPUT_IDX as isize)
        as *mut ia_mem_info_struct;
    memset(
        p_mem_info as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<ia_mem_info_struct>() as size_t,
    );
    (*p_mem_info).ui_size = ((*p_obj_drc).str_config.frame_size
        * ((*p_obj_drc).str_config.pcm_size >> 3 as core::ffi::c_int)
        * (*p_obj_drc).str_config.num_ch_in) as UWORD32;
    (*p_mem_info).ui_alignment = 4 as UWORD32;
    (*p_mem_info).ui_type = IA_MEMTYPE_OUTPUT as UWORD32;
    (*p_mem_info).ui_placement[0 as core::ffi::c_int as usize] = 0 as UWORD32;
    (*p_mem_info).ui_placement[1 as core::ffi::c_int as usize] = 0 as UWORD32;
    (*p_mem_info).ui_priority = IA_MEMPRIORITY_ANYWHERE as UWORD32;
    (*p_mem_info).ui_placed[0 as core::ffi::c_int as usize] = 0 as UWORD32;
    (*p_mem_info).ui_placed[1 as core::ffi::c_int as usize] = 0 as UWORD32;
    p_mem_info = &mut *((*p_obj_drc).p_mem_info).offset(IA_DRC_SCRATCH_IDX as isize)
        as *mut ia_mem_info_struct;
    memset(
        p_mem_info as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<ia_mem_info_struct>() as size_t,
    );
    (*p_mem_info).ui_size = impd_calc_scratch_size() as UWORD32;
    (*p_mem_info).ui_alignment = 8 as UWORD32;
    (*p_mem_info).ui_type = IA_MEMTYPE_SCRATCH as UWORD32;
    (*p_mem_info).ui_placement[0 as core::ffi::c_int as usize] = 0 as UWORD32;
    (*p_mem_info).ui_placement[1 as core::ffi::c_int as usize] = 0 as UWORD32;
    (*p_mem_info).ui_priority = IA_MEMPRIORITY_ANYWHERE as UWORD32;
    (*p_mem_info).ui_placed[0 as core::ffi::c_int as usize] = 0 as UWORD32;
    (*p_mem_info).ui_placed[1 as core::ffi::c_int as usize] = 0 as UWORD32;
    return IA_NO_ERROR;
}
