extern "C" {
    fn pow(__x: core::ffi::c_double, __y: core::ffi::c_double) -> core::ffi::c_double;
    fn impd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
    fn impd_skip_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
    fn impd_get_delta_gain_code_tbl(
        gain_coding_profile: WORD32,
        delta_time_code_tbl: *mut *const ia_delta_gain_code_table_struct,
        num_entries: *mut WORD32,
    );
    fn impd_parse_drc_instructions_uni_drc(
        it_bit_buff: *mut ia_bit_buf_struct,
        version: WORD32,
        drc_config: *mut ia_drc_config,
        str_drc_instruction_str: *mut ia_drc_instructions_struct,
    ) -> WORD32;
    fn impd_drc_parse_coeff(
        it_bit_buff: *mut ia_bit_buf_struct,
        version: WORD32,
        ia_drc_params_struct: *mut ia_drc_params_bs_dec_struct,
        str_p_loc_drc_coefficients_uni_drc: *mut ia_uni_drc_coeffs_struct,
    ) -> WORD32;
    fn impd_parse_dwnmix_instructions(
        it_bit_buff: *mut ia_bit_buf_struct,
        version: WORD32,
        ia_drc_params_struct: *mut ia_drc_params_bs_dec_struct,
        channel_layout: *mut ia_channel_layout_struct,
        dwnmix_instructions: *mut ia_downmix_instructions_struct,
    ) -> WORD32;
    static eq_slope_tbl: [FLOAT32; 0];
    static eq_gain_delta_tbl: [FLOAT32; 0];
    static zero_pole_radius_tbl: [FLOAT32; 0];
    static zero_pole_angle_tbl: [FLOAT32; 0];
    static slope_code_tbl_entries_by_size: [ia_slope_code_table_struct; 15];
}
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD32 = core::ffi::c_int;
pub type FLOAT32 = core::ffi::c_float;
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
pub struct ia_slope_code_table_struct {
    pub size: WORD32,
    pub code: WORD32,
    pub value: FLOAT32,
    pub index: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_delta_gain_code_table_struct {
    pub size: WORD32,
    pub code: WORD32,
    pub value: FLOAT32,
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
pub const N_DELTA_TIME_CODE_TABLE_ENTRIES_MAX: core::ffi::c_int = 512 as core::ffi::c_int
    + 14 as core::ffi::c_int;
pub const DOWNMIX_INSTRUCTION_COUNT_MAX: core::ffi::c_int = 16 as core::ffi::c_int;
pub const DRC_COEFF_COUNT_MAX: core::ffi::c_int = 8 as core::ffi::c_int;
pub const DRC_INSTRUCTIONS_COUNT_MAX: core::ffi::c_int = DOWNMIX_INSTRUCTION_COUNT_MAX
    + 20 as core::ffi::c_int;
pub const AUDIO_CODEC_FRAME_SIZE_MAX: core::ffi::c_int = 4096 as core::ffi::c_int;
pub const DRC_CODEC_FRAME_SIZE_MAX: core::ffi::c_int = AUDIO_CODEC_FRAME_SIZE_MAX
    / 8 as core::ffi::c_int;
pub const NODE_COUNT_MAX: core::ffi::c_int = DRC_CODEC_FRAME_SIZE_MAX;
pub const DOWNMIX_ID_COUNT_MAX: core::ffi::c_int = 8 as core::ffi::c_int;
pub const DRC_SET_ID_COUNT_MAX: core::ffi::c_int = 16 as core::ffi::c_int;
pub const EQ_SET_ID_COUNT_MAX: core::ffi::c_int = 8 as core::ffi::c_int;
pub const LOUD_EQ_GAIN_SEQUENCE_COUNT_MAX: core::ffi::c_int = 4 as core::ffi::c_int;
pub const EQ_SUBBAND_GAIN_COUNT_MAX: core::ffi::c_int = 135 as core::ffi::c_int;
pub const UNIQUE_SUBBAND_GAIN_COUNT_MAX: core::ffi::c_int = 16 as core::ffi::c_int;
pub const FILTER_BLOCK_COUNT_MAX: core::ffi::c_int = 16 as core::ffi::c_int;
pub const FILTER_ELEMENT_COUNT_MAX: core::ffi::c_int = 16 as core::ffi::c_int;
pub const EQ_CHANNEL_GROUP_COUNT_MAX: core::ffi::c_int = 4 as core::ffi::c_int;
pub const EQ_FILTER_BLOCK_COUNT_MAX: core::ffi::c_int = 4 as core::ffi::c_int;
pub const LOUD_EQ_INSTRUCTIONS_COUNT_MAX: core::ffi::c_int = 8 as core::ffi::c_int;
pub const EQ_INSTRUCTIONS_COUNT_MAX: core::ffi::c_int = 8 as core::ffi::c_int;
pub const DELAY_MODE_LOW_DELAY: core::ffi::c_int = 1 as core::ffi::c_int;
pub const PROC_COMPLETE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const UNEXPECTED_ERROR: core::ffi::c_int = 2 as core::ffi::c_int;
pub const PARAM_ERROR: core::ffi::c_int = 3 as core::ffi::c_int;
pub const BITSTREAM_ERROR: core::ffi::c_int = 6 as core::ffi::c_int;
pub const ID_FOR_ANY_DOWNMIX: core::ffi::c_int = 0x7f as core::ffi::c_int;
pub const EXT_COUNT_MAX: core::ffi::c_int = 8 as core::ffi::c_int;
pub const UNIDRCGAINEXT_TERM: core::ffi::c_int = 0 as core::ffi::c_int;
pub const MAXPACKETLOSSTIME: core::ffi::c_float = 2.5f32;
pub const GAIN_CODING_PROFILE_REGULAR: core::ffi::c_int = 0;
pub const GAIN_CODING_PROFILE_FADING: core::ffi::c_int = 1 as core::ffi::c_int;
pub const GAIN_CODING_PROFILE_CLIPPING: core::ffi::c_int = 2;
pub const GAIN_CODING_PROFILE_CONSTANT: core::ffi::c_int = 3;
pub const GAIN_INTERPOLATION_TYPE_SPLINE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const GAINFORMAT_UNIFORM: core::ffi::c_int = 0x7 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn impd_dec_initial_gain(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    gain_coding_profile: WORD32,
    mut initial_gain: *mut FLOAT32,
) -> WORD32 {
    let mut sign: WORD32 = 0;
    let mut magn: WORD32 = 0;
    let mut bit_2_extract: WORD32 = 0;
    match gain_coding_profile {
        GAIN_CODING_PROFILE_REGULAR => {
            sign = impd_read_bits_buf(it_bit_buff, 1 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            magn = impd_read_bits_buf(it_bit_buff, 8 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            *initial_gain = (magn as core::ffi::c_float * 0.125f32) as FLOAT32;
            if sign != 0 {
                *initial_gain = -*initial_gain;
            }
        }
        GAIN_CODING_PROFILE_FADING | GAIN_CODING_PROFILE_CLIPPING => {
            bit_2_extract = (if gain_coding_profile == GAIN_CODING_PROFILE_FADING {
                10 as core::ffi::c_int
            } else {
                8 as core::ffi::c_int
            }) as WORD32;
            sign = impd_read_bits_buf(it_bit_buff, 1 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if sign == 0 as core::ffi::c_int {
                *initial_gain = 0.0f32 as FLOAT32;
            } else {
                magn = impd_read_bits_buf(it_bit_buff, bit_2_extract as WORD);
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                *initial_gain = (-(magn as core::ffi::c_int + 1 as core::ffi::c_int)
                    as core::ffi::c_float * 0.125f32) as FLOAT32;
            }
        }
        GAIN_CODING_PROFILE_CONSTANT => {}
        _ => return 2 as WORD32,
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_dec_gains(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut no_nodes: WORD32,
    mut gain_coding_profile: WORD32,
    mut str_node: *mut ia_node_struct,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut k: WORD32 = 0;
    let mut e: WORD32 = 0;
    let mut m: WORD32 = 0;
    let mut bit: WORD32 = 0;
    let mut num_bits_read: WORD32 = 0;
    let mut code: WORD32 = 0;
    let mut code_found: WORD32 = 0;
    let mut drc_gain_delta: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
    let mut ptr_delta_gain_code_table: *const ia_delta_gain_code_table_struct = 0
        as *const ia_delta_gain_code_table_struct;
    let mut no_delta_gain_entries: WORD32 = 0;
    err = impd_dec_initial_gain(
        it_bit_buff,
        gain_coding_profile,
        &mut (*str_node.offset(0 as core::ffi::c_int as isize)).loc_db_gain,
    );
    if err != 0 {
        return err;
    }
    impd_get_delta_gain_code_tbl(
        gain_coding_profile,
        &mut ptr_delta_gain_code_table,
        &mut no_delta_gain_entries,
    );
    k = 1 as core::ffi::c_int as WORD32;
    while k < no_nodes {
        num_bits_read = 0 as core::ffi::c_int as WORD32;
        code = 0 as core::ffi::c_int as WORD32;
        code_found = 0 as core::ffi::c_int as WORD32;
        e = 0 as core::ffi::c_int as WORD32;
        while e < no_delta_gain_entries && code_found == 0 {
            m = 0 as core::ffi::c_int as WORD32;
            while m
                < (*ptr_delta_gain_code_table.offset(e as isize)).size - num_bits_read
            {
                bit = impd_read_bits_buf(it_bit_buff, 1 as WORD);
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                code = (code << 1 as core::ffi::c_int) + bit;
                num_bits_read += 1;
                m += 1;
            }
            while num_bits_read == (*ptr_delta_gain_code_table.offset(e as isize)).size {
                if code == (*ptr_delta_gain_code_table.offset(e as isize)).code {
                    drc_gain_delta = (*ptr_delta_gain_code_table.offset(e as isize))
                        .value;
                    code_found = 1 as core::ffi::c_int as WORD32;
                    break;
                } else {
                    e += 1;
                }
            }
        }
        if code_found == 0 as core::ffi::c_int {
            return 2 as WORD32;
        }
        (*str_node.offset(k as isize)).loc_db_gain = (*str_node
            .offset((k as core::ffi::c_int - 1 as core::ffi::c_int) as isize))
            .loc_db_gain + drc_gain_delta;
        k += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_dec_slopes(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut no_nodes: *mut WORD32,
    mut gain_interpolation_type: WORD32,
    mut str_node: *mut ia_node_struct,
) -> WORD32 {
    let mut k: WORD32 = 0;
    let mut e: WORD32 = 0;
    let mut bit: WORD32 = 0;
    let mut code: WORD32 = 0;
    let mut code_found: WORD32 = 0;
    let mut slope_value: FLOAT32 = 0 as core::ffi::c_int as FLOAT32;
    let mut end_marker: WORD32 = 0 as WORD32;
    let mut num_bits_read: WORD32 = 0;
    let mut ptr_slope_code_table: *const ia_slope_code_table_struct = 0
        as *const ia_slope_code_table_struct;
    let mut no_slope_code_entries: WORD32 = 0;
    ptr_slope_code_table = &*slope_code_tbl_entries_by_size
        .as_ptr()
        .offset(0 as core::ffi::c_int as isize) as *const ia_slope_code_table_struct;
    no_slope_code_entries = NUM_SLOPE_TBL_ENTRIES as WORD32;
    k = 0 as core::ffi::c_int as WORD32;
    while end_marker != 1 as core::ffi::c_int {
        k += 1;
        end_marker = impd_read_bits_buf(it_bit_buff, 1 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
    }
    if k > NODE_COUNT_MAX {
        return UNEXPECTED_ERROR;
    }
    *no_nodes = k;
    if gain_interpolation_type == GAIN_INTERPOLATION_TYPE_SPLINE {
        k = 0 as core::ffi::c_int as WORD32;
        while k < *no_nodes {
            num_bits_read = 0 as core::ffi::c_int as WORD32;
            code = 0 as core::ffi::c_int as WORD32;
            code_found = 0 as core::ffi::c_int as WORD32;
            e = 0 as core::ffi::c_int as WORD32;
            while e < no_slope_code_entries && code_found == 0 {
                while num_bits_read < (*ptr_slope_code_table.offset(e as isize)).size {
                    bit = impd_read_bits_buf(it_bit_buff, 1 as WORD);
                    if (*it_bit_buff).error != 0 {
                        return (*it_bit_buff).error;
                    }
                    code = (code << 1 as core::ffi::c_int) + bit;
                    num_bits_read += 1;
                }
                while num_bits_read == (*ptr_slope_code_table.offset(e as isize)).size {
                    if code == (*ptr_slope_code_table.offset(e as isize)).code {
                        slope_value = (*ptr_slope_code_table.offset(e as isize)).value;
                        code_found = 1 as core::ffi::c_int as WORD32;
                        break;
                    } else {
                        e += 1;
                        if e >= no_slope_code_entries {
                            return UNEXPECTED_ERROR;
                        }
                    }
                }
            }
            (*str_node.offset(k as isize)).slope = slope_value;
            k += 1;
        }
    } else {
        k = 0 as core::ffi::c_int as WORD32;
        while k < *no_nodes {
            (*str_node.offset(k as isize)).slope = 0.0f32 as FLOAT32;
            k += 1;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_dec_times(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut str_tables: *mut ia_tables_struct,
    mut num_nodes: WORD32,
    mut delta_tmin: WORD32,
    mut drc_frame_size: WORD32,
    mut full_frame: WORD32,
    mut time_offset: WORD32,
    mut str_node: *mut ia_node_struct,
) -> WORD32 {
    let mut k: WORD32 = 0;
    let mut e: WORD32 = 0;
    let mut m: WORD32 = 0;
    let mut bit: WORD32 = 0;
    let mut num_bits_read: WORD32 = 0;
    let mut code: WORD32 = 0;
    let mut code_found: WORD32 = 0 as WORD32;
    let mut time_delta: WORD32 = 0 as WORD32;
    let mut time_offs: WORD32 = time_offset;
    let mut delta_time_code_table: *mut ia_delta_time_code_table_entry_struct = ((*str_tables)
        .delta_time_code_table)
        .as_mut_ptr();
    let mut frame_end_flag: WORD32 = 0;
    let mut node_time_tmp: WORD32 = 0;
    let mut node_res_flag: WORD32 = 0;
    let mut exit_cnt: WORD32 = 0;
    if full_frame == 0 as core::ffi::c_int {
        frame_end_flag = impd_read_bits_buf(it_bit_buff, 1 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
    } else {
        frame_end_flag = 1 as core::ffi::c_int as WORD32;
    }
    if frame_end_flag == 1 as core::ffi::c_int {
        node_res_flag = 0 as core::ffi::c_int as WORD32;
        k = 0 as core::ffi::c_int as WORD32;
        while k < num_nodes as core::ffi::c_int - 1 as core::ffi::c_int {
            num_bits_read = 0 as core::ffi::c_int as WORD32;
            code = 0 as core::ffi::c_int as WORD32;
            code_found = 0 as core::ffi::c_int as WORD32;
            exit_cnt = 0 as core::ffi::c_int as WORD32;
            e = 1 as core::ffi::c_int as WORD32;
            while e < N_DELTA_TIME_CODE_TABLE_ENTRIES_MAX && code_found == 0 {
                exit_cnt += 1;
                if exit_cnt > 100000 as core::ffi::c_int {
                    return -(1 as WORD32);
                }
                m = 0 as core::ffi::c_int as WORD32;
                while m
                    < (*delta_time_code_table.offset(e as isize)).size - num_bits_read
                {
                    bit = impd_read_bits_buf(it_bit_buff, 1 as WORD);
                    if (*it_bit_buff).error != 0 {
                        return (*it_bit_buff).error;
                    }
                    code = (code << 1 as core::ffi::c_int) + bit;
                    num_bits_read += 1;
                    m += 1;
                }
                while num_bits_read == (*delta_time_code_table.offset(e as isize)).size {
                    if code == (*delta_time_code_table.offset(e as isize)).code {
                        time_delta = (*delta_time_code_table.offset(e as isize)).value;
                        code_found = 1 as core::ffi::c_int as WORD32;
                        break;
                    } else {
                        e += 1;
                    }
                }
            }
            node_time_tmp = time_offs + time_delta * delta_tmin;
            if node_time_tmp >= 2 as WORD32 * AUDIO_CODEC_FRAME_SIZE_MAX - drc_frame_size
            {
                return UNEXPECTED_ERROR;
            }
            if node_time_tmp > drc_frame_size + time_offset {
                if node_res_flag == 0 as core::ffi::c_int {
                    (*str_node.offset(k as isize)).time = drc_frame_size + time_offset;
                    node_res_flag = 1 as core::ffi::c_int as WORD32;
                }
                (*str_node
                    .offset((k as core::ffi::c_int + 1 as core::ffi::c_int) as isize))
                    .time = node_time_tmp;
            } else {
                (*str_node.offset(k as isize)).time = node_time_tmp;
            }
            time_offs = node_time_tmp;
            k += 1;
        }
        if node_res_flag == 0 as core::ffi::c_int {
            (*str_node.offset(k as isize)).time = drc_frame_size + time_offset;
        }
    } else {
        k = 0 as core::ffi::c_int as WORD32;
        while k < num_nodes {
            num_bits_read = 0 as core::ffi::c_int as WORD32;
            code = 0 as core::ffi::c_int as WORD32;
            code_found = 0 as core::ffi::c_int as WORD32;
            e = 1 as core::ffi::c_int as WORD32;
            exit_cnt = 0 as core::ffi::c_int as WORD32;
            while e < N_DELTA_TIME_CODE_TABLE_ENTRIES_MAX && code_found == 0 {
                exit_cnt += 1;
                if exit_cnt > 100000 as core::ffi::c_int {
                    return 6 as WORD32;
                }
                m = 0 as core::ffi::c_int as WORD32;
                while m
                    < (*delta_time_code_table.offset(e as isize)).size - num_bits_read
                {
                    bit = impd_read_bits_buf(it_bit_buff, 1 as WORD);
                    if (*it_bit_buff).error != 0 {
                        return (*it_bit_buff).error;
                    }
                    code = (code << 1 as core::ffi::c_int) + bit;
                    num_bits_read += 1;
                    m += 1;
                }
                while num_bits_read == (*delta_time_code_table.offset(e as isize)).size {
                    if code == (*delta_time_code_table.offset(e as isize)).code {
                        time_delta = (*delta_time_code_table.offset(e as isize)).value;
                        code_found = 1 as core::ffi::c_int as WORD32;
                        break;
                    } else {
                        e += 1;
                    }
                }
            }
            time_offs += time_delta * delta_tmin;
            if time_offs >= 2 as WORD32 * AUDIO_CODEC_FRAME_SIZE_MAX - drc_frame_size {
                return UNEXPECTED_ERROR;
            }
            (*str_node.offset(k as isize)).time = time_offs;
            k += 1;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_drc_uni_gain_read(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut pstr_drc_uni_bs_dec: *mut ia_drc_bits_dec_struct,
    mut drc_config: *mut ia_drc_config,
    mut pstr_uni_drc_gain: *mut ia_drc_gain_struct,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut seq: WORD32 = 0;
    static mut pkt_loss_frame_cnt: WORD32 = 0 as WORD32;
    let mut str_spline_nodes: *mut ia_spline_nodes_struct = 0
        as *mut ia_spline_nodes_struct;
    let mut gain_sequence_count: WORD32 = (*drc_config)
        .str_p_loc_drc_coefficients_uni_drc[0 as core::ffi::c_int as usize]
        .gain_sequence_count;
    seq = 0 as core::ffi::c_int as WORD32;
    while seq < gain_sequence_count {
        let mut index: WORD32 = (*drc_config)
            .str_p_loc_drc_coefficients_uni_drc[0 as core::ffi::c_int as usize]
            .gain_set_params_index_for_gain_sequence[seq as usize];
        let mut str_gain_set_params: ia_gain_set_params_struct = {
            let mut init = ia_gain_set_params_struct {
                gain_coding_profile: 0 as WORD32,
                gain_interpolation_type: 0,
                full_frame: 0,
                time_alignment: 0,
                time_delt_min_flag: 0,
                time_delt_min_val: 0,
                band_count: 0,
                drc_band_type: 0,
                gain_params: [ia_gain_params_struct {
                    gain_seq_idx: 0,
                    drc_characteristic: 0,
                    drc_characteristic_present: 0,
                    drc_characteristic_format_is_cicp: 0,
                    drc_characteristic_left_index: 0,
                    drc_characteristic_right_index: 0,
                    crossover_freq_idx: 0,
                    start_subband_index: 0,
                }; 8],
                num_gain_max_values: 0,
                str_tables: ia_tables_struct {
                    delta_time_code_table: [ia_delta_time_code_table_entry_struct {
                        size: 0,
                        code: 0,
                        value: 0,
                    }; 526],
                },
            };
            init
        };
        let mut gain_set_params: *mut ia_gain_set_params_struct = &mut str_gain_set_params;
        if index != -(1 as core::ffi::c_int) {
            gain_set_params = &mut *((*((*drc_config).str_p_loc_drc_coefficients_uni_drc)
                .as_mut_ptr())
                .gain_set_params)
                .as_mut_ptr()
                .offset(index as isize) as *mut ia_gain_set_params_struct;
        }
        if (*gain_set_params).gain_coding_profile == GAIN_CODING_PROFILE_CONSTANT {
            str_spline_nodes = &mut *((*((*pstr_uni_drc_gain).drc_gain_sequence)
                .as_mut_ptr()
                .offset(seq as isize))
                .str_spline_nodes)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut ia_spline_nodes_struct;
            (*str_spline_nodes).num_nodes = 1 as core::ffi::c_int as WORD32;
            (*str_spline_nodes).str_node[0 as core::ffi::c_int as usize].slope = 0.0f32;
            (*str_spline_nodes).str_node[0 as core::ffi::c_int as usize].time = ((*pstr_drc_uni_bs_dec)
                .ia_drc_params_struct
                .drc_frame_size as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
            (*str_spline_nodes).str_node[0 as core::ffi::c_int as usize].loc_db_gain = 0.0f32
                as FLOAT32;
        } else {
            err = impd_parse_drc_gain_sequence(
                it_bit_buff,
                pstr_drc_uni_bs_dec,
                gain_set_params,
                &mut *((*pstr_uni_drc_gain).drc_gain_sequence)
                    .as_mut_ptr()
                    .offset(seq as isize),
            );
            if err != 0 {
                return err;
            }
        }
        seq += 1;
    }
    if ((*it_bit_buff).ptr_bit_buf_base).is_null() {
        pkt_loss_frame_cnt += 1;
        if pkt_loss_frame_cnt as FLOAT32
            * (*pstr_drc_uni_bs_dec).ia_drc_params_struct.drc_frame_size as FLOAT32
            / (*drc_config).sampling_rate as FLOAT32 > MAXPACKETLOSSTIME
        {
            (*drc_config).apply_drc = 0 as core::ffi::c_int as WORD32;
        }
    } else {
        (*pstr_uni_drc_gain).uni_drc_gain_ext_flag = impd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*pstr_uni_drc_gain).uni_drc_gain_ext_flag == 1 as core::ffi::c_int {
            err = impd_parse_uni_drc_gain_ext(
                it_bit_buff,
                &mut (*pstr_uni_drc_gain).uni_drc_gain_ext,
            );
            if err != 0 {
                return err;
            }
        }
        pkt_loss_frame_cnt = 0 as core::ffi::c_int as WORD32;
        (*drc_config).apply_drc = 1 as core::ffi::c_int as WORD32;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_uni_drc_gain_ext(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut uni_drc_gain_ext: *mut ia_uni_drc_gain_ext_struct,
) -> WORD32 {
    let mut k: WORD32 = 0;
    let mut bit_size_len: WORD32 = 0;
    let mut ext_size_bits: WORD32 = 0;
    let mut bit_size: WORD32 = 0;
    k = 0 as core::ffi::c_int as WORD32;
    (*uni_drc_gain_ext).uni_drc_gain_ext_type[k as usize] = impd_read_bits_buf(
        it_bit_buff,
        4 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    while (*uni_drc_gain_ext).uni_drc_gain_ext_type[k as usize] != UNIDRCGAINEXT_TERM {
        if k >= EXT_COUNT_MAX - 1 as core::ffi::c_int {
            return UNEXPECTED_ERROR;
        }
        bit_size_len = impd_read_bits_buf(it_bit_buff, 3 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        ext_size_bits = (bit_size_len as core::ffi::c_int + 4 as core::ffi::c_int)
            as WORD32;
        bit_size = impd_read_bits_buf(it_bit_buff, ext_size_bits as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*uni_drc_gain_ext).ext_bit_size[k as usize] = (bit_size as core::ffi::c_int
            + 1 as core::ffi::c_int) as WORD32;
        impd_skip_bits_buf(it_bit_buff, (*uni_drc_gain_ext).ext_bit_size[k as usize]);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        k += 1;
        (*uni_drc_gain_ext).uni_drc_gain_ext_type[k as usize] = impd_read_bits_buf(
            it_bit_buff,
            4 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_spline_nodes(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut pstr_drc_uni_bs_dec: *mut ia_drc_bits_dec_struct,
    mut gain_set_params: *mut ia_gain_set_params_struct,
    mut str_spline_nodes: *mut ia_spline_nodes_struct,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut time_offset: WORD32 = 0;
    if (*gain_set_params).time_alignment == 0 as core::ffi::c_int {
        time_offset = -(1 as core::ffi::c_int) as WORD32;
    } else if (*gain_set_params).time_delt_min_flag != 0 {
        time_offset = (-((*gain_set_params).time_delt_min_val as core::ffi::c_int)
            + ((*gain_set_params).time_delt_min_val as core::ffi::c_int
                - 1 as core::ffi::c_int) / 2 as core::ffi::c_int) as WORD32;
    } else {
        time_offset = (-((*pstr_drc_uni_bs_dec).ia_drc_params_struct.delta_tmin_default
            as core::ffi::c_int)
            + ((*pstr_drc_uni_bs_dec).ia_drc_params_struct.delta_tmin_default
                as core::ffi::c_int - 1 as core::ffi::c_int) / 2 as core::ffi::c_int)
            as WORD32;
    }
    if ((*it_bit_buff).ptr_bit_buf_base).is_null() {
        if (*str_spline_nodes).num_nodes < 1 as core::ffi::c_int
            || (*str_spline_nodes).num_nodes > NODE_COUNT_MAX
        {
            return UNEXPECTED_ERROR;
        }
        let mut prev_db_gain: FLOAT32 = (*str_spline_nodes)
            .str_node[((*str_spline_nodes).num_nodes as core::ffi::c_int
                - 1 as core::ffi::c_int) as usize]
            .loc_db_gain;
        (*str_spline_nodes).drc_gain_coding_mode = 0 as core::ffi::c_int as WORD32;
        (*str_spline_nodes).num_nodes = 1 as core::ffi::c_int as WORD32;
        if prev_db_gain < 0 as core::ffi::c_int as FLOAT32 {
            (*str_spline_nodes).str_node[0 as core::ffi::c_int as usize].loc_db_gain = prev_db_gain;
        } else {
            (*str_spline_nodes).str_node[0 as core::ffi::c_int as usize].loc_db_gain = 0.0f32
                as FLOAT32;
        }
        (*str_spline_nodes).str_node[0 as core::ffi::c_int as usize].slope = 0.0f32;
        (*str_spline_nodes).str_node[0 as core::ffi::c_int as usize].time = (*pstr_drc_uni_bs_dec)
            .ia_drc_params_struct
            .drc_frame_size + time_offset;
    } else {
        (*str_spline_nodes).drc_gain_coding_mode = impd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        );
        if (*it_bit_buff).error == PROC_COMPLETE {
            if (*str_spline_nodes).num_nodes < 1 as core::ffi::c_int
                || (*str_spline_nodes).num_nodes > NODE_COUNT_MAX
            {
                return UNEXPECTED_ERROR;
            }
            (*str_spline_nodes).drc_gain_coding_mode = 0 as core::ffi::c_int as WORD32;
            (*str_spline_nodes).str_node[0 as core::ffi::c_int as usize].slope = 0.0f32;
            (*str_spline_nodes).str_node[0 as core::ffi::c_int as usize].time = (*pstr_drc_uni_bs_dec)
                .ia_drc_params_struct
                .drc_frame_size + time_offset;
            (*str_spline_nodes).str_node[0 as core::ffi::c_int as usize].loc_db_gain = (*str_spline_nodes)
                .str_node[((*str_spline_nodes).num_nodes as core::ffi::c_int
                    - 1 as core::ffi::c_int) as usize]
                .loc_db_gain;
            (*str_spline_nodes).num_nodes = 1 as core::ffi::c_int as WORD32;
        } else if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error
        }
        if (*str_spline_nodes).drc_gain_coding_mode == 0 as core::ffi::c_int {
            (*str_spline_nodes).num_nodes = 1 as core::ffi::c_int as WORD32;
            err = impd_dec_initial_gain(
                it_bit_buff,
                (*gain_set_params).gain_coding_profile,
                &mut (*((*str_spline_nodes).str_node)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize))
                    .loc_db_gain,
            );
            if err != 0 {
                return err;
            }
            (*str_spline_nodes).str_node[0 as core::ffi::c_int as usize].slope = 0.0f32;
            (*str_spline_nodes).str_node[0 as core::ffi::c_int as usize].time = (*pstr_drc_uni_bs_dec)
                .ia_drc_params_struct
                .drc_frame_size + time_offset;
        } else {
            err = impd_dec_slopes(
                it_bit_buff,
                &mut (*str_spline_nodes).num_nodes,
                (*gain_set_params).gain_interpolation_type,
                ((*str_spline_nodes).str_node).as_mut_ptr(),
            );
            if err != 0 {
                return err;
            }
            if (*gain_set_params).time_delt_min_flag != 0 {
                err = impd_dec_times(
                    it_bit_buff,
                    &mut (*gain_set_params).str_tables,
                    (*str_spline_nodes).num_nodes,
                    (*gain_set_params).time_delt_min_val,
                    (*pstr_drc_uni_bs_dec).ia_drc_params_struct.drc_frame_size,
                    (*gain_set_params).full_frame,
                    time_offset,
                    ((*str_spline_nodes).str_node).as_mut_ptr(),
                );
                if err != 0 {
                    return err;
                }
                err = impd_dec_gains(
                    it_bit_buff,
                    (*str_spline_nodes).num_nodes,
                    (*gain_set_params).gain_coding_profile,
                    ((*str_spline_nodes).str_node).as_mut_ptr(),
                );
                if err != 0 {
                    return err;
                }
            } else {
                err = impd_dec_times(
                    it_bit_buff,
                    &mut (*pstr_drc_uni_bs_dec).tables_default,
                    (*str_spline_nodes).num_nodes,
                    (*pstr_drc_uni_bs_dec).ia_drc_params_struct.delta_tmin_default,
                    (*pstr_drc_uni_bs_dec).ia_drc_params_struct.drc_frame_size,
                    (*gain_set_params).full_frame,
                    time_offset,
                    ((*str_spline_nodes).str_node).as_mut_ptr(),
                );
                if err != 0 {
                    return err;
                }
                err = impd_dec_gains(
                    it_bit_buff,
                    (*str_spline_nodes).num_nodes,
                    (*gain_set_params).gain_coding_profile,
                    ((*str_spline_nodes).str_node).as_mut_ptr(),
                );
                if err != 0 {
                    return err;
                }
            }
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_drc_gain_sequence(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut pstr_drc_uni_bs_dec: *mut ia_drc_bits_dec_struct,
    mut gain_set_params: *mut ia_gain_set_params_struct,
    mut drc_gain_sequence: *mut ia_drc_gain_sequence_struct,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut i: WORD32 = 0;
    let mut prev_frame_time_buf: [WORD32; 512] = [0; 512];
    let mut cur_frame_time_buf: [WORD32; 512] = [0; 512];
    let mut num_nodes_node_reservoir: WORD32 = 0;
    let mut num_nodes_cur: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut m: WORD32 = 0;
    if (*pstr_drc_uni_bs_dec).ia_drc_params_struct.delay_mode == DELAY_MODE_LOW_DELAY
        && (*gain_set_params).full_frame == 0 as core::ffi::c_int
    {
        return 3 as WORD32;
    }
    i = 0 as core::ffi::c_int as WORD32;
    err = impd_parse_spline_nodes(
        it_bit_buff,
        pstr_drc_uni_bs_dec,
        gain_set_params,
        &mut *((*drc_gain_sequence).str_spline_nodes).as_mut_ptr().offset(i as isize),
    );
    if err != 0 {
        return err;
    }
    num_nodes_node_reservoir = 0 as core::ffi::c_int as WORD32;
    num_nodes_cur = 0 as core::ffi::c_int as WORD32;
    k = 0 as core::ffi::c_int as WORD32;
    while k
        < (*((*drc_gain_sequence).str_spline_nodes).as_mut_ptr().offset(i as isize))
            .num_nodes
    {
        if (*((*drc_gain_sequence).str_spline_nodes).as_mut_ptr().offset(i as isize))
            .str_node[k as usize]
            .time >= (*pstr_drc_uni_bs_dec).ia_drc_params_struct.drc_frame_size
        {
            prev_frame_time_buf[num_nodes_node_reservoir as usize] = (*((*drc_gain_sequence)
                .str_spline_nodes)
                .as_mut_ptr()
                .offset(i as isize))
                .str_node[k as usize]
                .time;
            num_nodes_node_reservoir += 1;
        } else {
            cur_frame_time_buf[num_nodes_cur as usize] = (*((*drc_gain_sequence)
                .str_spline_nodes)
                .as_mut_ptr()
                .offset(i as isize))
                .str_node[k as usize]
                .time;
            num_nodes_cur += 1;
        }
        k += 1;
    }
    k = 0 as core::ffi::c_int as WORD32;
    while k < num_nodes_node_reservoir {
        let mut tmp: WORD32 = prev_frame_time_buf[k as usize]
            - 2 as WORD32 * (*pstr_drc_uni_bs_dec).ia_drc_params_struct.drc_frame_size;
        if tmp
            >= 2 as WORD32 * AUDIO_CODEC_FRAME_SIZE_MAX
                - (*pstr_drc_uni_bs_dec).ia_drc_params_struct.drc_frame_size
        {
            return UNEXPECTED_ERROR;
        }
        (*((*drc_gain_sequence).str_spline_nodes).as_mut_ptr().offset(i as isize))
            .str_node[k as usize]
            .time = tmp;
        k += 1;
    }
    m = 0 as core::ffi::c_int as WORD32;
    while m < num_nodes_cur {
        (*((*drc_gain_sequence).str_spline_nodes).as_mut_ptr().offset(i as isize))
            .str_node[k as usize]
            .time = cur_frame_time_buf[m as usize];
        m += 1;
        k += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_drc_ext_v1(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ia_drc_params_struct: *mut ia_drc_params_bs_dec_struct,
    mut drc_config: *mut ia_drc_config,
    mut str_drc_config_ext: *mut ia_drc_config_ext,
) -> WORD32 {
    let mut dwnmix_instructions_v1_flag: WORD32 = 0;
    let mut dwnmix_instructions_v1_count: WORD32 = 0;
    let mut drc_coeffs_and_instructions_uni_drc_v1_flag: WORD32 = 0;
    let mut drc_coefficients_uni_drc_v1_count: WORD32 = 0;
    let mut drc_instructions_uni_drc_v1_count: WORD32 = 0;
    let mut i: WORD32 = 0 as WORD32;
    let mut err: WORD32 = 0 as WORD32;
    let version: WORD32 = 1 as WORD32;
    dwnmix_instructions_v1_flag = impd_read_bits_buf(it_bit_buff, 1 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if dwnmix_instructions_v1_flag == 1 as core::ffi::c_int {
        dwnmix_instructions_v1_count = impd_read_bits_buf(it_bit_buff, 7 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if dwnmix_instructions_v1_count + (*drc_config).dwnmix_instructions_count
            > DOWNMIX_INSTRUCTION_COUNT_MAX
        {
            return UNEXPECTED_ERROR;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < dwnmix_instructions_v1_count {
            err = impd_parse_dwnmix_instructions(
                it_bit_buff,
                version,
                ia_drc_params_struct,
                &mut (*drc_config).channel_layout,
                &mut *((*drc_config).dwnmix_instructions)
                    .as_mut_ptr()
                    .offset((i + (*drc_config).dwnmix_instructions_count) as isize),
            );
            if err != 0 {
                return err;
            }
            i += 1;
        }
        (*drc_config).dwnmix_instructions_count += dwnmix_instructions_v1_count;
    }
    drc_coeffs_and_instructions_uni_drc_v1_flag = impd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if drc_coeffs_and_instructions_uni_drc_v1_flag == 1 as core::ffi::c_int {
        drc_coefficients_uni_drc_v1_count = impd_read_bits_buf(it_bit_buff, 3 as WORD);
        if drc_coefficients_uni_drc_v1_count + (*drc_config).drc_coefficients_drc_count
            > DRC_COEFF_COUNT_MAX
        {
            return 2 as WORD32;
        }
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < drc_coefficients_uni_drc_v1_count {
            err = impd_drc_parse_coeff(
                it_bit_buff,
                version,
                ia_drc_params_struct,
                &mut *((*drc_config).str_p_loc_drc_coefficients_uni_drc)
                    .as_mut_ptr()
                    .offset((i + (*drc_config).drc_coefficients_drc_count) as isize),
            );
            if err != 0 {
                return err;
            }
            i += 1;
        }
        (*drc_config).drc_coefficients_drc_count += drc_coefficients_uni_drc_v1_count;
        drc_instructions_uni_drc_v1_count = impd_read_bits_buf(it_bit_buff, 6 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*drc_config).drc_instructions_uni_drc_count
            + drc_instructions_uni_drc_v1_count > DRC_INSTRUCTIONS_COUNT_MAX
        {
            return 2 as WORD32;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < drc_instructions_uni_drc_v1_count {
            err = impd_parse_drc_instructions_uni_drc(
                it_bit_buff,
                version,
                drc_config,
                &mut *((*drc_config).str_drc_instruction_str)
                    .as_mut_ptr()
                    .offset((i + (*drc_config).drc_instructions_uni_drc_count) as isize),
            );
            if err != 0 {
                return err;
            }
            i += 1;
        }
        (*drc_config).drc_instructions_uni_drc_count
            += drc_instructions_uni_drc_v1_count;
    }
    (*str_drc_config_ext).loud_eq_instructions_flag = impd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*str_drc_config_ext).loud_eq_instructions_flag == 1 as core::ffi::c_int {
        (*str_drc_config_ext).loud_eq_instructions_count = impd_read_bits_buf(
            it_bit_buff,
            4 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*str_drc_config_ext).loud_eq_instructions_count
            > LOUD_EQ_INSTRUCTIONS_COUNT_MAX
        {
            return UNEXPECTED_ERROR;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*str_drc_config_ext).loud_eq_instructions_count {
            err = impd_parse_loud_eq_instructions(
                it_bit_buff,
                &mut *((*str_drc_config_ext).loud_eq_instructions)
                    .as_mut_ptr()
                    .offset(i as isize),
            );
            if err != 0 {
                return err;
            }
            i += 1;
        }
    } else {
        (*str_drc_config_ext).loud_eq_instructions_count = 0 as core::ffi::c_int
            as WORD32;
    }
    (*str_drc_config_ext).eq_flag = impd_read_bits_buf(it_bit_buff, 1 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*str_drc_config_ext).eq_flag == 1 as core::ffi::c_int {
        err = impd_parse_eq_coefficients(
            it_bit_buff,
            &mut (*str_drc_config_ext).str_eq_coeff,
        );
        if err != 0 {
            return err;
        }
        (*str_drc_config_ext).eq_instructions_count = impd_read_bits_buf(
            it_bit_buff,
            4 as WORD,
        );
        if (*str_drc_config_ext).eq_instructions_count > EQ_INSTRUCTIONS_COUNT_MAX {
            return UNEXPECTED_ERROR;
        }
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*str_drc_config_ext).eq_instructions_count {
            err = impd_parse_eq_instructions(
                it_bit_buff,
                drc_config,
                &mut *((*str_drc_config_ext).str_eq_instructions)
                    .as_mut_ptr()
                    .offset(i as isize),
            );
            if err != 0 {
                return err;
            }
            i += 1;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_leveling_instructions(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut pstr_drc_config: *mut ia_drc_config,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut err: WORD32 = IA_NO_ERROR;
    let mut drc_instruction_uni_drc_count: WORD32 = (*pstr_drc_config)
        .drc_instructions_uni_drc_count;
    i = 0 as core::ffi::c_int as WORD32;
    while i < drc_instruction_uni_drc_count {
        if (*pstr_drc_config).str_drc_instruction_str[i as usize].drc_set_effect
            as core::ffi::c_int & (1 as core::ffi::c_int) << 11 as core::ffi::c_int
            != 0 as core::ffi::c_int
        {
            (*pstr_drc_config).str_drc_instruction_str[i as usize].leveling_present = impd_read_bits_buf(
                it_bit_buff,
                1 as WORD,
            );
            if (*pstr_drc_config).str_drc_instruction_str[i as usize].leveling_present
                != 0
            {
                let mut ducking_only_drc_set_present: WORD32 = impd_read_bits_buf(
                    it_bit_buff,
                    1 as WORD,
                );
                if ducking_only_drc_set_present != 0 {
                    (*pstr_drc_config).drc_instructions_uni_drc_count += 1;
                    if (*pstr_drc_config).drc_instructions_uni_drc_count
                        > DRC_INSTRUCTIONS_COUNT_MAX
                    {
                        return IA_XHEAAC_DEC_INIT_NONFATAL_MAX_INSTRUCTIONS_ERROR;
                    }
                    err = impd_parse_drc_instructions_uni_drc(
                        it_bit_buff,
                        1 as WORD32,
                        pstr_drc_config,
                        &mut *((*pstr_drc_config).str_drc_instruction_str)
                            .as_mut_ptr()
                            .offset(
                                ((*pstr_drc_config).drc_instructions_uni_drc_count
                                    as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                            ),
                    );
                    if err != 0 {
                        return err;
                    }
                }
            }
        }
        i += 1;
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_filt_block(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut str_filter_block: *mut ia_filt_block_struct,
    mut block_count: WORD32,
) -> WORD32 {
    let mut k: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut str_filter_element: *mut ia_filt_ele_struct = 0 as *mut ia_filt_ele_struct;
    j = 0 as core::ffi::c_int as WORD32;
    while j < block_count {
        (*str_filter_block).filter_element_count = impd_read_bits_buf(
            it_bit_buff,
            6 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*str_filter_block).filter_element_count > FILTER_ELEMENT_COUNT_MAX {
            return UNEXPECTED_ERROR;
        }
        str_filter_element = &mut *((*str_filter_block).str_filter_element)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut ia_filt_ele_struct;
        k = 0 as core::ffi::c_int as WORD32;
        while k < (*str_filter_block).filter_element_count {
            temp = impd_read_bits_buf(it_bit_buff, 7 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            (*str_filter_element).filt_ele_idx = ((temp as core::ffi::c_int
                & 0x7e as core::ffi::c_int) >> 1 as core::ffi::c_int) as WORD32;
            if (*str_filter_element).filt_ele_idx >= FILTER_ELEMENT_COUNT_MAX {
                return 2 as WORD32;
            }
            (*str_filter_element).filt_ele_gain_flag = (temp as core::ffi::c_int
                & 1 as core::ffi::c_int) as WORD32;
            if (*str_filter_element).filt_ele_gain_flag != 0 {
                let mut bs_filter_element_gain: WORD32 = 0;
                bs_filter_element_gain = impd_read_bits_buf(it_bit_buff, 10 as WORD);
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                (*str_filter_element).filt_ele_gain = (bs_filter_element_gain
                    as core::ffi::c_float * 0.125f32 - 96.0f32) as FLOAT32;
            }
            str_filter_element = str_filter_element.offset(1);
            k += 1;
        }
        str_filter_block = str_filter_block.offset(1);
        j += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_unique_td_filt_ele(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut unique_td_filt_ele: *mut ia_unique_td_filt_element,
    mut td_filter_element_count: WORD32,
) -> WORD32 {
    let mut m: WORD32 = 0;
    let mut sign: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut tmp: FLOAT32 = 0.;
    j = 0 as core::ffi::c_int as WORD32;
    while j < td_filter_element_count {
        (*unique_td_filt_ele).eq_filter_format = impd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*unique_td_filt_ele).eq_filter_format == 0 as core::ffi::c_int {
            let mut bs_real_zero_radius: WORD32 = 0;
            let mut bs_generic_zero_radius: WORD32 = 0;
            let mut bs_generic_zero_angle: WORD32 = 0;
            let mut bs_real_pole_radius: WORD32 = 0;
            let mut bs_cmplx_pole_radius: WORD32 = 0;
            let mut bs_cmplx_pole_angle: WORD32 = 0;
            let mut bs_real_zero_radius_one_count: WORD32 = 0;
            temp = impd_read_bits_buf(it_bit_buff, 23 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            bs_real_zero_radius_one_count = (temp as core::ffi::c_int
                >> 20 as core::ffi::c_int & 7 as core::ffi::c_int) as WORD32;
            (*unique_td_filt_ele).bs_real_zero_radius_one_count = 2 as WORD32
                * bs_real_zero_radius_one_count;
            (*unique_td_filt_ele).real_zero_count = ((temp as core::ffi::c_int
                & 0xfc000 as core::ffi::c_int) >> 14 as core::ffi::c_int) as WORD32;
            (*unique_td_filt_ele).generic_zero_count = ((temp as core::ffi::c_int
                & 0x3f00 as core::ffi::c_int) >> 8 as core::ffi::c_int) as WORD32;
            (*unique_td_filt_ele).real_pole_count = ((temp as core::ffi::c_int
                & 0xf0 as core::ffi::c_int) >> 4 as core::ffi::c_int) as WORD32;
            (*unique_td_filt_ele).cmplx_pole_count = (temp as core::ffi::c_int
                & 0xf as core::ffi::c_int) as WORD32;
            temp = impd_read_bits_buf(
                it_bit_buff,
                (*unique_td_filt_ele).bs_real_zero_radius_one_count as WORD,
            );
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            m = ((*unique_td_filt_ele).bs_real_zero_radius_one_count as core::ffi::c_int
                - 1 as core::ffi::c_int) as WORD32;
            while m >= 0 as core::ffi::c_int {
                (*unique_td_filt_ele).zero_sign[m as usize] = (temp as core::ffi::c_int
                    & 1 as core::ffi::c_int) as WORD32;
                temp = temp >> 1 as core::ffi::c_int;
                m -= 1;
            }
            m = 0 as core::ffi::c_int as WORD32;
            while m < (*unique_td_filt_ele).real_zero_count {
                temp = impd_read_bits_buf(it_bit_buff, 8 as WORD);
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                bs_real_zero_radius = ((temp as core::ffi::c_int
                    & 0xfe as core::ffi::c_int) >> 1 as core::ffi::c_int) as WORD32;
                sign = (temp as core::ffi::c_int & 0x1 as core::ffi::c_int) as WORD32;
                tmp = 1.0f32
                    - *zero_pole_radius_tbl
                        .as_ptr()
                        .offset(bs_real_zero_radius as isize);
                sign = sign << 1 as core::ffi::c_int;
                (*unique_td_filt_ele).real_zero_radius[m as usize] = (1 as WORD32 - sign)
                    as FLOAT32 * tmp;
                m += 1;
            }
            m = 0 as core::ffi::c_int as WORD32;
            while m < (*unique_td_filt_ele).generic_zero_count {
                temp = impd_read_bits_buf(it_bit_buff, 14 as WORD);
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                bs_generic_zero_radius = ((temp as core::ffi::c_int
                    & 0x3f80 as core::ffi::c_int) >> 7 as core::ffi::c_int) as WORD32;
                (*unique_td_filt_ele).generic_zero_radius[m as usize] = 1.0f32
                    - *zero_pole_radius_tbl
                        .as_ptr()
                        .offset(bs_generic_zero_radius as isize);
                bs_generic_zero_angle = (temp as core::ffi::c_int
                    & 0x7f as core::ffi::c_int) as WORD32;
                (*unique_td_filt_ele).generic_zero_angle[m as usize] = *zero_pole_angle_tbl
                    .as_ptr()
                    .offset(bs_generic_zero_angle as isize);
                m += 1;
            }
            m = 0 as core::ffi::c_int as WORD32;
            while m < (*unique_td_filt_ele).real_pole_count {
                temp = impd_read_bits_buf(it_bit_buff, 8 as WORD);
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                bs_real_pole_radius = ((temp as core::ffi::c_int
                    & 0xfe as core::ffi::c_int) >> 1 as core::ffi::c_int) as WORD32;
                sign = (temp as core::ffi::c_int & 0x1 as core::ffi::c_int) as WORD32;
                tmp = 1.0f32
                    - *zero_pole_radius_tbl
                        .as_ptr()
                        .offset(bs_real_pole_radius as isize);
                sign = sign << 1 as core::ffi::c_int;
                (*unique_td_filt_ele).real_pole_radius[m as usize] = (1 as WORD32 - sign)
                    as FLOAT32 * tmp;
                m += 1;
            }
            m = 0 as core::ffi::c_int as WORD32;
            while m < (*unique_td_filt_ele).cmplx_pole_count {
                temp = impd_read_bits_buf(it_bit_buff, 14 as WORD);
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                bs_cmplx_pole_radius = ((temp as core::ffi::c_int
                    & 0x3f80 as core::ffi::c_int) >> 7 as core::ffi::c_int) as WORD32;
                (*unique_td_filt_ele).complex_pole_radius[m as usize] = 1.0f32
                    - *zero_pole_radius_tbl
                        .as_ptr()
                        .offset(bs_cmplx_pole_radius as isize);
                bs_cmplx_pole_angle = (temp as core::ffi::c_int
                    & 0x7f as core::ffi::c_int) as WORD32;
                (*unique_td_filt_ele).complex_pole_angle[m as usize] = *zero_pole_angle_tbl
                    .as_ptr()
                    .offset(bs_cmplx_pole_angle as isize);
                m += 1;
            }
        } else {
            temp = impd_read_bits_buf(it_bit_buff, 8 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            (*unique_td_filt_ele).fir_filt_order = ((temp as core::ffi::c_int
                & 0xfe as core::ffi::c_int) >> 1 as core::ffi::c_int) as WORD32;
            (*unique_td_filt_ele).fir_symmetry = (temp as core::ffi::c_int
                & 0x1 as core::ffi::c_int) as WORD32;
            m = 0 as core::ffi::c_int as WORD32;
            while m
                < (*unique_td_filt_ele).fir_filt_order as core::ffi::c_int
                    / 2 as core::ffi::c_int + 1 as core::ffi::c_int
            {
                let mut sign_0: WORD32 = 0;
                let mut bs_fir_coeff: WORD32 = 0;
                let mut tmp_0: FLOAT32 = 0.;
                temp = impd_read_bits_buf(it_bit_buff, 11 as WORD);
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                sign_0 = (temp as core::ffi::c_int >> 10 as core::ffi::c_int
                    & 0x1 as core::ffi::c_int) as WORD32;
                bs_fir_coeff = (temp as core::ffi::c_int & 0x3ff as core::ffi::c_int)
                    as WORD32;
                tmp_0 = pow(
                    10.0f64,
                    (-0.05f32 * bs_fir_coeff as core::ffi::c_float * 0.0625f32)
                        as core::ffi::c_double,
                ) as FLOAT32;
                sign_0 = sign_0 << 1 as core::ffi::c_int;
                (*unique_td_filt_ele).fir_coeff[m as usize] = (1 as WORD32 - sign_0)
                    as FLOAT32 * tmp_0;
                m += 1;
            }
        }
        unique_td_filt_ele = unique_td_filt_ele.offset(1);
        j += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_decode_eq_slope_code(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut eq_slope: *mut FLOAT32,
    mut num_eq_nodes: WORD32,
) -> WORD32 {
    let mut bits: WORD32 = 0 as WORD32;
    let mut k: WORD32 = 0;
    k = 0 as core::ffi::c_int as WORD32;
    while k < num_eq_nodes {
        bits = impd_read_bits_buf(it_bit_buff, 1 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if bits == 0x1 as core::ffi::c_int {
            *eq_slope = 0.0f32 as FLOAT32;
        } else {
            bits = impd_read_bits_buf(it_bit_buff, 4 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            *eq_slope = *eq_slope_tbl.as_ptr().offset(bits as isize);
        }
        eq_slope = eq_slope.offset(1);
        k += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_decode_gain_initial_code(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut eq_gain_initial: *mut FLOAT32,
) -> WORD32 {
    let mut bits: WORD32 = 0;
    let mut bits1: WORD32 = 0;
    bits1 = impd_read_bits_buf(it_bit_buff, 2 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    match bits1 {
        0 => {
            bits = impd_read_bits_buf(it_bit_buff, 5 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            *eq_gain_initial = (0.5f32 * bits as core::ffi::c_float - 8.0f32) as FLOAT32;
        }
        1 | 2 => {
            bits = impd_read_bits_buf(it_bit_buff, 4 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if bits < 8 as core::ffi::c_int {
                *eq_gain_initial = ((bits1 * bits) as core::ffi::c_float
                    - bits1 as core::ffi::c_float * 16.0f32) as FLOAT32;
            } else {
                *eq_gain_initial = bits1 as FLOAT32 * bits as FLOAT32;
            }
        }
        3 => {
            bits = impd_read_bits_buf(it_bit_buff, 3 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            *eq_gain_initial = (4.0f32 * bits as core::ffi::c_float - 64.0f32)
                as FLOAT32;
        }
        _ => {}
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_eq_subband_gain_spline(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut str_eq_subband_gain_spline: *mut ia_eq_subband_gain_spline_struct,
    mut eq_subband_gains_count: WORD32,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut eq_nodes_cnt: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut bits: WORD32 = 0;
    let mut eq_freq_delta: *mut WORD32 = 0 as *mut WORD32;
    let mut peq_gain_delta: *mut FLOAT32 = 0 as *mut FLOAT32;
    j = 0 as core::ffi::c_int as WORD32;
    while j < eq_subband_gains_count {
        eq_nodes_cnt = impd_read_bits_buf(it_bit_buff, 5 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*str_eq_subband_gain_spline).num_eq_nodes = (eq_nodes_cnt as core::ffi::c_int
            + 2 as core::ffi::c_int) as WORD32;
        err = impd_decode_eq_slope_code(
            it_bit_buff,
            &mut *((*str_eq_subband_gain_spline).eq_slope)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize),
            (*str_eq_subband_gain_spline).num_eq_nodes,
        );
        if err != 0 {
            return err;
        }
        eq_freq_delta = &mut *((*str_eq_subband_gain_spline).eq_freq_delta)
            .as_mut_ptr()
            .offset(1 as core::ffi::c_int as isize) as *mut WORD32;
        k = 1 as core::ffi::c_int as WORD32;
        while k < (*str_eq_subband_gain_spline).num_eq_nodes {
            bits = impd_read_bits_buf(it_bit_buff, 4 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            *eq_freq_delta = (bits as core::ffi::c_int + 1 as core::ffi::c_int)
                as WORD32;
            eq_freq_delta = eq_freq_delta.offset(1);
            k += 1;
        }
        err = impd_decode_gain_initial_code(
            it_bit_buff,
            &mut (*str_eq_subband_gain_spline).eq_gain_initial,
        );
        if err != 0 {
            return err;
        }
        peq_gain_delta = &mut *((*str_eq_subband_gain_spline).eq_gain_delta)
            .as_mut_ptr()
            .offset(1 as core::ffi::c_int as isize) as *mut FLOAT32;
        k = 1 as core::ffi::c_int as WORD32;
        while k < (*str_eq_subband_gain_spline).num_eq_nodes {
            bits = impd_read_bits_buf(it_bit_buff, 5 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            *peq_gain_delta = *eq_gain_delta_tbl.as_ptr().offset(bits as isize);
            peq_gain_delta = peq_gain_delta.offset(1);
            k += 1;
        }
        str_eq_subband_gain_spline = str_eq_subband_gain_spline.offset(1);
        j += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_eq_subband_gain_vector(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    eq_subband_gain_count: WORD32,
    mut str_eq_subband_gain_vector: *mut ia_eq_subband_gain_vector,
    mut eq_subband_gains_count: WORD32,
) -> WORD32 {
    let mut m: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut temp: WORD32 = 0;
    k = 0 as core::ffi::c_int as WORD32;
    while k < eq_subband_gains_count {
        m = 0 as core::ffi::c_int as WORD32;
        while m < eq_subband_gain_count {
            let mut sign: WORD32 = 0;
            let mut bs_eq_subband_gain: WORD32 = 0;
            temp = impd_read_bits_buf(it_bit_buff, 9 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            sign = (temp as core::ffi::c_int >> 8 as core::ffi::c_int
                & 1 as core::ffi::c_int) as WORD32;
            bs_eq_subband_gain = (temp as core::ffi::c_int & 0x7f as core::ffi::c_int)
                as WORD32;
            sign = sign << 1 as core::ffi::c_int;
            (*str_eq_subband_gain_vector).eq_subband_gain[m as usize] = (((1 as WORD32
                - sign) * bs_eq_subband_gain) as core::ffi::c_float * 0.125f32)
                as FLOAT32;
            m += 1;
        }
        str_eq_subband_gain_vector = str_eq_subband_gain_vector.offset(1);
        k += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_eq_coefficients(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut str_eq_coeff: *mut ia_eq_coeff_struct,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut eq_gain_cnt: WORD32 = 0;
    let mut mu: WORD32 = 0;
    let mut nu: WORD32 = 0;
    let mut temp: WORD32 = 0;
    static mut subband_gain_len_tbl: [WORD32; 7] = [
        0 as core::ffi::c_int,
        32 as core::ffi::c_int,
        39 as core::ffi::c_int,
        64 as core::ffi::c_int,
        71 as core::ffi::c_int,
        128 as core::ffi::c_int,
        135 as core::ffi::c_int,
    ];
    (*str_eq_coeff).eq_delay_max_present = impd_read_bits_buf(it_bit_buff, 1 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*str_eq_coeff).eq_delay_max_present != 0 {
        mu = impd_read_bits_buf(it_bit_buff, 5 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        nu = impd_read_bits_buf(it_bit_buff, 3 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*str_eq_coeff).eq_delay_max = (16 as core::ffi::c_int * mu as core::ffi::c_int
            * ((1 as core::ffi::c_int) << nu)) as WORD32;
    }
    (*str_eq_coeff).unique_filter_block_count = impd_read_bits_buf(
        it_bit_buff,
        6 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*str_eq_coeff).unique_filter_block_count > FILTER_BLOCK_COUNT_MAX {
        return 2 as WORD32;
    }
    err = impd_parse_filt_block(
        it_bit_buff,
        &mut *((*str_eq_coeff).str_filter_block)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize),
        (*str_eq_coeff).unique_filter_block_count,
    );
    if err != 0 {
        return err;
    }
    (*str_eq_coeff).unique_td_filter_element_count = impd_read_bits_buf(
        it_bit_buff,
        6 as WORD,
    );
    if (*str_eq_coeff).unique_td_filter_element_count > FILTER_ELEMENT_COUNT_MAX {
        return 2 as WORD32;
    }
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    err = impd_parse_unique_td_filt_ele(
        it_bit_buff,
        &mut *((*str_eq_coeff).unique_td_filt_ele)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize),
        (*str_eq_coeff).unique_td_filter_element_count,
    );
    if err != 0 {
        return err;
    }
    (*str_eq_coeff).unique_eq_subband_gains_count = impd_read_bits_buf(
        it_bit_buff,
        6 as WORD,
    );
    if (*str_eq_coeff).unique_eq_subband_gains_count > UNIQUE_SUBBAND_GAIN_COUNT_MAX {
        return 2 as WORD32;
    }
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*str_eq_coeff).unique_eq_subband_gains_count > 0 as core::ffi::c_int {
        temp = impd_read_bits_buf(it_bit_buff, 5 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*str_eq_coeff).eq_subband_gain_representation = (temp as core::ffi::c_int
            >> 4 as core::ffi::c_int & 0x1 as core::ffi::c_int) as WORD32;
        (*str_eq_coeff).eq_subband_gain_format = (temp as core::ffi::c_int
            & 0xf as core::ffi::c_int) as WORD32;
        if (*str_eq_coeff).eq_subband_gain_format > 0 as core::ffi::c_int
            && (*str_eq_coeff).eq_subband_gain_format < GAINFORMAT_UNIFORM
        {
            (*str_eq_coeff).eq_subband_gain_count = subband_gain_len_tbl[(*str_eq_coeff)
                .eq_subband_gain_format as usize];
        } else {
            eq_gain_cnt = impd_read_bits_buf(it_bit_buff, 8 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            (*str_eq_coeff).eq_subband_gain_count = (eq_gain_cnt as core::ffi::c_int
                + 1 as core::ffi::c_int) as WORD32;
            if (*str_eq_coeff).eq_subband_gain_count > EQ_SUBBAND_GAIN_COUNT_MAX {
                return UNEXPECTED_ERROR;
            }
        }
        if (*str_eq_coeff).eq_subband_gain_representation == 1 as core::ffi::c_int {
            err = impd_parse_eq_subband_gain_spline(
                it_bit_buff,
                &mut *((*str_eq_coeff).str_eq_subband_gain_spline)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize),
                (*str_eq_coeff).unique_eq_subband_gains_count,
            );
            if err != 0 {
                return err;
            }
        } else {
            err = impd_parse_eq_subband_gain_vector(
                it_bit_buff,
                (*str_eq_coeff).eq_subband_gain_count,
                &mut *((*str_eq_coeff).str_eq_subband_gain_vector)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize),
                (*str_eq_coeff).unique_eq_subband_gains_count,
            );
            if err != 0 {
                return err;
            }
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parser_td_filter_cascade(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut str_eq_instructions: *mut ia_eq_instructions_struct,
    mut str_td_filter_cascade: *mut ia_td_filter_cascade_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut ii: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut eq_cascade_gain: WORD32 = 0;
    let mut str_filter_block_refs: *mut ia_filter_block_refs_struct = &mut *((*str_td_filter_cascade)
        .str_filter_block_refs)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut ia_filter_block_refs_struct;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*str_eq_instructions).eq_ch_group_count {
        (*str_td_filter_cascade).eq_cascade_gain_present[i as usize] = impd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*str_td_filter_cascade).eq_cascade_gain_present[i as usize] != 0 {
            eq_cascade_gain = impd_read_bits_buf(it_bit_buff, 10 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            (*str_td_filter_cascade).eq_cascade_gain[i as usize] = (0.125f32
                * eq_cascade_gain as core::ffi::c_float - 96.0f32) as FLOAT32;
        } else {
            (*str_td_filter_cascade).eq_cascade_gain[i as usize] = 0.0f32 as FLOAT32;
        }
        (*str_filter_block_refs).filter_block_count = impd_read_bits_buf(
            it_bit_buff,
            4 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*str_filter_block_refs).filter_block_count > EQ_FILTER_BLOCK_COUNT_MAX {
            return 2 as WORD32;
        }
        ii = 0 as core::ffi::c_int as WORD32;
        while ii < (*str_filter_block_refs).filter_block_count {
            (*str_filter_block_refs).filter_block_index[ii as usize] = impd_read_bits_buf(
                it_bit_buff,
                7 as WORD,
            );
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if (*str_filter_block_refs).filter_block_index[ii as usize]
                >= FILTER_BLOCK_COUNT_MAX
            {
                return 2 as WORD32;
            }
            ii += 1;
        }
        str_filter_block_refs = str_filter_block_refs.offset(1);
        i += 1;
    }
    (*str_td_filter_cascade).eq_phase_alignment_present = impd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*str_td_filter_cascade).eq_phase_alignment_present != 0 {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*str_eq_instructions).eq_ch_group_count {
            k = (i as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
            while k < (*str_eq_instructions).eq_ch_group_count {
                (*str_td_filter_cascade).eq_phase_alignment[i as usize][k as usize] = impd_read_bits_buf(
                    it_bit_buff,
                    1 as WORD,
                );
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                k += 1;
            }
            i += 1;
        }
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*str_eq_instructions).eq_ch_group_count {
            k = (i as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
            while k < (*str_eq_instructions).eq_ch_group_count {
                (*str_td_filter_cascade).eq_phase_alignment[i as usize][k as usize] = 1
                    as core::ffi::c_int as WORD32;
                k += 1;
            }
            i += 1;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_eq_instructions(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut drc_config: *mut ia_drc_config,
    mut str_eq_instructions: *mut ia_eq_instructions_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut channel_count: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut dmix_id_present: WORD32 = 0;
    let mut additional_dmix_id_present: WORD32 = 0;
    let mut additional_dmix_id_cnt: WORD32 = 0 as WORD32;
    let mut additional_drc_set_id_present: WORD32 = 0;
    let mut additional_drc_set_id_cnt: WORD32 = 0;
    temp = impd_read_bits_buf(it_bit_buff, 11 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    (*str_eq_instructions).eq_set_id = (temp as core::ffi::c_int >> 5 as core::ffi::c_int
        & 0x3f as core::ffi::c_int) as WORD32;
    if (*str_eq_instructions).eq_set_id >= EQ_INSTRUCTIONS_COUNT_MAX {
        return UNEXPECTED_ERROR;
    }
    (*str_eq_instructions).eq_set_complexity_level = (temp as core::ffi::c_int
        >> 1 as core::ffi::c_int & 0xf as core::ffi::c_int) as WORD32;
    dmix_id_present = (temp as core::ffi::c_int & 0x1 as core::ffi::c_int) as WORD32;
    if dmix_id_present != 0 {
        temp = impd_read_bits_buf(it_bit_buff, 9 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*str_eq_instructions).downmix_id[0 as core::ffi::c_int as usize] = (temp
            as core::ffi::c_int >> 2 as core::ffi::c_int & 0x7f as core::ffi::c_int)
            as WORD32;
        (*str_eq_instructions).eq_apply_to_downmix = (temp as core::ffi::c_int
            >> 1 as core::ffi::c_int & 0x1 as core::ffi::c_int) as WORD32;
        additional_dmix_id_present = (temp as core::ffi::c_int & 0x1 as core::ffi::c_int)
            as WORD32;
        if additional_dmix_id_present != 0 {
            additional_dmix_id_cnt = impd_read_bits_buf(it_bit_buff, 7 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if additional_dmix_id_cnt >= DOWNMIX_ID_COUNT_MAX {
                return UNEXPECTED_ERROR;
            }
            i = 1 as core::ffi::c_int as WORD32;
            while i < additional_dmix_id_cnt as core::ffi::c_int + 1 as core::ffi::c_int
            {
                (*str_eq_instructions).downmix_id[i as usize] = impd_read_bits_buf(
                    it_bit_buff,
                    7 as WORD,
                );
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                i += 1;
            }
        }
    } else {
        (*str_eq_instructions).downmix_id[0 as core::ffi::c_int as usize] = 0
            as core::ffi::c_int as WORD32;
    }
    (*str_eq_instructions).dwnmix_id_count = 1 as WORD32 + additional_dmix_id_cnt;
    temp = impd_read_bits_buf(it_bit_buff, 7 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    (*str_eq_instructions).drc_set_id[0 as core::ffi::c_int as usize] = (temp
        as core::ffi::c_int >> 1 as core::ffi::c_int & 0x3f as core::ffi::c_int)
        as WORD32;
    additional_drc_set_id_present = (temp as core::ffi::c_int & 0x1 as core::ffi::c_int)
        as WORD32;
    if additional_drc_set_id_present != 0 {
        additional_drc_set_id_cnt = impd_read_bits_buf(it_bit_buff, 6 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if additional_drc_set_id_cnt >= DRC_SET_ID_COUNT_MAX {
            return UNEXPECTED_ERROR;
        }
        i = 1 as core::ffi::c_int as WORD32;
        while i < additional_drc_set_id_cnt as core::ffi::c_int + 1 as core::ffi::c_int {
            (*str_eq_instructions).drc_set_id[i as usize] = impd_read_bits_buf(
                it_bit_buff,
                6 as WORD,
            );
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            i += 1;
        }
    } else {
        additional_drc_set_id_cnt = 0 as core::ffi::c_int as WORD32;
    }
    (*str_eq_instructions).drc_set_id_count = 1 as WORD32 + additional_drc_set_id_cnt;
    temp = impd_read_bits_buf(it_bit_buff, 17 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    (*str_eq_instructions).eq_set_purpose = (temp as core::ffi::c_int
        >> 1 as core::ffi::c_int & 0xffff as core::ffi::c_int) as WORD32;
    (*str_eq_instructions).depends_on_eq_set_present = (temp as core::ffi::c_int
        & 0x1 as core::ffi::c_int) as WORD32;
    if (*str_eq_instructions).depends_on_eq_set_present != 0 {
        (*str_eq_instructions).depends_on_eq_set = impd_read_bits_buf(
            it_bit_buff,
            6 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
    } else {
        (*str_eq_instructions).no_independent_eq_use = impd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
    }
    channel_count = (*drc_config).channel_layout.base_channel_count;
    (*str_eq_instructions).eq_channel_count = channel_count;
    if dmix_id_present == 1 as core::ffi::c_int
        && (*str_eq_instructions).eq_apply_to_downmix == 1 as core::ffi::c_int
        && (*str_eq_instructions).downmix_id[0 as core::ffi::c_int as usize]
            != 0 as core::ffi::c_int
        && (*str_eq_instructions).downmix_id[0 as core::ffi::c_int as usize]
            != ID_FOR_ANY_DOWNMIX
        && (*str_eq_instructions).dwnmix_id_count == 1 as core::ffi::c_int
    {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*drc_config).dwnmix_instructions_count {
            if (*str_eq_instructions).downmix_id[0 as core::ffi::c_int as usize]
                == (*drc_config).dwnmix_instructions[i as usize].downmix_id
            {
                break;
            }
            i += 1;
        }
        if i == (*drc_config).dwnmix_instructions_count {
            return UNEXPECTED_ERROR;
        }
        channel_count = (*drc_config)
            .dwnmix_instructions[i as usize]
            .target_channel_count;
        (*str_eq_instructions).eq_channel_count = channel_count;
    } else if (*str_eq_instructions).downmix_id[0 as core::ffi::c_int as usize]
        == ID_FOR_ANY_DOWNMIX
        || (*str_eq_instructions).dwnmix_id_count > 1 as core::ffi::c_int
    {
        channel_count = 1 as core::ffi::c_int as WORD32;
    }
    (*str_eq_instructions).eq_ch_group_count = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < channel_count {
        let mut tmp: WORD32 = impd_read_bits_buf(it_bit_buff, 7 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if tmp >= EQ_CHANNEL_GROUP_COUNT_MAX {
            return UNEXPECTED_ERROR;
        }
        (*str_eq_instructions).eq_ch_group_of_channel[i as usize] = tmp;
        i += 1;
    }
    let mut total: WORD32 = 0;
    let mut groups_used: [WORD32; 4] = [0 as core::ffi::c_int; 4];
    i = 0 as core::ffi::c_int as WORD32;
    while i < channel_count {
        groups_used[(*str_eq_instructions).eq_ch_group_of_channel[i as usize]
            as usize] = 1 as core::ffi::c_int as WORD32;
        i += 1;
    }
    total = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < EQ_CHANNEL_GROUP_COUNT_MAX {
        if groups_used[i as usize] != 0 {
            total += 1;
        }
        i += 1;
    }
    (*str_eq_instructions).eq_ch_group_count = total;
    if (*str_eq_instructions).eq_ch_group_count > EQ_CHANNEL_GROUP_COUNT_MAX {
        return 2 as WORD32;
    }
    (*str_eq_instructions).td_filter_cascade_present = impd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*str_eq_instructions).td_filter_cascade_present != 0 {
        let mut err: WORD32 = impd_parser_td_filter_cascade(
            it_bit_buff,
            str_eq_instructions,
            &mut (*str_eq_instructions).str_td_filter_cascade,
        );
        if err != 0 {
            return err;
        }
    }
    (*str_eq_instructions).subband_gains_present = impd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*str_eq_instructions).subband_gains_present != 0 {
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*str_eq_instructions).eq_ch_group_count {
            let mut tmp_0: WORD32 = impd_read_bits_buf(it_bit_buff, 6 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if tmp_0 >= UNIQUE_SUBBAND_GAIN_COUNT_MAX {
                return UNEXPECTED_ERROR;
            }
            (*str_eq_instructions).subband_gains_index[i as usize] = tmp_0;
            i += 1;
        }
    }
    (*str_eq_instructions).eq_transition_duration_present = impd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*str_eq_instructions).eq_transition_duration_present != 0 {
        let mut bs_eq_transition_duration: WORD32 = 0;
        bs_eq_transition_duration = impd_read_bits_buf(it_bit_buff, 5 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*str_eq_instructions).eq_transition_duration = (0.001f32
            * pow(
                2.0f64,
                (2.0f32 + bs_eq_transition_duration as core::ffi::c_float * 0.0625f32)
                    as core::ffi::c_double,
            ) as FLOAT32) as WORD32;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_loud_eq_instructions(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut loud_eq_instructions: *mut ia_loud_eq_instructions_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut bs_loud_eq_scaling: WORD32 = 0;
    let mut bs_loud_eq_offset: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut dmix_id_present: WORD32 = 0;
    let mut additional_dmix_id_present: WORD32 = 0;
    let mut additional_dmix_id_cnt: WORD32 = 0 as WORD32;
    let mut drc_set_id_present: WORD32 = 0;
    let mut additional_drc_set_id_present: WORD32 = 0;
    let mut additional_drc_set_id_cnt: WORD32 = 0 as WORD32;
    let mut eq_set_id_present: WORD32 = 0;
    let mut additional_eq_set_id_present: WORD32 = 0;
    let mut additional_eq_set_id_cnt: WORD32 = 0 as WORD32;
    temp = impd_read_bits_buf(it_bit_buff, 9 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    (*loud_eq_instructions).loud_eq_set_id = (temp as core::ffi::c_int
        >> 5 as core::ffi::c_int & 0xf as core::ffi::c_int) as WORD32;
    (*loud_eq_instructions).drc_location = (temp as core::ffi::c_int
        >> 1 as core::ffi::c_int & 0xf as core::ffi::c_int) as WORD32;
    dmix_id_present = (temp as core::ffi::c_int & 0x1 as core::ffi::c_int) as WORD32;
    if dmix_id_present != 0 {
        temp = impd_read_bits_buf(it_bit_buff, 8 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*loud_eq_instructions).downmix_id[0 as core::ffi::c_int as usize] = (temp
            as core::ffi::c_int >> 1 as core::ffi::c_int & 0x7f as core::ffi::c_int)
            as WORD32;
        additional_dmix_id_present = (temp as core::ffi::c_int & 0x1 as core::ffi::c_int)
            as WORD32;
        if additional_dmix_id_present != 0 {
            additional_dmix_id_cnt = impd_read_bits_buf(it_bit_buff, 7 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if additional_dmix_id_cnt >= DOWNMIX_ID_COUNT_MAX {
                return UNEXPECTED_ERROR;
            }
            i = 1 as core::ffi::c_int as WORD32;
            while i < additional_dmix_id_cnt as core::ffi::c_int + 1 as core::ffi::c_int
            {
                (*loud_eq_instructions).downmix_id[i as usize] = impd_read_bits_buf(
                    it_bit_buff,
                    7 as WORD,
                );
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                i += 1;
            }
        }
    } else {
        (*loud_eq_instructions).downmix_id[0 as core::ffi::c_int as usize] = 0
            as core::ffi::c_int as WORD32;
    }
    (*loud_eq_instructions).dwnmix_id_count = 1 as WORD32 + additional_dmix_id_cnt;
    drc_set_id_present = impd_read_bits_buf(it_bit_buff, 1 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if drc_set_id_present != 0 {
        temp = impd_read_bits_buf(it_bit_buff, 7 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*loud_eq_instructions).drc_set_id[0 as core::ffi::c_int as usize] = (temp
            as core::ffi::c_int >> 1 as core::ffi::c_int & 0x3f as core::ffi::c_int)
            as WORD32;
        additional_drc_set_id_present = (temp as core::ffi::c_int
            & 0x1 as core::ffi::c_int) as WORD32;
        if additional_drc_set_id_present != 0 {
            additional_drc_set_id_cnt = impd_read_bits_buf(it_bit_buff, 6 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if additional_drc_set_id_cnt >= DRC_SET_ID_COUNT_MAX {
                return UNEXPECTED_ERROR;
            }
            i = 1 as core::ffi::c_int as WORD32;
            while i
                < additional_drc_set_id_cnt as core::ffi::c_int + 1 as core::ffi::c_int
            {
                (*loud_eq_instructions).drc_set_id[i as usize] = impd_read_bits_buf(
                    it_bit_buff,
                    6 as WORD,
                );
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                i += 1;
            }
        }
    } else {
        (*loud_eq_instructions).drc_set_id[0 as core::ffi::c_int as usize] = 0
            as core::ffi::c_int as WORD32;
    }
    (*loud_eq_instructions).drc_set_id_count = 1 as WORD32 + additional_drc_set_id_cnt;
    eq_set_id_present = impd_read_bits_buf(it_bit_buff, 1 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if eq_set_id_present != 0 {
        temp = impd_read_bits_buf(it_bit_buff, 7 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*loud_eq_instructions).eq_set_id[0 as core::ffi::c_int as usize] = (temp
            as core::ffi::c_int >> 1 as core::ffi::c_int & 0x3f as core::ffi::c_int)
            as WORD32;
        additional_eq_set_id_present = (temp as core::ffi::c_int
            & 0x1 as core::ffi::c_int) as WORD32;
        if additional_eq_set_id_present != 0 {
            additional_eq_set_id_cnt = impd_read_bits_buf(it_bit_buff, 6 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if additional_eq_set_id_cnt >= EQ_SET_ID_COUNT_MAX {
                return UNEXPECTED_ERROR;
            }
            i = 0 as core::ffi::c_int as WORD32;
            while i < additional_eq_set_id_cnt {
                (*loud_eq_instructions)
                    .eq_set_id[(i as core::ffi::c_int + 1 as core::ffi::c_int)
                    as usize] = impd_read_bits_buf(it_bit_buff, 6 as WORD);
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                i += 1;
            }
        }
    } else {
        (*loud_eq_instructions).eq_set_id[0 as core::ffi::c_int as usize] = 0
            as core::ffi::c_int as WORD32;
    }
    (*loud_eq_instructions).eq_set_id_count = 1 as WORD32 + additional_eq_set_id_cnt;
    temp = impd_read_bits_buf(it_bit_buff, 8 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    (*loud_eq_instructions).loudness_after_drc = (temp as core::ffi::c_int
        >> 7 as core::ffi::c_int & 0x1 as core::ffi::c_int) as WORD32;
    (*loud_eq_instructions).loudness_after_eq = (temp as core::ffi::c_int
        >> 6 as core::ffi::c_int & 0x1 as core::ffi::c_int) as WORD32;
    (*loud_eq_instructions).loud_eq_gain_sequence_count = (temp as core::ffi::c_int
        & 0x3f as core::ffi::c_int) as WORD32;
    if (*loud_eq_instructions).loud_eq_gain_sequence_count
        > LOUD_EQ_GAIN_SEQUENCE_COUNT_MAX
    {
        return UNEXPECTED_ERROR;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*loud_eq_instructions).loud_eq_gain_sequence_count {
        temp = impd_read_bits_buf(it_bit_buff, 7 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*loud_eq_instructions).gain_seq_idx[i as usize] = (temp as core::ffi::c_int
            >> 1 as core::ffi::c_int & 0x3f as core::ffi::c_int) as WORD32;
        (*loud_eq_instructions).drc_characteristic_format_is_cicp[i as usize] = (temp
            as core::ffi::c_int & 0x1 as core::ffi::c_int) as WORD32;
        if (*loud_eq_instructions).drc_characteristic_format_is_cicp[i as usize] != 0 {
            (*loud_eq_instructions).drc_characteristic[i as usize] = impd_read_bits_buf(
                it_bit_buff,
                7 as WORD,
            );
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
        } else {
            temp = impd_read_bits_buf(it_bit_buff, 8 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            (*loud_eq_instructions).drc_characteristic_left_index[i as usize] = (temp
                as core::ffi::c_int >> 4 as core::ffi::c_int & 0xf as core::ffi::c_int)
                as WORD32;
            (*loud_eq_instructions).drc_characteristic_right_index[i as usize] = (temp
                as core::ffi::c_int & 0xf as core::ffi::c_int) as WORD32;
        }
        temp = impd_read_bits_buf(it_bit_buff, 9 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*loud_eq_instructions).frequency_range_index[i as usize] = (temp
            as core::ffi::c_int >> 3 as core::ffi::c_int & 0x3f as core::ffi::c_int)
            as WORD32;
        bs_loud_eq_scaling = (temp as core::ffi::c_int & 0x7 as core::ffi::c_int)
            as WORD32;
        (*loud_eq_instructions).loud_eq_scaling[i as usize] = pow(
            2.0f64,
            (-0.5f32 * bs_loud_eq_scaling as core::ffi::c_float) as core::ffi::c_double,
        ) as FLOAT32;
        bs_loud_eq_offset = impd_read_bits_buf(it_bit_buff, 5 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*loud_eq_instructions).loud_eq_offset[i as usize] = (1.5f32
            * bs_loud_eq_offset as core::ffi::c_float - 16.0f32) as FLOAT32;
        i += 1;
    }
    return 0 as WORD32;
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const NUM_SLOPE_TBL_ENTRIES: core::ffi::c_int = 15 as core::ffi::c_int;
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_INIT_NONFATAL_MAX_INSTRUCTIONS_ERROR: core::ffi::c_int = 0x1005
    as core::ffi::c_int;
