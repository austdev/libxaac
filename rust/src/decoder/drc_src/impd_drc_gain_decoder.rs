extern "C" {
    fn impd_init_parametric_drc(
        drc_frame_size: WORD32,
        sampling_rate: WORD32,
        sub_band_domain_mode: WORD32,
        pstr_parametric_drc_params: *mut ia_parametric_drc_params_struct,
    ) -> WORD32;
    fn impd_init_parametric_drc_after_config(
        pstr_drc_config: *mut ia_drc_config,
        pstr_loudness_info: *mut ia_drc_loudness_info_set_struct,
        pstr_parametric_drc_params: *mut ia_parametric_drc_params_struct,
        mem_ptr: *mut pVOID,
    ) -> WORD32;
    fn impd_derive_eq_set(
        str_eq_coeff: *mut ia_eq_coeff_struct,
        str_eq_instructions: *mut ia_eq_instructions_struct,
        sample_rate: FLOAT32,
        drc_frame_size: WORD32,
        sub_band_domain_mode: WORD32,
        eq_set: *mut ia_eq_set_struct,
    ) -> WORD32;
    fn impd_get_eq_set_delay(
        eq_set: *mut ia_eq_set_struct,
        cascade_delay: *mut WORD32,
    ) -> VOID;
    fn impd_process_eq_set_time_domain(
        eq_set: *mut ia_eq_set_struct,
        channel: WORD32,
        audio_in: *mut FLOAT32,
        audio_out: *mut FLOAT32,
        frame_size: WORD32,
    ) -> WORD32;
    fn impd_filter_banks_process(
        pstr_drc_instruction_arr: *mut ia_drc_instructions_struct,
        drc_instructions_index: WORD32,
        ia_drc_params_struct: *mut ia_drc_params_struct,
        audio_io_buf: *mut *mut FLOAT32,
        audio_band_buffer: *mut ia_audio_band_buffer_struct,
        ia_filter_banks_struct: *mut ia_filter_banks_struct,
        passThru: WORD32,
    ) -> WORD32;
    fn impd_store_audio_io_buffer_time(
        audio_in_out_buf: *mut *mut FLOAT32,
        audio_io_buf_internal: *mut ia_audio_in_out_buf,
    ) -> VOID;
    fn impd_retrieve_audio_io_buffer_time(
        audio_in_out_buf: *mut *mut FLOAT32,
        audio_io_buf_internal: *mut ia_audio_in_out_buf,
    ) -> VOID;
    fn impd_advance_audio_io_buffer_time(
        audio_io_buf_internal: *mut ia_audio_in_out_buf,
    ) -> VOID;
    fn impd_get_drc_gain(
        p_drc_gain_dec_structs: *mut ia_drc_gain_dec_struct,
        pstr_drc_config: *mut ia_drc_config,
        pstr_drc_gain: *mut ia_drc_gain_struct,
        compress: FLOAT32,
        boost: FLOAT32,
        characteristic_index: WORD32,
        loudness_normalization_gain_db: FLOAT32,
        sub_drc_index: WORD32,
        drc_gain_buffers: *mut ia_drc_gain_buffers_struct,
    ) -> WORD32;
    fn impd_init_drc_params(
        frame_size: WORD32,
        sample_rate: WORD32,
        gain_delay_samples: WORD32,
        delay_mode: WORD32,
        sub_band_domain_mode: WORD32,
        ia_drc_params_struct: *mut ia_drc_params_struct,
    ) -> WORD32;
    fn impd_init_selected_drc_set(
        drc_config: *mut ia_drc_config,
        ia_drc_params_struct: *mut ia_drc_params_struct,
        p_parametricdrc_params: *mut ia_parametric_drc_params_struct,
        audio_num_chan: WORD32,
        drc_set_id_selected: WORD32,
        downmix_id_selected: WORD32,
        ia_filter_banks_struct: *mut ia_filter_banks_struct,
        pstr_overlap_params: *mut ia_overlap_params_struct,
        shape_filter_block: *mut shape_filter_block,
    ) -> WORD32;
    fn impd_apply_gains_and_add(
        pstr_drc_instruction_arr: *mut ia_drc_instructions_struct,
        drc_instructions_index: WORD32,
        ia_drc_params_struct: *mut ia_drc_params_struct,
        pstr_gain_buf: *mut ia_gain_buffer_struct,
        shape_filter_block: *mut shape_filter_block,
        deinterleaved_audio: *mut *mut FLOAT32,
        channel_audio: *mut *mut FLOAT32,
        impd_apply_gains: WORD32,
    ) -> VOID;
}
pub type size_t = usize;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type FLOAT32 = core::ffi::c_float;
pub type FLOAT64 = core::ffi::c_double;
pub type VOID = ();
pub type pVOID = *mut core::ffi::c_void;
pub type IA_ERRORCODE = WORD32;
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
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AUDIO_CODEC_FRAME_SIZE_MAX: core::ffi::c_int = 4096 as core::ffi::c_int;
pub const SEL_DRC_COUNT: core::ffi::c_int = 3 as core::ffi::c_int;
pub const MAX_SIGNAL_DELAY: core::ffi::c_int = 4500 as core::ffi::c_int;
pub const SUBBAND_DOMAIN_MODE_OFF: core::ffi::c_int = 0 as core::ffi::c_int;
pub const SUBBAND_DOMAIN_MODE_QMF64: core::ffi::c_int = 1 as core::ffi::c_int;
pub const SUBBAND_DOMAIN_MODE_QMF71: core::ffi::c_int = 2 as core::ffi::c_int;
pub const SUBBAND_DOMAIN_MODE_STFT256: core::ffi::c_int = 3 as core::ffi::c_int;
pub const AUDIO_CODEC_SUBBAND_COUNT_QMF64: core::ffi::c_int = 64 as core::ffi::c_int;
pub const AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_QMF64: core::ffi::c_int = 64
    as core::ffi::c_int;
pub const AUDIO_CODEC_SUBBAND_COUNT_QMF71: core::ffi::c_int = 71 as core::ffi::c_int;
pub const AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_QMF71: core::ffi::c_int = 64
    as core::ffi::c_int;
pub const AUDIO_CODEC_SUBBAND_COUNT_STFT256: core::ffi::c_int = 256 as core::ffi::c_int;
pub const AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_STFT256: core::ffi::c_int = 256
    as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn impd_init_drc_decode(
    mut frame_size: WORD32,
    mut sample_rate: WORD32,
    mut gain_delay_samples: WORD32,
    mut delay_mode: WORD32,
    mut sub_band_domain_mode: WORD32,
    mut p_drc_gain_dec_structs: *mut ia_drc_gain_dec_struct,
) -> WORD32 {
    let mut err_code: IA_ERRORCODE = IA_NO_ERROR;
    err_code = impd_init_drc_params(
        frame_size,
        sample_rate,
        gain_delay_samples,
        delay_mode,
        sub_band_domain_mode,
        &mut (*p_drc_gain_dec_structs).ia_drc_params_struct,
    ) as IA_ERRORCODE;
    if err_code != IA_NO_ERROR {
        return err_code as WORD32;
    }
    err_code = impd_init_parametric_drc(
        (*p_drc_gain_dec_structs).ia_drc_params_struct.drc_frame_size,
        sample_rate,
        sub_band_domain_mode,
        &mut (*p_drc_gain_dec_structs).parametricdrc_params,
    ) as IA_ERRORCODE;
    if err_code != IA_NO_ERROR {
        return err_code as WORD32;
    }
    return err_code as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_init_drc_decode_post_config(
    mut audio_num_chan: WORD32,
    mut drc_set_id_processed: *mut WORD32,
    mut downmix_id_processed: *mut WORD32,
    mut num_sets_processed: WORD32,
    mut eq_set_id_processed: WORD32,
    mut p_drc_gain_dec_structs: *mut ia_drc_gain_dec_struct,
    mut pstr_drc_config: *mut ia_drc_config,
    mut pstr_loudness_info: *mut ia_drc_loudness_info_set_struct,
    mut mem_ptr: *mut pVOID,
) -> WORD32 {
    let mut err_code: IA_ERRORCODE = 0 as IA_ERRORCODE;
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut maxMultibandAudioSignalCount: WORD32 = 0 as WORD32;
    let mut p_drc_params_struct: *mut ia_drc_params_struct = &mut (*p_drc_gain_dec_structs)
        .ia_drc_params_struct;
    let mut p_audio_in_out_buf: *mut ia_audio_in_out_buf = &mut (*p_drc_gain_dec_structs)
        .audio_in_out_buf;
    i = 0 as core::ffi::c_int as WORD32;
    while i < num_sets_processed {
        err_code = impd_init_selected_drc_set(
            pstr_drc_config,
            p_drc_params_struct,
            &mut (*p_drc_gain_dec_structs).parametricdrc_params,
            audio_num_chan,
            *drc_set_id_processed.offset(i as isize),
            *downmix_id_processed.offset(i as isize),
            &mut (*p_drc_gain_dec_structs).ia_filter_banks_struct,
            &mut (*p_drc_gain_dec_structs).str_overlap_params,
            ((*p_drc_gain_dec_structs).shape_filter_block).as_mut_ptr(),
        ) as IA_ERRORCODE;
        if err_code != 0 {
            return err_code as WORD32;
        }
        i += 1;
    }
    (*p_drc_gain_dec_structs).audio_num_chan = audio_num_chan;
    (*p_drc_gain_dec_structs).ia_drc_params_struct.audio_delay_samples = (*p_drc_gain_dec_structs)
        .ia_drc_params_struct
        .parametric_drc_delay;
    if (*pstr_drc_config).str_drc_config_ext.parametric_drc_present != 0 {
        err_code = impd_init_parametric_drc_after_config(
            pstr_drc_config,
            pstr_loudness_info,
            &mut (*p_drc_gain_dec_structs).parametricdrc_params,
            mem_ptr,
        ) as IA_ERRORCODE;
        if err_code != 0 {
            return err_code as WORD32;
        }
    }
    (*p_audio_in_out_buf).audio_num_chan = audio_num_chan;
    (*p_audio_in_out_buf).audio_delay_samples = (*p_drc_params_struct)
        .audio_delay_samples;
    (*p_audio_in_out_buf).frame_size = (*p_drc_params_struct).drc_frame_size;
    if (*p_drc_params_struct).sub_band_domain_mode == SUBBAND_DOMAIN_MODE_QMF64 {
        (*p_audio_in_out_buf).audio_delay_sub_band_samples = ((*p_drc_params_struct)
            .audio_delay_samples as core::ffi::c_int
            / AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_QMF64) as WORD32;
        (*p_audio_in_out_buf).audio_sub_band_frame_size = ((*p_drc_params_struct)
            .drc_frame_size as core::ffi::c_int
            / AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_QMF64) as WORD32;
        (*p_audio_in_out_buf).audio_sub_band_count = AUDIO_CODEC_SUBBAND_COUNT_QMF64
            as WORD32;
    } else if (*p_drc_params_struct).sub_band_domain_mode == SUBBAND_DOMAIN_MODE_QMF71 {
        (*p_audio_in_out_buf).audio_delay_sub_band_samples = ((*p_drc_params_struct)
            .audio_delay_samples as core::ffi::c_int
            / AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_QMF71) as WORD32;
        (*p_audio_in_out_buf).audio_sub_band_frame_size = ((*p_drc_params_struct)
            .drc_frame_size as core::ffi::c_int
            / AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_QMF71) as WORD32;
        (*p_audio_in_out_buf).audio_sub_band_count = AUDIO_CODEC_SUBBAND_COUNT_QMF71
            as WORD32;
    } else if (*p_drc_params_struct).sub_band_domain_mode == SUBBAND_DOMAIN_MODE_STFT256
    {
        (*p_audio_in_out_buf).audio_delay_sub_band_samples = ((*p_drc_params_struct)
            .audio_delay_samples as core::ffi::c_int
            / AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_STFT256) as WORD32;
        (*p_audio_in_out_buf).audio_sub_band_frame_size = ((*p_drc_params_struct)
            .drc_frame_size as core::ffi::c_int
            / AUDIO_CODEC_SUBBAND_DOWNSAMPLING_FACTOR_STFT256) as WORD32;
        (*p_audio_in_out_buf).audio_sub_band_count = AUDIO_CODEC_SUBBAND_COUNT_STFT256
            as WORD32;
    } else {
        (*p_audio_in_out_buf).audio_delay_sub_band_samples = 0 as core::ffi::c_int
            as WORD32;
        (*p_audio_in_out_buf).audio_sub_band_frame_size = 0 as core::ffi::c_int
            as WORD32;
        (*p_audio_in_out_buf).audio_sub_band_count = 0 as core::ffi::c_int as WORD32;
    }
    k = 0 as core::ffi::c_int as WORD32;
    while k < SEL_DRC_COUNT {
        if (*p_drc_params_struct).sel_drc_array[k as usize].drc_instructions_index
            >= 0 as core::ffi::c_int
        {
            let mut drc_instruction_str: *mut ia_drc_instructions_struct = &mut *((*pstr_drc_config)
                .str_drc_instruction_str)
                .as_mut_ptr()
                .offset(
                    (*((*p_drc_params_struct).sel_drc_array)
                        .as_mut_ptr()
                        .offset(k as isize))
                        .drc_instructions_index as isize,
                ) as *mut ia_drc_instructions_struct;
            if (*drc_instruction_str).gain_element_count > 0 as core::ffi::c_int {
                (*p_drc_gain_dec_structs)
                    .drc_gain_buffers
                    .pstr_gain_buf[k as usize]
                    .buf_interpolation = *mem_ptr as *mut ia_interp_buf_struct;
                *mem_ptr = (*mem_ptr as size_t)
                    .wrapping_add(
                        ((*drc_instruction_str).gain_element_count as size_t)
                            .wrapping_mul(
                                ::core::mem::size_of::<ia_interp_buf_struct>() as size_t,
                            )
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                            )
                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                    ) as pVOID;
                (*p_drc_gain_dec_structs)
                    .drc_gain_buffers
                    .pstr_gain_buf[k as usize]
                    .buf_interpolation_count = (*drc_instruction_str).gain_element_count;
                i = 0 as core::ffi::c_int as WORD32;
                while i
                    < (*p_drc_gain_dec_structs)
                        .drc_gain_buffers
                        .pstr_gain_buf[k as usize]
                        .buf_interpolation_count
                {
                    (*((*p_drc_gain_dec_structs)
                        .drc_gain_buffers
                        .pstr_gain_buf[k as usize]
                        .buf_interpolation)
                        .offset(i as isize))
                        .str_node
                        .time = 0 as core::ffi::c_int as WORD32;
                    (*((*p_drc_gain_dec_structs)
                        .drc_gain_buffers
                        .pstr_gain_buf[k as usize]
                        .buf_interpolation)
                        .offset(i as isize))
                        .prev_node
                        .time = -(1 as core::ffi::c_int) as WORD32;
                    (*((*p_drc_gain_dec_structs)
                        .drc_gain_buffers
                        .pstr_gain_buf[k as usize]
                        .buf_interpolation)
                        .offset(i as isize))
                        .str_node
                        .loc_db_gain = 0.0f32 as FLOAT32;
                    (*((*p_drc_gain_dec_structs)
                        .drc_gain_buffers
                        .pstr_gain_buf[k as usize]
                        .buf_interpolation)
                        .offset(i as isize))
                        .str_node
                        .slope = 0.0f32 as FLOAT32;
                    j = 0 as core::ffi::c_int as WORD32;
                    while j
                        < 2 as core::ffi::c_int * AUDIO_CODEC_FRAME_SIZE_MAX
                            + MAX_SIGNAL_DELAY
                    {
                        (*((*p_drc_gain_dec_structs)
                            .drc_gain_buffers
                            .pstr_gain_buf[k as usize]
                            .buf_interpolation)
                            .offset(i as isize))
                            .lpcm_gains[j as usize] = 1.0f32 as FLOAT32;
                        j += 1;
                    }
                    i += 1;
                }
            }
        }
        k += 1;
    }
    if eq_set_id_processed > 0 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*pstr_drc_config).str_drc_config_ext.eq_instructions_count {
            if (*pstr_drc_config)
                .str_drc_config_ext
                .str_eq_instructions[i as usize]
                .eq_set_id == eq_set_id_processed
            {
                break;
            }
            i += 1;
        }
        if i == (*pstr_drc_config).str_drc_config_ext.eq_instructions_count {
            return -(1 as WORD32);
        }
        (*p_drc_gain_dec_structs).eq_set = *mem_ptr as *mut ia_eq_set_struct;
        *mem_ptr = (*mem_ptr as size_t)
            .wrapping_add(
                (::core::mem::size_of::<ia_eq_set_struct>() as size_t)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
            ) as pVOID;
        if err_code != 0 {
            return err_code as WORD32;
        }
        err_code = impd_derive_eq_set(
            &mut (*pstr_drc_config).str_drc_config_ext.str_eq_coeff,
            &mut *((*pstr_drc_config).str_drc_config_ext.str_eq_instructions)
                .as_mut_ptr()
                .offset(i as isize),
            (*p_drc_gain_dec_structs).ia_drc_params_struct.sample_rate as FLOAT32,
            (*p_drc_gain_dec_structs).ia_drc_params_struct.drc_frame_size,
            (*p_drc_gain_dec_structs).ia_drc_params_struct.sub_band_domain_mode,
            (*p_drc_gain_dec_structs).eq_set,
        ) as IA_ERRORCODE;
        if err_code != 0 {
            return err_code as WORD32;
        }
        impd_get_eq_set_delay(
            (*p_drc_gain_dec_structs).eq_set,
            &mut (*p_drc_gain_dec_structs).ia_drc_params_struct.eq_delay,
        );
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*p_drc_params_struct).drc_set_counter {
        let mut drc_instruction_str_0: *mut ia_drc_instructions_struct = 0
            as *mut ia_drc_instructions_struct;
        drc_instruction_str_0 = &mut *((*pstr_drc_config).str_drc_instruction_str)
            .as_mut_ptr()
            .offset(
                (*((*p_drc_params_struct).sel_drc_array).as_mut_ptr().offset(i as isize))
                    .drc_instructions_index as isize,
            ) as *mut ia_drc_instructions_struct;
        maxMultibandAudioSignalCount = if maxMultibandAudioSignalCount
            > (*drc_instruction_str_0).multiband_audio_sig_count
        {
            maxMultibandAudioSignalCount
        } else {
            (*drc_instruction_str_0).multiband_audio_sig_count
        };
        i += 1;
    }
    (*p_drc_gain_dec_structs).audio_band_buffer.non_interleaved_audio = *mem_ptr
        as *mut *mut FLOAT32;
    *mem_ptr = (*mem_ptr as size_t)
        .wrapping_add(
            (maxMultibandAudioSignalCount as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as size_t)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
        ) as pVOID;
    i = 0 as core::ffi::c_int as WORD32;
    while i < maxMultibandAudioSignalCount {
        let ref mut fresh0 = *((*p_drc_gain_dec_structs)
            .audio_band_buffer
            .non_interleaved_audio)
            .offset(i as isize);
        *fresh0 = *mem_ptr as *mut FLOAT32;
        *mem_ptr = (*mem_ptr as size_t)
            .wrapping_add(
                ((*p_drc_params_struct).drc_frame_size as size_t)
                    .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
            ) as pVOID;
        i += 1;
    }
    (*p_drc_gain_dec_structs).audio_band_buffer.multiband_audio_sig_count = maxMultibandAudioSignalCount;
    (*p_drc_gain_dec_structs).audio_band_buffer.frame_size = (*p_drc_params_struct)
        .drc_frame_size;
    if (*p_drc_params_struct).sub_band_domain_mode == SUBBAND_DOMAIN_MODE_OFF
        && (*p_audio_in_out_buf).audio_delay_samples != 0
    {
        (*p_audio_in_out_buf).audio_io_buffer_delayed = *mem_ptr as *mut *mut FLOAT32;
        *mem_ptr = (*mem_ptr as size_t)
            .wrapping_add(
                ((*p_audio_in_out_buf).audio_num_chan as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as size_t)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
            ) as pVOID;
        (*p_audio_in_out_buf).audio_in_out_buf = *mem_ptr as *mut *mut FLOAT32;
        *mem_ptr = (*mem_ptr as size_t)
            .wrapping_add(
                ((*p_audio_in_out_buf).audio_num_chan as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as size_t)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
            ) as pVOID;
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*p_audio_in_out_buf).audio_num_chan {
            let ref mut fresh1 = *((*p_audio_in_out_buf).audio_io_buffer_delayed)
                .offset(i as isize);
            *fresh1 = *mem_ptr as *mut FLOAT32;
            *mem_ptr = (*mem_ptr as size_t)
                .wrapping_add(
                    (((*p_audio_in_out_buf).frame_size
                        + (*p_audio_in_out_buf).audio_delay_samples) as size_t)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                ) as pVOID;
            let ref mut fresh2 = *((*p_audio_in_out_buf).audio_in_out_buf)
                .offset(i as isize);
            *fresh2 = &mut *(*((*p_audio_in_out_buf).audio_io_buffer_delayed)
                .offset(i as isize))
                .offset((*p_audio_in_out_buf).audio_delay_samples as isize)
                as *mut FLOAT32;
            i += 1;
        }
    }
    if (*p_drc_params_struct).sub_band_domain_mode != SUBBAND_DOMAIN_MODE_OFF
        && (*p_audio_in_out_buf).audio_delay_sub_band_samples != 0
    {
        (*p_audio_in_out_buf).audio_buffer_delayed_real = *mem_ptr as *mut *mut FLOAT32;
        *mem_ptr = (*mem_ptr as size_t)
            .wrapping_add(
                ((*p_audio_in_out_buf).audio_num_chan as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as size_t)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
            ) as pVOID;
        (*p_audio_in_out_buf).audio_buffer_delayed_imag = *mem_ptr as *mut *mut FLOAT32;
        *mem_ptr = (*mem_ptr as size_t)
            .wrapping_add(
                ((*p_audio_in_out_buf).audio_num_chan as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as size_t)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
            ) as pVOID;
        (*p_audio_in_out_buf).audio_real_buff = *mem_ptr as *mut *mut FLOAT32;
        *mem_ptr = (*mem_ptr as size_t)
            .wrapping_add(
                ((*p_audio_in_out_buf).audio_num_chan as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as size_t)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
            ) as pVOID;
        (*p_audio_in_out_buf).audio_imag_buff = *mem_ptr as *mut *mut FLOAT32;
        *mem_ptr = (*mem_ptr as size_t)
            .wrapping_add(
                ((*p_audio_in_out_buf).audio_num_chan as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as size_t)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
            ) as pVOID;
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*p_audio_in_out_buf).audio_num_chan {
            let ref mut fresh3 = *((*p_audio_in_out_buf).audio_buffer_delayed_real)
                .offset(i as isize);
            *fresh3 = *mem_ptr as *mut FLOAT32;
            *mem_ptr = (*mem_ptr as size_t)
                .wrapping_add(
                    (((*p_audio_in_out_buf).audio_sub_band_frame_size
                        + (*p_audio_in_out_buf).audio_delay_sub_band_samples) as size_t)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                ) as pVOID;
            let ref mut fresh4 = *((*p_audio_in_out_buf).audio_buffer_delayed_imag)
                .offset(i as isize);
            *fresh4 = *mem_ptr as *mut FLOAT32;
            *mem_ptr = (*mem_ptr as size_t)
                .wrapping_add(
                    (((*p_audio_in_out_buf).audio_sub_band_frame_size
                        + (*p_audio_in_out_buf).audio_delay_sub_band_samples) as size_t)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as size_t)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                ) as pVOID;
            let ref mut fresh5 = *((*p_audio_in_out_buf).audio_real_buff)
                .offset(i as isize);
            *fresh5 = &mut *(*((*p_audio_in_out_buf).audio_buffer_delayed_real)
                .offset(i as isize))
                .offset(
                    ((*p_audio_in_out_buf).audio_delay_sub_band_samples
                        * (*p_audio_in_out_buf).audio_sub_band_count) as isize,
                ) as *mut FLOAT32;
            let ref mut fresh6 = *((*p_audio_in_out_buf).audio_imag_buff)
                .offset(i as isize);
            *fresh6 = &mut *(*((*p_audio_in_out_buf).audio_buffer_delayed_imag)
                .offset(i as isize))
                .offset(
                    ((*p_audio_in_out_buf).audio_delay_sub_band_samples
                        * (*p_audio_in_out_buf).audio_sub_band_count) as isize,
                ) as *mut FLOAT32;
            i += 1;
        }
    }
    return err_code as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_drc_process_time_domain(
    mut p_drc_gain_dec_structs: *mut ia_drc_gain_dec_struct,
    mut pstr_drc_config: *mut ia_drc_config,
    mut pstr_drc_gain: *mut ia_drc_gain_struct,
    mut audio_in_out_buf: *mut *mut FLOAT32,
    mut loudness_normalization_gain_db: FLOAT32,
    mut boost: FLOAT32,
    mut compress: FLOAT32,
    mut drc_characteristic_target: WORD32,
) -> WORD32 {
    let mut sel_drc_index: WORD32 = 0;
    let mut err_code: IA_ERRORCODE = 0 as IA_ERRORCODE;
    let mut passThru: WORD32 = 0;
    let mut str_drc_instruction_str: *mut ia_drc_instructions_struct = ((*pstr_drc_config)
        .str_drc_instruction_str)
        .as_mut_ptr();
    if !((*p_drc_gain_dec_structs).eq_set).is_null() {
        let mut ch: WORD32 = 0;
        let mut audio_channel: *mut FLOAT32 = 0 as *mut FLOAT32;
        ch = 0 as core::ffi::c_int as WORD32;
        while ch < (*(*p_drc_gain_dec_structs).eq_set).audio_num_chan {
            audio_channel = *audio_in_out_buf.offset(ch as isize);
            err_code = impd_process_eq_set_time_domain(
                (*p_drc_gain_dec_structs).eq_set,
                ch,
                audio_channel,
                audio_channel,
                (*p_drc_gain_dec_structs).ia_drc_params_struct.drc_frame_size,
            ) as IA_ERRORCODE;
            if err_code != 0 {
                return err_code as WORD32;
            }
            ch += 1;
        }
    }
    impd_store_audio_io_buffer_time(
        audio_in_out_buf,
        &mut (*p_drc_gain_dec_structs).audio_in_out_buf,
    );
    if (*pstr_drc_config).apply_drc != 0 {
        sel_drc_index = 0 as core::ffi::c_int as WORD32;
        while sel_drc_index
            < (*p_drc_gain_dec_structs).ia_drc_params_struct.drc_set_counter
        {
            err_code = impd_get_drc_gain(
                p_drc_gain_dec_structs,
                pstr_drc_config,
                pstr_drc_gain,
                compress,
                boost,
                drc_characteristic_target,
                loudness_normalization_gain_db,
                sel_drc_index,
                &mut (*p_drc_gain_dec_structs).drc_gain_buffers,
            ) as IA_ERRORCODE;
            if err_code != IA_NO_ERROR {
                return err_code as WORD32;
            }
            sel_drc_index += 1;
        }
        if (*p_drc_gain_dec_structs).ia_drc_params_struct.drc_set_counter
            == 0 as core::ffi::c_int
        {
            impd_retrieve_audio_io_buffer_time(
                audio_in_out_buf,
                &mut (*p_drc_gain_dec_structs).audio_in_out_buf,
            );
        } else {
            sel_drc_index = 0 as core::ffi::c_int as WORD32;
            while sel_drc_index
                < (*p_drc_gain_dec_structs).ia_drc_params_struct.drc_set_counter
            {
                if (*p_drc_gain_dec_structs).ia_drc_params_struct.multiband_sel_drc_idx
                    == sel_drc_index
                {
                    passThru = 0 as core::ffi::c_int as WORD32;
                } else {
                    passThru = 1 as core::ffi::c_int as WORD32;
                }
                err_code = impd_filter_banks_process(
                    str_drc_instruction_str,
                    (*p_drc_gain_dec_structs)
                        .ia_drc_params_struct
                        .sel_drc_array[sel_drc_index as usize]
                        .drc_instructions_index,
                    &mut (*p_drc_gain_dec_structs).ia_drc_params_struct,
                    (*p_drc_gain_dec_structs).audio_in_out_buf.audio_io_buffer_delayed
                        as *mut *mut FLOAT32,
                    &mut (*p_drc_gain_dec_structs).audio_band_buffer,
                    &mut (*p_drc_gain_dec_structs).ia_filter_banks_struct,
                    passThru,
                ) as IA_ERRORCODE;
                if err_code != IA_NO_ERROR {
                    return err_code as WORD32;
                }
                impd_apply_gains_and_add(
                    str_drc_instruction_str,
                    (*p_drc_gain_dec_structs)
                        .ia_drc_params_struct
                        .sel_drc_array[sel_drc_index as usize]
                        .drc_instructions_index,
                    &mut (*p_drc_gain_dec_structs).ia_drc_params_struct,
                    &mut *((*p_drc_gain_dec_structs).drc_gain_buffers.pstr_gain_buf)
                        .as_mut_ptr()
                        .offset(sel_drc_index as isize),
                    ((*p_drc_gain_dec_structs).shape_filter_block).as_mut_ptr(),
                    (*p_drc_gain_dec_structs).audio_band_buffer.non_interleaved_audio
                        as *mut *mut FLOAT32,
                    audio_in_out_buf,
                    1 as WORD32,
                );
                sel_drc_index += 1;
            }
        }
    }
    impd_advance_audio_io_buffer_time(&mut (*p_drc_gain_dec_structs).audio_in_out_buf);
    return err_code as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_get_parametric_drc_delay(
    mut p_drc_gain_dec_structs: *mut ia_drc_gain_dec_struct,
    mut pstr_drc_config: *mut ia_drc_config,
    mut parametric_drc_delay: *mut WORD32,
    mut parametric_drc_delay_max: *mut WORD32,
) -> VOID {
    *parametric_drc_delay = (*p_drc_gain_dec_structs)
        .ia_drc_params_struct
        .parametric_drc_delay;
    if (*pstr_drc_config).str_drc_config_ext.parametric_drc_present != 0
        && (*pstr_drc_config)
            .str_drc_config_ext
            .str_drc_coeff_param_drc
            .parametric_drc_delay_max_present != 0
    {
        *parametric_drc_delay_max = (*pstr_drc_config)
            .str_drc_config_ext
            .str_drc_coeff_param_drc
            .parametric_drc_delay_max;
    } else if (*pstr_drc_config).str_drc_config_ext.parametric_drc_present
        == 0 as core::ffi::c_int
    {
        *parametric_drc_delay_max = 0 as core::ffi::c_int as WORD32;
    } else {
        *parametric_drc_delay_max = -(1 as core::ffi::c_int) as WORD32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn impd_get_eq_delay(
    mut p_drc_gain_dec_structs: *mut ia_drc_gain_dec_struct,
    mut pstr_drc_config: *mut ia_drc_config,
    mut eq_delay: *mut WORD32,
    mut eq_delay_max: *mut WORD32,
) -> VOID {
    *eq_delay = (*p_drc_gain_dec_structs).ia_drc_params_struct.eq_delay;
    if (*pstr_drc_config).str_drc_config_ext.eq_flag != 0
        && (*pstr_drc_config).str_drc_config_ext.str_eq_coeff.eq_delay_max_present != 0
    {
        *eq_delay_max = (*pstr_drc_config).str_drc_config_ext.str_eq_coeff.eq_delay_max;
    } else if (*pstr_drc_config).str_drc_config_ext.eq_flag == 0 as core::ffi::c_int {
        *eq_delay_max = 0 as core::ffi::c_int as WORD32;
    } else {
        *eq_delay_max = -(1 as core::ffi::c_int) as WORD32;
    };
}
