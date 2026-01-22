extern "C" {
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn log10(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn pow(__x: core::ffi::c_double, __y: core::ffi::c_double) -> core::ffi::c_double;
    fn floor(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn impd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
    fn impd_init_tbls(
        num_gain_max_values: WORD32,
        str_tables: *mut ia_tables_struct,
    ) -> VOID;
    fn impd_parse_drc_ext_v1(
        it_bit_buff: *mut ia_bit_buf_struct,
        ia_drc_params_struct: *mut ia_drc_params_bs_dec_struct,
        drc_config: *mut ia_drc_config,
        str_drc_config_ext: *mut ia_drc_config_ext,
    ) -> WORD32;
    fn impd_leveling_instructions(
        it_bit_buff: *mut ia_bit_buf_struct,
        pstr_drc_config: *mut ia_drc_config,
    ) -> WORD32;
    static channel_weight: [FLOAT32; 0];
    static dwnmix_coeff_v1: [FLOAT32; 0];
    static dwnmix_coeff: [FLOAT32; 0];
    static dwnmix_coeff_lfe: [FLOAT32; 0];
}
pub type size_t = usize;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD32 = core::ffi::c_int;
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
pub struct ia_drc_params_bs_dec_struct {
    pub delta_tmin_default: WORD32,
    pub drc_frame_size: WORD32,
    pub num_gain_values_max_default: WORD32,
    pub delay_mode: WORD32,
    pub lfe_channel_map_count: WORD32,
    pub lfe_channel_map: [WORD32; 8],
}
pub const N_DELTA_TIME_CODE_TABLE_ENTRIES_MAX: core::ffi::c_int = 512 as core::ffi::c_int
    + 14 as core::ffi::c_int;
pub const DOWNMIX_COEFF_COUNT_MAX: core::ffi::c_int = 32 as core::ffi::c_int
    * 32 as core::ffi::c_int;
pub const MAX_CHANNEL_COUNT: core::ffi::c_int = 8 as core::ffi::c_int;
pub const BAND_COUNT_MAX: core::ffi::c_int = 8 as core::ffi::c_int;
pub const SEQUENCE_COUNT_MAX: core::ffi::c_int = 24 as core::ffi::c_int;
pub const DOWNMIX_INSTRUCTION_COUNT_MAX: core::ffi::c_int = 16 as core::ffi::c_int;
pub const DRC_INSTRUCTIONS_COUNT_MAX: core::ffi::c_int = DOWNMIX_INSTRUCTION_COUNT_MAX
    + 20 as core::ffi::c_int;
pub const LOUDNESS_INFO_COUNT_MAX: core::ffi::c_int = DOWNMIX_INSTRUCTION_COUNT_MAX
    + 20 as core::ffi::c_int;
pub const AUDIO_CODEC_FRAME_SIZE_MAX: core::ffi::c_int = 4096 as core::ffi::c_int;
pub const MAX_DRC_FRAME_SIZE: core::ffi::c_int = AUDIO_CODEC_FRAME_SIZE_MAX;
pub const SPLIT_CHARACTERISTIC_COUNT_MAX: core::ffi::c_int = 8 as core::ffi::c_int;
pub const GAIN_SET_COUNT_MAX: core::ffi::c_int = SEQUENCE_COUNT_MAX;
pub const SHAPE_FILTER_COUNT_MAX: core::ffi::c_int = 8 as core::ffi::c_int;
pub const UNEXPECTED_ERROR: core::ffi::c_int = 2 as core::ffi::c_int;
pub const PARAM_ERROR: core::ffi::c_int = 3 as core::ffi::c_int;
pub const EXTERNAL_ERROR: core::ffi::c_int = 4 as core::ffi::c_int;
pub const BITSTREAM_ERROR: core::ffi::c_int = 6 as core::ffi::c_int;
pub const ID_FOR_BASE_LAYOUT: core::ffi::c_int = 0 as core::ffi::c_int;
pub const ID_FOR_ANY_DOWNMIX: core::ffi::c_int = 0x7f as core::ffi::c_int;
pub const EXT_COUNT_MAX: core::ffi::c_int = 8 as core::ffi::c_int;
pub const UNIDRCLOUDEXT_TERM: core::ffi::c_int = 0 as core::ffi::c_int;
pub const UNIDRCCONFEXT_TERM: core::ffi::c_int = 0 as core::ffi::c_int;
pub const UNIDRCCONFEXT_PARAM_DRC: core::ffi::c_int = 1;
pub const PARAM_DRC_INSTRUCTIONS_COUNT_MAX: core::ffi::c_int = 8 as core::ffi::c_int;
pub const PARAM_DRC_TYPE_FF: core::ffi::c_int = 0 as core::ffi::c_int;
pub const PARAM_DRC_TYPE_LIM: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const PARAM_DRC_TYPE_LIM_THRESHOLD_DEFAULT: core::ffi::c_float = -1.0f32;
pub const PARAM_DRC_TYPE_LIM_ATTACK_DEFAULT: core::ffi::c_int = 5 as core::ffi::c_int;
pub const PARAM_DRC_TYPE_LIM_RELEASE_DEFAULT: core::ffi::c_int = 50 as core::ffi::c_int;
pub const UNIDRCCONFEXT_V1: core::ffi::c_int = 2;
pub const UNIDRCLOUDEXT_EQ: core::ffi::c_int = 1;
pub const UNIDRCCONFEXT_LEVELING: core::ffi::c_int = 4;
pub const METHOD_DEFINITION_UNKNOWN_OTHER: core::ffi::c_int = 0;
pub const METHOD_DEFINITION_PROGRAM_LOUDNESS: core::ffi::c_int = 1;
pub const METHOD_DEFINITION_ANCHOR_LOUDNESS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const METHOD_DEFINITION_MAX_OF_LOUDNESS_RANGE: core::ffi::c_int = 3;
pub const METHOD_DEFINITION_MOMENTARY_LOUDNESS_MAX: core::ffi::c_int = 4;
pub const METHOD_DEFINITION_SHORT_TERM_LOUDNESS_MAX: core::ffi::c_int = 5;
pub const METHOD_DEFINITION_LOUDNESS_RANGE: core::ffi::c_int = 6;
pub const METHOD_DEFINITION_MIXING_LEVEL: core::ffi::c_int = 7;
pub const METHOD_DEFINITION_ROOM_TYPE: core::ffi::c_int = 8;
pub const METHOD_DEFINITION_SHORT_TERM_LOUDNESS: core::ffi::c_int = 9;
pub const MEASUREMENT_SYSTEM_EXPERT_PANEL: core::ffi::c_int = 5 as core::ffi::c_int;
pub const MEASUREMENT_SYSTEM_RESERVED_E: core::ffi::c_int = 11 as core::ffi::c_int;
pub const EFFECT_BIT_DUCK_OTHER: core::ffi::c_int = 0x400 as core::ffi::c_int;
pub const EFFECT_BIT_DUCK_SELF: core::ffi::c_int = 0x800 as core::ffi::c_int;
pub const GAIN_CODING_PROFILE_CONSTANT: core::ffi::c_int = 3 as core::ffi::c_int;
pub const DRC_COMPLEXITY_LEVEL_MAX: core::ffi::c_int = 0xf as core::ffi::c_int;
pub const LEFT_SIDE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const RIGHT_SIDE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const DRC_INPUT_LOUDNESS_TARGET: core::ffi::c_float = -31.0f32;
unsafe extern "C" fn impd_parametric_drc_ffwd_init_drc_curve_params(
    mut drc_characteristic: WORD32,
    mut str_parametric_drc_type_feed_forward: *mut ia_parametric_drc_type_feed_forward_struct,
) -> VOID {
    let mut node_level: *mut WORD32 = ((*str_parametric_drc_type_feed_forward)
        .node_level)
        .as_mut_ptr();
    let mut node_gain: *mut WORD32 = ((*str_parametric_drc_type_feed_forward).node_gain)
        .as_mut_ptr();
    match drc_characteristic {
        7 => {
            (*str_parametric_drc_type_feed_forward).node_count = 5 as core::ffi::c_int
                as WORD32;
            *node_level.offset(0 as core::ffi::c_int as isize) = -(22
                as core::ffi::c_int) as WORD32;
            *node_gain.offset(0 as core::ffi::c_int as isize) = 6 as core::ffi::c_int
                as WORD32;
            *node_level.offset(1 as core::ffi::c_int as isize) = -(10
                as core::ffi::c_int) as WORD32;
            *node_gain.offset(1 as core::ffi::c_int as isize) = 0 as core::ffi::c_int
                as WORD32;
            *node_level.offset(2 as core::ffi::c_int as isize) = 10 as core::ffi::c_int
                as WORD32;
            *node_gain.offset(2 as core::ffi::c_int as isize) = 0 as core::ffi::c_int
                as WORD32;
            *node_level.offset(3 as core::ffi::c_int as isize) = 20 as core::ffi::c_int
                as WORD32;
            *node_gain.offset(3 as core::ffi::c_int as isize) = -(5 as core::ffi::c_int)
                as WORD32;
            *node_level.offset(4 as core::ffi::c_int as isize) = 40 as core::ffi::c_int
                as WORD32;
            *node_gain.offset(4 as core::ffi::c_int as isize) = -(24 as core::ffi::c_int)
                as WORD32;
        }
        8 => {
            (*str_parametric_drc_type_feed_forward).node_count = 5 as core::ffi::c_int
                as WORD32;
            *node_level.offset(0 as core::ffi::c_int as isize) = -(12
                as core::ffi::c_int) as WORD32;
            *node_gain.offset(0 as core::ffi::c_int as isize) = 6 as core::ffi::c_int
                as WORD32;
            *node_level.offset(1 as core::ffi::c_int as isize) = 0 as core::ffi::c_int
                as WORD32;
            *node_gain.offset(1 as core::ffi::c_int as isize) = 0 as core::ffi::c_int
                as WORD32;
            *node_level.offset(2 as core::ffi::c_int as isize) = 5 as core::ffi::c_int
                as WORD32;
            *node_gain.offset(2 as core::ffi::c_int as isize) = 0 as core::ffi::c_int
                as WORD32;
            *node_level.offset(3 as core::ffi::c_int as isize) = 15 as core::ffi::c_int
                as WORD32;
            *node_gain.offset(3 as core::ffi::c_int as isize) = -(5 as core::ffi::c_int)
                as WORD32;
            *node_level.offset(4 as core::ffi::c_int as isize) = 35 as core::ffi::c_int
                as WORD32;
            *node_gain.offset(4 as core::ffi::c_int as isize) = -(24 as core::ffi::c_int)
                as WORD32;
        }
        9 => {
            (*str_parametric_drc_type_feed_forward).node_count = 4 as core::ffi::c_int
                as WORD32;
            *node_level.offset(0 as core::ffi::c_int as isize) = -(34
                as core::ffi::c_int) as WORD32;
            *node_gain.offset(0 as core::ffi::c_int as isize) = 12 as core::ffi::c_int
                as WORD32;
            *node_level.offset(1 as core::ffi::c_int as isize) = -(10
                as core::ffi::c_int) as WORD32;
            *node_gain.offset(1 as core::ffi::c_int as isize) = 0 as core::ffi::c_int
                as WORD32;
            *node_level.offset(2 as core::ffi::c_int as isize) = 10 as core::ffi::c_int
                as WORD32;
            *node_gain.offset(2 as core::ffi::c_int as isize) = 0 as core::ffi::c_int
                as WORD32;
            *node_level.offset(3 as core::ffi::c_int as isize) = 40 as core::ffi::c_int
                as WORD32;
            *node_gain.offset(3 as core::ffi::c_int as isize) = -(15 as core::ffi::c_int)
                as WORD32;
        }
        10 => {
            (*str_parametric_drc_type_feed_forward).node_count = 5 as core::ffi::c_int
                as WORD32;
            *node_level.offset(0 as core::ffi::c_int as isize) = -(24
                as core::ffi::c_int) as WORD32;
            *node_gain.offset(0 as core::ffi::c_int as isize) = 12 as core::ffi::c_int
                as WORD32;
            *node_level.offset(1 as core::ffi::c_int as isize) = 0 as core::ffi::c_int
                as WORD32;
            *node_gain.offset(1 as core::ffi::c_int as isize) = 0 as core::ffi::c_int
                as WORD32;
            *node_level.offset(2 as core::ffi::c_int as isize) = 5 as core::ffi::c_int
                as WORD32;
            *node_gain.offset(2 as core::ffi::c_int as isize) = 0 as core::ffi::c_int
                as WORD32;
            *node_level.offset(3 as core::ffi::c_int as isize) = 15 as core::ffi::c_int
                as WORD32;
            *node_gain.offset(3 as core::ffi::c_int as isize) = -(5 as core::ffi::c_int)
                as WORD32;
            *node_level.offset(4 as core::ffi::c_int as isize) = 35 as core::ffi::c_int
                as WORD32;
            *node_gain.offset(4 as core::ffi::c_int as isize) = -(24 as core::ffi::c_int)
                as WORD32;
        }
        11 => {
            (*str_parametric_drc_type_feed_forward).node_count = 5 as core::ffi::c_int
                as WORD32;
            *node_level.offset(0 as core::ffi::c_int as isize) = -(19
                as core::ffi::c_int) as WORD32;
            *node_gain.offset(0 as core::ffi::c_int as isize) = 15 as core::ffi::c_int
                as WORD32;
            *node_level.offset(1 as core::ffi::c_int as isize) = 0 as core::ffi::c_int
                as WORD32;
            *node_gain.offset(1 as core::ffi::c_int as isize) = 0 as core::ffi::c_int
                as WORD32;
            *node_level.offset(2 as core::ffi::c_int as isize) = 5 as core::ffi::c_int
                as WORD32;
            *node_gain.offset(2 as core::ffi::c_int as isize) = 0 as core::ffi::c_int
                as WORD32;
            *node_level.offset(3 as core::ffi::c_int as isize) = 15 as core::ffi::c_int
                as WORD32;
            *node_gain.offset(3 as core::ffi::c_int as isize) = -(5 as core::ffi::c_int)
                as WORD32;
            *node_level.offset(4 as core::ffi::c_int as isize) = 35 as core::ffi::c_int
                as WORD32;
            *node_gain.offset(4 as core::ffi::c_int as isize) = -(24 as core::ffi::c_int)
                as WORD32;
        }
        _ => {
            (*str_parametric_drc_type_feed_forward).disable_paramteric_drc = 1
                as core::ffi::c_int as WORD32;
        }
    };
}
unsafe extern "C" fn impd_parametric_drc_ffwd_init_drc_gain_smooth_params(
    mut drc_characteristic: WORD32,
    mut str_parametric_drc_type_feed_forward: *mut ia_parametric_drc_type_feed_forward_struct,
) -> VOID {
    (*str_parametric_drc_type_feed_forward).gain_smooth_attack_time_slow = 100
        as core::ffi::c_int as WORD32;
    (*str_parametric_drc_type_feed_forward).gain_smooth_time_fast_present = 1
        as core::ffi::c_int as WORD32;
    (*str_parametric_drc_type_feed_forward).gain_smooth_attack_time_fast = 10
        as core::ffi::c_int as WORD32;
    (*str_parametric_drc_type_feed_forward).gain_smooth_threshold_present = 1
        as core::ffi::c_int as WORD32;
    (*str_parametric_drc_type_feed_forward).gain_smooth_hold_off_count_present = 1
        as core::ffi::c_int as WORD32;
    (*str_parametric_drc_type_feed_forward).gain_smooth_hold_off = 10 as core::ffi::c_int
        as WORD32;
    match drc_characteristic {
        7 | 8 | 9 => {
            (*str_parametric_drc_type_feed_forward).gain_smooth_release_time_slow = 3000
                as core::ffi::c_int as WORD32;
            (*str_parametric_drc_type_feed_forward).gain_smooth_release_time_fast = 1000
                as core::ffi::c_int as WORD32;
            (*str_parametric_drc_type_feed_forward).gain_smooth_attack_threshold = 15
                as core::ffi::c_int as WORD32;
            (*str_parametric_drc_type_feed_forward).gain_smooth_rel_threshold = 20
                as core::ffi::c_int as WORD32;
        }
        10 => {
            (*str_parametric_drc_type_feed_forward).gain_smooth_release_time_slow = 10000
                as core::ffi::c_int as WORD32;
            (*str_parametric_drc_type_feed_forward).gain_smooth_release_time_fast = 1000
                as core::ffi::c_int as WORD32;
            (*str_parametric_drc_type_feed_forward).gain_smooth_attack_threshold = 15
                as core::ffi::c_int as WORD32;
            (*str_parametric_drc_type_feed_forward).gain_smooth_rel_threshold = 20
                as core::ffi::c_int as WORD32;
        }
        11 => {
            (*str_parametric_drc_type_feed_forward).gain_smooth_release_time_slow = 1000
                as core::ffi::c_int as WORD32;
            (*str_parametric_drc_type_feed_forward).gain_smooth_release_time_fast = 200
                as core::ffi::c_int as WORD32;
            (*str_parametric_drc_type_feed_forward).gain_smooth_attack_threshold = 10
                as core::ffi::c_int as WORD32;
            (*str_parametric_drc_type_feed_forward).gain_smooth_rel_threshold = 10
                as core::ffi::c_int as WORD32;
        }
        _ => {
            (*str_parametric_drc_type_feed_forward).gain_smooth_release_time_slow = 3000
                as core::ffi::c_int as WORD32;
            (*str_parametric_drc_type_feed_forward).gain_smooth_release_time_fast = 1000
                as core::ffi::c_int as WORD32;
            (*str_parametric_drc_type_feed_forward).gain_smooth_attack_threshold = 15
                as core::ffi::c_int as WORD32;
            (*str_parametric_drc_type_feed_forward).gain_smooth_rel_threshold = 20
                as core::ffi::c_int as WORD32;
        }
    };
}
unsafe extern "C" fn impd_parse_parametric_drc_ffwd(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut parametric_drc_frame_size: WORD32,
    mut str_parametric_drc_type_feed_forward: *mut ia_parametric_drc_type_feed_forward_struct,
) -> WORD32 {
    let mut i: WORD32 = 0 as WORD32;
    let mut tmp: WORD32 = 0 as WORD32;
    (*str_parametric_drc_type_feed_forward).disable_paramteric_drc = 0
        as core::ffi::c_int as WORD32;
    tmp = impd_read_bits_buf(it_bit_buff, 3 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    (*str_parametric_drc_type_feed_forward).level_estim_k_weighting_type = (tmp
        as core::ffi::c_int >> 1 as core::ffi::c_int & 3 as core::ffi::c_int) as WORD32;
    (*str_parametric_drc_type_feed_forward).level_estim_integration_time_present = (tmp
        as core::ffi::c_int & 1 as core::ffi::c_int) as WORD32;
    if (*str_parametric_drc_type_feed_forward).level_estim_integration_time_present != 0
    {
        tmp = impd_read_bits_buf(it_bit_buff, 6 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*str_parametric_drc_type_feed_forward).level_estim_integration_time = (tmp
            + 1 as WORD32) * parametric_drc_frame_size;
    } else {
        (*str_parametric_drc_type_feed_forward).level_estim_integration_time = parametric_drc_frame_size;
    }
    (*str_parametric_drc_type_feed_forward).drc_curve_definition_type = impd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*str_parametric_drc_type_feed_forward).drc_curve_definition_type
        == 0 as core::ffi::c_int
    {
        (*str_parametric_drc_type_feed_forward).drc_characteristic = impd_read_bits_buf(
            it_bit_buff,
            7 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        impd_parametric_drc_ffwd_init_drc_curve_params(
            (*str_parametric_drc_type_feed_forward).drc_characteristic,
            str_parametric_drc_type_feed_forward,
        );
    } else {
        (*str_parametric_drc_type_feed_forward).drc_characteristic = 0
            as core::ffi::c_int as WORD32;
        tmp = impd_read_bits_buf(it_bit_buff, 15 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*str_parametric_drc_type_feed_forward).node_count = ((tmp as core::ffi::c_int
            >> 12 as core::ffi::c_int & 3 as core::ffi::c_int) + 2 as core::ffi::c_int)
            as WORD32;
        (*str_parametric_drc_type_feed_forward)
            .node_level[0 as core::ffi::c_int as usize] = (-(11 as core::ffi::c_int)
            - (tmp as core::ffi::c_int >> 6 as core::ffi::c_int
                & 0x3f as core::ffi::c_int)) as WORD32;
        (*str_parametric_drc_type_feed_forward)
            .node_gain[0 as core::ffi::c_int as usize] = ((tmp as core::ffi::c_int
            & 0x3f as core::ffi::c_int) - 39 as core::ffi::c_int) as WORD32;
        i = 1 as core::ffi::c_int as WORD32;
        while i < (*str_parametric_drc_type_feed_forward).node_count {
            tmp = impd_read_bits_buf(it_bit_buff, 11 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            (*str_parametric_drc_type_feed_forward).node_level[i as usize] = ((*str_parametric_drc_type_feed_forward)
                .node_level[(i as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
                + 1 as core::ffi::c_int
                + (tmp as core::ffi::c_int >> 6 as core::ffi::c_int
                    & 0x1f as core::ffi::c_int)) as WORD32;
            (*str_parametric_drc_type_feed_forward).node_gain[i as usize] = ((tmp
                as core::ffi::c_int & 0x3f as core::ffi::c_int) - 39 as core::ffi::c_int)
                as WORD32;
            i += 1;
        }
    }
    impd_parametric_drc_ffwd_init_drc_gain_smooth_params(
        (*str_parametric_drc_type_feed_forward).drc_characteristic,
        str_parametric_drc_type_feed_forward,
    );
    (*str_parametric_drc_type_feed_forward).drc_gain_smooth_parameters_present = impd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*str_parametric_drc_type_feed_forward).drc_gain_smooth_parameters_present != 0 {
        tmp = impd_read_bits_buf(it_bit_buff, 17 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*str_parametric_drc_type_feed_forward).gain_smooth_attack_time_slow = ((tmp
            as core::ffi::c_int >> 9 as core::ffi::c_int & 0xff as core::ffi::c_int)
            * 5 as core::ffi::c_int) as WORD32;
        (*str_parametric_drc_type_feed_forward).gain_smooth_release_time_slow = ((tmp
            as core::ffi::c_int >> 1 as core::ffi::c_int & 0xff as core::ffi::c_int)
            * 40 as core::ffi::c_int) as WORD32;
        (*str_parametric_drc_type_feed_forward).gain_smooth_time_fast_present = (tmp
            as core::ffi::c_int & 1 as core::ffi::c_int) as WORD32;
        if (*str_parametric_drc_type_feed_forward).gain_smooth_time_fast_present != 0 {
            tmp = impd_read_bits_buf(it_bit_buff, 17 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            (*str_parametric_drc_type_feed_forward).gain_smooth_attack_time_fast = ((tmp
                as core::ffi::c_int >> 9 as core::ffi::c_int & 0xff as core::ffi::c_int)
                * 5 as core::ffi::c_int) as WORD32;
            (*str_parametric_drc_type_feed_forward).gain_smooth_release_time_fast = ((tmp
                as core::ffi::c_int >> 1 as core::ffi::c_int & 0xff as core::ffi::c_int)
                * 20 as core::ffi::c_int) as WORD32;
            (*str_parametric_drc_type_feed_forward).gain_smooth_threshold_present = (tmp
                as core::ffi::c_int & 1 as core::ffi::c_int) as WORD32;
            if (*str_parametric_drc_type_feed_forward).gain_smooth_threshold_present != 0
            {
                (*str_parametric_drc_type_feed_forward).gain_smooth_attack_threshold = impd_read_bits_buf(
                    it_bit_buff,
                    5 as WORD,
                );
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                if (*str_parametric_drc_type_feed_forward).gain_smooth_attack_threshold
                    == 31 as core::ffi::c_int
                {
                    (*str_parametric_drc_type_feed_forward)
                        .gain_smooth_attack_threshold = 1000 as core::ffi::c_int
                        as WORD32;
                }
                (*str_parametric_drc_type_feed_forward).gain_smooth_rel_threshold = impd_read_bits_buf(
                    it_bit_buff,
                    5 as WORD,
                );
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                if (*str_parametric_drc_type_feed_forward).gain_smooth_rel_threshold
                    == 31 as core::ffi::c_int
                {
                    (*str_parametric_drc_type_feed_forward).gain_smooth_rel_threshold = 1000
                        as core::ffi::c_int as WORD32;
                }
            }
        } else {
            (*str_parametric_drc_type_feed_forward).gain_smooth_attack_time_fast = (*str_parametric_drc_type_feed_forward)
                .gain_smooth_attack_time_slow;
            (*str_parametric_drc_type_feed_forward).gain_smooth_release_time_fast = (*str_parametric_drc_type_feed_forward)
                .gain_smooth_release_time_slow;
        }
        (*str_parametric_drc_type_feed_forward).gain_smooth_hold_off_count_present = impd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*str_parametric_drc_type_feed_forward).gain_smooth_hold_off_count_present
            != 0
        {
            (*str_parametric_drc_type_feed_forward).gain_smooth_hold_off = impd_read_bits_buf(
                it_bit_buff,
                7 as WORD,
            );
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
        }
    }
    return 0 as WORD32;
}
unsafe extern "C" fn impd_parse_parametric_drc_lim(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut parametric_drc_lim: *mut ia_parametric_drc_lim_struct,
) -> WORD32 {
    let mut tmp: WORD32 = 0 as WORD32;
    (*parametric_drc_lim).disable_paramteric_drc = 0 as core::ffi::c_int as WORD32;
    (*parametric_drc_lim).parametric_lim_threshold_present = impd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*parametric_drc_lim).parametric_lim_threshold_present != 0 {
        tmp = impd_read_bits_buf(it_bit_buff, 8 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*parametric_drc_lim).parametric_lim_threshold = (-tmp as core::ffi::c_float
            * 0.125f32) as FLOAT32;
    } else {
        (*parametric_drc_lim).parametric_lim_threshold = PARAM_DRC_TYPE_LIM_THRESHOLD_DEFAULT
            as FLOAT32;
    }
    (*parametric_drc_lim).parametric_lim_release_present = impd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*parametric_drc_lim).parametric_lim_release_present != 0 {
        tmp = impd_read_bits_buf(it_bit_buff, 8 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*parametric_drc_lim).parametric_lim_release = (tmp as core::ffi::c_int
            * 10 as core::ffi::c_int) as WORD32;
    } else {
        (*parametric_drc_lim).parametric_lim_release = PARAM_DRC_TYPE_LIM_RELEASE_DEFAULT
            as WORD32;
    }
    (*parametric_drc_lim).parametric_lim_attack = PARAM_DRC_TYPE_LIM_ATTACK_DEFAULT
        as WORD32;
    (*parametric_drc_lim).drc_characteristic = 0 as core::ffi::c_int as WORD32;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parametric_drc_parse_gain_set_params(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut drc_config: *mut ia_drc_config,
    mut str_parametric_drc_gain_set_params: *mut ia_parametric_drc_gain_set_params_struct,
) -> WORD32 {
    let mut i: WORD32 = 0 as WORD32;
    let mut bsDrcInputLoudness: WORD32 = 0 as WORD32;
    let mut bs_channel_weight: WORD32 = 0 as WORD32;
    let mut temp: WORD32 = 0;
    temp = impd_read_bits_buf(it_bit_buff, 7 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    (*str_parametric_drc_gain_set_params).parametric_drc_id = (temp as core::ffi::c_int
        >> 3 as core::ffi::c_int & 0xf as core::ffi::c_int) as WORD32;
    (*str_parametric_drc_gain_set_params).side_chain_config_type = (temp
        as core::ffi::c_int & 7 as core::ffi::c_int) as WORD32;
    if (*str_parametric_drc_gain_set_params).side_chain_config_type
        == 1 as core::ffi::c_int
    {
        temp = impd_read_bits_buf(it_bit_buff, 8 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*str_parametric_drc_gain_set_params).downmix_id = (temp as core::ffi::c_int
            >> 1 as core::ffi::c_int & 0x7f as core::ffi::c_int) as WORD32;
        (*str_parametric_drc_gain_set_params).level_estim_channel_weight_format = (temp
            as core::ffi::c_int & 1 as core::ffi::c_int) as WORD32;
        if (*str_parametric_drc_gain_set_params).downmix_id == ID_FOR_BASE_LAYOUT {
            (*str_parametric_drc_gain_set_params).ch_count_from_dwnmix_id = (*drc_config)
                .channel_layout
                .base_channel_count;
        } else if (*str_parametric_drc_gain_set_params).downmix_id == ID_FOR_ANY_DOWNMIX
        {
            (*str_parametric_drc_gain_set_params).ch_count_from_dwnmix_id = 1
                as core::ffi::c_int as WORD32;
        } else {
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*drc_config).dwnmix_instructions_count {
                if (*str_parametric_drc_gain_set_params).downmix_id
                    == (*drc_config).dwnmix_instructions[i as usize].downmix_id
                {
                    break;
                }
                i += 1;
            }
            if i == (*drc_config).dwnmix_instructions_count {
                return 2 as WORD32;
            }
            (*str_parametric_drc_gain_set_params).ch_count_from_dwnmix_id = (*drc_config)
                .dwnmix_instructions[i as usize]
                .target_channel_count;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*str_parametric_drc_gain_set_params).ch_count_from_dwnmix_id {
            if (*str_parametric_drc_gain_set_params).level_estim_channel_weight_format
                == 0 as core::ffi::c_int
            {
                (*str_parametric_drc_gain_set_params)
                    .level_estim_ch_weight[i as usize] = impd_read_bits_buf(
                    it_bit_buff,
                    1 as WORD,
                ) as FLOAT32;
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
            } else {
                bs_channel_weight = impd_read_bits_buf(it_bit_buff, 4 as WORD);
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                (*str_parametric_drc_gain_set_params)
                    .level_estim_ch_weight[i as usize] = pow(
                    10.0f64,
                    (0.05f32
                        * *channel_weight.as_ptr().offset(bs_channel_weight as isize))
                        as core::ffi::c_double,
                ) as FLOAT32;
            }
            i += 1;
        }
    } else {
        (*str_parametric_drc_gain_set_params).downmix_id = 0 as core::ffi::c_int
            as WORD32;
        (*str_parametric_drc_gain_set_params).ch_count_from_dwnmix_id = 0
            as core::ffi::c_int as WORD32;
    }
    (*str_parametric_drc_gain_set_params).drc_input_loudness_present = impd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*str_parametric_drc_gain_set_params).drc_input_loudness_present != 0 {
        bsDrcInputLoudness = impd_read_bits_buf(it_bit_buff, 8 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*str_parametric_drc_gain_set_params).drc_input_loudness = (-57.75f32
            + bsDrcInputLoudness as core::ffi::c_float * 0.25f32) as FLOAT32;
    }
    return 0 as WORD32;
}
unsafe extern "C" fn impd_parametric_drc_gen_virtual_gain_sets(
    mut drc_config: *mut ia_drc_config,
) -> WORD32 {
    let mut i: WORD32 = 0 as WORD32;
    let mut j: WORD32 = 0 as WORD32;
    let mut c1: WORD32 = -(1 as WORD32);
    let mut c0: WORD32 = -(1 as WORD32);
    let mut parametric_drc_id: WORD32 = 0 as WORD32;
    let mut drc_characteristic: WORD32 = 0 as WORD32;
    let mut str_p_loc_drc_coefficients_uni_drc: *mut ia_uni_drc_coeffs_struct = 0
        as *mut ia_uni_drc_coeffs_struct;
    let mut str_parametric_drc_instructions: *mut ia_parametric_drc_instructions_struct = 0
        as *mut ia_parametric_drc_instructions_struct;
    let mut str_drc_coeff_param_drc: *mut ia_drc_coeff_parametric_drc_struct = &mut (*drc_config)
        .str_drc_config_ext
        .str_drc_coeff_param_drc;
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*drc_config).drc_coefficients_drc_count {
        if (*drc_config).str_p_loc_drc_coefficients_uni_drc[i as usize].drc_location
            == (*str_drc_coeff_param_drc).drc_location
        {
            if (*drc_config).str_p_loc_drc_coefficients_uni_drc[i as usize].version
                == 0 as core::ffi::c_int
            {
                c0 = i;
            } else {
                c1 = i;
            }
        }
        i += 1;
    }
    if c1 >= 0 as core::ffi::c_int {
        str_p_loc_drc_coefficients_uni_drc = &mut *((*drc_config)
            .str_p_loc_drc_coefficients_uni_drc)
            .as_mut_ptr()
            .offset(c1 as isize) as *mut ia_uni_drc_coeffs_struct;
    } else if c0 >= 0 as core::ffi::c_int {
        str_p_loc_drc_coefficients_uni_drc = &mut *((*drc_config)
            .str_p_loc_drc_coefficients_uni_drc)
            .as_mut_ptr()
            .offset(c0 as isize) as *mut ia_uni_drc_coeffs_struct;
    } else {
        str_p_loc_drc_coefficients_uni_drc = &mut *((*drc_config)
            .str_p_loc_drc_coefficients_uni_drc)
            .as_mut_ptr()
            .offset((*drc_config).drc_coefficients_drc_count as isize)
            as *mut ia_uni_drc_coeffs_struct;
        (*str_p_loc_drc_coefficients_uni_drc).version = 1 as core::ffi::c_int as WORD32;
        (*str_p_loc_drc_coefficients_uni_drc).drc_location = (*str_drc_coeff_param_drc)
            .drc_location;
        (*str_p_loc_drc_coefficients_uni_drc).drc_frame_size_present = 0
            as core::ffi::c_int as WORD32;
        (*str_p_loc_drc_coefficients_uni_drc).gain_set_count = 0 as core::ffi::c_int
            as WORD32;
        (*str_p_loc_drc_coefficients_uni_drc).gain_set_count_plus = 0 as core::ffi::c_int
            as WORD32;
        (*str_p_loc_drc_coefficients_uni_drc).drc_characteristic_left_present = 0
            as core::ffi::c_int as WORD32;
        (*str_p_loc_drc_coefficients_uni_drc).drc_characteristic_right_present = 0
            as core::ffi::c_int as WORD32;
        (*str_p_loc_drc_coefficients_uni_drc).shape_filters_present = 0
            as core::ffi::c_int as WORD32;
        (*str_p_loc_drc_coefficients_uni_drc).gain_sequence_count = 0 as core::ffi::c_int
            as WORD32;
        (*drc_config).drc_coefficients_drc_count += 1 as core::ffi::c_int;
    }
    let mut tmp: WORD32 = (*str_p_loc_drc_coefficients_uni_drc).gain_set_count
        + (*str_drc_coeff_param_drc).parametric_drc_gain_set_count;
    if tmp > GAIN_SET_COUNT_MAX {
        return UNEXPECTED_ERROR;
    }
    (*str_p_loc_drc_coefficients_uni_drc).gain_set_count_plus = tmp;
    i = (*str_p_loc_drc_coefficients_uni_drc).gain_set_count;
    while i < (*str_p_loc_drc_coefficients_uni_drc).gain_set_count_plus {
        (*str_p_loc_drc_coefficients_uni_drc).gain_set_params[i as usize].band_count = 1
            as core::ffi::c_int as WORD32;
        parametric_drc_id = (*drc_config)
            .str_drc_config_ext
            .str_drc_coeff_param_drc
            .str_parametric_drc_gain_set_params[(i
                - (*str_p_loc_drc_coefficients_uni_drc).gain_set_count) as usize]
            .parametric_drc_id;
        j = 0 as core::ffi::c_int as WORD32;
        while j < (*drc_config).str_drc_config_ext.parametric_drc_instructions_count {
            if parametric_drc_id
                == (*drc_config)
                    .str_drc_config_ext
                    .str_parametric_drc_instructions[j as usize]
                    .parametric_drc_id
            {
                break;
            }
            j += 1;
        }
        if j == (*drc_config).str_drc_config_ext.parametric_drc_instructions_count {
            return 2 as WORD32;
        }
        str_parametric_drc_instructions = &mut *((*drc_config)
            .str_drc_config_ext
            .str_parametric_drc_instructions)
            .as_mut_ptr()
            .offset(j as isize) as *mut ia_parametric_drc_instructions_struct;
        drc_characteristic = 0 as core::ffi::c_int as WORD32;
        if (*str_parametric_drc_instructions).parametric_drc_preset_id_present != 0 {
            drc_characteristic = (*str_parametric_drc_instructions).drc_characteristic;
        } else if (*str_parametric_drc_instructions).parametric_drc_type
            == PARAM_DRC_TYPE_FF
        {
            if (*str_parametric_drc_instructions)
                .str_parametric_drc_type_feed_forward
                .drc_curve_definition_type == 0 as core::ffi::c_int
            {
                drc_characteristic = (*str_parametric_drc_instructions)
                    .str_parametric_drc_type_feed_forward
                    .drc_characteristic;
            }
        }
        if drc_characteristic != 0 as core::ffi::c_int {
            (*str_p_loc_drc_coefficients_uni_drc)
                .gain_set_params[i as usize]
                .gain_params[0 as core::ffi::c_int as usize]
                .drc_characteristic_present = 1 as core::ffi::c_int as WORD32;
            (*str_p_loc_drc_coefficients_uni_drc)
                .gain_set_params[i as usize]
                .gain_params[0 as core::ffi::c_int as usize]
                .drc_characteristic_format_is_cicp = 1 as core::ffi::c_int as WORD32;
            (*str_p_loc_drc_coefficients_uni_drc)
                .gain_set_params[i as usize]
                .gain_params[0 as core::ffi::c_int as usize]
                .drc_characteristic = drc_characteristic;
        } else {
            (*str_p_loc_drc_coefficients_uni_drc)
                .gain_set_params[i as usize]
                .gain_params[0 as core::ffi::c_int as usize]
                .drc_characteristic_present = 0 as core::ffi::c_int as WORD32;
            (*str_p_loc_drc_coefficients_uni_drc)
                .gain_set_params[i as usize]
                .gain_params[0 as core::ffi::c_int as usize]
                .drc_characteristic_format_is_cicp = 0 as core::ffi::c_int as WORD32;
            (*str_p_loc_drc_coefficients_uni_drc)
                .gain_set_params[i as usize]
                .gain_params[0 as core::ffi::c_int as usize]
                .drc_characteristic = 0 as core::ffi::c_int as WORD32;
        }
        i += 1;
    }
    return 0 as WORD32;
}
unsafe extern "C" fn impd_parametic_drc_parse_coeff(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut drc_config: *mut ia_drc_config,
    mut str_drc_coeff_param_drc: *mut ia_drc_coeff_parametric_drc_struct,
) -> WORD32 {
    let mut i: WORD32 = 0 as WORD32;
    let mut err: WORD32 = 0 as WORD32;
    let mut code: WORD32 = 0 as WORD32;
    let mut mu: WORD32 = 0 as WORD32;
    let mut nu: WORD32 = 0 as WORD32;
    let mut temp: WORD32 = 0;
    temp = impd_read_bits_buf(it_bit_buff, 5 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    (*str_drc_coeff_param_drc).drc_location = (temp as core::ffi::c_int
        >> 1 as core::ffi::c_int & 0xf as core::ffi::c_int) as WORD32;
    if (*str_drc_coeff_param_drc).drc_location < 1 as core::ffi::c_int
        || (*str_drc_coeff_param_drc).drc_location > 4 as core::ffi::c_int
    {
        return UNEXPECTED_ERROR;
    }
    (*str_drc_coeff_param_drc).parametric_drc_frame_size_format = (temp
        as core::ffi::c_int & 1 as core::ffi::c_int) as WORD32;
    if (*str_drc_coeff_param_drc).parametric_drc_frame_size_format != 0 {
        code = impd_read_bits_buf(it_bit_buff, 15 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*str_drc_coeff_param_drc).parametric_drc_frame_size = (code as core::ffi::c_int
            + 1 as core::ffi::c_int) as WORD32;
    } else {
        code = impd_read_bits_buf(it_bit_buff, 4 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*str_drc_coeff_param_drc).parametric_drc_frame_size = ((1 as core::ffi::c_int)
            << code) as WORD32;
    }
    (*str_drc_coeff_param_drc).parametric_drc_delay_max_present = impd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*str_drc_coeff_param_drc).parametric_drc_delay_max_present != 0 {
        temp = impd_read_bits_buf(it_bit_buff, 8 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        mu = (temp as core::ffi::c_int >> 3 as core::ffi::c_int
            & 0x1f as core::ffi::c_int) as WORD32;
        nu = (temp as core::ffi::c_int & 3 as core::ffi::c_int) as WORD32;
        (*str_drc_coeff_param_drc).parametric_drc_delay_max = (16 as core::ffi::c_int
            * mu as core::ffi::c_int * ((1 as core::ffi::c_int) << nu)) as WORD32;
    }
    temp = impd_read_bits_buf(it_bit_buff, 7 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    (*str_drc_coeff_param_drc).reset_parametric_drc = (temp as core::ffi::c_int
        >> 6 as core::ffi::c_int & 1 as core::ffi::c_int) as WORD32;
    (*str_drc_coeff_param_drc).parametric_drc_gain_set_count = (temp as core::ffi::c_int
        & 0x3f as core::ffi::c_int) as WORD32;
    if (*str_drc_coeff_param_drc).parametric_drc_gain_set_count > SEQUENCE_COUNT_MAX {
        return 2 as WORD32;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*str_drc_coeff_param_drc).parametric_drc_gain_set_count {
        err = impd_parametric_drc_parse_gain_set_params(
            it_bit_buff,
            drc_config,
            &mut *((*str_drc_coeff_param_drc).str_parametric_drc_gain_set_params)
                .as_mut_ptr()
                .offset(i as isize),
        );
        if err != 0 {
            return err;
        }
        i += 1;
    }
    return 0 as WORD32;
}
unsafe extern "C" fn impd_parse_parametric_drc_instructions(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut parametric_drc_frame_size: WORD32,
    mut str_parametric_drc_instructions: *mut ia_parametric_drc_instructions_struct,
) -> WORD32 {
    let mut i: WORD32 = 0 as WORD32;
    let mut err: WORD32 = 0 as WORD32;
    let mut temp: WORD32 = 0;
    let mut bit_size_len: WORD32 = 0;
    let mut bit_size: WORD32 = 0;
    (*str_parametric_drc_instructions).drc_characteristic = 0 as core::ffi::c_int
        as WORD32;
    (*str_parametric_drc_instructions).disable_paramteric_drc = 0 as core::ffi::c_int
        as WORD32;
    temp = impd_read_bits_buf(it_bit_buff, 5 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    (*str_parametric_drc_instructions).parametric_drc_id = (temp as core::ffi::c_int
        >> 1 as core::ffi::c_int & 0xf as core::ffi::c_int) as WORD32;
    (*str_parametric_drc_instructions).parametric_drc_look_ahead_flag = (temp
        as core::ffi::c_int & 1 as core::ffi::c_int) as WORD32;
    if (*str_parametric_drc_instructions).parametric_drc_look_ahead_flag != 0 {
        (*str_parametric_drc_instructions).parametric_drc_look_ahead = impd_read_bits_buf(
            it_bit_buff,
            7 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
    } else {
        (*str_parametric_drc_instructions).parametric_drc_look_ahead = 0
            as core::ffi::c_int as WORD32;
    }
    (*str_parametric_drc_instructions).parametric_drc_preset_id_present = impd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*str_parametric_drc_instructions).parametric_drc_preset_id_present != 0 {
        (*str_parametric_drc_instructions).parametric_drc_preset_id = impd_read_bits_buf(
            it_bit_buff,
            7 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        match (*str_parametric_drc_instructions).parametric_drc_preset_id {
            0 | 1 | 2 | 3 | 4 => {
                (*str_parametric_drc_instructions).drc_characteristic = ((*str_parametric_drc_instructions)
                    .parametric_drc_preset_id as core::ffi::c_int
                    + 7 as core::ffi::c_int) as WORD32;
                (*str_parametric_drc_instructions).parametric_drc_type = PARAM_DRC_TYPE_FF
                    as WORD32;
                (*str_parametric_drc_instructions)
                    .str_parametric_drc_type_feed_forward
                    .level_estim_k_weighting_type = 2 as core::ffi::c_int as WORD32;
                (*str_parametric_drc_instructions)
                    .str_parametric_drc_type_feed_forward
                    .level_estim_integration_time = parametric_drc_frame_size;
                impd_parametric_drc_ffwd_init_drc_curve_params(
                    (*str_parametric_drc_instructions).drc_characteristic,
                    &mut (*str_parametric_drc_instructions)
                        .str_parametric_drc_type_feed_forward,
                );
                impd_parametric_drc_ffwd_init_drc_gain_smooth_params(
                    (*str_parametric_drc_instructions).drc_characteristic,
                    &mut (*str_parametric_drc_instructions)
                        .str_parametric_drc_type_feed_forward,
                );
            }
            _ => {
                (*str_parametric_drc_instructions).disable_paramteric_drc = 1
                    as core::ffi::c_int as WORD32;
            }
        }
    } else {
        (*str_parametric_drc_instructions).parametric_drc_type = impd_read_bits_buf(
            it_bit_buff,
            3 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*str_parametric_drc_instructions).parametric_drc_type == PARAM_DRC_TYPE_FF {
            err = impd_parse_parametric_drc_ffwd(
                it_bit_buff,
                parametric_drc_frame_size,
                &mut (*str_parametric_drc_instructions)
                    .str_parametric_drc_type_feed_forward,
            );
            if err != 0 {
                return err;
            }
            (*str_parametric_drc_instructions).disable_paramteric_drc = (*str_parametric_drc_instructions)
                .str_parametric_drc_type_feed_forward
                .disable_paramteric_drc;
            (*str_parametric_drc_instructions).drc_characteristic = (*str_parametric_drc_instructions)
                .str_parametric_drc_type_feed_forward
                .drc_characteristic;
        } else if (*str_parametric_drc_instructions).parametric_drc_type
            == PARAM_DRC_TYPE_LIM
        {
            err = impd_parse_parametric_drc_lim(
                it_bit_buff,
                &mut (*str_parametric_drc_instructions).parametric_drc_lim,
            );
            if err != 0 {
                return err;
            }
            (*str_parametric_drc_instructions).disable_paramteric_drc = (*str_parametric_drc_instructions)
                .parametric_drc_lim
                .disable_paramteric_drc;
            (*str_parametric_drc_instructions).drc_characteristic = (*str_parametric_drc_instructions)
                .parametric_drc_lim
                .drc_characteristic;
            if (*str_parametric_drc_instructions).parametric_drc_look_ahead_flag != 0 {
                (*str_parametric_drc_instructions)
                    .parametric_drc_lim
                    .parametric_lim_attack = (*str_parametric_drc_instructions)
                    .parametric_drc_look_ahead;
            }
        } else {
            bit_size_len = (impd_read_bits_buf(it_bit_buff, 3 as WORD)
                as core::ffi::c_int + 4 as core::ffi::c_int) as WORD32;
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            bit_size = impd_read_bits_buf(it_bit_buff, bit_size_len as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            (*str_parametric_drc_instructions).len_bit_size = (bit_size
                as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
            match (*str_parametric_drc_instructions).parametric_drc_type {
                _ => {}
            }
            (*str_parametric_drc_instructions).disable_paramteric_drc = 1
                as core::ffi::c_int as WORD32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*str_parametric_drc_instructions).len_bit_size {
                impd_read_bits_buf(it_bit_buff, 1 as WORD);
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                i += 1;
            }
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_loud_info_set_ext_eq(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut loudness_info_set: *mut ia_drc_loudness_info_set_struct,
) -> WORD32 {
    let mut err: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut offset: WORD32 = 0;
    let mut version: WORD32 = 1 as WORD32;
    let mut temp: WORD32 = 0;
    let mut loudness_info_v1_album_cnt: WORD32 = 0;
    let mut loudness_info_v1_cnt: WORD32 = 0;
    temp = impd_read_bits_buf(it_bit_buff, 12 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    loudness_info_v1_album_cnt = (temp as core::ffi::c_int >> 6 as core::ffi::c_int
        & 0x3f as core::ffi::c_int) as WORD32;
    loudness_info_v1_cnt = (temp as core::ffi::c_int & 0x3f as core::ffi::c_int)
        as WORD32;
    offset = (*loudness_info_set).loudness_info_album_count;
    (*loudness_info_set).loudness_info_album_count += loudness_info_v1_album_cnt;
    if offset + loudness_info_v1_album_cnt > LOUDNESS_INFO_COUNT_MAX {
        return 2 as WORD32;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < loudness_info_v1_album_cnt {
        err = impd_parse_loudness_info(
            it_bit_buff,
            version,
            &mut *((*loudness_info_set).str_loudness_info_album)
                .as_mut_ptr()
                .offset((i + offset) as isize),
        );
        if err != 0 {
            return err;
        }
        i += 1;
    }
    offset = (*loudness_info_set).loudness_info_count;
    (*loudness_info_set).loudness_info_count += loudness_info_v1_cnt;
    if offset + loudness_info_v1_cnt > LOUDNESS_INFO_COUNT_MAX {
        return 2 as WORD32;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < loudness_info_v1_cnt {
        err = impd_parse_loudness_info(
            it_bit_buff,
            version,
            &mut *((*loudness_info_set).loudness_info)
                .as_mut_ptr()
                .offset((i + offset) as isize),
        );
        if err != 0 {
            return err;
        }
        i += 1;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_ch_layout(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ia_drc_params_struct: *mut ia_drc_params_bs_dec_struct,
    mut channel_layout: *mut ia_channel_layout_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    (*channel_layout).base_channel_count = impd_read_bits_buf(it_bit_buff, 7 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*channel_layout).base_channel_count > MAX_CHANNEL_COUNT
        || (*channel_layout).base_channel_count == 0 as core::ffi::c_int
    {
        return 2 as WORD32;
    }
    if (*ia_drc_params_struct).lfe_channel_map_count != -(1 as core::ffi::c_int)
        && (*channel_layout).base_channel_count
            != (*ia_drc_params_struct).lfe_channel_map_count
    {
        return 2 as WORD32;
    }
    (*channel_layout).layout_signaling_present = impd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*channel_layout).layout_signaling_present != 0 {
        (*channel_layout).defined_layout = impd_read_bits_buf(it_bit_buff, 8 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*channel_layout).defined_layout == 0 as core::ffi::c_int {
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*channel_layout).base_channel_count {
                (*channel_layout).speaker_position[i as usize] = impd_read_bits_buf(
                    it_bit_buff,
                    7 as WORD,
                );
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                if (*channel_layout).speaker_position[i as usize]
                    == 3 as core::ffi::c_int
                    || (*channel_layout).speaker_position[i as usize]
                        == 26 as core::ffi::c_int
                {
                    (*ia_drc_params_struct).lfe_channel_map[i as usize] = 1
                        as core::ffi::c_int as WORD32;
                } else {
                    (*ia_drc_params_struct).lfe_channel_map[i as usize] = 0
                        as core::ffi::c_int as WORD32;
                }
                i += 1;
            }
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_dwnmix_instructions(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut version: WORD32,
    mut ia_drc_params_struct: *mut ia_drc_params_bs_dec_struct,
    mut channel_layout: *mut ia_channel_layout_struct,
    mut dwnmix_instructions: *mut ia_downmix_instructions_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut temp: WORD32 = 0;
    temp = impd_read_bits_buf(it_bit_buff, 23 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    (*dwnmix_instructions).downmix_id = (temp as core::ffi::c_int
        >> 16 as core::ffi::c_int & 0x7f as core::ffi::c_int) as WORD32;
    (*dwnmix_instructions).target_channel_count = (temp as core::ffi::c_int
        >> 9 as core::ffi::c_int & 0x7f as core::ffi::c_int) as WORD32;
    if (*dwnmix_instructions).target_channel_count > MAX_CHANNEL_COUNT {
        return 2 as WORD32;
    }
    (*dwnmix_instructions).target_layout = (temp as core::ffi::c_int
        >> 1 as core::ffi::c_int & 0xff as core::ffi::c_int) as WORD32;
    (*dwnmix_instructions).downmix_coefficients_present = (temp as core::ffi::c_int
        & 1 as core::ffi::c_int) as WORD32;
    if (*dwnmix_instructions).downmix_coefficients_present != 0 {
        if version == 0 as core::ffi::c_int {
            let mut dmix_coeff: WORD32 = 0;
            k = 0 as core::ffi::c_int as WORD32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*dwnmix_instructions).target_channel_count {
                j = 0 as core::ffi::c_int as WORD32;
                while j < (*channel_layout).base_channel_count {
                    dmix_coeff = impd_read_bits_buf(it_bit_buff, 4 as WORD);
                    if (*it_bit_buff).error != 0 {
                        return (*it_bit_buff).error;
                    }
                    if k >= DOWNMIX_COEFF_COUNT_MAX {
                        return 2 as WORD32;
                    }
                    if (*ia_drc_params_struct).lfe_channel_map[j as usize] != 0 {
                        (*dwnmix_instructions).downmix_coefficient[k as usize] = pow(
                            10.0f64,
                            (0.05f32
                                * *dwnmix_coeff_lfe.as_ptr().offset(dmix_coeff as isize))
                                as core::ffi::c_double,
                        ) as FLOAT32;
                    } else {
                        (*dwnmix_instructions).downmix_coefficient[k as usize] = pow(
                            10.0f64,
                            (0.05f32
                                * *dwnmix_coeff.as_ptr().offset(dmix_coeff as isize))
                                as core::ffi::c_double,
                        ) as FLOAT32;
                    }
                    k += 1;
                    j += 1;
                }
                i += 1;
            }
        } else {
            let mut dmix_coeff_v1: WORD32 = 0;
            let mut bs_dmix_offset: WORD32 = 0;
            let mut a: FLOAT32 = 0.;
            let mut b: FLOAT32 = 0.;
            let mut dmix_offset: FLOAT32 = 0.;
            let mut sum: FLOAT32 = 0.;
            bs_dmix_offset = impd_read_bits_buf(it_bit_buff, 4 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            k = 0 as core::ffi::c_int as WORD32;
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*dwnmix_instructions).target_channel_count {
                j = 0 as core::ffi::c_int as WORD32;
                while j < (*channel_layout).base_channel_count {
                    dmix_coeff_v1 = impd_read_bits_buf(it_bit_buff, 5 as WORD);
                    if (*it_bit_buff).error != 0 {
                        return (*it_bit_buff).error;
                    }
                    if k >= DOWNMIX_COEFF_COUNT_MAX {
                        return 2 as WORD32;
                    }
                    (*dwnmix_instructions).downmix_coefficient[k as usize] = *dwnmix_coeff_v1
                        .as_ptr()
                        .offset(dmix_coeff_v1 as isize);
                    k += 1;
                    j += 1;
                }
                i += 1;
            }
            match bs_dmix_offset {
                0 => {
                    dmix_offset = 0.0f32 as FLOAT32;
                }
                1 => {
                    a = 20.0f32
                        * log10(
                            ((*dwnmix_instructions).target_channel_count as FLOAT32
                                / (*channel_layout).base_channel_count as FLOAT32)
                                as core::ffi::c_double,
                        ) as FLOAT32;
                    dmix_offset = 0.5f32
                        * floor((0.5f32 + a) as core::ffi::c_double) as FLOAT32;
                }
                2 => {
                    a = 20.0f32
                        * log10(
                            ((*dwnmix_instructions).target_channel_count as FLOAT32
                                / (*channel_layout).base_channel_count as FLOAT32)
                                as core::ffi::c_double,
                        ) as FLOAT32;
                    dmix_offset = 0.5f32
                        * floor((0.5f32 + 2.0f32 * a) as core::ffi::c_double) as FLOAT32;
                }
                3 => {
                    sum = 0.0f32 as FLOAT32;
                    k = 0 as core::ffi::c_int as WORD32;
                    while k
                        < (*dwnmix_instructions).target_channel_count
                            * (*channel_layout).base_channel_count
                    {
                        sum
                            += pow(
                                10.0f64,
                                (0.1f32
                                    * (*dwnmix_instructions).downmix_coefficient[k as usize])
                                    as core::ffi::c_double,
                            ) as FLOAT32;
                        k += 1;
                    }
                    b = 10.0f32 * log10(sum as core::ffi::c_double) as FLOAT32;
                    dmix_offset = 0.5f32
                        * floor((0.5f32 + 2.0f32 * b) as core::ffi::c_double) as FLOAT32;
                }
                _ => return 6 as WORD32,
            }
            k = 0 as core::ffi::c_int as WORD32;
            while k
                < (*dwnmix_instructions).target_channel_count
                    * (*channel_layout).base_channel_count
            {
                (*dwnmix_instructions).downmix_coefficient[k as usize] = pow(
                    10.0f64,
                    (0.05f32
                        * ((*dwnmix_instructions).downmix_coefficient[k as usize]
                            + dmix_offset)) as core::ffi::c_double,
                ) as FLOAT32;
                k += 1;
            }
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_drc_gen_instructions_for_drc_off(
    mut drc_config: *mut ia_drc_config,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut s: WORD32 = 0;
    let mut str_drc_instruction_str: *mut ia_drc_instructions_struct = 0
        as *mut ia_drc_instructions_struct;
    s = -(1 as core::ffi::c_int) as WORD32;
    k = (*drc_config).drc_instructions_uni_drc_count;
    str_drc_instruction_str = &mut *((*drc_config).str_drc_instruction_str)
        .as_mut_ptr()
        .offset(k as isize) as *mut ia_drc_instructions_struct;
    memset(
        str_drc_instruction_str as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<ia_drc_instructions_struct>() as size_t,
    );
    (*str_drc_instruction_str).drc_set_id = s;
    s -= 1;
    (*str_drc_instruction_str).downmix_id[0 as core::ffi::c_int as usize] = ID_FOR_BASE_LAYOUT
        as WORD32;
    (*str_drc_instruction_str).dwnmix_id_count = 1 as core::ffi::c_int as WORD32;
    (*str_drc_instruction_str).drc_apply_to_dwnmix = 0 as core::ffi::c_int as WORD32;
    (*str_drc_instruction_str).depends_on_drc_set_present = 0 as core::ffi::c_int
        as WORD32;
    (*str_drc_instruction_str).no_independent_use = 0 as core::ffi::c_int as WORD32;
    (*str_drc_instruction_str).gain_element_count = 0 as core::ffi::c_int as WORD32;
    i = 1 as core::ffi::c_int as WORD32;
    while i
        < (*drc_config).dwnmix_instructions_count as core::ffi::c_int
            + 1 as core::ffi::c_int
    {
        str_drc_instruction_str = &mut *((*drc_config).str_drc_instruction_str)
            .as_mut_ptr()
            .offset((k + i) as isize) as *mut ia_drc_instructions_struct;
        memset(
            str_drc_instruction_str as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<ia_drc_instructions_struct>() as size_t,
        );
        (*str_drc_instruction_str).drc_set_id = s;
        s -= 1;
        (*str_drc_instruction_str).drc_set_complexity_level = 0 as core::ffi::c_int
            as WORD32;
        (*str_drc_instruction_str).requires_eq = 0 as core::ffi::c_int as WORD32;
        (*str_drc_instruction_str).downmix_id[0 as core::ffi::c_int as usize] = (*drc_config)
            .dwnmix_instructions[(i as core::ffi::c_int - 1 as core::ffi::c_int)
                as usize]
            .downmix_id;
        (*str_drc_instruction_str).dwnmix_id_count = 1 as core::ffi::c_int as WORD32;
        (*str_drc_instruction_str).drc_apply_to_dwnmix = 0 as core::ffi::c_int as WORD32;
        (*str_drc_instruction_str).depends_on_drc_set_present = 0 as core::ffi::c_int
            as WORD32;
        (*str_drc_instruction_str).no_independent_use = 0 as core::ffi::c_int as WORD32;
        (*str_drc_instruction_str).gain_element_count = 0 as core::ffi::c_int as WORD32;
        i += 1;
    }
    (*drc_config).drc_instructions_count_plus = ((*drc_config)
        .drc_instructions_uni_drc_count as core::ffi::c_int
        + (*drc_config).dwnmix_instructions_count as core::ffi::c_int
        + 1 as core::ffi::c_int) as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_drc_config_ext(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ia_drc_params_struct: *mut ia_drc_params_bs_dec_struct,
    mut drc_config: *mut ia_drc_config,
    mut str_drc_config_ext: *mut ia_drc_config_ext,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut bit_size_len: WORD32 = 0;
    let mut ext_size_bits: WORD32 = 0;
    let mut bit_size: WORD32 = 0;
    k = 0 as core::ffi::c_int as WORD32;
    (*str_drc_config_ext).drc_config_ext_type[k as usize] = impd_read_bits_buf(
        it_bit_buff,
        4 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    while (*str_drc_config_ext).drc_config_ext_type[k as usize] != UNIDRCCONFEXT_TERM {
        if k >= EXT_COUNT_MAX - 1 as core::ffi::c_int {
            return UNEXPECTED_ERROR;
        }
        bit_size_len = impd_read_bits_buf(it_bit_buff, 4 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        ext_size_bits = (bit_size_len as core::ffi::c_int + 4 as core::ffi::c_int)
            as WORD32;
        bit_size = impd_read_bits_buf(it_bit_buff, ext_size_bits as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*str_drc_config_ext).ext_bit_size[k as usize] = (bit_size as core::ffi::c_int
            + 1 as core::ffi::c_int) as WORD32;
        match (*str_drc_config_ext).drc_config_ext_type[k as usize] {
            UNIDRCCONFEXT_PARAM_DRC => {
                (*str_drc_config_ext).parametric_drc_present = 1 as core::ffi::c_int
                    as WORD32;
                err = impd_parametic_drc_parse_coeff(
                    it_bit_buff,
                    drc_config,
                    &mut (*str_drc_config_ext).str_drc_coeff_param_drc,
                );
                if err != 0 {
                    return err;
                }
                (*str_drc_config_ext).parametric_drc_instructions_count = impd_read_bits_buf(
                    it_bit_buff,
                    4 as WORD,
                );
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                if (*str_drc_config_ext).parametric_drc_instructions_count
                    > PARAM_DRC_INSTRUCTIONS_COUNT_MAX
                {
                    return 2 as WORD32;
                }
                i = 0 as core::ffi::c_int as WORD32;
                while i < (*str_drc_config_ext).parametric_drc_instructions_count {
                    err = impd_parse_parametric_drc_instructions(
                        it_bit_buff,
                        (*str_drc_config_ext)
                            .str_drc_coeff_param_drc
                            .parametric_drc_frame_size,
                        &mut *((*str_drc_config_ext).str_parametric_drc_instructions)
                            .as_mut_ptr()
                            .offset(i as isize),
                    );
                    if err != 0 {
                        return err;
                    }
                    i += 1;
                }
            }
            UNIDRCCONFEXT_V1 => {
                (*str_drc_config_ext).drc_extension_v1_present = 1 as core::ffi::c_int
                    as WORD32;
                err = impd_parse_drc_ext_v1(
                    it_bit_buff,
                    ia_drc_params_struct,
                    drc_config,
                    str_drc_config_ext,
                );
                if err != 0 {
                    return err;
                }
            }
            UNIDRCCONFEXT_LEVELING => {
                err = impd_leveling_instructions(it_bit_buff, drc_config);
                if err != 0 {
                    return err;
                }
            }
            _ => {
                i = 0 as core::ffi::c_int as WORD32;
                while i < (*str_drc_config_ext).ext_bit_size[k as usize] {
                    impd_read_bits_buf(it_bit_buff, 1 as WORD);
                    if (*it_bit_buff).error != 0 {
                        return (*it_bit_buff).error;
                    }
                    i += 1;
                }
            }
        }
        k += 1;
        (*str_drc_config_ext).drc_config_ext_type[k as usize] = impd_read_bits_buf(
            it_bit_buff,
            4 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
    }
    return 0 as WORD32;
}
unsafe extern "C" fn impd_parse_split_drc_characteristic(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    side: WORD32,
    mut split_drc_characteristic: *mut ia_split_drc_characteristic_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut temp: WORD32 = 0;
    (*split_drc_characteristic).characteristic_format = impd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*split_drc_characteristic).characteristic_format == 0 as core::ffi::c_int {
        let mut bsGain: WORD32 = 0;
        let mut bsIoRatio: WORD32 = 0;
        let mut bsExp: WORD32 = 0;
        bsGain = impd_read_bits_buf(it_bit_buff, 6 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if side == LEFT_SIDE {
            (*split_drc_characteristic).gain = bsGain as FLOAT32;
        } else {
            (*split_drc_characteristic).gain = -bsGain as FLOAT32;
        }
        temp = impd_read_bits_buf(it_bit_buff, 8 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        bsIoRatio = (temp as core::ffi::c_int >> 4 as core::ffi::c_int
            & 0xf as core::ffi::c_int) as WORD32;
        bsExp = (temp as core::ffi::c_int & 0xf as core::ffi::c_int) as WORD32;
        (*split_drc_characteristic).in_out_ratio = (0.05f32
            + 0.15f32 * bsIoRatio as core::ffi::c_float) as FLOAT32;
        if bsExp < 15 as core::ffi::c_int {
            (*split_drc_characteristic).exp = (1.0f32
                + 2.0f32 * bsExp as core::ffi::c_float) as FLOAT32;
        } else {
            (*split_drc_characteristic).exp = 1000.0f32 as FLOAT32;
        }
        (*split_drc_characteristic).flip_sign = impd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
    } else {
        let mut char_node_cnt: WORD32 = 0;
        let mut node_level_delta: WORD32 = 0;
        let mut node_gain: WORD32 = 0;
        char_node_cnt = impd_read_bits_buf(it_bit_buff, 2 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*split_drc_characteristic).characteristic_node_count = (char_node_cnt
            as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
        (*split_drc_characteristic).node_level[0 as core::ffi::c_int as usize] = DRC_INPUT_LOUDNESS_TARGET
            as FLOAT32;
        (*split_drc_characteristic).node_gain[0 as core::ffi::c_int as usize] = 0.0f32
            as FLOAT32;
        i = 1 as core::ffi::c_int as WORD32;
        while i <= (*split_drc_characteristic).characteristic_node_count {
            node_level_delta = impd_read_bits_buf(it_bit_buff, 5 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if side == LEFT_SIDE {
                (*split_drc_characteristic).node_level[i as usize] = ((*split_drc_characteristic)
                    .node_level[(i as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
                    - (1.0f32 + node_level_delta as core::ffi::c_float)) as FLOAT32;
            } else {
                (*split_drc_characteristic).node_level[i as usize] = ((*split_drc_characteristic)
                    .node_level[(i as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
                    + (1.0f32 + node_level_delta as core::ffi::c_float)) as FLOAT32;
            }
            node_gain = impd_read_bits_buf(it_bit_buff, 8 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            (*split_drc_characteristic).node_gain[i as usize] = (0.5f32
                * node_gain as core::ffi::c_float - 64.0f32) as FLOAT32;
            i += 1;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_drc_gen_instructions_derived_data(
    mut drc_config: *mut ia_drc_config,
    mut ia_drc_params_struct: *mut ia_drc_params_bs_dec_struct,
    mut str_drc_instruction_str: *mut ia_drc_instructions_struct,
) -> WORD32 {
    let mut n: WORD32 = 0;
    let mut g: WORD32 = 0;
    let mut str_p_loc_drc_coefficients_uni_drc: *mut ia_uni_drc_coeffs_struct = 0
        as *mut ia_uni_drc_coeffs_struct;
    let mut str_drc_coeff_param_drc: *mut ia_drc_coeff_parametric_drc_struct = 0
        as *mut ia_drc_coeff_parametric_drc_struct;
    let mut gain_element_count: WORD32 = 0 as WORD32;
    n = 0 as core::ffi::c_int as WORD32;
    while n < (*drc_config).drc_coefficients_drc_count {
        if (*drc_config).str_p_loc_drc_coefficients_uni_drc[n as usize].drc_location
            == (*str_drc_instruction_str).drc_location
        {
            break;
        }
        n += 1;
    }
    if n == (*drc_config).drc_coefficients_drc_count
        && (*drc_config).drc_coefficients_drc_count > 0 as core::ffi::c_int
    {
        return -(1 as WORD32);
    }
    str_p_loc_drc_coefficients_uni_drc = &mut *((*drc_config)
        .str_p_loc_drc_coefficients_uni_drc)
        .as_mut_ptr()
        .offset(n as isize) as *mut ia_uni_drc_coeffs_struct;
    if (*drc_config).drc_config_ext_present != 0
        && (*drc_config).str_drc_config_ext.parametric_drc_present != 0
        && (*drc_config).str_drc_config_ext.str_drc_coeff_param_drc.drc_location
            == (*str_drc_instruction_str).drc_location
    {
        str_drc_coeff_param_drc = &mut (*drc_config)
            .str_drc_config_ext
            .str_drc_coeff_param_drc;
    }
    g = 0 as core::ffi::c_int as WORD32;
    while g < (*str_drc_instruction_str).num_drc_ch_groups {
        let mut seq: WORD32 = (*str_drc_instruction_str)
            .gain_set_index_for_channel_group[g as usize];
        if seq != -(1 as core::ffi::c_int)
            && ((*drc_config).drc_coefficients_drc_count == 0 as core::ffi::c_int
                || seq >= (*str_p_loc_drc_coefficients_uni_drc).gain_set_count)
        {
            (*str_drc_instruction_str).ch_group_parametric_drc_flag[g as usize] = 1
                as core::ffi::c_int as WORD32;
            if (*drc_config).drc_coefficients_drc_count != 0 as core::ffi::c_int {
                seq = seq - (*str_p_loc_drc_coefficients_uni_drc).gain_set_count;
            }
            (*str_drc_instruction_str)
                .gain_set_idx_of_ch_group_parametric_drc[g as usize] = seq;
            if str_drc_coeff_param_drc.is_null()
                || seq >= (*str_drc_coeff_param_drc).parametric_drc_gain_set_count
            {
                return 4 as WORD32;
            }
            (*str_drc_instruction_str)
                .gain_interpolation_type_for_channel_group[g as usize] = 1
                as core::ffi::c_int as WORD32;
            (*str_drc_instruction_str).time_delta_min_for_channel_group[g as usize] = (*str_drc_coeff_param_drc)
                .parametric_drc_frame_size;
            (*str_drc_instruction_str).time_alignment_for_channel_group[g as usize] = 0
                as core::ffi::c_int as WORD32;
        } else {
            (*str_drc_instruction_str).ch_group_parametric_drc_flag[g as usize] = 0
                as core::ffi::c_int as WORD32;
        }
        if (*str_drc_instruction_str).ch_group_parametric_drc_flag[g as usize]
            == 0 as core::ffi::c_int
        {
            if seq >= (*str_p_loc_drc_coefficients_uni_drc).gain_set_count {
                return -(1 as WORD32);
            }
            (*str_drc_instruction_str)
                .gain_interpolation_type_for_channel_group[g as usize] = (*str_p_loc_drc_coefficients_uni_drc)
                .gain_set_params[seq as usize]
                .gain_interpolation_type;
            if (*str_p_loc_drc_coefficients_uni_drc)
                .gain_set_params[seq as usize]
                .time_delt_min_flag != 0
            {
                (*str_drc_instruction_str)
                    .time_delta_min_for_channel_group[g as usize] = (*str_p_loc_drc_coefficients_uni_drc)
                    .gain_set_params[seq as usize]
                    .time_delt_min_val;
            } else {
                (*str_drc_instruction_str)
                    .time_delta_min_for_channel_group[g as usize] = (*ia_drc_params_struct)
                    .delta_tmin_default;
            }
            (*str_drc_instruction_str).time_alignment_for_channel_group[g as usize] = (*str_p_loc_drc_coefficients_uni_drc)
                .gain_set_params[seq as usize]
                .time_alignment;
        }
        g += 1;
    }
    if (*str_drc_instruction_str).drc_set_effect as core::ffi::c_int
        & (EFFECT_BIT_DUCK_OTHER | EFFECT_BIT_DUCK_SELF) != 0
    {
        (*str_drc_instruction_str).gain_element_count = (*str_drc_instruction_str)
            .num_drc_ch_groups;
    } else {
        g = 0 as core::ffi::c_int as WORD32;
        while g < (*str_drc_instruction_str).num_drc_ch_groups {
            if (*str_drc_instruction_str).ch_group_parametric_drc_flag[g as usize]
                == 1 as core::ffi::c_int
            {
                gain_element_count += 1;
                (*str_drc_instruction_str).band_count_of_ch_group[g as usize] = 1
                    as core::ffi::c_int as WORD32;
            } else {
                let mut seq_0: WORD32 = 0;
                let mut band_count: WORD32 = 0;
                seq_0 = (*str_drc_instruction_str)
                    .gain_set_index_for_channel_group[g as usize];
                band_count = (*str_p_loc_drc_coefficients_uni_drc)
                    .gain_set_params[seq_0 as usize]
                    .band_count;
                (*str_drc_instruction_str).band_count_of_ch_group[g as usize] = band_count;
                gain_element_count += band_count;
            }
            g += 1;
        }
        (*str_drc_instruction_str).gain_element_count = gain_element_count;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_drc_config(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ia_drc_params_struct: *mut ia_drc_params_bs_dec_struct,
    mut drc_config: *mut ia_drc_config,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut err: WORD32 = 0 as WORD32;
    let mut temp: WORD32 = 0;
    let mut version: WORD32 = 0 as WORD32;
    (*drc_config).sample_rate_present = impd_read_bits_buf(it_bit_buff, 1 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*drc_config).sample_rate_present == 1 as core::ffi::c_int {
        let mut bssample_rate: WORD32 = 0;
        bssample_rate = impd_read_bits_buf(it_bit_buff, 18 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*drc_config).sampling_rate = (bssample_rate as core::ffi::c_int
            + 1000 as core::ffi::c_int) as WORD32;
    }
    temp = impd_read_bits_buf(it_bit_buff, 8 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    (*drc_config).dwnmix_instructions_count = (temp as core::ffi::c_int
        >> 1 as core::ffi::c_int & 0x7f as core::ffi::c_int) as WORD32;
    if (*drc_config).dwnmix_instructions_count > DOWNMIX_INSTRUCTION_COUNT_MAX {
        return 2 as WORD32;
    }
    (*drc_config).drc_description_basic_present = (temp as core::ffi::c_int
        & 1 as core::ffi::c_int) as WORD32;
    if (*drc_config).drc_description_basic_present == 1 as core::ffi::c_int {
        temp = impd_read_bits_buf(it_bit_buff, 7 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*drc_config).drc_coefficients_basic_count = (temp as core::ffi::c_int
            >> 4 as core::ffi::c_int & 7 as core::ffi::c_int) as WORD32;
        (*drc_config).drc_instructions_basic_count = (temp as core::ffi::c_int
            & 0xf as core::ffi::c_int) as WORD32;
    } else {
        (*drc_config).drc_coefficients_basic_count = 0 as core::ffi::c_int as WORD32;
        (*drc_config).drc_instructions_basic_count = 0 as core::ffi::c_int as WORD32;
    }
    temp = impd_read_bits_buf(it_bit_buff, 9 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    (*drc_config).drc_coefficients_drc_count = (temp as core::ffi::c_int
        >> 6 as core::ffi::c_int & 7 as core::ffi::c_int) as WORD32;
    (*drc_config).drc_instructions_uni_drc_count = (temp as core::ffi::c_int
        & 0x3f as core::ffi::c_int) as WORD32;
    if (*drc_config).drc_instructions_uni_drc_count > DRC_INSTRUCTIONS_COUNT_MAX {
        return 2 as WORD32;
    }
    err = impd_parse_ch_layout(
        it_bit_buff,
        ia_drc_params_struct,
        &mut (*drc_config).channel_layout,
    );
    if err != 0 {
        return err;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*drc_config).dwnmix_instructions_count {
        err = impd_parse_dwnmix_instructions(
            it_bit_buff,
            version,
            ia_drc_params_struct,
            &mut (*drc_config).channel_layout,
            &mut *((*drc_config).dwnmix_instructions).as_mut_ptr().offset(i as isize),
        );
        if err != 0 {
            return err;
        }
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*drc_config).drc_coefficients_basic_count {
        temp = impd_read_bits_buf(it_bit_buff, 11 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*drc_config).str_drc_coefficients_basic[i as usize].drc_location = (temp
            as core::ffi::c_int >> 7 as core::ffi::c_int & 0xf as core::ffi::c_int)
            as WORD32;
        (*drc_config).str_drc_coefficients_basic[i as usize].drc_characteristic = (temp
            as core::ffi::c_int & 0x3f as core::ffi::c_int) as WORD32;
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*drc_config).drc_instructions_basic_count {
        err = impd_drc_parse_instructions_basic(
            it_bit_buff,
            &mut *((*drc_config).str_drc_instructions_basic)
                .as_mut_ptr()
                .offset(i as isize),
        );
        if err != 0 {
            return err;
        }
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*drc_config).drc_coefficients_drc_count {
        err = impd_drc_parse_coeff(
            it_bit_buff,
            version,
            ia_drc_params_struct,
            &mut *((*drc_config).str_p_loc_drc_coefficients_uni_drc)
                .as_mut_ptr()
                .offset(i as isize),
        );
        if err != 0 {
            return err;
        }
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*drc_config).drc_instructions_uni_drc_count {
        err = impd_parse_drc_instructions_uni_drc(
            it_bit_buff,
            version,
            drc_config,
            &mut *((*drc_config).str_drc_instruction_str).as_mut_ptr().offset(i as isize),
        );
        if err != 0 {
            return err;
        }
        i += 1;
    }
    (*drc_config).drc_config_ext_present = impd_read_bits_buf(it_bit_buff, 1 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*drc_config).drc_config_ext_present == 1 as core::ffi::c_int {
        err = impd_parse_drc_config_ext(
            it_bit_buff,
            ia_drc_params_struct,
            drc_config,
            &mut (*drc_config).str_drc_config_ext,
        );
        if err != 0 {
            return err;
        }
    }
    if (*drc_config).str_drc_config_ext.parametric_drc_present != 0 {
        err = impd_parametric_drc_gen_virtual_gain_sets(drc_config);
        if err != 0 {
            return err;
        }
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*drc_config).drc_instructions_uni_drc_count {
        err = impd_drc_gen_instructions_derived_data(
            drc_config,
            ia_drc_params_struct,
            &mut *((*drc_config).str_drc_instruction_str).as_mut_ptr().offset(i as isize),
        );
        if err != 0 {
            return err;
        }
        i += 1;
    }
    if (*drc_config).drc_instructions_uni_drc_count
        + (*drc_config).dwnmix_instructions_count >= DRC_INSTRUCTIONS_COUNT_MAX
    {
        return 2 as WORD32;
    }
    impd_drc_gen_instructions_for_drc_off(drc_config);
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_dec_method_value(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut method_def: WORD32,
    mut method_val: *mut FLOAT32,
) -> WORD32 {
    let mut tmp: WORD32 = 0;
    let mut val: FLOAT32 = 0.;
    match method_def {
        METHOD_DEFINITION_UNKNOWN_OTHER
        | METHOD_DEFINITION_PROGRAM_LOUDNESS
        | METHOD_DEFINITION_ANCHOR_LOUDNESS
        | METHOD_DEFINITION_MAX_OF_LOUDNESS_RANGE
        | METHOD_DEFINITION_MOMENTARY_LOUDNESS_MAX
        | METHOD_DEFINITION_SHORT_TERM_LOUDNESS_MAX => {
            tmp = impd_read_bits_buf(it_bit_buff, 8 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            val = (-57.75f32 + tmp as core::ffi::c_float * 0.25f32) as FLOAT32;
        }
        METHOD_DEFINITION_LOUDNESS_RANGE => {
            tmp = impd_read_bits_buf(it_bit_buff, 8 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if tmp == 0 as core::ffi::c_int {
                val = 0.0f32 as FLOAT32;
            } else if tmp <= 128 as core::ffi::c_int {
                val = (tmp as core::ffi::c_float * 0.25f32) as FLOAT32;
            } else if tmp <= 204 as core::ffi::c_int {
                val = (0.5f32 * tmp as core::ffi::c_float - 32.0f32) as FLOAT32;
            } else {
                val = (tmp as core::ffi::c_float - 134.0f32) as FLOAT32;
            }
        }
        METHOD_DEFINITION_MIXING_LEVEL => {
            tmp = impd_read_bits_buf(it_bit_buff, 5 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            val = (tmp as core::ffi::c_float + 80.0f32) as FLOAT32;
        }
        METHOD_DEFINITION_ROOM_TYPE => {
            tmp = impd_read_bits_buf(it_bit_buff, 2 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            val = tmp as FLOAT32;
        }
        METHOD_DEFINITION_SHORT_TERM_LOUDNESS => {
            tmp = impd_read_bits_buf(it_bit_buff, 8 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            val = (-116.0f32 + tmp as core::ffi::c_float * 0.5f32) as FLOAT32;
        }
        _ => return -(1 as WORD32),
    }
    *method_val = val;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_loudness_info_set(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut loudness_info_set: *mut ia_drc_loudness_info_set_struct,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut i: WORD32 = 0;
    let mut version: WORD32 = 0 as WORD32;
    let mut offset: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut loudness_info_album_count: WORD32 = 0;
    let mut loudness_info_count: WORD32 = 0;
    temp = impd_read_bits_buf(it_bit_buff, 12 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    loudness_info_album_count = (temp as core::ffi::c_int >> 6 as core::ffi::c_int
        & 0x3f as core::ffi::c_int) as WORD32;
    loudness_info_count = (temp as core::ffi::c_int & 0x3f as core::ffi::c_int)
        as WORD32;
    offset = (*loudness_info_set).loudness_info_album_count;
    (*loudness_info_set).loudness_info_album_count += loudness_info_album_count;
    if offset + (*loudness_info_set).loudness_info_album_count > LOUDNESS_INFO_COUNT_MAX
    {
        return 2 as WORD32;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*loudness_info_set).loudness_info_album_count {
        err = impd_parse_loudness_info(
            it_bit_buff,
            version,
            &mut *((*loudness_info_set).str_loudness_info_album)
                .as_mut_ptr()
                .offset((i + offset) as isize),
        );
        if err != 0 {
            return err;
        }
        i += 1;
    }
    offset = (*loudness_info_set).loudness_info_count;
    (*loudness_info_set).loudness_info_count += loudness_info_count;
    if offset + (*loudness_info_set).loudness_info_count > LOUDNESS_INFO_COUNT_MAX {
        return 2 as WORD32;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*loudness_info_set).loudness_info_count {
        err = impd_parse_loudness_info(
            it_bit_buff,
            version,
            &mut *((*loudness_info_set).loudness_info)
                .as_mut_ptr()
                .offset((i + offset) as isize),
        );
        if err != 0 {
            return err;
        }
        i += 1;
    }
    (*loudness_info_set).loudness_info_set_ext_present = impd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*loudness_info_set).loudness_info_set_ext_present == 1 as core::ffi::c_int {
        err = impd_parse_loudness_info_set_ext(it_bit_buff, loudness_info_set);
        if err != 0 {
            return err;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_gain_set_params_characteristics(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut version: WORD32,
    mut gain_params: *mut ia_gain_params_struct,
) -> WORD32 {
    let mut temp: WORD32 = 0;
    if version == 0 as core::ffi::c_int {
        (*gain_params).drc_characteristic = impd_read_bits_buf(it_bit_buff, 7 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*gain_params).drc_characteristic > 0 as core::ffi::c_int {
            (*gain_params).drc_characteristic_present = 1 as core::ffi::c_int as WORD32;
            (*gain_params).drc_characteristic_format_is_cicp = 1 as core::ffi::c_int
                as WORD32;
        } else {
            (*gain_params).drc_characteristic_present = 0 as core::ffi::c_int as WORD32;
        }
    } else {
        (*gain_params).drc_characteristic_present = impd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*gain_params).drc_characteristic_present != 0 {
            (*gain_params).drc_characteristic_format_is_cicp = impd_read_bits_buf(
                it_bit_buff,
                1 as WORD,
            );
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if (*gain_params).drc_characteristic_format_is_cicp != 0 {
                (*gain_params).drc_characteristic = impd_read_bits_buf(
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
                (*gain_params).drc_characteristic_left_index = (temp as core::ffi::c_int
                    >> 4 as core::ffi::c_int & 0xf as core::ffi::c_int) as WORD32;
                (*gain_params).drc_characteristic_right_index = (temp as core::ffi::c_int
                    & 0xf as core::ffi::c_int) as WORD32;
            }
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_loudness_measure(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut loudness_measure: *mut ia_loudness_measure_struct,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut temp: WORD32 = 0;
    (*loudness_measure).method_def = impd_read_bits_buf(it_bit_buff, 4 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    err = impd_dec_method_value(
        it_bit_buff,
        (*loudness_measure).method_def,
        &mut (*loudness_measure).method_val,
    );
    if err != 0 {
        return err;
    }
    temp = impd_read_bits_buf(it_bit_buff, 6 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    (*loudness_measure).measurement_system = (temp as core::ffi::c_int
        >> 2 as core::ffi::c_int & 0xf as core::ffi::c_int) as WORD32;
    if (*loudness_measure).measurement_system > MEASUREMENT_SYSTEM_RESERVED_E {
        return 2 as WORD32;
    }
    (*loudness_measure).reliability = (temp as core::ffi::c_int & 3 as core::ffi::c_int)
        as WORD32;
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_dec_gain_modifiers(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut version: WORD32,
    mut band_count: WORD32,
    mut pstr_gain_modifiers: *mut ia_gain_modifiers_struct,
) -> WORD32 {
    let mut sign: WORD32 = 0;
    let mut temp: WORD32 = 0;
    if version > 0 as core::ffi::c_int {
        let mut b: WORD32 = 0;
        b = 0 as core::ffi::c_int as WORD32;
        while b < band_count {
            (*pstr_gain_modifiers).target_characteristic_left_present[b as usize] = impd_read_bits_buf(
                it_bit_buff,
                1 as WORD,
            );
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if (*pstr_gain_modifiers).target_characteristic_left_present[b as usize] != 0
            {
                let mut tmp_index: WORD32 = impd_read_bits_buf(it_bit_buff, 4 as WORD);
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                if tmp_index >= SPLIT_CHARACTERISTIC_COUNT_MAX {
                    return 2 as WORD32;
                }
                (*pstr_gain_modifiers).target_characteristic_left_index[b as usize] = tmp_index;
            }
            (*pstr_gain_modifiers).target_characteristic_right_present[b as usize] = impd_read_bits_buf(
                it_bit_buff,
                1 as WORD,
            );
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if (*pstr_gain_modifiers).target_characteristic_right_present[b as usize]
                != 0
            {
                let mut tmp_index_0: WORD32 = impd_read_bits_buf(it_bit_buff, 4 as WORD);
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                if tmp_index_0 >= SPLIT_CHARACTERISTIC_COUNT_MAX {
                    return 2 as WORD32;
                }
                (*pstr_gain_modifiers).target_characteristic_right_index[b as usize] = tmp_index_0;
            }
            (*pstr_gain_modifiers).gain_scaling_flag[b as usize] = impd_read_bits_buf(
                it_bit_buff,
                1 as WORD,
            );
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if (*pstr_gain_modifiers).gain_scaling_flag[b as usize] != 0 {
                temp = impd_read_bits_buf(it_bit_buff, 8 as WORD);
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                (*pstr_gain_modifiers).attn_scaling[b as usize] = ((temp
                    as core::ffi::c_int >> 4 as core::ffi::c_int
                    & 0xf as core::ffi::c_int) as core::ffi::c_float * 0.125f32)
                    as FLOAT32;
                (*pstr_gain_modifiers).ampl_scaling[b as usize] = ((temp
                    as core::ffi::c_int & 0xf as core::ffi::c_int) as core::ffi::c_float
                    * 0.125f32) as FLOAT32;
            }
            (*pstr_gain_modifiers).gain_offset_flag[b as usize] = impd_read_bits_buf(
                it_bit_buff,
                1 as WORD,
            );
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if (*pstr_gain_modifiers).gain_offset_flag[b as usize] != 0 {
                let mut gain_offset: FLOAT32 = 0.;
                temp = impd_read_bits_buf(it_bit_buff, 6 as WORD);
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                sign = (temp as core::ffi::c_int >> 5 as core::ffi::c_int
                    & 1 as core::ffi::c_int) as WORD32;
                gain_offset = ((1 as core::ffi::c_int
                    + (temp as core::ffi::c_int & 0x1f as core::ffi::c_int))
                    as core::ffi::c_float * 0.25f32) as FLOAT32;
                if sign != 0 {
                    gain_offset = -gain_offset;
                }
                (*pstr_gain_modifiers).gain_offset[b as usize] = gain_offset;
            }
            b += 1;
        }
        if band_count == 1 as core::ffi::c_int {
            let mut tmp: WORD32 = 0;
            (*pstr_gain_modifiers).shape_filter_flag = impd_read_bits_buf(
                it_bit_buff,
                1 as WORD,
            );
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if (*pstr_gain_modifiers).shape_filter_flag != 0 {
                tmp = impd_read_bits_buf(it_bit_buff, 4 as WORD);
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                if tmp >= SHAPE_FILTER_COUNT_MAX + 1 as core::ffi::c_int {
                    return UNEXPECTED_ERROR;
                }
                (*pstr_gain_modifiers).shape_filter_idx = tmp;
            }
        }
    } else if version == 0 as core::ffi::c_int {
        let mut b_0: WORD32 = 0;
        let mut gain_scaling_flag: WORD32 = 0;
        let mut gain_offset_flag: WORD32 = 0;
        let mut attn_scaling: FLOAT32 = 1.0f32;
        let mut ampl_scaling: FLOAT32 = 1.0f32;
        let mut gain_offset_0: FLOAT32 = 0.0f32;
        gain_scaling_flag = impd_read_bits_buf(it_bit_buff, 1 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if gain_scaling_flag != 0 {
            temp = impd_read_bits_buf(it_bit_buff, 8 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            attn_scaling = ((temp as core::ffi::c_int >> 4 as core::ffi::c_int
                & 0xf as core::ffi::c_int) as core::ffi::c_float * 0.125f32) as FLOAT32;
            ampl_scaling = ((temp as core::ffi::c_int & 0xf as core::ffi::c_int)
                as core::ffi::c_float * 0.125f32) as FLOAT32;
        }
        gain_offset_flag = impd_read_bits_buf(it_bit_buff, 1 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if gain_offset_flag != 0 {
            temp = impd_read_bits_buf(it_bit_buff, 6 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            sign = (temp as core::ffi::c_int >> 5 as core::ffi::c_int
                & 1 as core::ffi::c_int) as WORD32;
            gain_offset_0 = ((1 as core::ffi::c_int
                + (temp as core::ffi::c_int & 0x1f as core::ffi::c_int))
                as core::ffi::c_float * 0.25f32) as FLOAT32;
            if sign != 0 {
                gain_offset_0 = -gain_offset_0;
            }
        }
        b_0 = 0 as core::ffi::c_int as WORD32;
        while b_0 < band_count {
            (*pstr_gain_modifiers).target_characteristic_left_present[b_0 as usize] = 0
                as core::ffi::c_int as WORD32;
            (*pstr_gain_modifiers).target_characteristic_right_present[b_0 as usize] = 0
                as core::ffi::c_int as WORD32;
            (*pstr_gain_modifiers).gain_scaling_flag[b_0 as usize] = gain_scaling_flag;
            (*pstr_gain_modifiers).attn_scaling[b_0 as usize] = attn_scaling;
            (*pstr_gain_modifiers).ampl_scaling[b_0 as usize] = ampl_scaling;
            (*pstr_gain_modifiers).gain_offset_flag[b_0 as usize] = gain_offset_flag;
            (*pstr_gain_modifiers).gain_offset[b_0 as usize] = gain_offset_0;
            b_0 += 1;
        }
        (*pstr_gain_modifiers).shape_filter_flag = 0 as core::ffi::c_int as WORD32;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_gain_set_params(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut version: WORD32,
    mut gain_seq_idx: *mut WORD32,
    mut gain_set_params: *mut ia_gain_set_params_struct,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut i: WORD32 = 0;
    let mut temp: WORD32 = 0;
    temp = impd_read_bits_buf(it_bit_buff, 6 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    (*gain_set_params).gain_coding_profile = (temp as core::ffi::c_int
        >> 4 as core::ffi::c_int & 3 as core::ffi::c_int) as WORD32;
    (*gain_set_params).gain_interpolation_type = (temp as core::ffi::c_int
        >> 3 as core::ffi::c_int & 1 as core::ffi::c_int) as WORD32;
    (*gain_set_params).full_frame = (temp as core::ffi::c_int >> 2 as core::ffi::c_int
        & 1 as core::ffi::c_int) as WORD32;
    (*gain_set_params).time_alignment = (temp as core::ffi::c_int
        >> 1 as core::ffi::c_int & 1 as core::ffi::c_int) as WORD32;
    (*gain_set_params).time_delt_min_flag = (temp as core::ffi::c_int
        & 1 as core::ffi::c_int) as WORD32;
    if (*gain_set_params).time_delt_min_flag != 0 {
        let mut time_delta_min: WORD32 = 0;
        time_delta_min = impd_read_bits_buf(it_bit_buff, 11 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*gain_set_params).time_delt_min_val = (time_delta_min as core::ffi::c_int
            + 1 as core::ffi::c_int) as WORD32;
    }
    if (*gain_set_params).gain_coding_profile == GAIN_CODING_PROFILE_CONSTANT {
        (*gain_set_params).band_count = 1 as core::ffi::c_int as WORD32;
        *gain_seq_idx = (*gain_seq_idx + 1 as core::ffi::c_int) as WORD32;
    } else {
        (*gain_set_params).band_count = impd_read_bits_buf(it_bit_buff, 4 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*gain_set_params).band_count > BAND_COUNT_MAX {
            return 2 as WORD32;
        }
        if (*gain_set_params).band_count > 1 as core::ffi::c_int {
            (*gain_set_params).drc_band_type = impd_read_bits_buf(
                it_bit_buff,
                1 as WORD,
            );
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*gain_set_params).band_count {
            if version == 0 as core::ffi::c_int {
                *gain_seq_idx = (*gain_seq_idx + 1 as core::ffi::c_int) as WORD32;
            } else {
                let mut indexPresent: WORD32 = 0;
                indexPresent = impd_read_bits_buf(it_bit_buff, 1 as WORD);
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                if indexPresent != 0 {
                    let mut bsIndex: WORD32 = 0;
                    bsIndex = impd_read_bits_buf(it_bit_buff, 6 as WORD);
                    if (*it_bit_buff).error != 0 {
                        return (*it_bit_buff).error;
                    }
                    *gain_seq_idx = bsIndex;
                } else {
                    *gain_seq_idx = (*gain_seq_idx + 1 as core::ffi::c_int) as WORD32;
                }
            }
            if *gain_seq_idx >= SEQUENCE_COUNT_MAX {
                return UNEXPECTED_ERROR;
            }
            (*gain_set_params).gain_params[i as usize].gain_seq_idx = *gain_seq_idx;
            err = impd_parse_gain_set_params_characteristics(
                it_bit_buff,
                version,
                &mut *((*gain_set_params).gain_params).as_mut_ptr().offset(i as isize),
            );
            if err != 0 {
                return err;
            }
            i += 1;
        }
        if (*gain_set_params).drc_band_type != 0 {
            i = 1 as core::ffi::c_int as WORD32;
            while i < (*gain_set_params).band_count {
                (*gain_set_params).gain_params[i as usize].crossover_freq_idx = impd_read_bits_buf(
                    it_bit_buff,
                    4 as WORD,
                );
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                i += 1;
            }
        } else {
            i = 1 as core::ffi::c_int as WORD32;
            while i < (*gain_set_params).band_count {
                (*gain_set_params).gain_params[i as usize].start_subband_index = impd_read_bits_buf(
                    it_bit_buff,
                    10 as WORD,
                );
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                i += 1;
            }
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_sel_drc_coeff(
    mut drc_config: *mut ia_drc_config,
    mut location: WORD32,
    mut str_p_loc_drc_coefficients_uni_drc: *mut *mut ia_uni_drc_coeffs_struct,
) -> WORD32 {
    let mut n: WORD32 = 0;
    let mut c1: WORD32 = -(1 as WORD32);
    let mut c0: WORD32 = -(1 as WORD32);
    n = 0 as core::ffi::c_int as WORD32;
    while n < (*drc_config).drc_coefficients_drc_count {
        if (*drc_config).str_p_loc_drc_coefficients_uni_drc[n as usize].drc_location
            == location
        {
            if (*drc_config).str_p_loc_drc_coefficients_uni_drc[n as usize].version
                == 0 as core::ffi::c_int
            {
                c0 = n;
            } else {
                c1 = n;
            }
        }
        n += 1;
    }
    if c1 >= 0 as core::ffi::c_int {
        *str_p_loc_drc_coefficients_uni_drc = &mut *((*drc_config)
            .str_p_loc_drc_coefficients_uni_drc)
            .as_mut_ptr()
            .offset(c1 as isize) as *mut ia_uni_drc_coeffs_struct;
    } else if c0 >= 0 as core::ffi::c_int {
        *str_p_loc_drc_coefficients_uni_drc = &mut *((*drc_config)
            .str_p_loc_drc_coefficients_uni_drc)
            .as_mut_ptr()
            .offset(c0 as isize) as *mut ia_uni_drc_coeffs_struct;
    } else {
        *str_p_loc_drc_coefficients_uni_drc = 0 as *mut ia_uni_drc_coeffs_struct;
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_loudness_info_set_ext(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut loudness_info_set: *mut ia_drc_loudness_info_set_struct,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut i: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut bit_size_len: WORD32 = 0;
    let mut ext_size_bits: WORD32 = 0;
    let mut bit_size: WORD32 = 0;
    k = 0 as core::ffi::c_int as WORD32;
    (*loudness_info_set)
        .str_loudness_info_set_ext
        .loudness_info_set_ext_type[k as usize] = impd_read_bits_buf(
        it_bit_buff,
        4 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    while (*loudness_info_set)
        .str_loudness_info_set_ext
        .loudness_info_set_ext_type[k as usize] != UNIDRCLOUDEXT_TERM
    {
        bit_size_len = impd_read_bits_buf(it_bit_buff, 4 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        ext_size_bits = (bit_size_len as core::ffi::c_int + 4 as core::ffi::c_int)
            as WORD32;
        bit_size = impd_read_bits_buf(it_bit_buff, ext_size_bits as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if k >= EXT_COUNT_MAX - 1 as core::ffi::c_int {
            return UNEXPECTED_ERROR;
        }
        (*loudness_info_set).str_loudness_info_set_ext.ext_bit_size[k as usize] = (bit_size
            as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
        match (*loudness_info_set)
            .str_loudness_info_set_ext
            .loudness_info_set_ext_type[k as usize]
        {
            UNIDRCLOUDEXT_EQ => {
                err = impd_parse_loud_info_set_ext_eq(it_bit_buff, loudness_info_set);
                if err != 0 {
                    return err;
                }
            }
            _ => {
                i = 0 as core::ffi::c_int as WORD32;
                while i
                    < (*loudness_info_set)
                        .str_loudness_info_set_ext
                        .ext_bit_size[k as usize]
                {
                    impd_read_bits_buf(it_bit_buff, 1 as WORD);
                    if (*it_bit_buff).error != 0 {
                        return (*it_bit_buff).error;
                    }
                    i += 1;
                }
            }
        }
        k += 1;
        (*loudness_info_set)
            .str_loudness_info_set_ext
            .loudness_info_set_ext_type[k as usize] = impd_read_bits_buf(
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
pub unsafe extern "C" fn impd_drc_parse_coeff(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut version: WORD32,
    mut ia_drc_params_struct: *mut ia_drc_params_bs_dec_struct,
    mut str_p_loc_drc_coefficients_uni_drc: *mut ia_uni_drc_coeffs_struct,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut i: WORD32 = 0;
    let mut drc_frame_size: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut gain_seq_idx: WORD32 = -(1 as WORD32);
    (*str_p_loc_drc_coefficients_uni_drc).version = version;
    if version == 0 as core::ffi::c_int {
        let mut gain_sequence_count: WORD32 = 0 as WORD32;
        temp = impd_read_bits_buf(it_bit_buff, 5 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*str_p_loc_drc_coefficients_uni_drc).drc_location = (temp as core::ffi::c_int
            >> 1 as core::ffi::c_int & 0xf as core::ffi::c_int) as WORD32;
        (*str_p_loc_drc_coefficients_uni_drc).drc_frame_size_present = (temp
            as core::ffi::c_int & 1 as core::ffi::c_int) as WORD32;
        if (*str_p_loc_drc_coefficients_uni_drc).drc_frame_size_present
            == 1 as core::ffi::c_int
        {
            drc_frame_size = impd_read_bits_buf(it_bit_buff, 15 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            (*str_p_loc_drc_coefficients_uni_drc).drc_frame_size = (drc_frame_size
                as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
            if (*str_p_loc_drc_coefficients_uni_drc).drc_frame_size > MAX_DRC_FRAME_SIZE
            {
                return UNEXPECTED_ERROR;
            }
        }
        (*str_p_loc_drc_coefficients_uni_drc).drc_characteristic_left_present = 0
            as core::ffi::c_int as WORD32;
        (*str_p_loc_drc_coefficients_uni_drc).drc_characteristic_right_present = 0
            as core::ffi::c_int as WORD32;
        (*str_p_loc_drc_coefficients_uni_drc).shape_filters_present = 0
            as core::ffi::c_int as WORD32;
        (*str_p_loc_drc_coefficients_uni_drc).gain_set_count = impd_read_bits_buf(
            it_bit_buff,
            6 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*str_p_loc_drc_coefficients_uni_drc).gain_set_count > GAIN_SET_COUNT_MAX {
            return 2 as WORD32;
        }
        (*str_p_loc_drc_coefficients_uni_drc).gain_set_count_plus = (*str_p_loc_drc_coefficients_uni_drc)
            .gain_set_count;
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*str_p_loc_drc_coefficients_uni_drc).gain_set_count {
            err = impd_parse_gain_set_params(
                it_bit_buff,
                version,
                &mut gain_seq_idx,
                &mut *((*str_p_loc_drc_coefficients_uni_drc).gain_set_params)
                    .as_mut_ptr()
                    .offset(i as isize),
            );
            if err != 0 {
                return err;
            }
            if (*str_p_loc_drc_coefficients_uni_drc)
                .gain_set_params[i as usize]
                .time_delt_min_flag != 0
            {
                if (*str_p_loc_drc_coefficients_uni_drc)
                    .gain_set_params[i as usize]
                    .time_delt_min_val > (*ia_drc_params_struct).drc_frame_size
                {
                    return 3 as WORD32;
                }
                (*str_p_loc_drc_coefficients_uni_drc)
                    .gain_set_params[i as usize]
                    .num_gain_max_values = (*ia_drc_params_struct).drc_frame_size
                    / (*str_p_loc_drc_coefficients_uni_drc)
                        .gain_set_params[i as usize]
                        .time_delt_min_val;
                if (*str_p_loc_drc_coefficients_uni_drc)
                    .gain_set_params[i as usize]
                    .num_gain_max_values
                    > N_DELTA_TIME_CODE_TABLE_ENTRIES_MAX / 2 as core::ffi::c_int
                        - 1 as core::ffi::c_int
                {
                    return 2 as WORD32;
                }
                impd_init_tbls(
                    (*str_p_loc_drc_coefficients_uni_drc)
                        .gain_set_params[i as usize]
                        .num_gain_max_values,
                    &mut (*((*str_p_loc_drc_coefficients_uni_drc).gain_set_params)
                        .as_mut_ptr()
                        .offset(i as isize))
                        .str_tables,
                );
            }
            gain_sequence_count
                += (*str_p_loc_drc_coefficients_uni_drc)
                    .gain_set_params[i as usize]
                    .band_count;
            i += 1;
        }
        (*str_p_loc_drc_coefficients_uni_drc).gain_sequence_count = gain_sequence_count;
    } else {
        let mut pstr_shape_filter_block_params: *mut ia_shape_filter_block_params_struct = 0
            as *mut ia_shape_filter_block_params_struct;
        i = 0 as core::ffi::c_int as WORD32;
        while i < SEQUENCE_COUNT_MAX {
            (*str_p_loc_drc_coefficients_uni_drc)
                .gain_set_params_index_for_gain_sequence[i as usize] = -(1
                as core::ffi::c_int) as WORD32;
            i += 1;
        }
        temp = impd_read_bits_buf(it_bit_buff, 5 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*str_p_loc_drc_coefficients_uni_drc).drc_location = (temp as core::ffi::c_int
            >> 1 as core::ffi::c_int & 0xf as core::ffi::c_int) as WORD32;
        (*str_p_loc_drc_coefficients_uni_drc).drc_frame_size_present = (temp
            as core::ffi::c_int & 1 as core::ffi::c_int) as WORD32;
        if (*str_p_loc_drc_coefficients_uni_drc).drc_frame_size_present
            == 1 as core::ffi::c_int
        {
            drc_frame_size = impd_read_bits_buf(it_bit_buff, 15 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            (*str_p_loc_drc_coefficients_uni_drc).drc_frame_size = (drc_frame_size
                as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
        }
        (*str_p_loc_drc_coefficients_uni_drc).drc_characteristic_left_present = impd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*str_p_loc_drc_coefficients_uni_drc).drc_characteristic_left_present
            == 1 as core::ffi::c_int
        {
            (*str_p_loc_drc_coefficients_uni_drc).characteristic_left_count = impd_read_bits_buf(
                it_bit_buff,
                4 as WORD,
            );
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if (*str_p_loc_drc_coefficients_uni_drc).characteristic_left_count
                > SPLIT_CHARACTERISTIC_COUNT_MAX
            {
                return 2 as WORD32;
            }
            i = 1 as core::ffi::c_int as WORD32;
            while i <= (*str_p_loc_drc_coefficients_uni_drc).characteristic_left_count {
                err = impd_parse_split_drc_characteristic(
                    it_bit_buff,
                    LEFT_SIDE,
                    &mut *((*str_p_loc_drc_coefficients_uni_drc)
                        .str_split_characteristic_left)
                        .as_mut_ptr()
                        .offset(i as isize),
                );
                if err != 0 {
                    return err;
                }
                i += 1;
            }
        }
        (*str_p_loc_drc_coefficients_uni_drc).drc_characteristic_right_present = impd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*str_p_loc_drc_coefficients_uni_drc).drc_characteristic_right_present
            == 1 as core::ffi::c_int
        {
            (*str_p_loc_drc_coefficients_uni_drc).characteristic_right_count = impd_read_bits_buf(
                it_bit_buff,
                4 as WORD,
            );
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if (*str_p_loc_drc_coefficients_uni_drc).characteristic_right_count
                > SPLIT_CHARACTERISTIC_COUNT_MAX
            {
                return 2 as WORD32;
            }
            i = 1 as core::ffi::c_int as WORD32;
            while i <= (*str_p_loc_drc_coefficients_uni_drc).characteristic_right_count {
                err = impd_parse_split_drc_characteristic(
                    it_bit_buff,
                    RIGHT_SIDE,
                    &mut *((*str_p_loc_drc_coefficients_uni_drc)
                        .str_split_characteristic_right)
                        .as_mut_ptr()
                        .offset(i as isize),
                );
                if err != 0 {
                    return err;
                }
                i += 1;
            }
        }
        (*str_p_loc_drc_coefficients_uni_drc).shape_filters_present = impd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*str_p_loc_drc_coefficients_uni_drc).shape_filters_present
            == 1 as core::ffi::c_int
        {
            (*str_p_loc_drc_coefficients_uni_drc).shape_num_filter = impd_read_bits_buf(
                it_bit_buff,
                4 as WORD,
            );
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if (*str_p_loc_drc_coefficients_uni_drc).shape_num_filter
                > SHAPE_FILTER_COUNT_MAX
            {
                return 2 as WORD32;
            }
            i = 1 as core::ffi::c_int as WORD32;
            while i <= (*str_p_loc_drc_coefficients_uni_drc).shape_num_filter {
                pstr_shape_filter_block_params = &mut *((*str_p_loc_drc_coefficients_uni_drc)
                    .str_shape_filter_block_params)
                    .as_mut_ptr()
                    .offset(i as isize) as *mut ia_shape_filter_block_params_struct;
                (*pstr_shape_filter_block_params).lf_cut_filter_present = impd_read_bits_buf(
                    it_bit_buff,
                    1 as WORD,
                );
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                if (*pstr_shape_filter_block_params).lf_cut_filter_present
                    == 1 as core::ffi::c_int
                {
                    temp = impd_read_bits_buf(it_bit_buff, 5 as WORD);
                    if (*it_bit_buff).error != 0 {
                        return (*it_bit_buff).error;
                    }
                    (*pstr_shape_filter_block_params)
                        .str_lf_cut_params
                        .corner_freq_index = (temp as core::ffi::c_int
                        >> 2 as core::ffi::c_int & 7 as core::ffi::c_int) as WORD32;
                    (*pstr_shape_filter_block_params)
                        .str_lf_cut_params
                        .filter_strength_index = (temp as core::ffi::c_int
                        & 3 as core::ffi::c_int) as WORD32;
                }
                (*pstr_shape_filter_block_params).lf_boost_filter_present = impd_read_bits_buf(
                    it_bit_buff,
                    1 as WORD,
                );
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                if (*pstr_shape_filter_block_params).lf_boost_filter_present
                    == 1 as core::ffi::c_int
                {
                    temp = impd_read_bits_buf(it_bit_buff, 5 as WORD);
                    if (*it_bit_buff).error != 0 {
                        return (*it_bit_buff).error;
                    }
                    (*pstr_shape_filter_block_params)
                        .str_lf_boost_params
                        .corner_freq_index = (temp as core::ffi::c_int
                        >> 2 as core::ffi::c_int & 7 as core::ffi::c_int) as WORD32;
                    (*pstr_shape_filter_block_params)
                        .str_lf_boost_params
                        .filter_strength_index = (temp as core::ffi::c_int
                        & 3 as core::ffi::c_int) as WORD32;
                }
                (*pstr_shape_filter_block_params).hf_cut_filter_present = impd_read_bits_buf(
                    it_bit_buff,
                    1 as WORD,
                );
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                if (*pstr_shape_filter_block_params).hf_cut_filter_present
                    == 1 as core::ffi::c_int
                {
                    temp = impd_read_bits_buf(it_bit_buff, 5 as WORD);
                    if (*it_bit_buff).error != 0 {
                        return (*it_bit_buff).error;
                    }
                    (*pstr_shape_filter_block_params)
                        .str_hfCutParams
                        .corner_freq_index = (temp as core::ffi::c_int
                        >> 2 as core::ffi::c_int & 7 as core::ffi::c_int) as WORD32;
                    (*pstr_shape_filter_block_params)
                        .str_hfCutParams
                        .filter_strength_index = (temp as core::ffi::c_int
                        & 3 as core::ffi::c_int) as WORD32;
                }
                (*pstr_shape_filter_block_params).hf_boost_filter_present = impd_read_bits_buf(
                    it_bit_buff,
                    1 as WORD,
                );
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                if (*pstr_shape_filter_block_params).hf_boost_filter_present
                    == 1 as core::ffi::c_int
                {
                    temp = impd_read_bits_buf(it_bit_buff, 5 as WORD);
                    if (*it_bit_buff).error != 0 {
                        return (*it_bit_buff).error;
                    }
                    (*pstr_shape_filter_block_params)
                        .str_hf_boost_params
                        .corner_freq_index = (temp as core::ffi::c_int
                        >> 2 as core::ffi::c_int & 7 as core::ffi::c_int) as WORD32;
                    (*pstr_shape_filter_block_params)
                        .str_hf_boost_params
                        .filter_strength_index = (temp as core::ffi::c_int
                        & 3 as core::ffi::c_int) as WORD32;
                }
                i += 1;
            }
        }
        temp = impd_read_bits_buf(it_bit_buff, 12 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*str_p_loc_drc_coefficients_uni_drc).gain_sequence_count = (temp
            as core::ffi::c_int >> 6 as core::ffi::c_int & 0x3f as core::ffi::c_int)
            as WORD32;
        if (*str_p_loc_drc_coefficients_uni_drc).gain_sequence_count > SEQUENCE_COUNT_MAX
        {
            return UNEXPECTED_ERROR;
        }
        (*str_p_loc_drc_coefficients_uni_drc).gain_set_count = (temp as core::ffi::c_int
            & 0x3f as core::ffi::c_int) as WORD32;
        if (*str_p_loc_drc_coefficients_uni_drc).gain_set_count > GAIN_SET_COUNT_MAX {
            return 2 as WORD32;
        }
        (*str_p_loc_drc_coefficients_uni_drc).gain_set_count_plus = (*str_p_loc_drc_coefficients_uni_drc)
            .gain_set_count;
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*str_p_loc_drc_coefficients_uni_drc).gain_set_count {
            err = impd_parse_gain_set_params(
                it_bit_buff,
                version,
                &mut gain_seq_idx,
                &mut *((*str_p_loc_drc_coefficients_uni_drc).gain_set_params)
                    .as_mut_ptr()
                    .offset(i as isize),
            );
            if err != 0 {
                return err;
            }
            if (*str_p_loc_drc_coefficients_uni_drc)
                .gain_set_params[i as usize]
                .time_delt_min_flag != 0
            {
                if (*str_p_loc_drc_coefficients_uni_drc)
                    .gain_set_params[i as usize]
                    .time_delt_min_val > (*ia_drc_params_struct).drc_frame_size
                {
                    return 3 as WORD32;
                }
                (*str_p_loc_drc_coefficients_uni_drc)
                    .gain_set_params[i as usize]
                    .num_gain_max_values = (*ia_drc_params_struct).drc_frame_size
                    / (*str_p_loc_drc_coefficients_uni_drc)
                        .gain_set_params[i as usize]
                        .time_delt_min_val;
                if (*str_p_loc_drc_coefficients_uni_drc)
                    .gain_set_params[i as usize]
                    .num_gain_max_values
                    > N_DELTA_TIME_CODE_TABLE_ENTRIES_MAX / 2 as core::ffi::c_int
                        - 1 as core::ffi::c_int
                {
                    return 2 as WORD32;
                }
                impd_init_tbls(
                    (*str_p_loc_drc_coefficients_uni_drc)
                        .gain_set_params[i as usize]
                        .num_gain_max_values,
                    &mut (*((*str_p_loc_drc_coefficients_uni_drc).gain_set_params)
                        .as_mut_ptr()
                        .offset(i as isize))
                        .str_tables,
                );
            }
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < (*str_p_loc_drc_coefficients_uni_drc).gain_set_count {
            let mut b: WORD32 = 0;
            b = 0 as core::ffi::c_int as WORD32;
            while b
                < (*str_p_loc_drc_coefficients_uni_drc)
                    .gain_set_params[i as usize]
                    .band_count
            {
                (*str_p_loc_drc_coefficients_uni_drc)
                    .gain_set_params_index_for_gain_sequence[(*str_p_loc_drc_coefficients_uni_drc)
                    .gain_set_params[i as usize]
                    .gain_params[b as usize]
                    .gain_seq_idx as usize] = i;
                b += 1;
            }
            i += 1;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_drc_parse_instructions_basic(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut str_drc_instructions_basic: *mut ia_drc_instructions_basic_struct,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut limiter_peak_target: WORD32 = 0;
    let mut temp: WORD32 = 0;
    let mut additional_dmix_id_present: WORD32 = 0;
    let mut additional_dmix_id_cnt: WORD32 = 0;
    temp = impd_read_bits_buf(it_bit_buff, 18 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    (*str_drc_instructions_basic).drc_set_id = (temp as core::ffi::c_int
        >> 12 as core::ffi::c_int & 0x3f as core::ffi::c_int) as WORD32;
    (*str_drc_instructions_basic).drc_location = (temp as core::ffi::c_int
        >> 8 as core::ffi::c_int & 0xf as core::ffi::c_int) as WORD32;
    (*str_drc_instructions_basic).downmix_id[0 as core::ffi::c_int as usize] = (temp
        as core::ffi::c_int >> 1 as core::ffi::c_int & 0x7f as core::ffi::c_int)
        as WORD32;
    additional_dmix_id_present = (temp as core::ffi::c_int & 1 as core::ffi::c_int)
        as WORD32;
    (*str_drc_instructions_basic).dwnmix_id_count = 1 as core::ffi::c_int as WORD32;
    if additional_dmix_id_present != 0 {
        additional_dmix_id_cnt = impd_read_bits_buf(it_bit_buff, 3 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < additional_dmix_id_cnt {
            (*str_drc_instructions_basic)
                .downmix_id[(i as core::ffi::c_int + 1 as core::ffi::c_int) as usize] = impd_read_bits_buf(
                it_bit_buff,
                7 as WORD,
            );
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            i += 1;
        }
        (*str_drc_instructions_basic).dwnmix_id_count = 1 as WORD32
            + additional_dmix_id_cnt;
    }
    (*str_drc_instructions_basic).drc_set_effect = impd_read_bits_buf(
        it_bit_buff,
        16 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*str_drc_instructions_basic).drc_set_effect as core::ffi::c_int
        & (EFFECT_BIT_DUCK_OTHER | EFFECT_BIT_DUCK_SELF) == 0 as core::ffi::c_int
    {
        (*str_drc_instructions_basic).limiter_peak_target_present = impd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*str_drc_instructions_basic).limiter_peak_target_present != 0 {
            limiter_peak_target = impd_read_bits_buf(it_bit_buff, 8 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            (*str_drc_instructions_basic).limiter_peak_target = (-limiter_peak_target
                as core::ffi::c_float * 0.125f32) as FLOAT32;
        }
    }
    (*str_drc_instructions_basic).drc_set_target_loudness_present = impd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    (*str_drc_instructions_basic).drc_set_target_loudness_value_upper = 0
        as core::ffi::c_int as WORD32;
    (*str_drc_instructions_basic).drc_set_target_loudness_value_lower = -(63
        as core::ffi::c_int) as WORD32;
    if (*str_drc_instructions_basic).drc_set_target_loudness_present
        == 1 as core::ffi::c_int
    {
        let mut bsDrcSetTargetLoudnessValueUpper: WORD32 = 0;
        let mut bsDrcSetTargetLoudnessValueLower: WORD32 = 0;
        bsDrcSetTargetLoudnessValueUpper = impd_read_bits_buf(it_bit_buff, 6 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*str_drc_instructions_basic).drc_set_target_loudness_value_upper = (bsDrcSetTargetLoudnessValueUpper
            as core::ffi::c_int - 63 as core::ffi::c_int) as WORD32;
        (*str_drc_instructions_basic).drc_set_target_loudness_value_lower_present = impd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*str_drc_instructions_basic).drc_set_target_loudness_value_lower_present
            == 1 as core::ffi::c_int
        {
            bsDrcSetTargetLoudnessValueLower = impd_read_bits_buf(
                it_bit_buff,
                6 as WORD,
            );
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            (*str_drc_instructions_basic).drc_set_target_loudness_value_lower = (bsDrcSetTargetLoudnessValueLower
                as core::ffi::c_int - 63 as core::ffi::c_int) as WORD32;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_dec_ducking_scaling(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut ducking_scaling_flag: *mut WORD32,
    mut p_ducking_scaling: *mut FLOAT32,
) -> WORD32 {
    let mut ducking_scaling_present: WORD32 = 0;
    let mut ducking_scaling: WORD32 = 0;
    let mut sigma: WORD32 = 0;
    let mut mu: WORD32 = 0;
    ducking_scaling_present = impd_read_bits_buf(it_bit_buff, 1 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if ducking_scaling_present == 0 as core::ffi::c_int {
        *ducking_scaling_flag = 0 as core::ffi::c_int as WORD32;
        *p_ducking_scaling = 1.0f32 as FLOAT32;
    } else {
        *ducking_scaling_flag = 1 as core::ffi::c_int as WORD32;
        ducking_scaling = impd_read_bits_buf(it_bit_buff, 4 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        sigma = ducking_scaling >> 3 as core::ffi::c_int;
        mu = (ducking_scaling as core::ffi::c_int & 0x7 as core::ffi::c_int) as WORD32;
        if sigma == 0 as core::ffi::c_int {
            *p_ducking_scaling = (1.0f32
                + 0.125f32 * (1.0f32 + mu as core::ffi::c_float)) as FLOAT32;
        } else {
            *p_ducking_scaling = (1.0f32
                - 0.125f32 * (1.0f32 + mu as core::ffi::c_float)) as FLOAT32;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_drc_instructions_uni_drc(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut version: WORD32,
    mut drc_config: *mut ia_drc_config,
    mut str_drc_instruction_str: *mut ia_drc_instructions_struct,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut i: WORD32 = 0;
    let mut n: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut g: WORD32 = 0;
    let mut c: WORD32 = 0;
    let mut limiter_peak_target: WORD32 = 0;
    let mut idx: WORD32 = 0;
    let mut additional_dmix_id_present: WORD32 = 0;
    let mut additional_dmix_id_cnt: WORD32 = 0;
    let mut str_p_loc_drc_coefficients_uni_drc: *mut ia_uni_drc_coeffs_struct = 0
        as *mut ia_uni_drc_coeffs_struct;
    let mut ch_cnt: WORD32 = 0;
    let mut unique_idx: [WORD32; 8] = [0; 8];
    let mut unique_scaling: [FLOAT32; 8] = [0.; 8];
    let mut match_0: WORD32 = 0;
    let mut dmix_id_present: WORD32 = 0;
    let mut repeat_parameters: WORD32 = 0;
    let mut repeat_parameters_cnt: WORD32 = 0;
    let mut ducking_sequence: WORD32 = 0;
    let mut factor: FLOAT32 = 0.;
    (*str_drc_instruction_str).drc_set_id = impd_read_bits_buf(it_bit_buff, 6 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*str_drc_instruction_str).drc_set_id >= DRC_INSTRUCTIONS_COUNT_MAX {
        return UNEXPECTED_ERROR;
    }
    if version == 0 as core::ffi::c_int {
        (*str_drc_instruction_str).drc_set_complexity_level = DRC_COMPLEXITY_LEVEL_MAX
            as WORD32;
    } else {
        (*str_drc_instruction_str).drc_set_complexity_level = impd_read_bits_buf(
            it_bit_buff,
            4 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
    }
    (*str_drc_instruction_str).drc_location = impd_read_bits_buf(it_bit_buff, 4 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    dmix_id_present = 1 as core::ffi::c_int as WORD32;
    if version >= 1 as core::ffi::c_int {
        dmix_id_present = impd_read_bits_buf(it_bit_buff, 1 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
    }
    if dmix_id_present == 1 as core::ffi::c_int {
        (*str_drc_instruction_str).downmix_id[0 as core::ffi::c_int as usize] = impd_read_bits_buf(
            it_bit_buff,
            7 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if version >= 1 as core::ffi::c_int {
            (*str_drc_instruction_str).drc_apply_to_dwnmix = impd_read_bits_buf(
                it_bit_buff,
                1 as WORD,
            );
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
        }
        if version == 0 as core::ffi::c_int {
            if (*str_drc_instruction_str).downmix_id[0 as core::ffi::c_int as usize]
                == 0 as core::ffi::c_int
            {
                (*str_drc_instruction_str).drc_apply_to_dwnmix = 0 as core::ffi::c_int
                    as WORD32;
            } else {
                (*str_drc_instruction_str).drc_apply_to_dwnmix = 1 as core::ffi::c_int
                    as WORD32;
            }
        }
        additional_dmix_id_present = impd_read_bits_buf(it_bit_buff, 1 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if additional_dmix_id_present != 0 {
            additional_dmix_id_cnt = impd_read_bits_buf(it_bit_buff, 3 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            i = 0 as core::ffi::c_int as WORD32;
            while i < additional_dmix_id_cnt {
                (*str_drc_instruction_str)
                    .downmix_id[(i as core::ffi::c_int + 1 as core::ffi::c_int)
                    as usize] = impd_read_bits_buf(it_bit_buff, 7 as WORD);
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                i += 1;
            }
            (*str_drc_instruction_str).dwnmix_id_count = 1 as WORD32
                + additional_dmix_id_cnt;
        } else {
            (*str_drc_instruction_str).dwnmix_id_count = 1 as core::ffi::c_int as WORD32;
        }
    } else {
        (*str_drc_instruction_str).downmix_id[0 as core::ffi::c_int as usize] = 0
            as core::ffi::c_int as WORD32;
        (*str_drc_instruction_str).dwnmix_id_count = 1 as core::ffi::c_int as WORD32;
    }
    (*str_drc_instruction_str).drc_set_effect = impd_read_bits_buf(
        it_bit_buff,
        16 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*str_drc_instruction_str).drc_set_effect as core::ffi::c_int
        & (EFFECT_BIT_DUCK_OTHER | EFFECT_BIT_DUCK_SELF) == 0 as core::ffi::c_int
    {
        (*str_drc_instruction_str).limiter_peak_target_present = impd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*str_drc_instruction_str).limiter_peak_target_present != 0 {
            limiter_peak_target = impd_read_bits_buf(it_bit_buff, 8 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            (*str_drc_instruction_str).limiter_peak_target = (-limiter_peak_target
                as core::ffi::c_float * 0.125f32) as FLOAT32;
        }
    }
    (*str_drc_instruction_str).drc_set_target_loudness_present = impd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    (*str_drc_instruction_str).drc_set_target_loudness_value_upper = 0
        as core::ffi::c_int as WORD32;
    (*str_drc_instruction_str).drc_set_target_loudness_value_lower = -(63
        as core::ffi::c_int) as WORD32;
    if (*str_drc_instruction_str).drc_set_target_loudness_present
        == 1 as core::ffi::c_int
    {
        let mut bsDrcSetTargetLoudnessValueUpper: WORD32 = 0;
        let mut bsDrcSetTargetLoudnessValueLower: WORD32 = 0;
        bsDrcSetTargetLoudnessValueUpper = impd_read_bits_buf(it_bit_buff, 6 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*str_drc_instruction_str).drc_set_target_loudness_value_upper = (bsDrcSetTargetLoudnessValueUpper
            as core::ffi::c_int - 63 as core::ffi::c_int) as WORD32;
        (*str_drc_instruction_str).drc_set_target_loudness_value_lower_present = impd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if (*str_drc_instruction_str).drc_set_target_loudness_value_lower_present
            == 1 as core::ffi::c_int
        {
            bsDrcSetTargetLoudnessValueLower = impd_read_bits_buf(
                it_bit_buff,
                6 as WORD,
            );
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            (*str_drc_instruction_str).drc_set_target_loudness_value_lower = (bsDrcSetTargetLoudnessValueLower
                as core::ffi::c_int - 63 as core::ffi::c_int) as WORD32;
        }
    }
    (*str_drc_instruction_str).depends_on_drc_set_present = impd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    (*str_drc_instruction_str).no_independent_use = 0 as core::ffi::c_int as WORD32;
    if (*str_drc_instruction_str).depends_on_drc_set_present != 0 {
        (*str_drc_instruction_str).depends_on_drc_set = impd_read_bits_buf(
            it_bit_buff,
            6 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
    } else {
        (*str_drc_instruction_str).no_independent_use = impd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
    }
    if version == 0 as core::ffi::c_int {
        (*str_drc_instruction_str).requires_eq = 0 as core::ffi::c_int as WORD32;
    } else {
        (*str_drc_instruction_str).requires_eq = impd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        );
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
    }
    err = impd_sel_drc_coeff(
        drc_config,
        (*str_drc_instruction_str).drc_location,
        &mut str_p_loc_drc_coefficients_uni_drc,
    );
    if err != 0 {
        return err;
    }
    ch_cnt = (*drc_config).channel_layout.base_channel_count;
    if ch_cnt > MAX_CHANNEL_COUNT {
        return 2 as WORD32;
    }
    c = 0 as core::ffi::c_int as WORD32;
    while c < MAX_CHANNEL_COUNT {
        unique_idx[c as usize] = -(10 as core::ffi::c_int) as WORD32;
        unique_scaling[c as usize] = -10.0f32 as FLOAT32;
        c += 1;
    }
    if (*str_drc_instruction_str).drc_set_effect as core::ffi::c_int
        & (EFFECT_BIT_DUCK_OTHER | EFFECT_BIT_DUCK_SELF) != 0
    {
        c = 0 as core::ffi::c_int as WORD32;
        while c < ch_cnt {
            let mut bs_gain_set_idx: WORD32 = 0;
            bs_gain_set_idx = impd_read_bits_buf(it_bit_buff, 6 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if bs_gain_set_idx > GAIN_SET_COUNT_MAX {
                return UNEXPECTED_ERROR;
            }
            (*str_drc_instruction_str).gain_set_index[c as usize] = (bs_gain_set_idx
                as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
            impd_dec_ducking_scaling(
                it_bit_buff,
                &mut (*((*str_drc_instruction_str).str_ducking_modifiers_for_channel)
                    .as_mut_ptr()
                    .offset(c as isize))
                    .ducking_scaling_flag,
                &mut (*((*str_drc_instruction_str).str_ducking_modifiers_for_channel)
                    .as_mut_ptr()
                    .offset(c as isize))
                    .ducking_scaling,
            );
            c += 1;
            repeat_parameters = impd_read_bits_buf(it_bit_buff, 1 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            if repeat_parameters == 1 as core::ffi::c_int {
                repeat_parameters_cnt = impd_read_bits_buf(it_bit_buff, 5 as WORD);
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                repeat_parameters_cnt += 1 as core::ffi::c_int;
                k = 0 as core::ffi::c_int as WORD32;
                while k < repeat_parameters_cnt {
                    if c > MAX_CHANNEL_COUNT - 1 as core::ffi::c_int {
                        return 2 as WORD32;
                    }
                    (*str_drc_instruction_str).gain_set_index[c as usize] = (*str_drc_instruction_str)
                        .gain_set_index[(c as core::ffi::c_int - 1 as core::ffi::c_int)
                        as usize];
                    (*str_drc_instruction_str)
                        .str_ducking_modifiers_for_channel[c as usize] = (*str_drc_instruction_str)
                        .str_ducking_modifiers_for_channel[(c as core::ffi::c_int
                        - 1 as core::ffi::c_int) as usize];
                    c += 1;
                    k += 1;
                }
            }
        }
        if c > ch_cnt {
            return 2 as WORD32;
        }
        ducking_sequence = -(1 as core::ffi::c_int) as WORD32;
        g = 0 as core::ffi::c_int as WORD32;
        if (*str_drc_instruction_str).drc_set_effect as core::ffi::c_int
            & EFFECT_BIT_DUCK_OTHER != 0
        {
            c = 0 as core::ffi::c_int as WORD32;
            while c < ch_cnt {
                match_0 = 0 as core::ffi::c_int as WORD32;
                idx = (*str_drc_instruction_str).gain_set_index[c as usize];
                factor = (*str_drc_instruction_str)
                    .str_ducking_modifiers_for_channel[c as usize]
                    .ducking_scaling;
                if idx < 0 as core::ffi::c_int {
                    n = 0 as core::ffi::c_int as WORD32;
                    while n < g {
                        if unique_scaling[n as usize] == factor {
                            match_0 = 1 as core::ffi::c_int as WORD32;
                            (*str_drc_instruction_str).channel_group_of_ch[c as usize] = n;
                            break;
                        } else {
                            n += 1;
                        }
                    }
                    if match_0 == 0 as core::ffi::c_int {
                        unique_idx[g as usize] = idx;
                        unique_scaling[g as usize] = factor;
                        (*str_drc_instruction_str).channel_group_of_ch[c as usize] = g;
                        g += 1;
                    }
                } else {
                    if ducking_sequence > 0 as core::ffi::c_int
                        && ducking_sequence != idx
                    {
                        return 2 as WORD32;
                    }
                    ducking_sequence = idx;
                    (*str_drc_instruction_str).channel_group_of_ch[c as usize] = -(1
                        as core::ffi::c_int) as WORD32;
                }
                c += 1;
            }
            (*str_drc_instruction_str).num_drc_ch_groups = g;
            if ducking_sequence == -(1 as core::ffi::c_int) {
                return 2 as WORD32;
            }
        } else if (*str_drc_instruction_str).drc_set_effect as core::ffi::c_int
            & EFFECT_BIT_DUCK_SELF != 0
        {
            c = 0 as core::ffi::c_int as WORD32;
            while c < ch_cnt {
                match_0 = 0 as core::ffi::c_int as WORD32;
                idx = (*str_drc_instruction_str).gain_set_index[c as usize];
                factor = (*str_drc_instruction_str)
                    .str_ducking_modifiers_for_channel[c as usize]
                    .ducking_scaling;
                if idx >= 0 as core::ffi::c_int {
                    n = 0 as core::ffi::c_int as WORD32;
                    while n < g {
                        if unique_idx[n as usize] == idx
                            && unique_scaling[n as usize] == factor
                        {
                            match_0 = 1 as core::ffi::c_int as WORD32;
                            (*str_drc_instruction_str).channel_group_of_ch[c as usize] = n;
                            break;
                        } else {
                            n += 1;
                        }
                    }
                    if match_0 == 0 as core::ffi::c_int {
                        unique_idx[g as usize] = idx;
                        unique_scaling[g as usize] = factor;
                        (*str_drc_instruction_str).channel_group_of_ch[c as usize] = g;
                        g += 1;
                    }
                } else {
                    (*str_drc_instruction_str).channel_group_of_ch[c as usize] = -(1
                        as core::ffi::c_int) as WORD32;
                }
                c += 1;
            }
            (*str_drc_instruction_str).num_drc_ch_groups = g;
        }
        if (*str_drc_instruction_str).num_drc_ch_groups
            > (if (24 as core::ffi::c_int) < 8 as core::ffi::c_int {
                24 as core::ffi::c_int
            } else {
                8 as core::ffi::c_int
            })
        {
            return UNEXPECTED_ERROR;
        }
        g = 0 as core::ffi::c_int as WORD32;
        while g < (*str_drc_instruction_str).num_drc_ch_groups {
            let mut set: WORD32 = if (*str_drc_instruction_str).drc_set_effect
                as core::ffi::c_int & EFFECT_BIT_DUCK_OTHER != 0
            {
                ducking_sequence
            } else {
                unique_idx[g as usize]
            };
            if set < 0 as core::ffi::c_int {
                return UNEXPECTED_ERROR;
            }
            (*str_drc_instruction_str).gain_set_index_for_channel_group[g as usize] = set;
            (*str_drc_instruction_str)
                .str_ducking_modifiers_for_channel_group[g as usize]
                .ducking_scaling = unique_scaling[g as usize];
            if unique_scaling[g as usize] != 1.0f32 {
                (*str_drc_instruction_str)
                    .str_ducking_modifiers_for_channel_group[g as usize]
                    .ducking_scaling_flag = 1 as core::ffi::c_int as WORD32;
            } else {
                (*str_drc_instruction_str)
                    .str_ducking_modifiers_for_channel_group[g as usize]
                    .ducking_scaling_flag = 0 as core::ffi::c_int as WORD32;
            }
            (*str_drc_instruction_str).band_count_of_ch_group[g as usize] = 1
                as core::ffi::c_int as WORD32;
            g += 1;
        }
    } else {
        if (version == 0 as core::ffi::c_int
            || (*str_drc_instruction_str).drc_apply_to_dwnmix != 0 as core::ffi::c_int)
            && (*str_drc_instruction_str).downmix_id[0 as core::ffi::c_int as usize]
                != 0 as core::ffi::c_int
            && (*str_drc_instruction_str).downmix_id[0 as core::ffi::c_int as usize]
                != ID_FOR_ANY_DOWNMIX
            && (*str_drc_instruction_str).dwnmix_id_count == 1 as core::ffi::c_int
        {
            i = 0 as core::ffi::c_int as WORD32;
            while i < (*drc_config).dwnmix_instructions_count {
                if (*str_drc_instruction_str).downmix_id[0 as core::ffi::c_int as usize]
                    == (*drc_config).dwnmix_instructions[i as usize].downmix_id
                {
                    break;
                }
                i += 1;
            }
            if i == (*drc_config).dwnmix_instructions_count {
                return 2 as WORD32;
            }
            ch_cnt = (*drc_config).dwnmix_instructions[i as usize].target_channel_count;
        } else if (version == 0 as core::ffi::c_int
            || (*str_drc_instruction_str).drc_apply_to_dwnmix != 0 as core::ffi::c_int)
            && ((*str_drc_instruction_str).downmix_id[0 as core::ffi::c_int as usize]
                == ID_FOR_ANY_DOWNMIX
                || (*str_drc_instruction_str).dwnmix_id_count > 1 as core::ffi::c_int)
        {
            ch_cnt = 1 as core::ffi::c_int as WORD32;
        }
        if ch_cnt > MAX_CHANNEL_COUNT {
            return 2 as WORD32;
        }
        c = 0 as core::ffi::c_int as WORD32;
        while c < ch_cnt {
            let mut bs_gain_set_idx_0: WORD32 = 0;
            let mut repeat_gain_set_idx: WORD32 = 0;
            let mut repeat_gain_set_idx_cnt: WORD32 = 0;
            let mut temp: WORD32 = 0;
            temp = impd_read_bits_buf(it_bit_buff, 7 as WORD);
            if (*it_bit_buff).error != 0 {
                return (*it_bit_buff).error;
            }
            bs_gain_set_idx_0 = (temp as core::ffi::c_int >> 1 as core::ffi::c_int
                & 0x7f as core::ffi::c_int) as WORD32;
            repeat_gain_set_idx = (temp as core::ffi::c_int & 1 as core::ffi::c_int)
                as WORD32;
            if bs_gain_set_idx_0 > GAIN_SET_COUNT_MAX {
                return UNEXPECTED_ERROR;
            }
            (*str_drc_instruction_str).gain_set_index[c as usize] = (bs_gain_set_idx_0
                as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
            c += 1;
            if repeat_gain_set_idx == 1 as core::ffi::c_int {
                repeat_gain_set_idx_cnt = impd_read_bits_buf(it_bit_buff, 5 as WORD);
                if (*it_bit_buff).error != 0 {
                    return (*it_bit_buff).error;
                }
                repeat_gain_set_idx_cnt += 1 as core::ffi::c_int;
                k = 0 as core::ffi::c_int as WORD32;
                while k < repeat_gain_set_idx_cnt {
                    if c > MAX_CHANNEL_COUNT - 1 as core::ffi::c_int {
                        return 2 as WORD32;
                    }
                    (*str_drc_instruction_str).gain_set_index[c as usize] = (bs_gain_set_idx_0
                        as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
                    c += 1;
                    k += 1;
                }
            }
        }
        if c > ch_cnt {
            return 2 as WORD32;
        }
        g = 0 as core::ffi::c_int as WORD32;
        if (*str_drc_instruction_str).downmix_id[0 as core::ffi::c_int as usize]
            == ID_FOR_ANY_DOWNMIX
            || (*str_drc_instruction_str).dwnmix_id_count > 1 as core::ffi::c_int
        {
            let mut idx_0: WORD32 = (*str_drc_instruction_str)
                .gain_set_index[0 as core::ffi::c_int as usize];
            if idx_0 >= 0 as core::ffi::c_int {
                unique_idx[0 as core::ffi::c_int as usize] = idx_0;
                g = 1 as core::ffi::c_int as WORD32;
            }
        } else {
            c = 0 as core::ffi::c_int as WORD32;
            while c < ch_cnt {
                let mut idx_1: WORD32 = (*str_drc_instruction_str)
                    .gain_set_index[c as usize];
                match_0 = 0 as core::ffi::c_int as WORD32;
                if idx_1 >= 0 as core::ffi::c_int {
                    n = 0 as core::ffi::c_int as WORD32;
                    while n < g {
                        if unique_idx[n as usize] == idx_1 {
                            match_0 = 1 as core::ffi::c_int as WORD32;
                            (*str_drc_instruction_str).channel_group_of_ch[c as usize] = n;
                            break;
                        } else {
                            n += 1;
                        }
                    }
                    if match_0 == 0 as core::ffi::c_int {
                        unique_idx[g as usize] = idx_1;
                        (*str_drc_instruction_str).channel_group_of_ch[c as usize] = g;
                        g += 1;
                    }
                } else {
                    (*str_drc_instruction_str).channel_group_of_ch[c as usize] = -(1
                        as core::ffi::c_int) as WORD32;
                }
                c += 1;
            }
        }
        (*str_drc_instruction_str).num_drc_ch_groups = g;
        if (*str_drc_instruction_str).num_drc_ch_groups
            > (if (24 as core::ffi::c_int) < 8 as core::ffi::c_int {
                24 as core::ffi::c_int
            } else {
                8 as core::ffi::c_int
            })
        {
            return UNEXPECTED_ERROR;
        }
        g = 0 as core::ffi::c_int as WORD32;
        while g < (*str_drc_instruction_str).num_drc_ch_groups {
            let mut set_0: WORD32 = 0;
            let mut band_count: WORD32 = 0;
            set_0 = unique_idx[g as usize];
            if set_0 < 0 as core::ffi::c_int {
                return UNEXPECTED_ERROR;
            }
            (*str_drc_instruction_str).gain_set_index_for_channel_group[g as usize] = set_0;
            if !str_p_loc_drc_coefficients_uni_drc.is_null()
                && set_0 < (*str_p_loc_drc_coefficients_uni_drc).gain_set_count
            {
                band_count = (*str_p_loc_drc_coefficients_uni_drc)
                    .gain_set_params[set_0 as usize]
                    .band_count;
            } else {
                band_count = 1 as core::ffi::c_int as WORD32;
            }
            err = impd_dec_gain_modifiers(
                it_bit_buff,
                version,
                band_count,
                &mut *((*str_drc_instruction_str).str_gain_modifiers_of_ch_group)
                    .as_mut_ptr()
                    .offset(g as isize),
            );
            if err != 0 {
                return err;
            }
            g += 1;
        }
    }
    return 0 as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn impd_parse_loudness_info(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut version: WORD32,
    mut loudness_info: *mut ia_loudness_info_struct,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut sample_peak_level: WORD32 = 0;
    let mut true_peak_level: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut temp: WORD32 = 0;
    (*loudness_info).drc_set_id = impd_read_bits_buf(it_bit_buff, 6 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if version >= 1 as core::ffi::c_int {
        (*loudness_info).eq_set_id = impd_read_bits_buf(it_bit_buff, 6 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
    } else {
        (*loudness_info).eq_set_id = 0 as core::ffi::c_int as WORD32;
    }
    temp = impd_read_bits_buf(it_bit_buff, 8 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    (*loudness_info).downmix_id = (temp as core::ffi::c_int >> 1 as core::ffi::c_int
        & 0x7f as core::ffi::c_int) as WORD32;
    (*loudness_info).sample_peak_level_present = (temp as core::ffi::c_int
        & 1 as core::ffi::c_int) as WORD32;
    if (*loudness_info).sample_peak_level_present != 0 {
        sample_peak_level = impd_read_bits_buf(it_bit_buff, 12 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if sample_peak_level == 0 as core::ffi::c_int {
            (*loudness_info).sample_peak_level_present = 0 as core::ffi::c_int as WORD32;
            (*loudness_info).sample_peak_level = 0.0f32 as FLOAT32;
        } else {
            (*loudness_info).sample_peak_level = (20.0f32
                - sample_peak_level as core::ffi::c_float * 0.03125f32) as FLOAT32;
        }
    }
    (*loudness_info).true_peak_level_present = impd_read_bits_buf(
        it_bit_buff,
        1 as WORD,
    );
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    if (*loudness_info).true_peak_level_present != 0 {
        true_peak_level = impd_read_bits_buf(it_bit_buff, 12 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        if true_peak_level == 0 as core::ffi::c_int {
            (*loudness_info).true_peak_level_present = 0 as core::ffi::c_int as WORD32;
            (*loudness_info).true_peak_level = 0.0f32 as FLOAT32;
        } else {
            (*loudness_info).true_peak_level = (20.0f32
                - true_peak_level as core::ffi::c_float * 0.03125f32) as FLOAT32;
        }
        temp = impd_read_bits_buf(it_bit_buff, 6 as WORD);
        if (*it_bit_buff).error != 0 {
            return (*it_bit_buff).error;
        }
        (*loudness_info).true_peak_level_measurement_system = (temp as core::ffi::c_int
            >> 2 as core::ffi::c_int & 0xf as core::ffi::c_int) as WORD32;
        (*loudness_info).true_peak_level_reliability = (temp as core::ffi::c_int
            & 3 as core::ffi::c_int) as WORD32;
    }
    (*loudness_info).measurement_count = impd_read_bits_buf(it_bit_buff, 4 as WORD);
    if (*it_bit_buff).error != 0 {
        return (*it_bit_buff).error;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*loudness_info).measurement_count {
        err = impd_parse_loudness_measure(
            it_bit_buff,
            &mut *((*loudness_info).loudness_measure).as_mut_ptr().offset(i as isize),
        );
        if (*loudness_info).loudness_measure[i as usize].method_def
            == METHOD_DEFINITION_ANCHOR_LOUDNESS
        {
            (*loudness_info).anchor_loudness_present = 1 as core::ffi::c_int as WORD32;
        }
        if (*loudness_info).loudness_measure[i as usize].measurement_system
            == MEASUREMENT_SYSTEM_EXPERT_PANEL
        {
            (*loudness_info).expert_loudness_present = 1 as core::ffi::c_int as WORD32;
        }
        if err != 0 {
            return err;
        }
        i += 1;
    }
    return 0 as WORD32;
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
