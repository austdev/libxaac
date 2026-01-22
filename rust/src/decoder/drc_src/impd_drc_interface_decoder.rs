pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type FLOAT32 = core::ffi::c_float;
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
pub const UNEXPECTED_ERROR: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_NUM_DOWNMIX_ID_REQUESTS: core::ffi::c_int = 15 as core::ffi::c_int;
pub const MAX_SIGNATURE_DATA_LENGTH_PLUS_ONE: core::ffi::c_int = 256 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn impd_drc_dec_interface_add_effect_type(
    mut pstr_drc_interface: *mut ia_drc_interface_struct,
    mut drc_effect_type: WORD32,
    mut target_loudness: WORD32,
    mut loud_norm: WORD32,
    mut album_mode: WORD32,
    mut boost: FLOAT32,
    mut compress: FLOAT32,
    mut loudness_leveling_flag: WORD32,
) -> WORD32 {
    let mut err: WORD32 = 0 as WORD32;
    let mut i: WORD32 = 0 as WORD32;
    if !pstr_drc_interface.is_null() {
        (*pstr_drc_interface).interface_signat_flag = 0 as core::ffi::c_int as WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < MAX_SIGNATURE_DATA_LENGTH_PLUS_ONE * 8 as core::ffi::c_int {
            (*pstr_drc_interface)
                .drc_uni_interface_signature
                .interface_signat_data[i as usize] = 0 as UWORD32;
            i += 1;
        }
        (*pstr_drc_interface).drc_uni_interface_signature.interface_signat_data_len = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_interface).system_interface_flag = 1 as core::ffi::c_int as WORD32;
        (*pstr_drc_interface).system_interface.target_config_request_type = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_interface).system_interface.num_downmix_id_requests = 0
            as core::ffi::c_int as WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < MAX_NUM_DOWNMIX_ID_REQUESTS {
            (*pstr_drc_interface).system_interface.requested_dwnmix_id[i as usize] = 0
                as core::ffi::c_int as WORD32;
            i += 1;
        }
        (*pstr_drc_interface).system_interface.requested_target_layout = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_interface).system_interface.requested_target_ch_count = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_interface).loudness_norm_ctrl_interface_flag = 1 as core::ffi::c_int
            as WORD32;
        if loud_norm == 1 as core::ffi::c_int {
            (*pstr_drc_interface)
                .loudness_norm_ctrl_interface
                .loudness_normalization_on = 1 as core::ffi::c_int as WORD32;
        } else {
            (*pstr_drc_interface)
                .loudness_norm_ctrl_interface
                .loudness_normalization_on = 0 as core::ffi::c_int as WORD32;
        }
        (*pstr_drc_interface).loudness_norm_ctrl_interface.target_loudness = target_loudness
            as FLOAT32;
        (*pstr_drc_interface).drc_uni_interface_ext.loudness_leveling_on = loudness_leveling_flag;
        (*pstr_drc_interface).loudness_norm_parameter_interface_flag = 1
            as core::ffi::c_int as WORD32;
        (*pstr_drc_interface).loudness_norm_param_interface.album_mode = album_mode;
        (*pstr_drc_interface).loudness_norm_param_interface.peak_limiter = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_interface)
            .loudness_norm_param_interface
            .change_loudness_deviation_max = 1 as core::ffi::c_int as WORD32;
        (*pstr_drc_interface).loudness_norm_param_interface.loudness_deviation_max = 63
            as core::ffi::c_int as WORD32;
        (*pstr_drc_interface)
            .loudness_norm_param_interface
            .change_loudness_measur_method = 1 as core::ffi::c_int as WORD32;
        (*pstr_drc_interface)
            .loudness_norm_param_interface
            .loudness_measurement_method = 1 as core::ffi::c_int as WORD32;
        (*pstr_drc_interface)
            .loudness_norm_param_interface
            .change_loudness_measur_system = 1 as core::ffi::c_int as WORD32;
        (*pstr_drc_interface)
            .loudness_norm_param_interface
            .loudness_measurement_system = 1 as core::ffi::c_int as WORD32;
        (*pstr_drc_interface)
            .loudness_norm_param_interface
            .change_loudness_measur_pre_proc = 0 as core::ffi::c_int as WORD32;
        (*pstr_drc_interface)
            .loudness_norm_param_interface
            .loudness_measurement_pre_proc = 0 as core::ffi::c_int as WORD32;
        (*pstr_drc_interface).loudness_norm_param_interface.change_device_cut_off_freq = 1
            as core::ffi::c_int as WORD32;
        (*pstr_drc_interface).loudness_norm_param_interface.device_cut_off_frequency = 20
            as core::ffi::c_int as WORD32;
        (*pstr_drc_interface)
            .loudness_norm_param_interface
            .change_loudness_norm_gain_db_max = 1 as core::ffi::c_int as WORD32;
        (*pstr_drc_interface).loudness_norm_param_interface.loudness_norm_gain_db_max = 1000.0f32;
        (*pstr_drc_interface)
            .loudness_norm_param_interface
            .change_loudness_norm_gain_modification_db = 1 as core::ffi::c_int as WORD32;
        (*pstr_drc_interface)
            .loudness_norm_param_interface
            .loudness_norm_gain_modification_db = 0.0f32;
        (*pstr_drc_interface)
            .loudness_norm_param_interface
            .change_output_peak_level_max = 1 as core::ffi::c_int as WORD32;
        (*pstr_drc_interface).loudness_norm_param_interface.output_peak_level_max = 0.0f32;
        (*pstr_drc_interface).drc_interface_flag = 1 as core::ffi::c_int as WORD32;
        if drc_effect_type == -(1 as core::ffi::c_int) {
            (*pstr_drc_interface).drc_ctrl_interface.dynamic_range_control_on = 0
                as core::ffi::c_int as WORD32;
            (*pstr_drc_interface)
                .drc_ctrl_interface
                .requested_drc_effect_type[0 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int
                as WORD32;
        } else if drc_effect_type == 0 as core::ffi::c_int {
            (*pstr_drc_interface).drc_ctrl_interface.dynamic_range_control_on = 1
                as core::ffi::c_int as WORD32;
            (*pstr_drc_interface).drc_ctrl_interface.num_drc_feature_requests = 0
                as core::ffi::c_int as WORD32;
        } else {
            (*pstr_drc_interface).drc_ctrl_interface.dynamic_range_control_on = 1
                as core::ffi::c_int as WORD32;
            (*pstr_drc_interface)
                .drc_ctrl_interface
                .requested_drc_effect_type[0 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize] = drc_effect_type;
            (*pstr_drc_interface).drc_ctrl_interface.num_drc_feature_requests = 3
                as core::ffi::c_int as WORD32;
            (*pstr_drc_interface)
                .drc_ctrl_interface
                .drc_feature_req_type[0 as core::ffi::c_int as usize] = 0
                as core::ffi::c_int as WORD32;
            (*pstr_drc_interface)
                .drc_ctrl_interface
                .drc_feature_req_type[1 as core::ffi::c_int as usize] = 1
                as core::ffi::c_int as WORD32;
            (*pstr_drc_interface)
                .drc_ctrl_interface
                .drc_feature_req_type[2 as core::ffi::c_int as usize] = 2
                as core::ffi::c_int as WORD32;
            (*pstr_drc_interface)
                .drc_ctrl_interface
                .requested_num_drc_effects[0 as core::ffi::c_int as usize] = 1
                as core::ffi::c_int as WORD32;
            (*pstr_drc_interface)
                .drc_ctrl_interface
                .desired_num_drc_effects_of_requested[0 as core::ffi::c_int as usize] = 1
                as core::ffi::c_int as WORD32;
        }
        (*pstr_drc_interface)
            .drc_ctrl_interface
            .requested_dyn_rng_measurement_type[0 as core::ffi::c_int as usize] = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_interface)
            .drc_ctrl_interface
            .requested_dyn_range_is_single_val_flag[0 as core::ffi::c_int as usize] = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_interface)
            .drc_ctrl_interface
            .requested_dyn_range_is_single_val_flag[1 as core::ffi::c_int as usize] = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_interface)
            .drc_ctrl_interface
            .requested_dyn_range_min_val[1 as core::ffi::c_int as usize] = 3.0f32
            as FLOAT32;
        (*pstr_drc_interface)
            .drc_ctrl_interface
            .requested_dyn_range_max_val[1 as core::ffi::c_int as usize] = 10.0f32
            as FLOAT32;
        (*pstr_drc_interface)
            .drc_ctrl_interface
            .requested_drc_characteristic[2 as core::ffi::c_int as usize] = 3
            as core::ffi::c_int as WORD32;
        (*pstr_drc_interface).drc_parameter_interface_flag = 1 as core::ffi::c_int
            as WORD32;
        (*pstr_drc_interface).drc_parameter_interface.change_compress = 1
            as core::ffi::c_int as WORD32;
        (*pstr_drc_interface).drc_parameter_interface.change_boost = 1
            as core::ffi::c_int as WORD32;
        (*pstr_drc_interface).drc_parameter_interface.compress = compress;
        (*pstr_drc_interface).drc_parameter_interface.boost = boost;
        (*pstr_drc_interface).drc_parameter_interface.change_drc_characteristic_target = 1
            as core::ffi::c_int as WORD32;
        (*pstr_drc_interface).drc_parameter_interface.drc_characteristic_target = 0
            as core::ffi::c_int as WORD32;
        (*pstr_drc_interface).drc_uni_interface_ext_flag = 0 as core::ffi::c_int
            as WORD32;
    } else {
        return UNEXPECTED_ERROR
    }
    return err;
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
