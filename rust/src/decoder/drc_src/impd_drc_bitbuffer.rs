extern "C" {
    fn impd_init_tbls(
        num_gain_max_values: WORD32,
        str_tables: *mut ia_tables_struct,
    ) -> VOID;
    fn impd_get_delta_tmin(sampling_rate: WORD32) -> WORD32;
    fn impd_parse_drc_config(
        it_bit_buff: *mut ia_bit_buf_struct,
        ia_drc_params_struct: *mut ia_drc_params_bs_dec_struct,
        drc_config: *mut ia_drc_config,
    ) -> WORD32;
    fn impd_parse_loudness_info_set(
        it_bit_buff: *mut ia_bit_buf_struct,
        loudness_info_set: *mut ia_drc_loudness_info_set_struct,
    ) -> WORD32;
    fn impd_drc_uni_gain_read(
        it_bit_buff: *mut ia_bit_buf_struct,
        pstr_drc_uni_bs_dec: *mut ia_drc_bits_dec_struct,
        drc_config: *mut ia_drc_config,
        pstr_uni_drc_gain: *mut ia_drc_gain_struct,
    ) -> WORD32;
}
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type WORD = core::ffi::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn impd_read_bits_buf(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut no_of_bits: WORD,
) -> WORD32 {
    let mut ret_val: UWORD32 = 0;
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buff).ptr_read_next;
    let mut bit_pos: WORD = (*it_bit_buff).bit_pos as WORD;
    if (*it_bit_buff).cnt_bits <= 0 as core::ffi::c_int {
        (*it_bit_buff).error = 1 as core::ffi::c_int as WORD32;
        return -(1 as WORD32);
    }
    if no_of_bits == 0 as core::ffi::c_int {
        return 0 as WORD32;
    }
    (*it_bit_buff).cnt_bits -= no_of_bits as core::ffi::c_int;
    ret_val = *ptr_read_next as UWORD32;
    bit_pos -= no_of_bits;
    while bit_pos < 0 as core::ffi::c_int {
        bit_pos += 8 as core::ffi::c_int;
        ptr_read_next = ptr_read_next.offset(1);
        if ptr_read_next > (*it_bit_buff).ptr_bit_buf_end {
            ptr_read_next = (*it_bit_buff).ptr_bit_buf_base;
        }
        ret_val <<= 8 as core::ffi::c_int;
        ret_val |= *ptr_read_next as UWORD32;
    }
    ret_val = ret_val << 31 as WORD - no_of_bits - bit_pos >> 32 as WORD - no_of_bits;
    (*it_bit_buff).ptr_read_next = ptr_read_next;
    (*it_bit_buff).bit_pos = bit_pos as WORD16 as WORD32;
    return ret_val as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_skip_bits_buf(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut no_of_bits: WORD,
) -> WORD32 {
    let mut ptr_read_next: *mut UWORD8 = (*it_bit_buff).ptr_read_next;
    let mut bit_pos: WORD = (*it_bit_buff).bit_pos as WORD;
    if (*it_bit_buff).cnt_bits < no_of_bits {
        (*it_bit_buff).error = 1 as core::ffi::c_int as WORD32;
        return -(1 as WORD32);
    }
    (*it_bit_buff).cnt_bits -= no_of_bits as core::ffi::c_int;
    bit_pos -= no_of_bits;
    while bit_pos < 0 as core::ffi::c_int {
        bit_pos += 8 as core::ffi::c_int;
        ptr_read_next = ptr_read_next.offset(1);
    }
    (*it_bit_buff).ptr_read_next = ptr_read_next;
    (*it_bit_buff).bit_pos = bit_pos as WORD16 as WORD32;
    return no_of_bits as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_create_bit_buf(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_bit_buf_base: *mut UWORD8,
    mut bit_buf_size: WORD32,
) -> VOID {
    (*it_bit_buff).ptr_bit_buf_base = ptr_bit_buf_base;
    (*it_bit_buff).ptr_bit_buf_end = ptr_bit_buf_base
        .offset(bit_buf_size as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    (*it_bit_buff).ptr_read_next = ptr_bit_buf_base;
    (*it_bit_buff).bit_pos = 7 as core::ffi::c_int as WORD32;
    (*it_bit_buff).cnt_bits = 0 as core::ffi::c_int as WORD32;
    (*it_bit_buff).size = bit_buf_size << 3 as core::ffi::c_int;
    (*it_bit_buff).error = 0 as core::ffi::c_int as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_create_init_bit_buf(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ptr_bit_buf_base: *mut UWORD8,
    mut bit_buf_size: WORD32,
) -> VOID {
    impd_create_bit_buf(it_bit_buff, ptr_bit_buf_base, bit_buf_size);
    (*it_bit_buff).cnt_bits = bit_buf_size << 3 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn impd_init_drc_bitstream_dec(
    mut p_drc_bs_dec_struct: *mut ia_drc_bits_dec_struct,
    mut sample_rate: WORD32,
    mut frame_size: WORD32,
    mut delay_mode: WORD32,
    mut lfe_channel_map_count: WORD32,
    mut lfe_channel_map: *mut WORD32,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut err_code: WORD32 = 0 as WORD32;
    let mut ia_drc_params_struct: *mut ia_drc_params_bs_dec_struct = &mut (*p_drc_bs_dec_struct)
        .ia_drc_params_struct;
    (*ia_drc_params_struct).drc_frame_size = frame_size;
    if sample_rate < MIN_DRC_SAMP_FREQ {
        return -(1 as WORD32);
    }
    (*ia_drc_params_struct).delta_tmin_default = impd_get_delta_tmin(sample_rate);
    (*ia_drc_params_struct).num_gain_values_max_default = (*ia_drc_params_struct)
        .drc_frame_size / (*ia_drc_params_struct).delta_tmin_default;
    (*ia_drc_params_struct).delay_mode = delay_mode;
    if frame_size < 1 as core::ffi::c_int || frame_size > AUDIO_CODEC_FRAME_SIZE_MAX
        || ((*ia_drc_params_struct).drc_frame_size as core::ffi::c_float)
            < 0.001f32 * sample_rate as core::ffi::c_float
    {
        return -(1 as WORD32);
    }
    if (*ia_drc_params_struct).delta_tmin_default
        > (*ia_drc_params_struct).drc_frame_size
    {
        return -(1 as WORD32);
    }
    if lfe_channel_map_count >= 0 as core::ffi::c_int {
        if lfe_channel_map.is_null() || lfe_channel_map_count > MAX_CHANNEL_COUNT {
            return -(1 as WORD32);
        }
        (*ia_drc_params_struct).lfe_channel_map_count = lfe_channel_map_count;
        i = 0 as core::ffi::c_int as WORD32;
        while i < lfe_channel_map_count {
            (*ia_drc_params_struct).lfe_channel_map[i as usize] = *lfe_channel_map
                .offset(i as isize);
            i += 1;
        }
    } else {
        (*ia_drc_params_struct).lfe_channel_map_count = -(1 as core::ffi::c_int)
            as WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < MAX_CHANNEL_COUNT {
            (*ia_drc_params_struct).lfe_channel_map[i as usize] = 0 as core::ffi::c_int
                as WORD32;
            i += 1;
        }
    }
    impd_init_tbls(
        (*ia_drc_params_struct).num_gain_values_max_default,
        &mut (*p_drc_bs_dec_struct).tables_default,
    );
    return err_code;
}
#[no_mangle]
pub unsafe extern "C" fn impd_process_drc_bitstream_dec_config(
    mut p_drc_bs_dec_struct: *mut ia_drc_bits_dec_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut pstr_drc_config: *mut ia_drc_config,
    mut bitstream_config: *mut UWORD8,
    mut num_bytes: WORD32,
) -> WORD32 {
    let mut err_code: WORD32 = 0 as WORD32;
    impd_create_init_bit_buf(it_bit_buff, bitstream_config, num_bytes);
    err_code = impd_parse_drc_config(
        it_bit_buff,
        &mut (*p_drc_bs_dec_struct).ia_drc_params_struct,
        pstr_drc_config,
    );
    if err_code != 0 {
        return err_code;
    }
    return err_code;
}
#[no_mangle]
pub unsafe extern "C" fn impd_process_drc_bitstream_dec_gain(
    mut p_drc_bs_dec_struct: *mut ia_drc_bits_dec_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut pstr_drc_config: *mut ia_drc_config,
    mut pstr_drc_gain: *mut ia_drc_gain_struct,
    mut bitstream_gain: *mut UWORD8,
    mut num_bytes: WORD32,
    mut num_bits_offset: WORD32,
    mut num_bits_read: *mut WORD32,
) -> WORD32 {
    let mut err_code: WORD32 = 0 as WORD32;
    impd_create_init_bit_buf(it_bit_buff, bitstream_gain, num_bytes);
    impd_read_bits_buf(it_bit_buff, num_bits_offset as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    err_code = impd_drc_uni_gain_read(
        it_bit_buff,
        p_drc_bs_dec_struct,
        pstr_drc_config,
        pstr_drc_gain,
    );
    if err_code > PROC_COMPLETE {
        return err_code;
    }
    *num_bits_read = (*it_bit_buff).size - (*it_bit_buff).cnt_bits;
    if err_code == PROC_COMPLETE {
        return err_code;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_process_drc_bitstream_dec_loudness_info_set(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut pstr_loudness_info: *mut ia_drc_loudness_info_set_struct,
    mut bit_stream_loudness: *mut UWORD8,
    mut num_bytes_loudness: WORD32,
) -> WORD32 {
    let mut err_code: WORD32 = 0 as WORD32;
    impd_create_init_bit_buf(it_bit_buff, bit_stream_loudness, num_bytes_loudness);
    err_code = impd_parse_loudness_info_set(it_bit_buff, pstr_loudness_info);
    return err_code;
}
pub const MAX_CHANNEL_COUNT: core::ffi::c_int = 8 as core::ffi::c_int;
pub const AUDIO_CODEC_FRAME_SIZE_MAX: core::ffi::c_int = 4096 as core::ffi::c_int;
pub const PROC_COMPLETE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const MIN_DRC_SAMP_FREQ: core::ffi::c_int = 1000 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
