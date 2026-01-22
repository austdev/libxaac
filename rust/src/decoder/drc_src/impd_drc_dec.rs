extern "C" {
    fn impd_get_delta_tmin(sampling_rate: WORD32) -> WORD32;
    fn impd_init_all_filter_banks(
        str_p_loc_drc_coefficients_uni_drc: *mut ia_uni_drc_coeffs_struct,
        str_drc_instruction_str: *mut ia_drc_instructions_struct,
        ia_filter_banks_struct: *mut ia_filter_banks_struct,
    ) -> WORD32;
    fn impd_shape_filt_block_init(
        pstr_shape_filter_block_params: *mut ia_shape_filter_block_params_struct,
        shape_filter_block: *mut shape_filter_block,
    ) -> VOID;
    fn impd_init_overlap_weight(
        str_p_loc_drc_coefficients_uni_drc: *mut ia_uni_drc_coeffs_struct,
        str_drc_instruction_str: *mut ia_drc_instructions_struct,
        sub_band_domain_mode: WORD32,
        pstr_overlap_params: *mut ia_overlap_params_struct,
    ) -> VOID;
}
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
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AUDIO_CODEC_FRAME_SIZE_MAX: core::ffi::c_int = 4096 as core::ffi::c_int;
pub const SEL_DRC_COUNT: core::ffi::c_int = 3 as core::ffi::c_int;
pub const MAX_SIGNAL_DELAY: core::ffi::c_int = 4500 as core::ffi::c_int;
pub const DELAY_MODE_REGULAR_DELAY: core::ffi::c_int = 0 as core::ffi::c_int;
pub const DELAY_MODE_LOW_DELAY: core::ffi::c_int = 1 as core::ffi::c_int;
pub const UNEXPECTED_ERROR: core::ffi::c_int = 2 as core::ffi::c_int;
pub const ID_FOR_BASE_LAYOUT: core::ffi::c_int = 0 as core::ffi::c_int;
pub const ID_FOR_ANY_DOWNMIX: core::ffi::c_int = 0x7f as core::ffi::c_int;
pub const SUBBAND_DOMAIN_MODE_OFF: core::ffi::c_int = 0;
pub const SUBBAND_DOMAIN_MODE_QMF64: core::ffi::c_int = 1;
pub const SUBBAND_DOMAIN_MODE_QMF71: core::ffi::c_int = 2;
pub const SUBBAND_DOMAIN_MODE_STFT256: core::ffi::c_int = 3;
pub const PARAM_DRC_TYPE_FF: core::ffi::c_int = 0 as core::ffi::c_int;
pub const PARAM_DRC_TYPE_LIM: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const EFFECT_BIT_DUCK_OTHER: core::ffi::c_int = 0x400 as core::ffi::c_int;
pub const EFFECT_BIT_DUCK_SELF: core::ffi::c_int = 0x800 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn impd_init_drc_params(
    mut frame_size: WORD32,
    mut sample_rate: WORD32,
    mut gain_delay_samples: WORD32,
    mut delay_mode: WORD32,
    mut sub_band_domain_mode: WORD32,
    mut ia_drc_params_struct: *mut ia_drc_params_struct,
) -> WORD32 {
    let mut k: WORD32 = 0;
    if frame_size < 1 as core::ffi::c_int || frame_size > AUDIO_CODEC_FRAME_SIZE_MAX {
        return -(1 as WORD32);
    }
    if sample_rate < 1000 as core::ffi::c_int {
        return -(1 as WORD32);
    }
    (*ia_drc_params_struct).drc_frame_size = frame_size;
    if ((*ia_drc_params_struct).drc_frame_size as core::ffi::c_float)
        < 0.001f32 * sample_rate as core::ffi::c_float
    {
        return -(1 as WORD32);
    }
    (*ia_drc_params_struct).sample_rate = sample_rate;
    (*ia_drc_params_struct).delta_tmin_default = impd_get_delta_tmin(sample_rate);
    if (*ia_drc_params_struct).delta_tmin_default
        > (*ia_drc_params_struct).drc_frame_size
    {
        return -(1 as WORD32);
    }
    if delay_mode != DELAY_MODE_REGULAR_DELAY && delay_mode != DELAY_MODE_LOW_DELAY {
        return -(1 as WORD32);
    }
    (*ia_drc_params_struct).delay_mode = delay_mode;
    (*ia_drc_params_struct).drc_set_counter = 0 as core::ffi::c_int as WORD32;
    (*ia_drc_params_struct).multiband_sel_drc_idx = -(1 as core::ffi::c_int) as WORD32;
    k = 0 as core::ffi::c_int as WORD32;
    while k < SEL_DRC_COUNT {
        (*ia_drc_params_struct).sel_drc_array[k as usize].drc_instructions_index = -(1
            as core::ffi::c_int) as WORD32;
        (*ia_drc_params_struct).sel_drc_array[k as usize].dwnmix_instructions_index = -(1
            as core::ffi::c_int) as WORD32;
        (*ia_drc_params_struct).sel_drc_array[k as usize].drc_coeff_idx = -(1
            as core::ffi::c_int) as WORD32;
        k += 1;
    }
    if gain_delay_samples > MAX_SIGNAL_DELAY
        || gain_delay_samples < 0 as core::ffi::c_int
    {
        return -(1 as WORD32)
    } else {
        (*ia_drc_params_struct).gain_delay_samples = gain_delay_samples;
    }
    match sub_band_domain_mode {
        SUBBAND_DOMAIN_MODE_OFF
        | SUBBAND_DOMAIN_MODE_QMF64
        | SUBBAND_DOMAIN_MODE_QMF71
        | SUBBAND_DOMAIN_MODE_STFT256 => {
            (*ia_drc_params_struct).sub_band_domain_mode = sub_band_domain_mode;
        }
        _ => return -(1 as WORD32),
    }
    (*ia_drc_params_struct).parametric_drc_delay = 0 as core::ffi::c_int as WORD32;
    (*ia_drc_params_struct).eq_delay = 0 as core::ffi::c_int as WORD32;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_select_drc_coefficients(
    mut drc_config: *mut ia_drc_config,
    mut drc_coefficients_drc: *mut *mut ia_uni_drc_coeffs_struct,
    mut drc_coefficients_selected: *mut WORD32,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut cof1: WORD32 = -(1 as WORD32);
    let mut cof0: WORD32 = -(1 as WORD32);
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*drc_config).drc_coefficients_drc_count {
        if (*drc_config).str_p_loc_drc_coefficients_uni_drc[i as usize].drc_location
            == 1 as core::ffi::c_int
        {
            if (*drc_config).str_p_loc_drc_coefficients_uni_drc[i as usize].version
                == 0 as core::ffi::c_int
            {
                cof0 = i;
                *drc_coefficients_selected = cof0;
            } else {
                cof1 = i;
                *drc_coefficients_selected = cof1;
            }
        }
        i += 1;
    }
    if cof1 >= 0 as core::ffi::c_int {
        *drc_coefficients_drc = &mut *((*drc_config).str_p_loc_drc_coefficients_uni_drc)
            .as_mut_ptr()
            .offset(cof1 as isize) as *mut ia_uni_drc_coeffs_struct;
    } else if cof0 >= 0 as core::ffi::c_int {
        *drc_coefficients_drc = &mut *((*drc_config).str_p_loc_drc_coefficients_uni_drc)
            .as_mut_ptr()
            .offset(cof0 as isize) as *mut ia_uni_drc_coeffs_struct;
    } else {
        *drc_coefficients_drc = 0 as *mut ia_uni_drc_coeffs_struct;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_init_selected_drc_set(
    mut drc_config: *mut ia_drc_config,
    mut ia_drc_params_struct: *mut ia_drc_params_struct,
    mut p_parametric_drc_params: *mut ia_parametric_drc_params_struct,
    mut audio_num_chan: WORD32,
    mut drc_set_id_selected: WORD32,
    mut downmix_id_selected: WORD32,
    mut ia_filter_banks_struct: *mut ia_filter_banks_struct,
    mut pstr_overlap_params: *mut ia_overlap_params_struct,
    mut shape_filter_block: *mut shape_filter_block,
) -> WORD32 {
    let mut g: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut c: WORD32 = 0;
    let mut err: WORD32 = 0 as WORD32;
    let mut channel_count: WORD32 = 0 as WORD32;
    let mut i: WORD32 = 0;
    let mut drc_instructions_uni_drc: *mut ia_drc_instructions_struct = 0
        as *mut ia_drc_instructions_struct;
    let mut drc_coefficients_uni_drc: *mut ia_uni_drc_coeffs_struct = 0
        as *mut ia_uni_drc_coeffs_struct;
    let mut selected_drc_is_multiband: WORD32 = 0 as WORD32;
    let mut drc_instructions_selected: WORD32 = -(1 as WORD32);
    let mut downmix_instructions_selected: WORD32 = -(1 as WORD32);
    let mut drc_coefficients_selected: WORD32 = -(1 as WORD32);
    (*p_parametric_drc_params).parametric_drc_instance_count = 0 as core::ffi::c_int
        as WORD32;
    if (*drc_config).drc_coefficients_drc_count != 0
        && (*((*drc_config).str_p_loc_drc_coefficients_uni_drc).as_mut_ptr())
            .drc_frame_size_present != 0
    {
        if (*((*drc_config).str_p_loc_drc_coefficients_uni_drc).as_mut_ptr())
            .drc_frame_size != (*ia_drc_params_struct).drc_frame_size
        {
            return -(1 as WORD32);
        }
    }
    n = 0 as core::ffi::c_int as WORD32;
    while n < (*drc_config).drc_instructions_count_plus {
        if (*drc_config).str_drc_instruction_str[n as usize].drc_set_id
            == drc_set_id_selected
        {
            break;
        }
        n += 1;
    }
    if n == (*drc_config).drc_instructions_count_plus {
        return -(1 as WORD32);
    }
    drc_instructions_selected = n;
    drc_instructions_uni_drc = &mut *((*drc_config).str_drc_instruction_str)
        .as_mut_ptr()
        .offset(drc_instructions_selected as isize) as *mut ia_drc_instructions_struct;
    if downmix_id_selected == ID_FOR_BASE_LAYOUT {
        channel_count = (*drc_config).channel_layout.base_channel_count;
    } else if downmix_id_selected == ID_FOR_ANY_DOWNMIX {
        channel_count = audio_num_chan;
    } else {
        n = 0 as core::ffi::c_int as WORD32;
        while n < (*drc_config).dwnmix_instructions_count {
            if (*drc_config).dwnmix_instructions[n as usize].downmix_id
                == downmix_id_selected
            {
                break;
            }
            n += 1;
        }
        if n == (*drc_config).dwnmix_instructions_count {
            return 2 as WORD32;
        }
        channel_count = (*drc_config)
            .dwnmix_instructions[n as usize]
            .target_channel_count;
        downmix_instructions_selected = n;
    }
    (*drc_instructions_uni_drc).audio_num_chan = channel_count;
    if (*drc_instructions_uni_drc).drc_set_id <= 0 as core::ffi::c_int {
        drc_coefficients_selected = 0 as core::ffi::c_int as WORD32;
    } else {
        err = impd_select_drc_coefficients(
            drc_config,
            &mut drc_coefficients_uni_drc,
            &mut drc_coefficients_selected,
        );
        if err != 0 {
            return err;
        }
    }
    (*ia_drc_params_struct)
        .sel_drc_array[(*ia_drc_params_struct).drc_set_counter as usize]
        .drc_instructions_index = drc_instructions_selected;
    (*ia_drc_params_struct)
        .sel_drc_array[(*ia_drc_params_struct).drc_set_counter as usize]
        .dwnmix_instructions_index = downmix_instructions_selected;
    (*ia_drc_params_struct)
        .sel_drc_array[(*ia_drc_params_struct).drc_set_counter as usize]
        .drc_coeff_idx = drc_coefficients_selected;
    if (*drc_instructions_uni_drc).downmix_id[0 as core::ffi::c_int as usize]
        == ID_FOR_ANY_DOWNMIX
        || (*drc_instructions_uni_drc).dwnmix_id_count > 1 as core::ffi::c_int
    {
        let mut idx: WORD32 = (*drc_instructions_uni_drc)
            .gain_set_index[0 as core::ffi::c_int as usize];
        c = 0 as core::ffi::c_int as WORD32;
        while c < (*drc_instructions_uni_drc).audio_num_chan {
            (*drc_instructions_uni_drc).channel_group_of_ch[c as usize] = (if idx
                >= 0 as core::ffi::c_int
            {
                0 as core::ffi::c_int
            } else {
                -(1 as core::ffi::c_int)
            }) as WORD32;
            c += 1;
        }
    }
    g = 0 as core::ffi::c_int as WORD32;
    while g < (*drc_instructions_uni_drc).num_drc_ch_groups {
        (*drc_instructions_uni_drc).num_chan_per_ch_group[g as usize] = 0
            as core::ffi::c_int as WORD32;
        c = 0 as core::ffi::c_int as WORD32;
        while c < (*drc_instructions_uni_drc).audio_num_chan {
            if (*drc_instructions_uni_drc).channel_group_of_ch[c as usize] == g {
                (*drc_instructions_uni_drc).num_chan_per_ch_group[g as usize] += 1;
            }
            c += 1;
        }
        g += 1;
    }
    if (*drc_instructions_uni_drc).drc_set_effect as core::ffi::c_int
        & (EFFECT_BIT_DUCK_OTHER | EFFECT_BIT_DUCK_SELF) != 0
    {
        (*drc_instructions_uni_drc).multiband_audio_sig_count = (*drc_instructions_uni_drc)
            .audio_num_chan;
    } else {
        (*drc_instructions_uni_drc).multiband_audio_sig_count = 0 as core::ffi::c_int
            as WORD32;
        c = 0 as core::ffi::c_int as WORD32;
        while c < (*drc_instructions_uni_drc).audio_num_chan {
            g = (*drc_instructions_uni_drc).channel_group_of_ch[c as usize];
            if g < 0 as core::ffi::c_int {
                (*drc_instructions_uni_drc).multiband_audio_sig_count += 1;
            } else {
                (*drc_instructions_uni_drc).multiband_audio_sig_count
                    += (*drc_instructions_uni_drc).band_count_of_ch_group[g as usize];
            }
            c += 1;
        }
    }
    g = 0 as core::ffi::c_int as WORD32;
    while g < (*drc_instructions_uni_drc).num_drc_ch_groups {
        if g == 0 as core::ffi::c_int {
            (*drc_instructions_uni_drc).parametric_drc_look_ahead_samples_max = 0
                as core::ffi::c_int as WORD32;
        }
        if (*drc_instructions_uni_drc).ch_group_parametric_drc_flag[g as usize]
            == 0 as core::ffi::c_int
        {
            if (*drc_instructions_uni_drc).band_count_of_ch_group[g as usize]
                > 1 as core::ffi::c_int
            {
                if (*ia_drc_params_struct).multiband_sel_drc_idx
                    != -(1 as core::ffi::c_int)
                {
                    return 2 as WORD32;
                }
                selected_drc_is_multiband = 1 as core::ffi::c_int as WORD32;
            }
        } else {
            let mut gain_set_index: WORD32 = (*drc_instructions_uni_drc)
                .gain_set_idx_of_ch_group_parametric_drc[g as usize];
            let mut parametric_drc_id: WORD32 = (*drc_config)
                .str_drc_config_ext
                .str_drc_coeff_param_drc
                .str_parametric_drc_gain_set_params[gain_set_index as usize]
                .parametric_drc_id;
            let mut parametric_drc_look_ahead_samples: WORD32 = 0 as WORD32;
            let mut str_parametric_drc_instructions: *mut ia_parametric_drc_instructions_struct = 0
                as *mut ia_parametric_drc_instructions_struct;
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*drc_config).str_drc_config_ext.parametric_drc_instructions_count
            {
                if parametric_drc_id
                    == (*drc_config)
                        .str_drc_config_ext
                        .str_parametric_drc_instructions[i as usize]
                        .parametric_drc_id
                {
                    break;
                }
                i += 1;
            }
            if i == (*drc_config).str_drc_config_ext.parametric_drc_instructions_count {
                return 2 as WORD32;
            }
            str_parametric_drc_instructions = &mut *((*drc_config)
                .str_drc_config_ext
                .str_parametric_drc_instructions)
                .as_mut_ptr()
                .offset(i as isize) as *mut ia_parametric_drc_instructions_struct;
            (*p_parametric_drc_params)
                .parametric_drc_idx[(*p_parametric_drc_params)
                .parametric_drc_instance_count as usize] = i;
            (*p_parametric_drc_params)
                .gain_set_index[(*p_parametric_drc_params).parametric_drc_instance_count
                as usize] = gain_set_index;
            if (*drc_instructions_uni_drc).drc_apply_to_dwnmix == 0 as core::ffi::c_int {
                (*p_parametric_drc_params)
                    .dwnmix_id_from_drc_instructions[(*p_parametric_drc_params)
                    .parametric_drc_instance_count as usize] = ID_FOR_BASE_LAYOUT
                    as WORD32;
            } else if (*drc_instructions_uni_drc).dwnmix_id_count > 1 as core::ffi::c_int
            {
                (*p_parametric_drc_params)
                    .dwnmix_id_from_drc_instructions[(*p_parametric_drc_params)
                    .parametric_drc_instance_count as usize] = ID_FOR_ANY_DOWNMIX
                    as WORD32;
            } else {
                (*p_parametric_drc_params)
                    .dwnmix_id_from_drc_instructions[(*p_parametric_drc_params)
                    .parametric_drc_instance_count as usize] = (*drc_instructions_uni_drc)
                    .downmix_id[0 as core::ffi::c_int as usize];
            }
            (*p_parametric_drc_params).audio_num_chan = channel_count;
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*p_parametric_drc_params).audio_num_chan {
                if (*drc_instructions_uni_drc).channel_group_of_ch[i as usize] == g {
                    (*p_parametric_drc_params)
                        .channel_map[(*p_parametric_drc_params)
                        .parametric_drc_instance_count as usize][i as usize] = 1
                        as core::ffi::c_int as WORD32;
                } else {
                    (*p_parametric_drc_params)
                        .channel_map[(*p_parametric_drc_params)
                        .parametric_drc_instance_count as usize][i as usize] = 0
                        as core::ffi::c_int as WORD32;
                }
                i += 1;
            }
            (*drc_instructions_uni_drc).parametric_drc_look_ahead_samples[g as usize] = 0
                as core::ffi::c_int as WORD32;
            if (*str_parametric_drc_instructions).parametric_drc_look_ahead_flag != 0 {
                parametric_drc_look_ahead_samples = ((*str_parametric_drc_instructions)
                    .parametric_drc_look_ahead as core::ffi::c_float
                    * (*p_parametric_drc_params).sampling_rate as core::ffi::c_float
                    * 0.001f32) as WORD32;
            } else if (*str_parametric_drc_instructions).parametric_drc_type
                == PARAM_DRC_TYPE_FF
            {
                parametric_drc_look_ahead_samples = (10.0f32
                    * (*p_parametric_drc_params).sampling_rate as core::ffi::c_float
                    * 0.001f32) as WORD32;
            } else if (*str_parametric_drc_instructions).parametric_drc_type
                == PARAM_DRC_TYPE_LIM
            {
                parametric_drc_look_ahead_samples = (5.0f32
                    * (*p_parametric_drc_params).sampling_rate as core::ffi::c_float
                    * 0.001f32) as WORD32;
            } else {
                return 2 as WORD32
            }
            (*drc_instructions_uni_drc).parametric_drc_look_ahead_samples[g as usize] = parametric_drc_look_ahead_samples;
            if (*drc_instructions_uni_drc).parametric_drc_look_ahead_samples_max
                < (*drc_instructions_uni_drc)
                    .parametric_drc_look_ahead_samples[g as usize]
            {
                (*drc_instructions_uni_drc).parametric_drc_look_ahead_samples_max = (*drc_instructions_uni_drc)
                    .parametric_drc_look_ahead_samples[g as usize];
            }
            (*p_parametric_drc_params).parametric_drc_instance_count
                += 1 as core::ffi::c_int;
            selected_drc_is_multiband = 0 as core::ffi::c_int as WORD32;
        }
        g += 1;
    }
    (*ia_drc_params_struct).parametric_drc_delay
        += (*drc_instructions_uni_drc).parametric_drc_look_ahead_samples_max;
    if selected_drc_is_multiband == 1 as core::ffi::c_int {
        (*ia_drc_params_struct).multiband_sel_drc_idx = (*ia_drc_params_struct)
            .drc_set_counter;
        err = impd_init_all_filter_banks(
            drc_coefficients_uni_drc,
            &mut *((*drc_config).str_drc_instruction_str)
                .as_mut_ptr()
                .offset(drc_instructions_selected as isize),
            ia_filter_banks_struct,
        );
        if err != 0 {
            return err;
        }
        impd_init_overlap_weight(
            drc_coefficients_uni_drc,
            &mut *((*drc_config).str_drc_instruction_str)
                .as_mut_ptr()
                .offset(drc_instructions_selected as isize),
            (*ia_drc_params_struct).sub_band_domain_mode,
            pstr_overlap_params,
        );
    } else {
        let mut gain_modifiers: *mut ia_gain_modifiers_struct = ((*((*drc_config)
            .str_drc_instruction_str)
            .as_mut_ptr())
            .str_gain_modifiers_of_ch_group)
            .as_mut_ptr();
        g = 0 as core::ffi::c_int as WORD32;
        while g < (*drc_instructions_uni_drc).num_drc_ch_groups {
            if (*gain_modifiers.offset(g as isize)).shape_filter_flag
                == 1 as core::ffi::c_int
            {
                impd_shape_filt_block_init(
                    &mut *((*drc_coefficients_uni_drc).str_shape_filter_block_params)
                        .as_mut_ptr()
                        .offset(
                            (*gain_modifiers.offset(g as isize)).shape_filter_idx
                                as isize,
                        ),
                    &mut *shape_filter_block.offset(g as isize),
                );
            } else {
                (*shape_filter_block.offset(g as isize)).shape_flter_block_flag = 0
                    as core::ffi::c_int as WORD32;
            }
            g += 1;
        }
    }
    (*ia_drc_params_struct).drc_set_counter += 1;
    return 0 as WORD32;
}
