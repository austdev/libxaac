extern "C" {
    fn cos(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn sin(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn exp(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn log10(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn pow(__x: core::ffi::c_double, __y: core::ffi::c_double) -> core::ffi::c_double;
    fn sqrt(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn fabs(__x: core::ffi::c_double) -> core::ffi::c_double;
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
    static samp_rate_tbl: [[FLOAT32; 12]; 13];
    static f_bands_nrm_QMF71: [FLOAT32; 71];
    static f_bands_nrm_QMF64: [FLOAT32; 64];
    static f_bands_nrm_STFT256: [FLOAT32; 257];
}
pub type size_t = usize;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type FLOAT32 = core::ffi::c_float;
pub type FLOAT64 = core::ffi::c_double;
pub type VOID = ();
pub type pVOID = *mut core::ffi::c_void;
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
pub const MAX_CHANNEL_COUNT: core::ffi::c_int = 8 as core::ffi::c_int;
pub const UNEXPECTED_ERROR: core::ffi::c_int = 2 as core::ffi::c_int;
pub const EXTERNAL_ERROR: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SUBBAND_DOMAIN_MODE_OFF: core::ffi::c_int = 0 as core::ffi::c_int;
pub const SUBBAND_DOMAIN_MODE_QMF64: core::ffi::c_int = 1;
pub const SUBBAND_DOMAIN_MODE_QMF71: core::ffi::c_int = 2;
pub const SUBBAND_DOMAIN_MODE_STFT256: core::ffi::c_int = 3;
pub const AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_QMF64: core::ffi::c_int = 64
    as core::ffi::c_int;
pub const AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_QMF71: core::ffi::c_int = 64
    as core::ffi::c_int;
pub const AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_STFT256: core::ffi::c_int = 256
    as core::ffi::c_int;
pub const PARAM_DRC_TYPE_FF: core::ffi::c_int = 0 as core::ffi::c_int;
pub const PARAM_DRC_TYPE_FF_LEVEL_ESTIM_FRAME_COUNT_MAX: core::ffi::c_int = 64
    as core::ffi::c_int;
pub const PARAM_DRC_TYPE_LIM: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const METHOD_DEFINITION_PROGRAM_LOUDNESS: core::ffi::c_int = 1 as core::ffi::c_int;
pub const METHOD_DEFINITION_ANCHOR_LOUDNESS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const PI: core::ffi::c_float = 3.14159265f32;
#[no_mangle]
pub unsafe extern "C" fn impd_init_parametric_drc(
    mut drc_frame_size: WORD32,
    mut sampling_rate: WORD32,
    mut sub_band_domain_mode: WORD32,
    mut p_parametricdrc_params: *mut ia_parametric_drc_params_struct,
) -> WORD32 {
    static mut sub_band_count_tbl: [WORD32; 4] = [
        0 as core::ffi::c_int,
        64 as core::ffi::c_int,
        71 as core::ffi::c_int,
        256 as core::ffi::c_int,
    ];
    (*p_parametricdrc_params).drc_frame_size = drc_frame_size;
    (*p_parametricdrc_params).sampling_rate = sampling_rate;
    (*p_parametricdrc_params).sub_band_domain_mode = sub_band_domain_mode;
    (*p_parametricdrc_params).sub_band_count = sub_band_count_tbl[sub_band_domain_mode
        as usize];
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_init_parametric_drc_feed_fwd(
    mut pstr_drc_config: *mut ia_drc_config,
    mut instance_idx: WORD32,
    mut ch_count_from_dwnmix_id: WORD32,
    mut p_parametricdrc_params: *mut ia_parametric_drc_params_struct,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut i: WORD32 = 0 as WORD32;
    let mut parametric_drc_idx: WORD32 = (*p_parametricdrc_params)
        .parametric_drc_idx[instance_idx as usize];
    let mut gain_set_index: WORD32 = (*p_parametricdrc_params)
        .gain_set_index[instance_idx as usize];
    let mut channel_map: *mut WORD32 = ((*p_parametricdrc_params)
        .channel_map[instance_idx as usize])
        .as_mut_ptr();
    let mut hDrcCoefficientsParametricDrcBs: *mut ia_drc_coeff_parametric_drc_struct = &mut (*pstr_drc_config)
        .str_drc_config_ext
        .str_drc_coeff_param_drc;
    let mut hParametricDrcTypeFeedForwardBs: *mut ia_parametric_drc_type_feed_forward_struct = &mut (*((*pstr_drc_config)
        .str_drc_config_ext
        .str_parametric_drc_instructions)
        .as_mut_ptr()
        .offset(parametric_drc_idx as isize))
        .str_parametric_drc_type_feed_forward;
    let mut pstr_parametric_ffwd_type_drc_params: *mut ia_parametric_drc_type_ff_params_struct = &mut (*((*p_parametricdrc_params)
        .str_parametric_drc_instance_params)
        .as_mut_ptr()
        .offset(instance_idx as isize))
        .str_parametric_drc_type_ff_params;
    (*pstr_parametric_ffwd_type_drc_params).frame_size = (*p_parametricdrc_params)
        .parametric_drc_frame_size;
    (*pstr_parametric_ffwd_type_drc_params).sub_band_domain_mode = (*p_parametricdrc_params)
        .sub_band_domain_mode;
    (*pstr_parametric_ffwd_type_drc_params).sub_band_count = (*p_parametricdrc_params)
        .sub_band_count;
    (*pstr_parametric_ffwd_type_drc_params).sub_band_compensation_type = 0
        as core::ffi::c_int as WORD32;
    if (*pstr_parametric_ffwd_type_drc_params).sub_band_domain_mode
        == SUBBAND_DOMAIN_MODE_QMF64
    {
        if (*p_parametricdrc_params).sampling_rate == 48000 as core::ffi::c_int {
            (*pstr_parametric_ffwd_type_drc_params).sub_band_compensation_type = 1
                as core::ffi::c_int as WORD32;
        } else {
            return UNEXPECTED_ERROR
        }
    }
    (*pstr_parametric_ffwd_type_drc_params).audio_num_chan = (*p_parametricdrc_params)
        .audio_num_chan;
    (*pstr_parametric_ffwd_type_drc_params).level_estim_k_weighting_type = (*hParametricDrcTypeFeedForwardBs)
        .level_estim_k_weighting_type;
    (*pstr_parametric_ffwd_type_drc_params).level_estim_integration_time = (*hParametricDrcTypeFeedForwardBs)
        .level_estim_integration_time;
    (*pstr_parametric_ffwd_type_drc_params).level_estim_frame_index = 0
        as core::ffi::c_int as WORD32;
    (*pstr_parametric_ffwd_type_drc_params).level_estim_frame_count = (*hParametricDrcTypeFeedForwardBs)
        .level_estim_integration_time
        / (*pstr_parametric_ffwd_type_drc_params).frame_size;
    memset(
        ((*pstr_parametric_ffwd_type_drc_params).level).as_mut_ptr()
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (PARAM_DRC_TYPE_FF_LEVEL_ESTIM_FRAME_COUNT_MAX as size_t)
            .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
    );
    if ch_count_from_dwnmix_id != 0 as core::ffi::c_int {
        memcpy(
            ((*pstr_parametric_ffwd_type_drc_params).level_estim_ch_weight).as_mut_ptr()
                as *mut core::ffi::c_void,
            ((*hDrcCoefficientsParametricDrcBs)
                .str_parametric_drc_gain_set_params[gain_set_index as usize]
                .level_estim_ch_weight)
                .as_mut_ptr() as *const core::ffi::c_void,
            (ch_count_from_dwnmix_id as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*pstr_parametric_ffwd_type_drc_params).audio_num_chan {
            (*pstr_parametric_ffwd_type_drc_params).level_estim_ch_weight[i as usize] = *channel_map
                .offset(i as isize) as FLOAT32;
            i += 1;
        }
    }
    if (*pstr_parametric_ffwd_type_drc_params).sub_band_domain_mode
        == SUBBAND_DOMAIN_MODE_OFF
    {
        err = impd_init_lvl_est_filt_time(
            (*pstr_parametric_ffwd_type_drc_params).level_estim_k_weighting_type,
            (*p_parametricdrc_params).sampling_rate,
            &mut (*pstr_parametric_ffwd_type_drc_params).pre_filt_coeff,
            &mut (*pstr_parametric_ffwd_type_drc_params).rlb_filt_coeff,
        );
        if err != 0 {
            return err;
        }
    } else {
        err = impd_init_lvl_est_filt_subband(
            (*pstr_parametric_ffwd_type_drc_params).level_estim_k_weighting_type,
            (*p_parametricdrc_params).sampling_rate,
            (*p_parametricdrc_params).sub_band_domain_mode,
            (*p_parametricdrc_params).sub_band_count,
            (*pstr_parametric_ffwd_type_drc_params).sub_band_compensation_type,
            ((*pstr_parametric_ffwd_type_drc_params).weighting_filt).as_mut_ptr(),
            &mut (*pstr_parametric_ffwd_type_drc_params).filt_coeff_subband,
        );
        if err != 0 {
            return err;
        }
    }
    (*pstr_parametric_ffwd_type_drc_params).node_count = (*hParametricDrcTypeFeedForwardBs)
        .node_count;
    memcpy(
        ((*pstr_parametric_ffwd_type_drc_params).node_level).as_mut_ptr()
            as *mut core::ffi::c_void,
        ((*hParametricDrcTypeFeedForwardBs).node_level).as_mut_ptr()
            as *const core::ffi::c_void,
        ((*pstr_parametric_ffwd_type_drc_params).node_count as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
    memcpy(
        ((*pstr_parametric_ffwd_type_drc_params).node_gain).as_mut_ptr()
            as *mut core::ffi::c_void,
        ((*hParametricDrcTypeFeedForwardBs).node_gain).as_mut_ptr()
            as *const core::ffi::c_void,
        ((*pstr_parametric_ffwd_type_drc_params).node_count as size_t)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
    (*pstr_parametric_ffwd_type_drc_params).ref_level_parametric_drc = (*hDrcCoefficientsParametricDrcBs)
        .str_parametric_drc_gain_set_params[gain_set_index as usize]
        .drc_input_loudness;
    let mut gain_smooth_attack_time_fast: WORD32 = (*hParametricDrcTypeFeedForwardBs)
        .gain_smooth_attack_time_fast;
    let mut gain_smooth_release_time_fast: WORD32 = (*hParametricDrcTypeFeedForwardBs)
        .gain_smooth_release_time_fast;
    let mut gain_smooth_attack_time_slow: WORD32 = (*hParametricDrcTypeFeedForwardBs)
        .gain_smooth_attack_time_slow;
    let mut gain_smooth_release_time_slow: WORD32 = (*hParametricDrcTypeFeedForwardBs)
        .gain_smooth_release_time_slow;
    let mut gain_smooth_hold_off: WORD32 = (*hParametricDrcTypeFeedForwardBs)
        .gain_smooth_hold_off;
    let mut sampling_rate: WORD32 = (*p_parametricdrc_params).sampling_rate;
    let mut parametric_drc_frame_size: WORD32 = (*p_parametricdrc_params)
        .parametric_drc_frame_size;
    (*pstr_parametric_ffwd_type_drc_params).gain_smooth_attack_alpha_fast = 1
        as core::ffi::c_int as FLOAT32
        - exp(
            -1.0f64 * parametric_drc_frame_size as core::ffi::c_double
                / ((gain_smooth_attack_time_fast * sampling_rate) as core::ffi::c_double
                    * 0.001f64),
        ) as FLOAT32;
    (*pstr_parametric_ffwd_type_drc_params).gain_smooth_rel_alpha_fast = 1
        as core::ffi::c_int as FLOAT32
        - exp(
            -1.0f64 * parametric_drc_frame_size as core::ffi::c_double
                / ((gain_smooth_release_time_fast * sampling_rate) as core::ffi::c_double
                    * 0.001f64),
        ) as FLOAT32;
    (*pstr_parametric_ffwd_type_drc_params).gain_smooth_attack_alpha_slow = 1
        as core::ffi::c_int as FLOAT32
        - exp(
            -1.0f64 * parametric_drc_frame_size as core::ffi::c_double
                / ((gain_smooth_attack_time_slow * sampling_rate) as core::ffi::c_double
                    * 0.001f64),
        ) as FLOAT32;
    (*pstr_parametric_ffwd_type_drc_params).gain_smooth_rel_alpha_slow = 1
        as core::ffi::c_int as FLOAT32
        - exp(
            -1.0f64 * parametric_drc_frame_size as core::ffi::c_double
                / ((gain_smooth_release_time_slow * sampling_rate) as core::ffi::c_double
                    * 0.001f64),
        ) as FLOAT32;
    (*pstr_parametric_ffwd_type_drc_params).gain_smooth_hold_off_count = (gain_smooth_hold_off
        as core::ffi::c_int * 256 as core::ffi::c_int * sampling_rate as core::ffi::c_int
        / (parametric_drc_frame_size as core::ffi::c_int * 48000 as core::ffi::c_int))
        as WORD32;
    (*pstr_parametric_ffwd_type_drc_params).gain_smooth_attack_threshold = (*hParametricDrcTypeFeedForwardBs)
        .gain_smooth_attack_threshold;
    (*pstr_parametric_ffwd_type_drc_params).gain_smooth_rel_threshold = (*hParametricDrcTypeFeedForwardBs)
        .gain_smooth_rel_threshold;
    err = impd_parametric_ffwd_type_drc_reset(pstr_parametric_ffwd_type_drc_params);
    if err != 0 {
        return err;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_init_parametric_drc_lim(
    mut pstr_drc_config: *mut ia_drc_config,
    mut instance_idx: WORD32,
    mut ch_count_from_dwnmix_id: WORD32,
    mut p_parametricdrc_params: *mut ia_parametric_drc_params_struct,
    mut mem_ptr: *mut pVOID,
) -> VOID {
    let mut i: WORD32 = 0 as WORD32;
    let mut j: UWORD32 = 0;
    let mut attack: UWORD32 = 0;
    let mut sec_len: UWORD32 = 0;
    let mut parametric_drc_idx: WORD32 = (*p_parametricdrc_params)
        .parametric_drc_idx[instance_idx as usize];
    let mut gain_set_index: WORD32 = (*p_parametricdrc_params)
        .gain_set_index[instance_idx as usize];
    let mut channel_map: *mut WORD32 = ((*p_parametricdrc_params)
        .channel_map[instance_idx as usize])
        .as_mut_ptr();
    let mut hDrcCoefficientsParametricDrcBs: *mut ia_drc_coeff_parametric_drc_struct = &mut (*pstr_drc_config)
        .str_drc_config_ext
        .str_drc_coeff_param_drc;
    let mut hParametricDrcTypeLimBs: *mut ia_parametric_drc_lim_struct = &mut (*((*pstr_drc_config)
        .str_drc_config_ext
        .str_parametric_drc_instructions)
        .as_mut_ptr()
        .offset(parametric_drc_idx as isize))
        .parametric_drc_lim;
    let mut pstr_parametric_lim_type_drc_params: *mut ia_parametric_drc_type_lim_params_struct = &mut (*((*p_parametricdrc_params)
        .str_parametric_drc_instance_params)
        .as_mut_ptr()
        .offset(instance_idx as isize))
        .str_parametric_drc_type_lim_params;
    (*pstr_parametric_lim_type_drc_params).frame_size = (*p_parametricdrc_params)
        .drc_frame_size;
    (*pstr_parametric_lim_type_drc_params).audio_num_chan = (*p_parametricdrc_params)
        .audio_num_chan;
    if ch_count_from_dwnmix_id != 0 as core::ffi::c_int {
        memcpy(
            ((*pstr_parametric_lim_type_drc_params).level_estim_ch_weight).as_mut_ptr()
                as *mut core::ffi::c_void,
            ((*hDrcCoefficientsParametricDrcBs)
                .str_parametric_drc_gain_set_params[gain_set_index as usize]
                .level_estim_ch_weight)
                .as_mut_ptr() as *const core::ffi::c_void,
            (ch_count_from_dwnmix_id as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t),
        );
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*pstr_parametric_lim_type_drc_params).audio_num_chan {
            (*pstr_parametric_lim_type_drc_params).level_estim_ch_weight[i as usize] = *channel_map
                .offset(i as isize) as FLOAT32;
            i += 1;
        }
    }
    attack = ((*hParametricDrcTypeLimBs).parametric_lim_attack as core::ffi::c_int
        * (*p_parametricdrc_params).sampling_rate as core::ffi::c_int
        / 1000 as core::ffi::c_int) as UWORD32;
    sec_len = sqrt(attack.wrapping_add(1 as UWORD32) as core::ffi::c_double) as UWORD32;
    (*pstr_parametric_lim_type_drc_params).sec_len = sec_len;
    (*pstr_parametric_lim_type_drc_params).num_max_buf_sec = attack
        .wrapping_add(1 as UWORD32)
        .wrapping_div(sec_len);
    if ((*pstr_parametric_lim_type_drc_params).num_max_buf_sec).wrapping_mul(sec_len)
        < attack.wrapping_add(1 as UWORD32)
    {
        (*pstr_parametric_lim_type_drc_params).num_max_buf_sec = ((*pstr_parametric_lim_type_drc_params)
            .num_max_buf_sec)
            .wrapping_add(1);
    }
    (*pstr_parametric_lim_type_drc_params).max_buf = *mem_ptr as *mut FLOAT32;
    *mem_ptr = (*mem_ptr as size_t)
        .wrapping_add(
            (((*pstr_parametric_lim_type_drc_params).num_max_buf_sec)
                .wrapping_mul(sec_len) as size_t)
                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
        ) as pVOID;
    (*pstr_parametric_lim_type_drc_params).attack_ms = (*hParametricDrcTypeLimBs)
        .parametric_lim_attack as FLOAT32;
    (*pstr_parametric_lim_type_drc_params).release_ms = (*hParametricDrcTypeLimBs)
        .parametric_lim_release as FLOAT32;
    (*pstr_parametric_lim_type_drc_params).attack = attack;
    (*pstr_parametric_lim_type_drc_params).attack_constant = pow(
        0.1f64,
        1.0f64 / attack.wrapping_add(1 as UWORD32) as core::ffi::c_double,
    ) as FLOAT32;
    (*pstr_parametric_lim_type_drc_params).release_constant = pow(
        0.1f64,
        1.0f64
            / ((*hParametricDrcTypeLimBs).parametric_lim_release as core::ffi::c_int
                * (*p_parametricdrc_params).sampling_rate as core::ffi::c_int
                / 1000 as core::ffi::c_int + 1 as core::ffi::c_int)
                as core::ffi::c_double,
    ) as FLOAT32;
    (*pstr_parametric_lim_type_drc_params).threshold = pow(
        10.0f64,
        (0.05f32 * (*hParametricDrcTypeLimBs).parametric_lim_threshold)
            as core::ffi::c_double,
    ) as FLOAT32;
    (*pstr_parametric_lim_type_drc_params).channels = (*pstr_parametric_lim_type_drc_params)
        .audio_num_chan as UWORD32;
    (*pstr_parametric_lim_type_drc_params).sampling_rate = (*p_parametricdrc_params)
        .sampling_rate as UWORD32;
    (*pstr_parametric_lim_type_drc_params).cor = 1.0f32 as FLOAT32;
    (*pstr_parametric_lim_type_drc_params).smooth_state_0 = 1.0f64 as FLOAT64;
    j = 0 as UWORD32;
    while j
        < ((*pstr_parametric_lim_type_drc_params).num_max_buf_sec)
            .wrapping_mul((*pstr_parametric_lim_type_drc_params).sec_len)
    {
        *((*pstr_parametric_lim_type_drc_params).max_buf).offset(j as isize) = 0.0f32
            as FLOAT32;
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn impd_init_parametric_drcInstance(
    mut pstr_drc_config: *mut ia_drc_config,
    mut instance_idx: WORD32,
    mut ch_count_from_dwnmix_id: WORD32,
    mut p_parametricdrc_params: *mut ia_parametric_drc_params_struct,
    mut mem_ptr: *mut pVOID,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut parametric_drc_idx: WORD32 = (*p_parametricdrc_params)
        .parametric_drc_idx[instance_idx as usize];
    let mut hParametricDrcInstructions: *mut ia_parametric_drc_instructions_struct = &mut *((*pstr_drc_config)
        .str_drc_config_ext
        .str_parametric_drc_instructions)
        .as_mut_ptr()
        .offset(parametric_drc_idx as isize)
        as *mut ia_parametric_drc_instructions_struct;
    (*p_parametricdrc_params)
        .str_parametric_drc_instance_params[instance_idx as usize]
        .disable_paramteric_drc = (*hParametricDrcInstructions).disable_paramteric_drc;
    (*p_parametricdrc_params)
        .str_parametric_drc_instance_params[instance_idx as usize]
        .parametric_drc_type = (*hParametricDrcInstructions).parametric_drc_type;
    (*p_parametricdrc_params)
        .str_parametric_drc_instance_params[instance_idx as usize]
        .str_spline_nodes
        .num_nodes = (*p_parametricdrc_params).num_nodes;
    if (*p_parametricdrc_params)
        .str_parametric_drc_instance_params[instance_idx as usize]
        .disable_paramteric_drc == 0 as core::ffi::c_int
    {
        if (*p_parametricdrc_params)
            .str_parametric_drc_instance_params[instance_idx as usize]
            .parametric_drc_type == PARAM_DRC_TYPE_FF
        {
            err = impd_init_parametric_drc_feed_fwd(
                pstr_drc_config,
                instance_idx,
                ch_count_from_dwnmix_id,
                p_parametricdrc_params,
            );
            if err != 0 {
                return err;
            }
        } else if (*p_parametricdrc_params)
            .str_parametric_drc_instance_params[instance_idx as usize]
            .parametric_drc_type == PARAM_DRC_TYPE_LIM
        {
            (*p_parametricdrc_params)
                .str_parametric_drc_instance_params[instance_idx as usize]
                .str_spline_nodes
                .num_nodes = (*p_parametricdrc_params).drc_frame_size;
            impd_init_parametric_drc_lim(
                pstr_drc_config,
                instance_idx,
                ch_count_from_dwnmix_id,
                p_parametricdrc_params,
                mem_ptr,
            );
        } else {
            return 2 as WORD32
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_init_parametric_drc_after_config(
    mut pstr_drc_config: *mut ia_drc_config,
    mut pstr_loudness_info: *mut ia_drc_loudness_info_set_struct,
    mut p_parametricdrc_params: *mut ia_parametric_drc_params_struct,
    mut mem_ptr: *mut pVOID,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut instance_idx: WORD32 = 0 as WORD32;
    let mut gain_set_index: WORD32 = 0 as WORD32;
    let mut side_chain_config_type: WORD32 = 0 as WORD32;
    let mut downmix_id: WORD32 = 0 as WORD32;
    let mut ch_count_from_dwnmix_id: WORD32 = 0 as WORD32;
    let mut L: WORD32 = 0 as WORD32;
    (*p_parametricdrc_params).parametric_drc_frame_size = (*pstr_drc_config)
        .str_drc_config_ext
        .str_drc_coeff_param_drc
        .parametric_drc_frame_size;
    (*p_parametricdrc_params).reset_parametric_drc = (*pstr_drc_config)
        .str_drc_config_ext
        .str_drc_coeff_param_drc
        .reset_parametric_drc;
    (*p_parametricdrc_params).num_nodes = (*p_parametricdrc_params).drc_frame_size
        / (*p_parametricdrc_params).parametric_drc_frame_size;
    match (*p_parametricdrc_params).sub_band_domain_mode {
        SUBBAND_DOMAIN_MODE_QMF64 => {
            L = AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_QMF64 as WORD32;
        }
        SUBBAND_DOMAIN_MODE_QMF71 => {
            L = AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_QMF71 as WORD32;
        }
        SUBBAND_DOMAIN_MODE_STFT256 => {
            L = AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_STFT256 as WORD32;
        }
        SUBBAND_DOMAIN_MODE_OFF | _ => {
            L = 0 as core::ffi::c_int as WORD32;
        }
    }
    if (*p_parametricdrc_params).sub_band_domain_mode != SUBBAND_DOMAIN_MODE_OFF
        && (*p_parametricdrc_params).parametric_drc_frame_size != L
    {
        return 4 as WORD32;
    }
    instance_idx = 0 as core::ffi::c_int as WORD32;
    while instance_idx < (*p_parametricdrc_params).parametric_drc_instance_count {
        gain_set_index = (*p_parametricdrc_params).gain_set_index[instance_idx as usize];
        side_chain_config_type = (*pstr_drc_config)
            .str_drc_config_ext
            .str_drc_coeff_param_drc
            .str_parametric_drc_gain_set_params[gain_set_index as usize]
            .side_chain_config_type;
        downmix_id = (*pstr_drc_config)
            .str_drc_config_ext
            .str_drc_coeff_param_drc
            .str_parametric_drc_gain_set_params[gain_set_index as usize]
            .downmix_id;
        if side_chain_config_type == 1 as core::ffi::c_int
            && downmix_id
                == (*p_parametricdrc_params)
                    .dwnmix_id_from_drc_instructions[instance_idx as usize]
        {
            ch_count_from_dwnmix_id = (*pstr_drc_config)
                .str_drc_config_ext
                .str_drc_coeff_param_drc
                .str_parametric_drc_gain_set_params[gain_set_index as usize]
                .ch_count_from_dwnmix_id;
        } else {
            ch_count_from_dwnmix_id = 0 as core::ffi::c_int as WORD32;
        }
        if (*pstr_drc_config)
            .str_drc_config_ext
            .str_drc_coeff_param_drc
            .str_parametric_drc_gain_set_params[gain_set_index as usize]
            .drc_input_loudness_present == 0 as core::ffi::c_int
        {
            let mut n: WORD32 = 0 as WORD32;
            let mut m: WORD32 = 0 as WORD32;
            let mut drcInputLoudnessFound: WORD32 = 0 as WORD32;
            let mut drc_input_loudness: FLOAT32 = 0.0f32;
            n = 0 as core::ffi::c_int as WORD32;
            while n < (*pstr_loudness_info).loudness_info_count {
                let mut loudness_info: *mut ia_loudness_info_struct = &mut *((*pstr_loudness_info)
                    .loudness_info)
                    .as_mut_ptr()
                    .offset(n as isize) as *mut ia_loudness_info_struct;
                if (*p_parametricdrc_params)
                    .dwnmix_id_from_drc_instructions[instance_idx as usize]
                    == (*loudness_info).downmix_id
                {
                    if 0 as core::ffi::c_int == (*loudness_info).drc_set_id {
                        m = 0 as core::ffi::c_int as WORD32;
                        while m < (*loudness_info).measurement_count {
                            if (*loudness_info).loudness_measure[m as usize].method_def
                                == METHOD_DEFINITION_PROGRAM_LOUDNESS
                            {
                                drc_input_loudness = (*loudness_info)
                                    .loudness_measure[m as usize]
                                    .method_val;
                                drcInputLoudnessFound = 1 as core::ffi::c_int as WORD32;
                                break;
                            } else {
                                m += 1;
                            }
                        }
                        if drcInputLoudnessFound == 0 as core::ffi::c_int {
                            m = 0 as core::ffi::c_int as WORD32;
                            while m < (*loudness_info).measurement_count {
                                if (*loudness_info).loudness_measure[m as usize].method_def
                                    == METHOD_DEFINITION_ANCHOR_LOUDNESS
                                {
                                    drc_input_loudness = (*loudness_info)
                                        .loudness_measure[m as usize]
                                        .method_val;
                                    drcInputLoudnessFound = 1 as core::ffi::c_int as WORD32;
                                    break;
                                } else {
                                    m += 1;
                                }
                            }
                        }
                    }
                }
                n += 1;
            }
            if drcInputLoudnessFound == 0 as core::ffi::c_int {
                n = 0 as core::ffi::c_int as WORD32;
                while n < (*pstr_loudness_info).loudness_info_count {
                    let mut loudness_info_0: *mut ia_loudness_info_struct = &mut *((*pstr_loudness_info)
                        .loudness_info)
                        .as_mut_ptr()
                        .offset(n as isize) as *mut ia_loudness_info_struct;
                    if 0 as core::ffi::c_int == (*loudness_info_0).downmix_id {
                        if 0 as core::ffi::c_int == (*loudness_info_0).drc_set_id {
                            m = 0 as core::ffi::c_int as WORD32;
                            while m < (*loudness_info_0).measurement_count {
                                if (*loudness_info_0)
                                    .loudness_measure[m as usize]
                                    .method_def == METHOD_DEFINITION_PROGRAM_LOUDNESS
                                {
                                    drc_input_loudness = (*loudness_info_0)
                                        .loudness_measure[m as usize]
                                        .method_val;
                                    drcInputLoudnessFound = 1 as core::ffi::c_int as WORD32;
                                    break;
                                } else {
                                    m += 1;
                                }
                            }
                            if drcInputLoudnessFound == 0 as core::ffi::c_int {
                                m = 0 as core::ffi::c_int as WORD32;
                                while m < (*loudness_info_0).measurement_count {
                                    if (*loudness_info_0)
                                        .loudness_measure[m as usize]
                                        .method_def == METHOD_DEFINITION_ANCHOR_LOUDNESS
                                    {
                                        drc_input_loudness = (*loudness_info_0)
                                            .loudness_measure[m as usize]
                                            .method_val;
                                        drcInputLoudnessFound = 1 as core::ffi::c_int as WORD32;
                                        break;
                                    } else {
                                        m += 1;
                                    }
                                }
                            }
                        }
                    }
                    n += 1;
                }
            }
            if drcInputLoudnessFound == 0 as core::ffi::c_int {
                return 2 as WORD32
            } else {
                (*pstr_drc_config)
                    .str_drc_config_ext
                    .str_drc_coeff_param_drc
                    .str_parametric_drc_gain_set_params[gain_set_index as usize]
                    .drc_input_loudness = drc_input_loudness;
            }
        }
        err = impd_init_parametric_drcInstance(
            pstr_drc_config,
            instance_idx,
            ch_count_from_dwnmix_id,
            p_parametricdrc_params,
            mem_ptr,
        );
        if err != 0 {
            return err;
        }
        instance_idx += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_init_lvl_est_filt_time(
    mut level_estim_k_weighting_type: WORD32,
    mut sampling_rate: WORD32,
    mut pre_filt_coeff: *mut ia_2nd_order_filt_coeff_struct,
    mut rlb_filt_coeff: *mut ia_2nd_order_filt_coeff_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut ptr_samp_tbl: *const FLOAT32 = 0 as *const FLOAT32;
    match sampling_rate {
        96000 => {
            i = 0 as core::ffi::c_int as WORD32;
        }
        88200 => {
            i = 1 as core::ffi::c_int as WORD32;
        }
        64000 => {
            i = 2 as core::ffi::c_int as WORD32;
        }
        48000 => {
            i = 3 as core::ffi::c_int as WORD32;
        }
        44100 => {
            i = 4 as core::ffi::c_int as WORD32;
        }
        32000 => {
            i = 5 as core::ffi::c_int as WORD32;
        }
        24000 => {
            i = 6 as core::ffi::c_int as WORD32;
        }
        22050 => {
            i = 7 as core::ffi::c_int as WORD32;
        }
        16000 => {
            i = 8 as core::ffi::c_int as WORD32;
        }
        12000 => {
            i = 9 as core::ffi::c_int as WORD32;
        }
        11025 => {
            i = 10 as core::ffi::c_int as WORD32;
        }
        8000 => {
            i = 11 as core::ffi::c_int as WORD32;
        }
        7350 => {
            i = 12 as core::ffi::c_int as WORD32;
        }
        _ => {
            i = 3 as core::ffi::c_int as WORD32;
        }
    }
    ptr_samp_tbl = (samp_rate_tbl[i as usize]).as_ptr();
    if level_estim_k_weighting_type == 2 as core::ffi::c_int {
        (*pre_filt_coeff).b0 = *ptr_samp_tbl.offset(0 as core::ffi::c_int as isize);
        (*pre_filt_coeff).b1 = *ptr_samp_tbl.offset(1 as core::ffi::c_int as isize);
        (*pre_filt_coeff).b2 = *ptr_samp_tbl.offset(2 as core::ffi::c_int as isize);
        (*pre_filt_coeff).a1 = *ptr_samp_tbl.offset(3 as core::ffi::c_int as isize);
        (*pre_filt_coeff).a2 = *ptr_samp_tbl.offset(4 as core::ffi::c_int as isize);
    }
    if level_estim_k_weighting_type == 1 as core::ffi::c_int
        || level_estim_k_weighting_type == 2 as core::ffi::c_int
    {
        (*rlb_filt_coeff).b0 = *ptr_samp_tbl.offset(5 as core::ffi::c_int as isize);
        (*rlb_filt_coeff).b1 = *ptr_samp_tbl.offset(6 as core::ffi::c_int as isize);
        (*rlb_filt_coeff).b2 = *ptr_samp_tbl.offset(7 as core::ffi::c_int as isize);
        (*rlb_filt_coeff).a1 = *ptr_samp_tbl.offset(8 as core::ffi::c_int as isize);
        (*rlb_filt_coeff).a2 = *ptr_samp_tbl.offset(9 as core::ffi::c_int as isize);
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_init_lvl_est_filt_subband(
    mut level_estim_k_weighting_type: WORD32,
    mut sampling_rate: WORD32,
    mut sub_band_domain_mode: WORD32,
    mut sub_band_count: WORD32,
    mut sub_band_compensation_type: WORD32,
    mut weighting_filt: *mut FLOAT32,
    mut filt_coeff_subband: *mut ia_2nd_order_filt_coeff_struct,
) -> WORD32 {
    let mut w0: FLOAT32 = 0.;
    let mut alpha: FLOAT32 = 0.;
    let mut sinw0: FLOAT32 = 0.;
    let mut cosw0: FLOAT32 = 0.;
    let mut b0: FLOAT32 = 0.;
    let mut b1: FLOAT32 = 0.;
    let mut b2: FLOAT32 = 0.;
    let mut a0: FLOAT32 = 0.;
    let mut a1: FLOAT32 = 0.;
    let mut a2: FLOAT32 = 0.;
    let mut num_real: FLOAT32 = 0.;
    let mut num_imag: FLOAT32 = 0.;
    let mut den_real: FLOAT32 = 0.;
    let mut den_imag: FLOAT32 = 0.;
    let mut f_bands_nrm: *const FLOAT32 = 0 as *const FLOAT32;
    let mut b: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut ptr_samp_tbl: *const FLOAT32 = 0 as *const FLOAT32;
    match sampling_rate {
        96000 => {
            i = 0 as core::ffi::c_int as WORD32;
        }
        88200 => {
            i = 1 as core::ffi::c_int as WORD32;
        }
        64000 => {
            i = 2 as core::ffi::c_int as WORD32;
        }
        48000 => {
            i = 3 as core::ffi::c_int as WORD32;
        }
        44100 => {
            i = 4 as core::ffi::c_int as WORD32;
        }
        32000 => {
            i = 5 as core::ffi::c_int as WORD32;
        }
        24000 => {
            i = 6 as core::ffi::c_int as WORD32;
        }
        22050 => {
            i = 7 as core::ffi::c_int as WORD32;
        }
        16000 => {
            i = 8 as core::ffi::c_int as WORD32;
        }
        12000 => {
            i = 9 as core::ffi::c_int as WORD32;
        }
        11025 => {
            i = 10 as core::ffi::c_int as WORD32;
        }
        8000 => {
            i = 11 as core::ffi::c_int as WORD32;
        }
        7350 => {
            i = 12 as core::ffi::c_int as WORD32;
        }
        _ => {
            i = 3 as core::ffi::c_int as WORD32;
        }
    }
    ptr_samp_tbl = (samp_rate_tbl[i as usize]).as_ptr();
    match sub_band_domain_mode {
        SUBBAND_DOMAIN_MODE_QMF64 => {
            f_bands_nrm = f_bands_nrm_QMF64.as_ptr();
        }
        SUBBAND_DOMAIN_MODE_QMF71 => {
            f_bands_nrm = f_bands_nrm_QMF71.as_ptr();
        }
        SUBBAND_DOMAIN_MODE_STFT256 => {
            f_bands_nrm = f_bands_nrm_STFT256.as_ptr();
        }
        _ => return UNEXPECTED_ERROR,
    }
    b = 0 as core::ffi::c_int as WORD32;
    while b < sub_band_count {
        *weighting_filt.offset(b as isize) = 1.0f32 as FLOAT32;
        b += 1;
    }
    if level_estim_k_weighting_type == 2 as core::ffi::c_int {
        b0 = *ptr_samp_tbl.offset(0 as core::ffi::c_int as isize);
        b1 = *ptr_samp_tbl.offset(1 as core::ffi::c_int as isize);
        b2 = *ptr_samp_tbl.offset(2 as core::ffi::c_int as isize);
        a1 = *ptr_samp_tbl.offset(3 as core::ffi::c_int as isize);
        a2 = *ptr_samp_tbl.offset(4 as core::ffi::c_int as isize);
        a0 = 1.0f32 as FLOAT32;
        b = 0 as core::ffi::c_int as WORD32;
        while b < sub_band_count {
            num_real = b0
                + b1
                    * cos((PI * *f_bands_nrm.offset(b as isize)) as core::ffi::c_double)
                        as FLOAT32
                + b2
                    * cos(
                        (PI * 2 as core::ffi::c_int as FLOAT32
                            * *f_bands_nrm.offset(b as isize)) as core::ffi::c_double,
                    ) as FLOAT32;
            num_imag = -b1
                * sin((PI * *f_bands_nrm.offset(b as isize)) as core::ffi::c_double)
                    as FLOAT32
                - b2
                    * sin(
                        (PI * 2 as core::ffi::c_int as FLOAT32
                            * *f_bands_nrm.offset(b as isize)) as core::ffi::c_double,
                    ) as FLOAT32;
            den_real = a0
                + a1
                    * cos((PI * *f_bands_nrm.offset(b as isize)) as core::ffi::c_double)
                        as FLOAT32
                + a2
                    * cos(
                        (PI * 2 as core::ffi::c_int as FLOAT32
                            * *f_bands_nrm.offset(b as isize)) as core::ffi::c_double,
                    ) as FLOAT32;
            den_imag = -a1
                * sin((PI * *f_bands_nrm.offset(b as isize)) as core::ffi::c_double)
                    as FLOAT32
                - a2
                    * sin(
                        (PI * 2 as core::ffi::c_int as FLOAT32
                            * *f_bands_nrm.offset(b as isize)) as core::ffi::c_double,
                    ) as FLOAT32;
            *weighting_filt.offset(b as isize)
                *= sqrt(
                    ((num_real * num_real + num_imag * num_imag)
                        / (den_real * den_real + den_imag * den_imag))
                        as core::ffi::c_double,
                ) as FLOAT32;
            b += 1;
        }
    }
    if level_estim_k_weighting_type == 1 as core::ffi::c_int
        || level_estim_k_weighting_type == 2 as core::ffi::c_int
    {
        b0 = *ptr_samp_tbl.offset(5 as core::ffi::c_int as isize);
        b1 = *ptr_samp_tbl.offset(6 as core::ffi::c_int as isize);
        b2 = *ptr_samp_tbl.offset(7 as core::ffi::c_int as isize);
        a1 = *ptr_samp_tbl.offset(8 as core::ffi::c_int as isize);
        a2 = *ptr_samp_tbl.offset(9 as core::ffi::c_int as isize);
        a0 = 1.0f32 as FLOAT32;
        b = 0 as core::ffi::c_int as WORD32;
        while b < sub_band_count {
            if !(sub_band_compensation_type == 1 as core::ffi::c_int
                && b == 0 as core::ffi::c_int)
            {
                num_real = (b0 as core::ffi::c_double
                    + b1 as core::ffi::c_double
                        * cos(
                            (PI * *f_bands_nrm.offset(b as isize)) as core::ffi::c_double,
                        )
                    + b2 as core::ffi::c_double
                        * cos(
                            (PI * 2 as core::ffi::c_int as FLOAT32
                                * *f_bands_nrm.offset(b as isize)) as core::ffi::c_double,
                        )) as FLOAT32;
                num_imag = (-b1 as core::ffi::c_double
                    * sin((PI * *f_bands_nrm.offset(b as isize)) as core::ffi::c_double)
                    - b2 as core::ffi::c_double
                        * sin(
                            (PI * 2 as core::ffi::c_int as FLOAT32
                                * *f_bands_nrm.offset(b as isize)) as core::ffi::c_double,
                        )) as FLOAT32;
                den_real = (a0 as core::ffi::c_double
                    + a1 as core::ffi::c_double
                        * cos(
                            (PI * *f_bands_nrm.offset(b as isize)) as core::ffi::c_double,
                        )
                    + a2 as core::ffi::c_double
                        * cos(
                            (PI * 2 as core::ffi::c_int as FLOAT32
                                * *f_bands_nrm.offset(b as isize)) as core::ffi::c_double,
                        )) as FLOAT32;
                den_imag = (-a1 as core::ffi::c_double
                    * sin((PI * *f_bands_nrm.offset(b as isize)) as core::ffi::c_double)
                    - a2 as core::ffi::c_double
                        * sin(
                            (PI * 2 as core::ffi::c_int as FLOAT32
                                * *f_bands_nrm.offset(b as isize)) as core::ffi::c_double,
                        )) as FLOAT32;
                *weighting_filt.offset(b as isize)
                    *= sqrt(
                        ((num_real * num_real + num_imag * num_imag)
                            / (den_real * den_real + den_imag * den_imag))
                            as core::ffi::c_double,
                    ) as FLOAT32;
            }
            b += 1;
        }
        if sub_band_compensation_type == 1 as core::ffi::c_int {
            w0 = (2.0f32 * PI * 38.0f32 / sampling_rate as core::ffi::c_float
                * AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_QMF64 as core::ffi::c_float)
                as FLOAT32;
            sinw0 = sin(w0 as core::ffi::c_double) as FLOAT32;
            cosw0 = cos(w0 as core::ffi::c_double) as FLOAT32;
            alpha = sinw0;
            b0 = (1 as core::ffi::c_int as FLOAT32 + cosw0)
                / 2 as core::ffi::c_int as FLOAT32;
            b1 = -(1 as core::ffi::c_int as FLOAT32 + cosw0);
            b2 = (1 as core::ffi::c_int as FLOAT32 + cosw0)
                / 2 as core::ffi::c_int as FLOAT32;
            a0 = 1 as core::ffi::c_int as FLOAT32 + alpha;
            a1 = -(2 as core::ffi::c_int) as FLOAT32 * cosw0;
            a2 = 1 as core::ffi::c_int as FLOAT32 - alpha;
            (*filt_coeff_subband).b0 = b0 / a0;
            (*filt_coeff_subband).b1 = b1 / a0;
            (*filt_coeff_subband).b2 = b2 / a0;
            (*filt_coeff_subband).a1 = a1 / a0;
            (*filt_coeff_subband).a2 = a2 / a0;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parametric_ffwd_type_drc_reset(
    mut pstr_parametric_ffwd_type_drc_params: *mut ia_parametric_drc_type_ff_params_struct,
) -> WORD32 {
    let mut i: WORD32 = 0 as WORD32;
    (*pstr_parametric_ffwd_type_drc_params).level_estim_frame_index = 0
        as core::ffi::c_int as WORD32;
    (*pstr_parametric_ffwd_type_drc_params).start_up_phase = 1 as core::ffi::c_int
        as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < PARAM_DRC_TYPE_FF_LEVEL_ESTIM_FRAME_COUNT_MAX {
        (*pstr_parametric_ffwd_type_drc_params).level[i as usize] = 0.0f32 as FLOAT32;
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < MAX_CHANNEL_COUNT {
        (*pstr_parametric_ffwd_type_drc_params).pre_filt_state[i as usize].z1 = 0.0f32
            as FLOAT32;
        (*pstr_parametric_ffwd_type_drc_params).pre_filt_state[i as usize].z2 = 0.0f32
            as FLOAT32;
        (*pstr_parametric_ffwd_type_drc_params).rlb_filt_state[i as usize].z1 = 0.0f32
            as FLOAT32;
        (*pstr_parametric_ffwd_type_drc_params).rlb_filt_state[i as usize].z2 = 0.0f32
            as FLOAT32;
        (*pstr_parametric_ffwd_type_drc_params).filt_state_subband_real[i as usize].z1 = 0.0f32
            as FLOAT32;
        (*pstr_parametric_ffwd_type_drc_params).filt_state_subband_real[i as usize].z2 = 0.0f32
            as FLOAT32;
        (*pstr_parametric_ffwd_type_drc_params).filt_state_subband_imag[i as usize].z1 = 0.0f32
            as FLOAT32;
        (*pstr_parametric_ffwd_type_drc_params).filt_state_subband_imag[i as usize].z2 = 0.0f32
            as FLOAT32;
        i += 1;
    }
    (*pstr_parametric_ffwd_type_drc_params).db_level_smooth = -135.0f32 as FLOAT32;
    (*pstr_parametric_ffwd_type_drc_params).db_gain_smooth = 0.0f32 as FLOAT32;
    (*pstr_parametric_ffwd_type_drc_params).hold_counter = 0 as core::ffi::c_int
        as WORD32;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parametric_drc_instance_process(
    mut audio_in_out_buf: *mut *mut FLOAT32,
    mut audio_real_buff: *mut *mut FLOAT32,
    mut audio_imag_buff: *mut *mut FLOAT32,
    mut p_parametricdrc_params: *mut ia_parametric_drc_params_struct,
    mut pstr_parametric_drc_instance_params: *mut ia_parametric_drc_instance_params_struct,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut i: WORD32 = 0 as WORD32;
    if (*pstr_parametric_drc_instance_params).disable_paramteric_drc != 0 {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*p_parametricdrc_params).num_nodes {
            (*pstr_parametric_drc_instance_params)
                .str_spline_nodes
                .str_node[i as usize]
                .loc_db_gain = 0.0f32 as FLOAT32;
            (*pstr_parametric_drc_instance_params)
                .str_spline_nodes
                .str_node[i as usize]
                .slope = 0.0f32 as FLOAT32;
            (*pstr_parametric_drc_instance_params)
                .str_spline_nodes
                .str_node[i as usize]
                .time = ((i as core::ffi::c_int + 1 as core::ffi::c_int)
                * (*p_parametricdrc_params).parametric_drc_frame_size as core::ffi::c_int
                - 1 as core::ffi::c_int) as WORD32;
            i += 1;
        }
    } else if (*pstr_parametric_drc_instance_params).parametric_drc_type
        == PARAM_DRC_TYPE_FF
    {
        let mut pstr_parametric_ffwd_type_drc_params: *mut ia_parametric_drc_type_ff_params_struct = &mut (*pstr_parametric_drc_instance_params)
            .str_parametric_drc_type_ff_params;
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*p_parametricdrc_params).num_nodes {
            err = impd_parametric_ffwd_type_drc_process(
                audio_in_out_buf,
                audio_real_buff,
                audio_imag_buff,
                i,
                pstr_parametric_ffwd_type_drc_params,
                &mut (*pstr_parametric_drc_instance_params).str_spline_nodes,
            );
            if err != 0 {
                return err;
            }
            i += 1;
        }
    } else if (*pstr_parametric_drc_instance_params).parametric_drc_type
        == PARAM_DRC_TYPE_LIM
    {
        return 2 as WORD32
    } else {
        return 2 as WORD32
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn iir_second_order_filter(
    mut coeff: *mut ia_2nd_order_filt_coeff_struct,
    mut state: *mut ia_2nd_order_filt_state_struct,
    mut frame_len: WORD32,
    mut input: *mut FLOAT32,
    mut output: *mut FLOAT32,
) -> VOID {
    let mut z2: FLOAT32 = (*state).z2;
    let mut z1: FLOAT32 = (*state).z1;
    let mut z0: FLOAT32 = 0.;
    let mut i: WORD32 = 0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < frame_len {
        z0 = *input.offset(i as isize) - (*coeff).a1 * z1 - (*coeff).a2 * z2;
        *output.offset(i as isize) = (*coeff).b0 * z0 + (*coeff).b1 * z1
            + (*coeff).b2 * z2;
        z2 = z1;
        z1 = z0;
        i += 1;
    }
    (*state).z1 = z1;
    (*state).z2 = z2;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parametric_ffwd_type_drc_process(
    mut audio_in_out_buf: *mut *mut FLOAT32,
    mut audio_real_buff: *mut *mut FLOAT32,
    mut audio_imag_buff: *mut *mut FLOAT32,
    mut nodeIdx: WORD32,
    mut pstr_parametric_ffwd_type_drc_params: *mut ia_parametric_drc_type_ff_params_struct,
    mut str_spline_nodes: *mut ia_spline_nodes_struct,
) -> WORD32 {
    let mut c: WORD32 = 0;
    let mut t: WORD32 = 0;
    let mut b: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut offset: WORD32 = 0;
    let mut x: FLOAT32 = 0.;
    let mut y: FLOAT32 = 0.;
    let mut channelLevel: FLOAT32 = 0.;
    let mut level: FLOAT32 = 0.;
    let mut levelDb: FLOAT32 = 0.;
    let mut loc_db_gain: FLOAT32 = 0.;
    let mut levelDelta: FLOAT32 = 0.;
    let mut alpha: FLOAT32 = 0.;
    let mut frame_size: WORD32 = (*pstr_parametric_ffwd_type_drc_params).frame_size;
    let mut sub_band_count: WORD32 = (*pstr_parametric_ffwd_type_drc_params)
        .sub_band_count;
    let mut level_estim_ch_weight: *mut FLOAT32 = ((*pstr_parametric_ffwd_type_drc_params)
        .level_estim_ch_weight)
        .as_mut_ptr();
    let mut level_estim_k_weighting_type: WORD32 = (*pstr_parametric_ffwd_type_drc_params)
        .level_estim_k_weighting_type;
    let mut preC: ia_2nd_order_filt_coeff_struct = (*pstr_parametric_ffwd_type_drc_params)
        .pre_filt_coeff;
    let mut rlbC: ia_2nd_order_filt_coeff_struct = (*pstr_parametric_ffwd_type_drc_params)
        .rlb_filt_coeff;
    let mut preS: *mut ia_2nd_order_filt_state_struct = ((*pstr_parametric_ffwd_type_drc_params)
        .pre_filt_state)
        .as_mut_ptr();
    let mut rlbS: *mut ia_2nd_order_filt_state_struct = ((*pstr_parametric_ffwd_type_drc_params)
        .rlb_filt_state)
        .as_mut_ptr();
    let mut rlbC_sb: ia_2nd_order_filt_coeff_struct = (*pstr_parametric_ffwd_type_drc_params)
        .filt_coeff_subband;
    let mut rlbS_sbReal: *mut ia_2nd_order_filt_state_struct = ((*pstr_parametric_ffwd_type_drc_params)
        .filt_state_subband_real)
        .as_mut_ptr();
    let mut rlbS_sbImag: *mut ia_2nd_order_filt_state_struct = ((*pstr_parametric_ffwd_type_drc_params)
        .filt_state_subband_imag)
        .as_mut_ptr();
    let mut weighting_filt: *mut FLOAT32 = ((*pstr_parametric_ffwd_type_drc_params)
        .weighting_filt)
        .as_mut_ptr();
    let mut sub_band_compensation_type: WORD32 = (*pstr_parametric_ffwd_type_drc_params)
        .sub_band_compensation_type;
    if !audio_in_out_buf.is_null() {
        level = 0 as core::ffi::c_int as FLOAT32;
        offset = nodeIdx * (*pstr_parametric_ffwd_type_drc_params).frame_size;
        c = 0 as core::ffi::c_int as WORD32;
        while c < (*pstr_parametric_ffwd_type_drc_params).audio_num_chan {
            channelLevel = 0.0f32 as FLOAT32;
            if !(*level_estim_ch_weight.offset(c as isize) == 0.) {
                if level_estim_k_weighting_type == 0 as core::ffi::c_int {
                    t = 0 as core::ffi::c_int as WORD32;
                    while t < frame_size {
                        x = *(*audio_in_out_buf.offset(c as isize))
                            .offset((offset + t) as isize);
                        channelLevel += x * x;
                        t += 1;
                    }
                } else if level_estim_k_weighting_type == 1 as core::ffi::c_int {
                    t = 0 as core::ffi::c_int as WORD32;
                    while t < frame_size {
                        x = *(*audio_in_out_buf.offset(c as isize))
                            .offset((offset + t) as isize);
                        iir_second_order_filter(
                            &mut rlbC,
                            &mut *rlbS.offset(c as isize),
                            1 as WORD32,
                            &mut x,
                            &mut x,
                        );
                        channelLevel += x * x;
                        t += 1;
                    }
                } else if level_estim_k_weighting_type == 2 as core::ffi::c_int {
                    t = 0 as core::ffi::c_int as WORD32;
                    while t < frame_size {
                        x = *(*audio_in_out_buf.offset(c as isize))
                            .offset((offset + t) as isize);
                        iir_second_order_filter(
                            &mut preC,
                            &mut *preS.offset(c as isize),
                            1 as WORD32,
                            &mut x,
                            &mut x,
                        );
                        iir_second_order_filter(
                            &mut rlbC,
                            &mut *rlbS.offset(c as isize),
                            1 as WORD32,
                            &mut x,
                            &mut x,
                        );
                        channelLevel += x * x;
                        t += 1;
                    }
                } else {
                    return 2 as WORD32
                }
                level += *level_estim_ch_weight.offset(c as isize) * channelLevel;
            }
            c += 1;
        }
    } else {
        level = 0 as core::ffi::c_int as FLOAT32;
        offset = nodeIdx * (*pstr_parametric_ffwd_type_drc_params).sub_band_count;
        c = 0 as core::ffi::c_int as WORD32;
        while c < (*pstr_parametric_ffwd_type_drc_params).audio_num_chan {
            channelLevel = 0.0f32 as FLOAT32;
            if !(*level_estim_ch_weight.offset(c as isize) == 0.) {
                if level_estim_k_weighting_type == 0 as core::ffi::c_int {
                    b = 0 as core::ffi::c_int as WORD32;
                    while b < sub_band_count {
                        x = *(*audio_real_buff.offset(c as isize))
                            .offset((offset + b) as isize);
                        y = *(*audio_imag_buff.offset(c as isize))
                            .offset((offset + b) as isize);
                        channelLevel += x * x + y * y;
                        b += 1;
                    }
                } else if level_estim_k_weighting_type == 1 as core::ffi::c_int
                    || level_estim_k_weighting_type == 2 as core::ffi::c_int
                {
                    b = 0 as core::ffi::c_int as WORD32;
                    while b < sub_band_count {
                        x = *(*audio_real_buff.offset(c as isize))
                            .offset((offset + b) as isize)
                            * *weighting_filt.offset(b as isize);
                        y = *(*audio_imag_buff.offset(c as isize))
                            .offset((offset + b) as isize)
                            * *weighting_filt.offset(b as isize);
                        if b == 0 as core::ffi::c_int
                            && sub_band_compensation_type == 1 as core::ffi::c_int
                        {
                            iir_second_order_filter(
                                &mut rlbC_sb,
                                &mut *rlbS_sbReal.offset(c as isize),
                                1 as WORD32,
                                &mut x,
                                &mut x,
                            );
                            iir_second_order_filter(
                                &mut rlbC_sb,
                                &mut *rlbS_sbImag.offset(c as isize),
                                1 as WORD32,
                                &mut y,
                                &mut y,
                            );
                        }
                        channelLevel += x * x + y * y;
                        b += 1;
                    }
                } else {
                    return 2 as WORD32
                }
                level += *level_estim_ch_weight.offset(c as isize) * channelLevel;
            }
            c += 1;
        }
        level /= sub_band_count as FLOAT32;
    }
    (*pstr_parametric_ffwd_type_drc_params)
        .level[(*pstr_parametric_ffwd_type_drc_params).level_estim_frame_index
        as usize] = level;
    (*pstr_parametric_ffwd_type_drc_params).level_estim_frame_index += 1;
    level = 0.0f32 as FLOAT32;
    if (*pstr_parametric_ffwd_type_drc_params).start_up_phase != 0 {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*pstr_parametric_ffwd_type_drc_params).level_estim_frame_index {
            level += (*pstr_parametric_ffwd_type_drc_params).level[i as usize];
            i += 1;
        }
        level
            /= ((*pstr_parametric_ffwd_type_drc_params).level_estim_frame_index
                * (*pstr_parametric_ffwd_type_drc_params).frame_size) as FLOAT32;
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*pstr_parametric_ffwd_type_drc_params).level_estim_frame_count {
            level += (*pstr_parametric_ffwd_type_drc_params).level[i as usize];
            i += 1;
        }
        level
            /= (*pstr_parametric_ffwd_type_drc_params).level_estim_integration_time
                as FLOAT32;
    }
    if (*pstr_parametric_ffwd_type_drc_params).level_estim_frame_index
        == (*pstr_parametric_ffwd_type_drc_params).level_estim_frame_count
    {
        (*pstr_parametric_ffwd_type_drc_params).level_estim_frame_index = 0
            as core::ffi::c_int as WORD32;
        (*pstr_parametric_ffwd_type_drc_params).start_up_phase = 0 as core::ffi::c_int
            as WORD32;
    }
    if level < 1e-10f32 {
        level = 1e-10f32 as FLOAT32;
    }
    if level_estim_k_weighting_type == 2 as core::ffi::c_int {
        levelDb = (-0.691f32
            + 10 as core::ffi::c_int as core::ffi::c_float
                * log10(level as core::ffi::c_double) as core::ffi::c_float
            + 3 as core::ffi::c_int as core::ffi::c_float) as FLOAT32;
    } else {
        levelDb = 10 as core::ffi::c_int as FLOAT32
            * log10(level as core::ffi::c_double) as FLOAT32
            + 3 as core::ffi::c_int as FLOAT32;
    }
    levelDb -= (*pstr_parametric_ffwd_type_drc_params).ref_level_parametric_drc;
    n = 0 as core::ffi::c_int as WORD32;
    while n < (*pstr_parametric_ffwd_type_drc_params).node_count {
        if levelDb
            <= (*pstr_parametric_ffwd_type_drc_params).node_level[n as usize] as FLOAT32
        {
            break;
        }
        n += 1;
    }
    if n == 0 as core::ffi::c_int {
        loc_db_gain = (*pstr_parametric_ffwd_type_drc_params).node_gain[n as usize]
            as FLOAT32;
    } else if n == (*pstr_parametric_ffwd_type_drc_params).node_count {
        loc_db_gain = (*pstr_parametric_ffwd_type_drc_params)
            .node_gain[(n as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
            as FLOAT32 - levelDb
            + (*pstr_parametric_ffwd_type_drc_params)
                .node_level[(n as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
                as FLOAT32;
    } else {
        loc_db_gain = (*pstr_parametric_ffwd_type_drc_params).node_gain[n as usize]
            as FLOAT32
            + (levelDb
                - (*pstr_parametric_ffwd_type_drc_params).node_level[n as usize]
                    as FLOAT32)
                / ((*pstr_parametric_ffwd_type_drc_params)
                    .node_level[(n as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
                    - (*pstr_parametric_ffwd_type_drc_params).node_level[n as usize])
                    as FLOAT32
                * ((*pstr_parametric_ffwd_type_drc_params)
                    .node_gain[(n as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
                    - (*pstr_parametric_ffwd_type_drc_params).node_gain[n as usize])
                    as FLOAT32;
    }
    levelDelta = levelDb - (*pstr_parametric_ffwd_type_drc_params).db_level_smooth;
    if loc_db_gain < (*pstr_parametric_ffwd_type_drc_params).db_gain_smooth {
        if levelDelta
            > (*pstr_parametric_ffwd_type_drc_params).gain_smooth_attack_threshold
                as FLOAT32
        {
            alpha = (*pstr_parametric_ffwd_type_drc_params)
                .gain_smooth_attack_alpha_fast;
        } else {
            alpha = (*pstr_parametric_ffwd_type_drc_params)
                .gain_smooth_attack_alpha_slow;
        }
    } else if levelDelta
        < -(*pstr_parametric_ffwd_type_drc_params).gain_smooth_rel_threshold as FLOAT32
    {
        alpha = (*pstr_parametric_ffwd_type_drc_params).gain_smooth_rel_alpha_fast;
    } else {
        alpha = (*pstr_parametric_ffwd_type_drc_params).gain_smooth_rel_alpha_slow;
    }
    if loc_db_gain < (*pstr_parametric_ffwd_type_drc_params).db_gain_smooth
        || (*pstr_parametric_ffwd_type_drc_params).hold_counter == 0 as core::ffi::c_int
    {
        (*pstr_parametric_ffwd_type_drc_params).db_level_smooth = (1 as core::ffi::c_int
            as FLOAT32 - alpha) * (*pstr_parametric_ffwd_type_drc_params).db_level_smooth
            + alpha * levelDb;
        (*pstr_parametric_ffwd_type_drc_params).db_gain_smooth = (1 as core::ffi::c_int
            as FLOAT32 - alpha) * (*pstr_parametric_ffwd_type_drc_params).db_gain_smooth
            + alpha * loc_db_gain;
    }
    if (*pstr_parametric_ffwd_type_drc_params).hold_counter != 0 {
        (*pstr_parametric_ffwd_type_drc_params).hold_counter -= 1 as core::ffi::c_int;
    }
    if loc_db_gain < (*pstr_parametric_ffwd_type_drc_params).db_gain_smooth {
        (*pstr_parametric_ffwd_type_drc_params).hold_counter = (*pstr_parametric_ffwd_type_drc_params)
            .gain_smooth_hold_off_count;
    }
    (*str_spline_nodes).str_node[nodeIdx as usize].loc_db_gain = (*pstr_parametric_ffwd_type_drc_params)
        .db_gain_smooth;
    (*str_spline_nodes).str_node[nodeIdx as usize].slope = 0.0f32 as FLOAT32;
    (*str_spline_nodes).str_node[nodeIdx as usize].time = ((*pstr_parametric_ffwd_type_drc_params)
        .frame_size as core::ffi::c_int + offset as core::ffi::c_int
        - 1 as core::ffi::c_int) as WORD32;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parametric_lim_type_drc_process(
    mut samples: *mut *mut FLOAT32,
    mut loudness_normalization_gain_db: FLOAT32,
    mut pstr_parametric_lim_type_drc_params: *mut ia_parametric_drc_type_lim_params_struct,
    mut lpcm_gains: *mut FLOAT32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut tmp: FLOAT32 = 0.;
    let mut gain: FLOAT32 = 0.;
    let mut maximum: FLOAT32 = 0.;
    let mut sectionMaximum: FLOAT32 = 0.;
    let mut loudness_normalization_gain: FLOAT32 = pow(
        10.0f64,
        (0.05f32 * loudness_normalization_gain_db) as core::ffi::c_double,
    ) as FLOAT32;
    let mut level_estim_ch_weight: *mut FLOAT32 = ((*pstr_parametric_lim_type_drc_params)
        .level_estim_ch_weight)
        .as_mut_ptr();
    let mut num_channels: WORD32 = (*pstr_parametric_lim_type_drc_params).channels
        as WORD32;
    let mut attack_time_samples: WORD32 = (*pstr_parametric_lim_type_drc_params).attack
        as WORD32;
    let mut attack_constant: FLOAT32 = (*pstr_parametric_lim_type_drc_params)
        .attack_constant;
    let mut release_constant: FLOAT32 = (*pstr_parametric_lim_type_drc_params)
        .release_constant;
    let mut limit_threshold: FLOAT32 = (*pstr_parametric_lim_type_drc_params).threshold;
    let mut max_buf: *mut FLOAT32 = (*pstr_parametric_lim_type_drc_params).max_buf;
    let mut gain_modified: FLOAT32 = (*pstr_parametric_lim_type_drc_params).cor;
    let mut pre_smoothed_gain: FLOAT64 = (*pstr_parametric_lim_type_drc_params)
        .smooth_state_0;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*pstr_parametric_lim_type_drc_params).frame_size {
        tmp = 0.0f32 as FLOAT32;
        j = 0 as core::ffi::c_int as WORD32;
        while j < num_channels {
            if !(*level_estim_ch_weight.offset(j as isize) == 0.) {
                tmp = if tmp
                    > fabs(
                        (loudness_normalization_gain
                            * *level_estim_ch_weight.offset(j as isize)
                            * *(*samples.offset(j as isize)).offset(i as isize))
                            as core::ffi::c_double,
                    ) as FLOAT32
                {
                    tmp
                } else {
                    fabs(
                        (loudness_normalization_gain
                            * *level_estim_ch_weight.offset(j as isize)
                            * *(*samples.offset(j as isize)).offset(i as isize))
                            as core::ffi::c_double,
                    ) as FLOAT32
                };
            }
            j += 1;
        }
        j = attack_time_samples;
        while j > 0 as core::ffi::c_int {
            *max_buf.offset(j as isize) = *max_buf
                .offset((j as core::ffi::c_int - 1 as core::ffi::c_int) as isize);
            j -= 1;
        }
        *max_buf.offset(0 as core::ffi::c_int as isize) = tmp;
        sectionMaximum = tmp;
        j = 1 as core::ffi::c_int as WORD32;
        while j < attack_time_samples as core::ffi::c_int + 1 as core::ffi::c_int {
            if *max_buf.offset(j as isize) > sectionMaximum {
                sectionMaximum = *max_buf.offset(j as isize);
            }
            j += 1;
        }
        maximum = sectionMaximum;
        if maximum > limit_threshold {
            gain = limit_threshold / maximum;
        } else {
            gain = 1 as core::ffi::c_int as FLOAT32;
        }
        if (gain as FLOAT64) < pre_smoothed_gain {
            gain_modified = (if gain_modified
                < (gain as core::ffi::c_float
                    - 0.1f32 * pre_smoothed_gain as core::ffi::c_float) * 1.11111111f32
            {
                gain_modified as core::ffi::c_float
            } else {
                (gain as core::ffi::c_float
                    - 0.1f32 * pre_smoothed_gain as core::ffi::c_float) * 1.11111111f32
            }) as FLOAT32;
        } else {
            gain_modified = gain;
        }
        if (gain_modified as FLOAT64) < pre_smoothed_gain {
            pre_smoothed_gain = attack_constant as FLOAT64
                * (pre_smoothed_gain - gain_modified as FLOAT64)
                + gain_modified as FLOAT64;
            pre_smoothed_gain = if pre_smoothed_gain > gain as FLOAT64 {
                pre_smoothed_gain
            } else {
                gain as FLOAT64
            };
        } else {
            pre_smoothed_gain = release_constant as FLOAT64
                * (pre_smoothed_gain - gain_modified as FLOAT64)
                + gain_modified as FLOAT64;
        }
        gain = pre_smoothed_gain as FLOAT32;
        *lpcm_gains.offset(i as isize) = gain;
        i += 1;
    }
    (*pstr_parametric_lim_type_drc_params).cor = gain_modified;
    (*pstr_parametric_lim_type_drc_params).smooth_state_0 = pre_smoothed_gain;
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
