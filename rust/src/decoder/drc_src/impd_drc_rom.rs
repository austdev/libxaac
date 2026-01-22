pub type WORD32 = core::ffi::c_int;
pub type FLOAT32 = core::ffi::c_float;
pub type FLOAT64 = core::ffi::c_double;
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
pub struct ia_cicp_sigmoid_characteristic_param_struct {
    pub in_out_ratio: FLOAT32,
    pub exp_lo: FLOAT32,
    pub exp_hi: FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_filter_bank_params_struct {
    pub f_cross_norm: FLOAT32,
    pub gamma: FLOAT32,
    pub delta: FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_loc_sys_interface_struct {
    pub target_config_request_type: WORD32,
    pub num_downmix_id_requests: WORD32,
    pub requested_dwnmix_id: [WORD32; 3],
    pub requested_target_layout: WORD32,
    pub requested_target_ch_count: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_loc_loudness_norm_ctrl_interface_struct {
    pub loudness_normalization_on: WORD32,
    pub target_loudness: FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_loc_loudness_norm_param_interface_struct {
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
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_loc_drc_interface_struct {
    pub dynamic_range_control_on: WORD32,
    pub num_drc_feature_requests: WORD32,
    pub drc_feature_req_type: [WORD32; 3],
    pub requested_dyn_rng_measurement_type: WORD32,
    pub requested_dyn_range_is_single_val_flag: WORD32,
    pub requested_dyn_range_value: FLOAT32,
    pub requested_dyn_range_min_val: FLOAT32,
    pub requested_dyn_range_max_val: FLOAT32,
    pub requested_drc_characteristic: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_loc_requested_drc_effect_struct {
    pub requested_num_drc_effects: WORD32,
    pub desired_num_drc_effects_of_requested: WORD32,
    pub requested_drc_effect_type: [WORD32; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_loc_drc_parameter_interface_struct {
    pub compress: FLOAT32,
    pub boost: FLOAT32,
    pub drc_characteristic_target: WORD32,
}
pub const USER_METHOD_DEFINITION_PROGRAM_LOUDNESS: core::ffi::c_int = 1
    as core::ffi::c_int;
pub const USER_MEASUREMENT_SYSTEM_BS_1770_4: core::ffi::c_int = 1 as core::ffi::c_int;
pub const USER_MEASUREMENT_SYSTEM_BS_1770_3: core::ffi::c_int = USER_MEASUREMENT_SYSTEM_BS_1770_4;
pub const USER_LOUDNESS_PREPROCESSING_OFF: core::ffi::c_int = 1 as core::ffi::c_int;
pub const LOUDNESS_DEVIATION_MAX_DEFAULT: core::ffi::c_int = 63 as core::ffi::c_int;
pub const LOUDNESS_NORMALIZATION_GAIN_MAX_DEFAULT: core::ffi::c_int = 1000
    as core::ffi::c_int;
pub const SHORT_TERM_LOUDNESS_TO_AVG: core::ffi::c_int = 0 as core::ffi::c_int;
pub const TOP_OF_LOUDNESS_RANGE_TO_AVG: core::ffi::c_int = 2 as core::ffi::c_int;
pub const EFFECT_TYPE_REQUESTED_NIGHT: core::ffi::c_int = 1 as core::ffi::c_int;
pub const EFFECT_TYPE_REQUESTED_LIMITED: core::ffi::c_int = 3 as core::ffi::c_int;
pub const EFFECT_TYPE_REQUESTED_LOWLEVEL: core::ffi::c_int = 4 as core::ffi::c_int;
pub const EFFECT_TYPE_REQUESTED_DIALOG: core::ffi::c_int = 5 as core::ffi::c_int;
pub const EFFECT_TYPE_REQUESTED_GENERAL_COMPR: core::ffi::c_int = 6 as core::ffi::c_int;
pub const EFFECT_TYPE_REQUESTED_ARTISTIC: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MATCH_EFFECT_TYPE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const MATCH_DYNAMIC_RANGE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const MATCH_DRC_CHARACTERISTIC: core::ffi::c_int = 2 as core::ffi::c_int;
#[no_mangle]
pub static mut samp_rate_tbl: [[FLOAT32; 12]; 13] = [
    [
        1.559742927551f32,
        -2.926673889160f32,
        1.378173947334f32,
        -1.844531774521f32,
        0.855774641037f32,
        0.997517585754f32,
        -1.995035171509f32,
        0.997517585754f32,
        -1.995032072067f32,
        0.995038211346f32,
        0.,
        0.,
    ],
    [
        1.557545065880f32,
        -2.905559301376f32,
        1.361245870590f32,
        -1.830842256546f32,
        0.844074070454f32,
        0.997298538685f32,
        -1.994597077370f32,
        0.997298538685f32,
        -1.994593381882f32,
        0.994600534439f32,
        0.,
        0.,
    ],
    [
        1.547380685806f32,
        -2.808161973953f32,
        1.285161137581f32,
        -1.767331838608f32,
        0.791711509228f32,
        0.996279776096f32,
        -1.992559552193f32,
        0.996279776096f32,
        -1.992552518845f32,
        0.992566406727f32,
        0.,
        0.,
    ],
    [
        1.535182833672f32,
        -2.691803932190f32,
        1.198426246643f32,
        -1.690699458122f32,
        0.732504665852f32,
        0.995044350624f32,
        -1.990088701248f32,
        0.995044350624f32,
        -1.990076303482f32,
        0.990100920200f32,
        0.,
        0.,
    ],
    [
        1.530909657478f32,
        -2.651169300079f32,
        1.169166922569f32,
        -1.663750052452f32,
        0.712657511234f32,
        0.994607865810f32,
        -1.989215731621f32,
        0.994607865810f32,
        -1.989201068878f32,
        0.989230215549f32,
        0.,
        0.,
    ],
    [
        1.511321425438f32,
        -2.465713739395f32,
        1.042117238045f32,
        -1.539572954178f32,
        0.627297878265f32,
        0.992580235004f32,
        -1.985160470009f32,
        0.992580235004f32,
        -1.985132932663f32,
        0.985188126564f32,
        0.,
        0.,
    ],
    [
        1.488207340240f32,
        -2.248480796814f32,
        0.906192243099f32,
        -1.391770243645f32,
        0.537688970566f32,
        0.990125238895f32,
        -1.980250477791f32,
        0.990125238895f32,
        -1.980201482773f32,
        0.980299532413f32,
        0.,
        0.,
    ],
    [
        1.480212569237f32,
        -2.173723459244f32,
        0.862487196922f32,
        -1.340346932411f32,
        0.509323298931f32,
        0.989259064198f32,
        -1.978518128395f32,
        0.989259064198f32,
        -1.978460073471f32,
        0.978576123714f32,
        0.,
        0.,
    ],
    [
        1.444234013557f32,
        -1.839543223381f32,
        0.685432314873f32,
        -1.107139229774f32,
        0.397262454033f32,
        0.985242545605f32,
        -1.970485091209f32,
        0.985242545605f32,
        -1.970375299454f32,
        0.970594763756f32,
        0.,
        0.,
    ],
    [
        1.403080821037f32,
        -1.461511373520f32,
        0.519652962685f32,
        -0.837045848370f32,
        0.298268288374f32,
        0.980395674706f32,
        -1.960791349411f32,
        0.980395674706f32,
        -1.960597157478f32,
        0.960985362530f32,
        0.,
        0.,
    ],
    [
        1.389132857323f32,
        -1.334347724915f32,
        0.471855700016f32,
        -0.744739949703f32,
        0.271380871534f32,
        0.978689610958f32,
        -1.957379221916f32,
        0.978689610958f32,
        -1.957149744034f32,
        0.957608699799f32,
        0.,
        0.,
    ],
    [
        1.327733159065f32,
        -0.780098080635f32,
        0.309720277786f32,
        -0.334008187056f32,
        0.191363617778f32,
        0.970807731152f32,
        -1.941615462303f32,
        0.970807731152f32,
        -1.941183090210f32,
        0.942047894001f32,
        0.,
        0.,
    ],
    [
        1.308971643448f32,
        -0.612507879734f32,
        0.275601744652f32,
        -0.207099482417f32,
        0.179165065289f32,
        0.968287348747f32,
        -1.936574697495f32,
        0.968287348747f32,
        -1.936063885689f32,
        0.937085747719f32,
        0.,
        0.,
    ],
];
#[no_mangle]
pub static mut ia_drc_gain_tbls_prof_0_1: [ia_delta_gain_code_table_struct; 25] = [
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 2 as WORD32,
            code: 0x3 as WORD32,
            value: -0.125f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 2 as WORD32,
            code: 0x2 as WORD32,
            value: 0.125f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 3 as WORD32,
            code: 0x1 as WORD32,
            value: -0.250f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 3 as WORD32,
            code: 0x2 as WORD32,
            value: 0.000f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 4 as WORD32,
            code: 0 as WORD32,
            value: -2.000f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 5 as WORD32,
            code: 0x2 as WORD32,
            value: -0.500f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 5 as WORD32,
            code: 0xf as WORD32,
            value: -0.375f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 5 as WORD32,
            code: 0xe as WORD32,
            value: 1.000f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 6 as WORD32,
            code: 0x19 as WORD32,
            value: -0.625f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 6 as WORD32,
            code: 0x18 as WORD32,
            value: 0.250f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 6 as WORD32,
            code: 0x6 as WORD32,
            value: 0.375f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 7 as WORD32,
            code: 0xf as WORD32,
            value: -1.000f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 7 as WORD32,
            code: 0x34 as WORD32,
            value: -0.875f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 7 as WORD32,
            code: 0x36 as WORD32,
            value: -0.750f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 7 as WORD32,
            code: 0x37 as WORD32,
            value: 0.500f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 8 as WORD32,
            code: 0x1d as WORD32,
            value: 0.625f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 9 as WORD32,
            code: 0x39 as WORD32,
            value: -1.875f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 9 as WORD32,
            code: 0xd5 as WORD32,
            value: -1.125f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 9 as WORD32,
            code: 0xd7 as WORD32,
            value: 0.750f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 9 as WORD32,
            code: 0xd4 as WORD32,
            value: 0.875f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 10 as WORD32,
            code: 0x70 as WORD32,
            value: -1.500f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 10 as WORD32,
            code: 0x1ac as WORD32,
            value: -1.375f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 10 as WORD32,
            code: 0x1ad as WORD32,
            value: -1.250f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 11 as WORD32,
            code: 0xe2 as WORD32,
            value: -1.750f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 11 as WORD32,
            code: 0xe3 as WORD32,
            value: -1.625f32,
        };
        init
    },
];
#[no_mangle]
pub static mut ia_drc_gain_tbls_prof_2: [ia_delta_gain_code_table_struct; 49] = [
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 3 as WORD32,
            code: 0x7 as WORD32,
            value: -0.125f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 4 as WORD32,
            code: 0xc as WORD32,
            value: -0.625f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 4 as WORD32,
            code: 0x9 as WORD32,
            value: -0.500f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 4 as WORD32,
            code: 0x5 as WORD32,
            value: -0.375f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 4 as WORD32,
            code: 0x3 as WORD32,
            value: -0.250f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 4 as WORD32,
            code: 0x1 as WORD32,
            value: 0.000f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 4 as WORD32,
            code: 0xb as WORD32,
            value: 0.125f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 5 as WORD32,
            code: 0x11 as WORD32,
            value: -0.875f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 5 as WORD32,
            code: 0xe as WORD32,
            value: -0.750f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 5 as WORD32,
            code: 0x5 as WORD32,
            value: 0.250f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 5 as WORD32,
            code: 0x4 as WORD32,
            value: 0.375f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 5 as WORD32,
            code: 0x8 as WORD32,
            value: 0.500f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 5 as WORD32,
            code: 0 as WORD32,
            value: 0.625f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 5 as WORD32,
            code: 0xd as WORD32,
            value: 0.750f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 5 as WORD32,
            code: 0xf as WORD32,
            value: 0.875f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 5 as WORD32,
            code: 0x10 as WORD32,
            value: 1.000f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 5 as WORD32,
            code: 0x1b as WORD32,
            value: 1.125f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 6 as WORD32,
            code: 0x2b as WORD32,
            value: -1.250f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 6 as WORD32,
            code: 0x28 as WORD32,
            value: -1.125f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 6 as WORD32,
            code: 0x2 as WORD32,
            value: -1.000f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 6 as WORD32,
            code: 0x12 as WORD32,
            value: 1.250f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 6 as WORD32,
            code: 0x18 as WORD32,
            value: 1.375f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 6 as WORD32,
            code: 0x29 as WORD32,
            value: 1.500f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 7 as WORD32,
            code: 0x6a as WORD32,
            value: -4.000f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 7 as WORD32,
            code: 0x54 as WORD32,
            value: -1.750f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 7 as WORD32,
            code: 0x68 as WORD32,
            value: -1.625f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 7 as WORD32,
            code: 0x26 as WORD32,
            value: -1.500f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 7 as WORD32,
            code: 0x6 as WORD32,
            value: -1.375f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 7 as WORD32,
            code: 0x32 as WORD32,
            value: 1.625f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 8 as WORD32,
            code: 0xd2 as WORD32,
            value: -2.250f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 8 as WORD32,
            code: 0xab as WORD32,
            value: -2.125f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 8 as WORD32,
            code: 0xaa as WORD32,
            value: -2.000f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 8 as WORD32,
            code: 0x4f as WORD32,
            value: -1.875f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 8 as WORD32,
            code: 0x4e as WORD32,
            value: 1.750f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 8 as WORD32,
            code: 0xd7 as WORD32,
            value: 1.875f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 8 as WORD32,
            code: 0xe as WORD32,
            value: 2.000f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 9 as WORD32,
            code: 0x1ad as WORD32,
            value: -3.625f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 9 as WORD32,
            code: 0x1ac as WORD32,
            value: -3.375f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 9 as WORD32,
            code: 0x1a6 as WORD32,
            value: -3.250f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 9 as WORD32,
            code: 0xcd as WORD32,
            value: -3.125f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 9 as WORD32,
            code: 0xce as WORD32,
            value: -2.750f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 9 as WORD32,
            code: 0x1a7 as WORD32,
            value: -2.625f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 9 as WORD32,
            code: 0x1f as WORD32,
            value: -2.500f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 9 as WORD32,
            code: 0xcc as WORD32,
            value: -2.375f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 10 as WORD32,
            code: 0x3c as WORD32,
            value: -3.500f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 10 as WORD32,
            code: 0x19e as WORD32,
            value: -3.000f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 10 as WORD32,
            code: 0x19f as WORD32,
            value: -2.875f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 11 as WORD32,
            code: 0x7a as WORD32,
            value: -3.875f32,
        };
        init
    },
    {
        let mut init = ia_delta_gain_code_table_struct {
            size: 11 as WORD32,
            code: 0x7b as WORD32,
            value: -3.750f32,
        };
        init
    },
];
#[no_mangle]
pub static mut channel_weight: [FLOAT32; 16] = [
    10.0f32,
    6.0f32,
    4.5f32,
    3.0f32,
    1.5f32,
    0.0f32,
    -1.5f32,
    -3.0f32,
    -4.5f32,
    -6.0f32,
    -10.0f32,
    -15.0f32,
    -20.0f32,
    -30.0f32,
    -40.0f32,
    -1000.0f32,
];
#[no_mangle]
pub static mut dwnmix_coeff_v1: [FLOAT32; 32] = [
    10.00f32,
    6.00f32,
    4.50f32,
    3.00f32,
    1.50f32,
    0.00f32,
    -0.50f32,
    -1.00f32,
    -1.50f32,
    -2.00f32,
    -2.50f32,
    -3.00f32,
    -3.50f32,
    -4.00f32,
    -4.50f32,
    -5.00f32,
    -5.50f32,
    -6.00f32,
    -6.50f32,
    -7.00f32,
    -7.50f32,
    -8.00f32,
    -9.00f32,
    -10.00f32,
    -11.00f32,
    -12.00f32,
    -15.00f32,
    -20.00f32,
    -25.00f32,
    -30.00f32,
    -40.00f32,
    -100000.0f32,
];
#[no_mangle]
pub static mut eq_slope_tbl: [FLOAT32; 16] = [
    -32.0f32,
    -24.0f32,
    -18.0f32,
    -12.0f32,
    -7.0f32,
    -4.0f32,
    -2.0f32,
    -1.0f32,
    1.0f32,
    2.0f32,
    4.0f32,
    7.0f32,
    12.0f32,
    18.0f32,
    24.0f32,
    32.0f32,
];
#[no_mangle]
pub static mut eq_gain_delta_tbl: [FLOAT32; 32] = [
    -22.0f32,
    -16.0f32,
    -13.0f32,
    -11.0f32,
    -9.0f32,
    -7.0f32,
    -6.0f32,
    -5.0f32,
    -4.0f32,
    -3.0f32,
    -2.5f32,
    -2.0f32,
    -1.5f32,
    -1.0f32,
    -0.5f32,
    0.0f32,
    0.5f32,
    1.0f32,
    1.5f32,
    2.0f32,
    2.5f32,
    3.0f32,
    4.0f32,
    5.0f32,
    6.0f32,
    7.0f32,
    9.0f32,
    11.0f32,
    13.0f32,
    16.0f32,
    22.0f32,
    32.0f32,
];
#[no_mangle]
pub static mut zero_pole_radius_tbl: [FLOAT32; 128] = [
    0.00000000E+00f32,
    7.57409621E-11f32,
    7.47451079E-09f32,
    7.37623509E-08f32,
    3.37872933E-07f32,
    1.05439995E-06f32,
    2.61370951E-06f32,
    5.55702854E-06f32,
    1.05878771E-05f32,
    1.85806475E-05f32,
    3.05868707E-05f32,
    4.78395414E-05f32,
    7.17558214E-05f32,
    1.03938342E-04f32,
    1.46175269E-04f32,
    2.00439375E-04f32,
    2.68886099E-04f32,
    3.53850890E-04f32,
    4.57845890E-04f32,
    5.83555840E-04f32,
    7.33833469E-04f32,
    9.11694835E-04f32,
    1.12031354E-03f32,
    1.36301492E-03f32,
    1.64327072E-03f32,
    1.96469179E-03f32,
    2.33102194E-03f32,
    2.74613220E-03f32,
    3.21401190E-03f32,
    3.73876374E-03f32,
    4.32459544E-03f32,
    4.97581391E-03f32,
    5.69681637E-03f32,
    6.49208482E-03f32,
    7.36617809E-03f32,
    8.32372531E-03f32,
    9.36941616E-03f32,
    1.05079999E-02f32,
    1.17442720E-02f32,
    1.30830696E-02f32,
    1.45292655E-02f32,
    1.60877611E-02f32,
    1.77634824E-02f32,
    1.95613634E-02f32,
    2.14863531E-02f32,
    2.35434026E-02f32,
    2.57374570E-02f32,
    2.80734543E-02f32,
    3.05563174E-02f32,
    3.31909470E-02f32,
    3.59822176E-02f32,
    3.89349759E-02f32,
    4.20540236E-02f32,
    4.53441292E-02f32,
    4.88100089E-02f32,
    5.24563305E-02f32,
    5.62877022E-02f32,
    6.03086725E-02f32,
    6.45237267E-02f32,
    6.89372867E-02f32,
    7.35536888E-02f32,
    7.83772022E-02f32,
    8.34120139E-02f32,
    8.86622295E-02f32,
    9.41318572E-02f32,
    9.98248383E-02f32,
    1.05744988E-01f32,
    1.11896060E-01f32,
    1.18281692E-01f32,
    1.24905407E-01f32,
    1.31770656E-01f32,
    1.38880774E-01f32,
    1.46238968E-01f32,
    1.53848350E-01f32,
    1.61711931E-01f32,
    1.69832602E-01f32,
    1.78213134E-01f32,
    1.86856180E-01f32,
    1.95764288E-01f32,
    2.04939872E-01f32,
    2.14385241E-01f32,
    2.24102572E-01f32,
    2.34093949E-01f32,
    2.44361281E-01f32,
    2.54906416E-01f32,
    2.65731007E-01f32,
    2.76836663E-01f32,
    2.88224846E-01f32,
    2.99896836E-01f32,
    3.11853856E-01f32,
    3.24096978E-01f32,
    3.36627185E-01f32,
    3.49445283E-01f32,
    3.62551987E-01f32,
    3.75947863E-01f32,
    3.89633417E-01f32,
    4.03608948E-01f32,
    4.17874694E-01f32,
    4.32430804E-01f32,
    4.47277188E-01f32,
    4.62413728E-01f32,
    4.77840215E-01f32,
    4.93556231E-01f32,
    5.09561300E-01f32,
    5.25854886E-01f32,
    5.42436182E-01f32,
    5.59304416E-01f32,
    5.76458573E-01f32,
    5.93897760E-01f32,
    6.11620665E-01f32,
    6.29626155E-01f32,
    6.47912800E-01f32,
    6.66479111E-01f32,
    6.85323536E-01f32,
    7.04444408E-01f32,
    7.23839939E-01f32,
    7.43508339E-01f32,
    7.63447523E-01f32,
    7.83655465E-01f32,
    8.04130018E-01f32,
    8.24868977E-01f32,
    8.45869958E-01f32,
    8.67130578E-01f32,
    8.88648331E-01f32,
    9.10420537E-01f32,
    9.32444632E-01f32,
    9.54717815E-01f32,
    9.77237225E-01f32,
];
#[no_mangle]
pub static mut zero_pole_angle_tbl: [FLOAT32; 128] = [
    0.00000000E+00f32,
    6.90533966E-04f32,
    7.31595252E-04f32,
    7.75098170E-04f32,
    8.21187906E-04f32,
    8.70018279E-04f32,
    9.21752258E-04f32,
    9.76562500E-04f32,
    1.03463193E-03f32,
    1.09615434E-03f32,
    1.16133507E-03f32,
    1.23039165E-03f32,
    1.30355455E-03f32,
    1.38106793E-03f32,
    1.46319050E-03f32,
    1.55019634E-03f32,
    1.64237581E-03f32,
    1.74003656E-03f32,
    1.84350452E-03f32,
    1.95312500E-03f32,
    2.06926386E-03f32,
    2.19230869E-03f32,
    2.32267015E-03f32,
    2.46078330E-03f32,
    2.60710909E-03f32,
    2.76213586E-03f32,
    2.92638101E-03f32,
    3.10039268E-03f32,
    3.28475162E-03f32,
    3.48007312E-03f32,
    3.68700903E-03f32,
    3.90625000E-03f32,
    4.13852771E-03f32,
    4.38461738E-03f32,
    4.64534029E-03f32,
    4.92156660E-03f32,
    5.21421818E-03f32,
    5.52427173E-03f32,
    5.85276202E-03f32,
    6.20078536E-03f32,
    6.56950324E-03f32,
    6.96014624E-03f32,
    7.37401807E-03f32,
    7.81250000E-03f32,
    8.27705542E-03f32,
    8.76923475E-03f32,
    9.29068059E-03f32,
    9.84313320E-03f32,
    1.04284364E-02f32,
    1.10485435E-02f32,
    1.17055240E-02f32,
    1.24015707E-02f32,
    1.31390065E-02f32,
    1.39202925E-02f32,
    1.47480361E-02f32,
    1.56250000E-02f32,
    1.65541108E-02f32,
    1.75384695E-02f32,
    1.85813612E-02f32,
    1.96862664E-02f32,
    2.08568727E-02f32,
    2.20970869E-02f32,
    2.34110481E-02f32,
    2.48031414E-02f32,
    2.62780130E-02f32,
    2.78405849E-02f32,
    2.94960723E-02f32,
    3.12500000E-02f32,
    3.31082217E-02f32,
    3.50769390E-02f32,
    3.71627223E-02f32,
    3.93725328E-02f32,
    4.17137454E-02f32,
    4.41941738E-02f32,
    4.68220962E-02f32,
    4.96062829E-02f32,
    5.25560260E-02f32,
    5.56811699E-02f32,
    5.89921445E-02f32,
    6.25000000E-02f32,
    6.62164434E-02f32,
    7.01538780E-02f32,
    7.43254447E-02f32,
    7.87450656E-02f32,
    8.34274909E-02f32,
    8.83883476E-02f32,
    9.36441923E-02f32,
    9.92125657E-02f32,
    1.05112052E-01f32,
    1.11362340E-01f32,
    1.17984289E-01f32,
    1.25000000E-01f32,
    1.32432887E-01f32,
    1.40307756E-01f32,
    1.48650889E-01f32,
    1.57490131E-01f32,
    1.66854982E-01f32,
    1.76776695E-01f32,
    1.87288385E-01f32,
    1.98425131E-01f32,
    2.10224104E-01f32,
    2.22724680E-01f32,
    2.35968578E-01f32,
    2.50000000E-01f32,
    2.64865774E-01f32,
    2.80615512E-01f32,
    2.97301779E-01f32,
    3.14980262E-01f32,
    3.33709964E-01f32,
    3.53553391E-01f32,
    3.74576769E-01f32,
    3.96850263E-01f32,
    4.20448208E-01f32,
    4.45449359E-01f32,
    4.71937156E-01f32,
    5.00000000E-01f32,
    5.29731547E-01f32,
    5.61231024E-01f32,
    5.94603558E-01f32,
    6.29960525E-01f32,
    6.67419927E-01f32,
    7.07106781E-01f32,
    7.49153538E-01f32,
    7.93700526E-01f32,
    8.40896415E-01f32,
    8.90898718E-01f32,
    9.43874313E-01f32,
    1.00000000E+00f32,
];
#[no_mangle]
pub static mut shape_filt_lf_y1_bound_tbl: [[FLOAT32; 3]; 4] = [
    [-0.994f32, -0.996f32, -1.0f32],
    [-0.99f32, -0.995f32, -0.999f32],
    [-0.98f32, -0.989f32, -0.996f32],
    [-0.97f32, -0.983f32, -0.994f32],
];
#[no_mangle]
pub static mut shape_filt_hf_y1_bound_tbl: [[FLOAT32; 3]; 5] = [
    [0.15f32, 0.75f32, 1.05f32],
    [0.43f32, 0.87f32, 1.07f32],
    [0.60f32, 0.92f32, 1.07f32],
    [0.80f32, 1.00f32, 1.06f32],
    [0.90f32, 1.04f32, 1.073f32],
];
#[no_mangle]
pub static mut shape_filt_lf_gain_offset_tbl: [[FLOAT32; 3]; 4] = [
    [3.0f32, 2.0f32, 1.2f32],
    [3.0f32, 2.0f32, 1.5f32],
    [3.0f32, 2.0f32, 2.0f32],
    [3.0f32, 2.0f32, 2.0f32],
];
#[no_mangle]
pub static mut shape_filt_hf_gain_offset_tbl: [[FLOAT32; 3]; 5] = [
    [4.5f32, 6.0f32, 3.5f32],
    [3.7f32, 4.0f32, 2.7f32],
    [3.0f32, 3.5f32, 2.0f32],
    [2.0f32, 2.5f32, 1.5f32],
    [1.5f32, 2.0f32, 1.31f32],
];
#[no_mangle]
pub static mut shape_filt_lf_radius_tbl: [FLOAT32; 4] = [
    0.988f32,
    0.98f32,
    0.96f32,
    0.94f32,
];
#[no_mangle]
pub static mut shape_filt_hf_radius_tbl: [FLOAT32; 5] = [
    0.45f32,
    0.40f32,
    0.35f32,
    0.30f32,
    0.30f32,
];
#[no_mangle]
pub static mut shape_filt_cutoff_freq_norm_hf_tbl: [FLOAT32; 5] = [
    0.15f32,
    0.20f32,
    0.25f32,
    0.35f32,
    0.45f32,
];
#[no_mangle]
pub static mut pstr_cicp_sigmoid_characteristic_param: [ia_cicp_sigmoid_characteristic_param_struct; 6] = [
    {
        let mut init = ia_cicp_sigmoid_characteristic_param_struct {
            in_out_ratio: 0.0f32,
            exp_lo: 9.0f32,
            exp_hi: 12.0f32,
        };
        init
    },
    {
        let mut init = ia_cicp_sigmoid_characteristic_param_struct {
            in_out_ratio: 0.2f32,
            exp_lo: 9.0f32,
            exp_hi: 12.0f32,
        };
        init
    },
    {
        let mut init = ia_cicp_sigmoid_characteristic_param_struct {
            in_out_ratio: 0.4f32,
            exp_lo: 9.0f32,
            exp_hi: 12.0f32,
        };
        init
    },
    {
        let mut init = ia_cicp_sigmoid_characteristic_param_struct {
            in_out_ratio: 0.6f32,
            exp_lo: 9.0f32,
            exp_hi: 12.0f32,
        };
        init
    },
    {
        let mut init = ia_cicp_sigmoid_characteristic_param_struct {
            in_out_ratio: 0.8f32,
            exp_lo: 6.0f32,
            exp_hi: 8.0f32,
        };
        init
    },
    {
        let mut init = ia_cicp_sigmoid_characteristic_param_struct {
            in_out_ratio: 1.0f32,
            exp_lo: 5.0f32,
            exp_hi: 6.0f32,
        };
        init
    },
];
#[no_mangle]
pub static mut slope_code_tbl_entries_by_size: [ia_slope_code_table_struct; 15] = [
    {
        let mut init = ia_slope_code_table_struct {
            size: 1 as WORD32,
            code: 0x1 as WORD32,
            value: 0.0f32,
            index: 7 as WORD32,
        };
        init
    },
    {
        let mut init = ia_slope_code_table_struct {
            size: 2 as WORD32,
            code: 0 as WORD32,
            value: -0.005f32,
            index: 6 as WORD32,
        };
        init
    },
    {
        let mut init = ia_slope_code_table_struct {
            size: 4 as WORD32,
            code: 0x7 as WORD32,
            value: 0.005f32,
            index: 8 as WORD32,
        };
        init
    },
    {
        let mut init = ia_slope_code_table_struct {
            size: 5 as WORD32,
            code: 0xa as WORD32,
            value: -0.1953f32,
            index: 3 as WORD32,
        };
        init
    },
    {
        let mut init = ia_slope_code_table_struct {
            size: 5 as WORD32,
            code: 0x9 as WORD32,
            value: -0.0781f32,
            index: 4 as WORD32,
        };
        init
    },
    {
        let mut init = ia_slope_code_table_struct {
            size: 5 as WORD32,
            code: 0xd as WORD32,
            value: -0.0312f32,
            index: 5 as WORD32,
        };
        init
    },
    {
        let mut init = ia_slope_code_table_struct {
            size: 5 as WORD32,
            code: 0xb as WORD32,
            value: 0.0312f32,
            index: 9 as WORD32,
        };
        init
    },
    {
        let mut init = ia_slope_code_table_struct {
            size: 6 as WORD32,
            code: 0x18 as WORD32,
            value: -3.0518f32,
            index: 0 as WORD32,
        };
        init
    },
    {
        let mut init = ia_slope_code_table_struct {
            size: 6 as WORD32,
            code: 0x11 as WORD32,
            value: 0.0781f32,
            index: 10 as WORD32,
        };
        init
    },
    {
        let mut init = ia_slope_code_table_struct {
            size: 7 as WORD32,
            code: 0x32 as WORD32,
            value: -0.4883f32,
            index: 2 as WORD32,
        };
        init
    },
    {
        let mut init = ia_slope_code_table_struct {
            size: 7 as WORD32,
            code: 0x20 as WORD32,
            value: 1.2207f32,
            index: 13 as WORD32,
        };
        init
    },
    {
        let mut init = ia_slope_code_table_struct {
            size: 7 as WORD32,
            code: 0x33 as WORD32,
            value: 3.0518f32,
            index: 14 as WORD32,
        };
        init
    },
    {
        let mut init = ia_slope_code_table_struct {
            size: 8 as WORD32,
            code: 0x42 as WORD32,
            value: -1.2207f32,
            index: 1 as WORD32,
        };
        init
    },
    {
        let mut init = ia_slope_code_table_struct {
            size: 9 as WORD32,
            code: 0x87 as WORD32,
            value: 0.1953f32,
            index: 11 as WORD32,
        };
        init
    },
    {
        let mut init = ia_slope_code_table_struct {
            size: 9 as WORD32,
            code: 0x86 as WORD32,
            value: 0.4883f32,
            index: 12 as WORD32,
        };
        init
    },
];
#[no_mangle]
pub static mut dwnmix_coeff: [FLOAT32; 16] = [
    0.0f32,
    -0.5f32,
    -1.0f32,
    -1.5f32,
    -2.0f32,
    -2.5f32,
    -3.0f32,
    -3.5f32,
    -4.0f32,
    -4.5f32,
    -5.0f32,
    -5.5f32,
    -6.0f32,
    -7.5f32,
    -9.0f32,
    -1000.0f32,
];
#[no_mangle]
pub static mut dwnmix_coeff_lfe: [FLOAT32; 16] = [
    10.0f32,
    6.0f32,
    4.5f32,
    3.0f32,
    1.5f32,
    0.0f32,
    -1.5f32,
    -3.0f32,
    -4.5f32,
    -6.0f32,
    -10.0f32,
    -15.0f32,
    -20.0f32,
    -30.0f32,
    -40.0f32,
    -1000.0f32,
];
#[no_mangle]
pub static mut drc_characteristic_order_default: [[WORD32; 3]; 11] = [
    [1 as core::ffi::c_int, 2 as core::ffi::c_int, -(1 as core::ffi::c_int)],
    [2 as core::ffi::c_int, 3 as core::ffi::c_int, 1 as core::ffi::c_int],
    [3 as core::ffi::c_int, 4 as core::ffi::c_int, 2 as core::ffi::c_int],
    [4 as core::ffi::c_int, 5 as core::ffi::c_int, 3 as core::ffi::c_int],
    [5 as core::ffi::c_int, 6 as core::ffi::c_int, 4 as core::ffi::c_int],
    [6 as core::ffi::c_int, 5 as core::ffi::c_int, -(1 as core::ffi::c_int)],
    [7 as core::ffi::c_int, 9 as core::ffi::c_int, -(1 as core::ffi::c_int)],
    [8 as core::ffi::c_int, 10 as core::ffi::c_int, -(1 as core::ffi::c_int)],
    [9 as core::ffi::c_int, 7 as core::ffi::c_int, -(1 as core::ffi::c_int)],
    [10 as core::ffi::c_int, 8 as core::ffi::c_int, -(1 as core::ffi::c_int)],
    [11 as core::ffi::c_int, 10 as core::ffi::c_int, 9 as core::ffi::c_int],
];
#[no_mangle]
pub static mut measurement_system_default_tbl: [WORD32; 12] = [
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
];
#[no_mangle]
pub static mut measurement_system_bs1770_3_tbl: [WORD32; 12] = [
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    8 as core::ffi::c_int,
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    3 as core::ffi::c_int,
    0 as core::ffi::c_int,
    5 as core::ffi::c_int,
    6 as core::ffi::c_int,
    7 as core::ffi::c_int,
    4 as core::ffi::c_int,
    2 as core::ffi::c_int,
];
#[no_mangle]
pub static mut measurement_system_user_tbl: [WORD32; 12] = [
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    0 as core::ffi::c_int,
    8 as core::ffi::c_int,
    5 as core::ffi::c_int,
    0 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    6 as core::ffi::c_int,
    7 as core::ffi::c_int,
];
#[no_mangle]
pub static mut measurement_system_expert_tbl: [WORD32; 12] = [
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    3 as core::ffi::c_int,
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    8 as core::ffi::c_int,
    0 as core::ffi::c_int,
    4 as core::ffi::c_int,
    5 as core::ffi::c_int,
    6 as core::ffi::c_int,
    7 as core::ffi::c_int,
    2 as core::ffi::c_int,
];
#[no_mangle]
pub static mut measurement_system_rms_a_tbl: [WORD32; 12] = [
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    5 as core::ffi::c_int,
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    3 as core::ffi::c_int,
    0 as core::ffi::c_int,
    8 as core::ffi::c_int,
    6 as core::ffi::c_int,
    7 as core::ffi::c_int,
    4 as core::ffi::c_int,
    2 as core::ffi::c_int,
];
#[no_mangle]
pub static mut measurement_system_rms_b_tbl: [WORD32; 12] = [
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    5 as core::ffi::c_int,
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    3 as core::ffi::c_int,
    0 as core::ffi::c_int,
    6 as core::ffi::c_int,
    8 as core::ffi::c_int,
    7 as core::ffi::c_int,
    4 as core::ffi::c_int,
    2 as core::ffi::c_int,
];
#[no_mangle]
pub static mut measurement_system_rms_c_tbl: [WORD32; 12] = [
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    5 as core::ffi::c_int,
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    3 as core::ffi::c_int,
    0 as core::ffi::c_int,
    6 as core::ffi::c_int,
    7 as core::ffi::c_int,
    8 as core::ffi::c_int,
    4 as core::ffi::c_int,
    2 as core::ffi::c_int,
];
#[no_mangle]
pub static mut measurement_system_rms_d_tbl: [WORD32; 12] = [
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    3 as core::ffi::c_int,
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    7 as core::ffi::c_int,
    0 as core::ffi::c_int,
    4 as core::ffi::c_int,
    5 as core::ffi::c_int,
    6 as core::ffi::c_int,
    8 as core::ffi::c_int,
    2 as core::ffi::c_int,
];
#[no_mangle]
pub static mut measurement_system_rms_e_tbl: [WORD32; 12] = [
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    0 as core::ffi::c_int,
    7 as core::ffi::c_int,
    5 as core::ffi::c_int,
    0 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    6 as core::ffi::c_int,
    8 as core::ffi::c_int,
];
#[no_mangle]
pub static mut measurement_method_prog_loudness_tbl: [WORD32; 12] = [
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
];
#[no_mangle]
pub static mut measurement_method_peak_loudness_tbl: [WORD32; 12] = [
    0 as core::ffi::c_int,
    7 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    6 as core::ffi::c_int,
    5 as core::ffi::c_int,
    4 as core::ffi::c_int,
    3 as core::ffi::c_int,
    2 as core::ffi::c_int,
    1 as core::ffi::c_int,
];
#[no_mangle]
pub static mut loc_sys_interface: [ia_loc_sys_interface_struct; 2] = [
    {
        let mut init = ia_loc_sys_interface_struct {
            target_config_request_type: 0 as WORD32,
            num_downmix_id_requests: 1 as WORD32,
            requested_dwnmix_id: [
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
            ],
            requested_target_layout: 0 as WORD32,
            requested_target_ch_count: 0 as WORD32,
        };
        init
    },
    {
        let mut init = ia_loc_sys_interface_struct {
            target_config_request_type: 0 as WORD32,
            num_downmix_id_requests: 1 as WORD32,
            requested_dwnmix_id: [
                2 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
            ],
            requested_target_layout: 0 as WORD32,
            requested_target_ch_count: 0 as WORD32,
        };
        init
    },
];
#[no_mangle]
pub static mut loc_loudness_norm_ctrl_interface: [ia_loc_loudness_norm_ctrl_interface_struct; 2] = [
    {
        let mut init = ia_loc_loudness_norm_ctrl_interface_struct {
            loudness_normalization_on: 0 as WORD32,
            target_loudness: 0.0f32,
        };
        init
    },
    {
        let mut init = ia_loc_loudness_norm_ctrl_interface_struct {
            loudness_normalization_on: 0 as WORD32,
            target_loudness: 0.0f32,
        };
        init
    },
];
#[no_mangle]
pub static mut loc_loudness_norm_param_interface: [ia_loc_loudness_norm_param_interface_struct; 2] = [
    {
        let mut init = ia_loc_loudness_norm_param_interface_struct {
            album_mode: 0 as WORD32,
            peak_limiter: 0 as WORD32,
            loudness_deviation_max: LOUDNESS_DEVIATION_MAX_DEFAULT,
            loudness_measurement_method: USER_METHOD_DEFINITION_PROGRAM_LOUDNESS,
            loudness_measurement_system: USER_MEASUREMENT_SYSTEM_BS_1770_3,
            loudness_measurement_pre_proc: USER_LOUDNESS_PREPROCESSING_OFF,
            device_cut_off_frequency: 20 as WORD32,
            loudness_norm_gain_db_max: LOUDNESS_NORMALIZATION_GAIN_MAX_DEFAULT
                as FLOAT32,
            loudness_norm_gain_modification_db: 0.0f32,
            output_peak_level_max: 0.0f32,
        };
        init
    },
    {
        let mut init = ia_loc_loudness_norm_param_interface_struct {
            album_mode: 0 as WORD32,
            peak_limiter: 0 as WORD32,
            loudness_deviation_max: LOUDNESS_DEVIATION_MAX_DEFAULT,
            loudness_measurement_method: USER_METHOD_DEFINITION_PROGRAM_LOUDNESS,
            loudness_measurement_system: USER_MEASUREMENT_SYSTEM_BS_1770_3,
            loudness_measurement_pre_proc: USER_LOUDNESS_PREPROCESSING_OFF,
            device_cut_off_frequency: 20 as WORD32,
            loudness_norm_gain_db_max: LOUDNESS_NORMALIZATION_GAIN_MAX_DEFAULT
                as FLOAT32,
            loudness_norm_gain_modification_db: 0.0f32,
            output_peak_level_max: 0.0f32,
        };
        init
    },
];
#[no_mangle]
pub static mut loc_dyn_range_ctrl_interface: [ia_loc_drc_interface_struct; 2] = [
    {
        let mut init = ia_loc_drc_interface_struct {
            dynamic_range_control_on: 1 as WORD32,
            num_drc_feature_requests: 3 as WORD32,
            drc_feature_req_type: [
                MATCH_EFFECT_TYPE,
                MATCH_DYNAMIC_RANGE,
                MATCH_DRC_CHARACTERISTIC,
            ],
            requested_dyn_rng_measurement_type: SHORT_TERM_LOUDNESS_TO_AVG,
            requested_dyn_range_is_single_val_flag: 1 as WORD32,
            requested_dyn_range_value: 5.0f32,
            requested_dyn_range_min_val: 3.0f32,
            requested_dyn_range_max_val: 10.0f32,
            requested_drc_characteristic: 3 as WORD32,
        };
        init
    },
    {
        let mut init = ia_loc_drc_interface_struct {
            dynamic_range_control_on: 1 as WORD32,
            num_drc_feature_requests: 3 as WORD32,
            drc_feature_req_type: [
                MATCH_EFFECT_TYPE,
                MATCH_DYNAMIC_RANGE,
                MATCH_DRC_CHARACTERISTIC,
            ],
            requested_dyn_rng_measurement_type: TOP_OF_LOUDNESS_RANGE_TO_AVG,
            requested_dyn_range_is_single_val_flag: 0 as WORD32,
            requested_dyn_range_value: 5.0f32,
            requested_dyn_range_min_val: 3.0f32,
            requested_dyn_range_max_val: 10.0f32,
            requested_drc_characteristic: 3 as WORD32,
        };
        init
    },
];
#[no_mangle]
pub static mut loc_requested_drc_effect_type_str: [ia_loc_requested_drc_effect_struct; 2] = [
    {
        let mut init = ia_loc_requested_drc_effect_struct {
            requested_num_drc_effects: 1 as WORD32,
            desired_num_drc_effects_of_requested: 1 as WORD32,
            requested_drc_effect_type: [
                EFFECT_TYPE_REQUESTED_NIGHT,
                EFFECT_TYPE_REQUESTED_GENERAL_COMPR,
                EFFECT_TYPE_REQUESTED_ARTISTIC,
                EFFECT_TYPE_REQUESTED_LIMITED,
                EFFECT_TYPE_REQUESTED_DIALOG,
            ],
        };
        init
    },
    {
        let mut init = ia_loc_requested_drc_effect_struct {
            requested_num_drc_effects: 4 as WORD32,
            desired_num_drc_effects_of_requested: 1 as WORD32,
            requested_drc_effect_type: [
                EFFECT_TYPE_REQUESTED_LOWLEVEL,
                EFFECT_TYPE_REQUESTED_NIGHT,
                EFFECT_TYPE_REQUESTED_LIMITED,
                EFFECT_TYPE_REQUESTED_ARTISTIC,
                EFFECT_TYPE_REQUESTED_DIALOG,
            ],
        };
        init
    },
];
#[no_mangle]
pub static mut loc_drc_parameter_interface: [ia_loc_drc_parameter_interface_struct; 2] = [
    {
        let mut init = ia_loc_drc_parameter_interface_struct {
            compress: 1.0f32,
            boost: 1.0f32,
            drc_characteristic_target: 0 as WORD32,
        };
        init
    },
    {
        let mut init = ia_loc_drc_parameter_interface_struct {
            compress: 1.0f32,
            boost: 1.0f32,
            drc_characteristic_target: 0 as WORD32,
        };
        init
    },
];
#[no_mangle]
pub static mut f_bands_nrm_QMF71: [FLOAT32; 71] = [
    0.004583300000000f32,
    0.000833330000000f32,
    0.002083300000000f32,
    0.005875000000000f32,
    0.009791700000000f32,
    0.014292000000000f32,
    0.019792000000000f32,
    0.027000000000000f32,
    0.035417000000000f32,
    0.042625000000000f32,
    0.056750000000000f32,
    0.072375000000000f32,
    0.088000000000000f32,
    0.103620000000000f32,
    0.119250000000000f32,
    0.134870000000000f32,
    0.150500000000000f32,
    0.166120000000000f32,
    0.181750000000000f32,
    0.197370000000000f32,
    0.213000000000000f32,
    0.228620000000000f32,
    0.244250000000000f32,
    0.259880000000000f32,
    0.275500000000000f32,
    0.291130000000000f32,
    0.306750000000000f32,
    0.322380000000000f32,
    0.338000000000000f32,
    0.353630000000000f32,
    0.369250000000000f32,
    0.384880000000000f32,
    0.400500000000000f32,
    0.416130000000000f32,
    0.431750000000000f32,
    0.447380000000000f32,
    0.463000000000000f32,
    0.478630000000000f32,
    0.494250000000000f32,
    0.509870000000000f32,
    0.525500000000000f32,
    0.541120000000000f32,
    0.556750000000000f32,
    0.572370000000000f32,
    0.588000000000000f32,
    0.603620000000000f32,
    0.619250000000000f32,
    0.634870000000000f32,
    0.650500000000000f32,
    0.666120000000000f32,
    0.681750000000000f32,
    0.697370000000000f32,
    0.713000000000000f32,
    0.728620000000000f32,
    0.744250000000000f32,
    0.759870000000000f32,
    0.775500000000000f32,
    0.791120000000000f32,
    0.806750000000000f32,
    0.822370000000000f32,
    0.838000000000000f32,
    0.853620000000000f32,
    0.869250000000000f32,
    0.884870000000000f32,
    0.900500000000000f32,
    0.916120000000000f32,
    0.931750000000000f32,
    0.947370000000000f32,
    0.963000000000000f32,
    0.974540000000000f32,
    0.999040000000000f32,
];
#[no_mangle]
pub static mut f_bands_nrm_QMF64: [FLOAT32; 64] = [
    0.0078125000000000f32,
    0.0234380000000000f32,
    0.0390620000000000f32,
    0.0546880000000000f32,
    0.0703120000000000f32,
    0.0859380000000000f32,
    0.1015600000000000f32,
    0.1171900000000000f32,
    0.1328100000000000f32,
    0.1484400000000000f32,
    0.1640600000000000f32,
    0.1796900000000000f32,
    0.1953100000000000f32,
    0.2109400000000000f32,
    0.2265600000000000f32,
    0.2421900000000000f32,
    0.2578100000000000f32,
    0.2734400000000000f32,
    0.2890600000000000f32,
    0.3046900000000000f32,
    0.3203100000000000f32,
    0.3359400000000000f32,
    0.3515600000000000f32,
    0.3671900000000000f32,
    0.3828100000000000f32,
    0.3984400000000000f32,
    0.4140600000000000f32,
    0.4296900000000000f32,
    0.4453100000000000f32,
    0.4609400000000000f32,
    0.4765600000000000f32,
    0.4921900000000000f32,
    0.5078100000000000f32,
    0.5234400000000000f32,
    0.5390600000000000f32,
    0.5546900000000000f32,
    0.5703100000000000f32,
    0.5859400000000000f32,
    0.6015600000000000f32,
    0.6171900000000000f32,
    0.6328100000000000f32,
    0.6484400000000000f32,
    0.6640600000000000f32,
    0.6796900000000000f32,
    0.6953100000000000f32,
    0.7109400000000000f32,
    0.7265600000000000f32,
    0.7421900000000000f32,
    0.7578100000000000f32,
    0.7734400000000000f32,
    0.7890600000000000f32,
    0.8046900000000000f32,
    0.8203100000000000f32,
    0.8359400000000000f32,
    0.8515600000000000f32,
    0.8671900000000000f32,
    0.8828100000000000f32,
    0.8984400000000000f32,
    0.9140600000000000f32,
    0.9296900000000000f32,
    0.9453100000000000f32,
    0.9609400000000000f32,
    0.9765600000000000f32,
    0.9921900000000000f32,
];
#[no_mangle]
pub static mut f_bands_nrm_STFT256: [FLOAT32; 257] = [
    0.000000000000000f32,
    0.003906250000000f32,
    0.007812500000000f32,
    0.011718750000000f32,
    0.015625000000000f32,
    0.019531250000000f32,
    0.023437500000000f32,
    0.027343750000000f32,
    0.031250000000000f32,
    0.035156250000000f32,
    0.039062500000000f32,
    0.042968750000000f32,
    0.046875000000000f32,
    0.050781250000000f32,
    0.054687500000000f32,
    0.058593750000000f32,
    0.062500000000000f32,
    0.066406250000000f32,
    0.070312500000000f32,
    0.074218750000000f32,
    0.078125000000000f32,
    0.082031250000000f32,
    0.085937500000000f32,
    0.089843750000000f32,
    0.093750000000000f32,
    0.097656250000000f32,
    0.101562500000000f32,
    0.105468750000000f32,
    0.109375000000000f32,
    0.113281250000000f32,
    0.117187500000000f32,
    0.121093750000000f32,
    0.125000000000000f32,
    0.128906250000000f32,
    0.132812500000000f32,
    0.136718750000000f32,
    0.140625000000000f32,
    0.144531250000000f32,
    0.148437500000000f32,
    0.152343750000000f32,
    0.156250000000000f32,
    0.160156250000000f32,
    0.164062500000000f32,
    0.167968750000000f32,
    0.171875000000000f32,
    0.175781250000000f32,
    0.179687500000000f32,
    0.183593750000000f32,
    0.187500000000000f32,
    0.191406250000000f32,
    0.195312500000000f32,
    0.199218750000000f32,
    0.203125000000000f32,
    0.207031250000000f32,
    0.210937500000000f32,
    0.214843750000000f32,
    0.218750000000000f32,
    0.222656250000000f32,
    0.226562500000000f32,
    0.230468750000000f32,
    0.234375000000000f32,
    0.238281250000000f32,
    0.242187500000000f32,
    0.246093750000000f32,
    0.250000000000000f32,
    0.253906250000000f32,
    0.257812500000000f32,
    0.261718750000000f32,
    0.265625000000000f32,
    0.269531250000000f32,
    0.273437500000000f32,
    0.277343750000000f32,
    0.281250000000000f32,
    0.285156250000000f32,
    0.289062500000000f32,
    0.292968750000000f32,
    0.296875000000000f32,
    0.300781250000000f32,
    0.304687500000000f32,
    0.308593750000000f32,
    0.312500000000000f32,
    0.316406250000000f32,
    0.320312500000000f32,
    0.324218750000000f32,
    0.328125000000000f32,
    0.332031250000000f32,
    0.335937500000000f32,
    0.339843750000000f32,
    0.343750000000000f32,
    0.347656250000000f32,
    0.351562500000000f32,
    0.355468750000000f32,
    0.359375000000000f32,
    0.363281250000000f32,
    0.367187500000000f32,
    0.371093750000000f32,
    0.375000000000000f32,
    0.378906250000000f32,
    0.382812500000000f32,
    0.386718750000000f32,
    0.390625000000000f32,
    0.394531250000000f32,
    0.398437500000000f32,
    0.402343750000000f32,
    0.406250000000000f32,
    0.410156250000000f32,
    0.414062500000000f32,
    0.417968750000000f32,
    0.421875000000000f32,
    0.425781250000000f32,
    0.429687500000000f32,
    0.433593750000000f32,
    0.437500000000000f32,
    0.441406250000000f32,
    0.445312500000000f32,
    0.449218750000000f32,
    0.453125000000000f32,
    0.457031250000000f32,
    0.460937500000000f32,
    0.464843750000000f32,
    0.468750000000000f32,
    0.472656250000000f32,
    0.476562500000000f32,
    0.480468750000000f32,
    0.484375000000000f32,
    0.488281250000000f32,
    0.492187500000000f32,
    0.496093750000000f32,
    0.500000000000000f32,
    0.503906250000000f32,
    0.507812500000000f32,
    0.511718750000000f32,
    0.515625000000000f32,
    0.519531250000000f32,
    0.523437500000000f32,
    0.527343750000000f32,
    0.531250000000000f32,
    0.535156250000000f32,
    0.539062500000000f32,
    0.542968750000000f32,
    0.546875000000000f32,
    0.550781250000000f32,
    0.554687500000000f32,
    0.558593750000000f32,
    0.562500000000000f32,
    0.566406250000000f32,
    0.570312500000000f32,
    0.574218750000000f32,
    0.578125000000000f32,
    0.582031250000000f32,
    0.585937500000000f32,
    0.589843750000000f32,
    0.593750000000000f32,
    0.597656250000000f32,
    0.601562500000000f32,
    0.605468750000000f32,
    0.609375000000000f32,
    0.613281250000000f32,
    0.617187500000000f32,
    0.621093750000000f32,
    0.625000000000000f32,
    0.628906250000000f32,
    0.632812500000000f32,
    0.636718750000000f32,
    0.640625000000000f32,
    0.644531250000000f32,
    0.648437500000000f32,
    0.652343750000000f32,
    0.656250000000000f32,
    0.660156250000000f32,
    0.664062500000000f32,
    0.667968750000000f32,
    0.671875000000000f32,
    0.675781250000000f32,
    0.679687500000000f32,
    0.683593750000000f32,
    0.687500000000000f32,
    0.691406250000000f32,
    0.695312500000000f32,
    0.699218750000000f32,
    0.703125000000000f32,
    0.707031250000000f32,
    0.710937500000000f32,
    0.714843750000000f32,
    0.718750000000000f32,
    0.722656250000000f32,
    0.726562500000000f32,
    0.730468750000000f32,
    0.734375000000000f32,
    0.738281250000000f32,
    0.742187500000000f32,
    0.746093750000000f32,
    0.750000000000000f32,
    0.753906250000000f32,
    0.757812500000000f32,
    0.761718750000000f32,
    0.765625000000000f32,
    0.769531250000000f32,
    0.773437500000000f32,
    0.777343750000000f32,
    0.781250000000000f32,
    0.785156250000000f32,
    0.789062500000000f32,
    0.792968750000000f32,
    0.796875000000000f32,
    0.800781250000000f32,
    0.804687500000000f32,
    0.808593750000000f32,
    0.812500000000000f32,
    0.816406250000000f32,
    0.820312500000000f32,
    0.824218750000000f32,
    0.828125000000000f32,
    0.832031250000000f32,
    0.835937500000000f32,
    0.839843750000000f32,
    0.843750000000000f32,
    0.847656250000000f32,
    0.851562500000000f32,
    0.855468750000000f32,
    0.859375000000000f32,
    0.863281250000000f32,
    0.867187500000000f32,
    0.871093750000000f32,
    0.875000000000000f32,
    0.878906250000000f32,
    0.882812500000000f32,
    0.886718750000000f32,
    0.890625000000000f32,
    0.894531250000000f32,
    0.898437500000000f32,
    0.902343750000000f32,
    0.906250000000000f32,
    0.910156250000000f32,
    0.914062500000000f32,
    0.917968750000000f32,
    0.921875000000000f32,
    0.925781250000000f32,
    0.929687500000000f32,
    0.933593750000000f32,
    0.937500000000000f32,
    0.941406250000000f32,
    0.945312500000000f32,
    0.949218750000000f32,
    0.953125000000000f32,
    0.957031250000000f32,
    0.960937500000000f32,
    0.964843750000000f32,
    0.968750000000000f32,
    0.972656250000000f32,
    0.976562500000000f32,
    0.980468750000000f32,
    0.984375000000000f32,
    0.988281250000000f32,
    0.992187500000000f32,
    0.996093750000000f32,
    1.000000000000000f32,
];
#[no_mangle]
pub static mut qmf_filter_coeff: [FLOAT64; 640] = [
    0 as core::ffi::c_int as FLOAT64,
    -0.00055252865047f64,
    -0.00056176925738f64,
    -0.00049475180896f64,
    -0.00048752279712f64,
    -0.00048937912498f64,
    -0.00050407143497f64,
    -0.00052265642972f64,
    -0.00054665656337f64,
    -0.00056778025613f64,
    -0.00058709304852f64,
    -0.00061327473938f64,
    -0.00063124935319f64,
    -0.00065403333621f64,
    -0.00067776907764f64,
    -0.00069416146273f64,
    -0.00071577364744f64,
    -0.00072550431222f64,
    -0.00074409418541f64,
    -0.00074905980532f64,
    -0.00076813719270f64,
    -0.00077248485949f64,
    -0.00078343322877f64,
    -0.00077798694927f64,
    -0.00078036647100f64,
    -0.00078014496257f64,
    -0.00077579773310f64,
    -0.00076307935757f64,
    -0.00075300014201f64,
    -0.00073193571525f64,
    -0.00072153919876f64,
    -0.00069179375372f64,
    -0.00066504150893f64,
    -0.00063415949025f64,
    -0.00059461189330f64,
    -0.00055645763906f64,
    -0.00051455722108f64,
    -0.00046063254803f64,
    -0.00040951214522f64,
    -0.00035011758756f64,
    -0.00028969811748f64,
    -0.00020983373440f64,
    -0.00014463809349f64,
    -0.00006173344072f64,
    0.00001349497418f64,
    0.00010943831274f64,
    0.00020430170688f64,
    0.00029495311041f64,
    0.00040265402160f64,
    0.00051073884952f64,
    0.00062393761391f64,
    0.00074580258865f64,
    0.00086084433262f64,
    0.00098859883015f64,
    0.00112501551307f64,
    0.00125778846475f64,
    0.00139024948272f64,
    0.00154432198471f64,
    0.00168680832531f64,
    0.00183482654224f64,
    0.00198411407369f64,
    0.00214615835557f64,
    0.00230172547746f64,
    0.00246256169126f64,
    0.00262017586902f64,
    0.00278704643465f64,
    0.00294694477165f64,
    0.00311254206525f64,
    0.00327396134847f64,
    0.00344188741828f64,
    0.00360082681231f64,
    0.00376039229104f64,
    0.00392074323703f64,
    0.00408197531935f64,
    0.00422642692270f64,
    0.00437307196781f64,
    0.00452098527825f64,
    0.00466064606118f64,
    0.00479325608498f64,
    0.00491376035745f64,
    0.00503930226013f64,
    0.00514073539032f64,
    0.00524611661324f64,
    0.00534716811982f64,
    0.00541967759307f64,
    0.00548760401507f64,
    0.00554757145088f64,
    0.00559380230045f64,
    0.00562206432097f64,
    0.00564551969164f64,
    0.00563891995151f64,
    0.00562661141932f64,
    0.00559171286630f64,
    0.00554043639400f64,
    0.00547537830770f64,
    0.00538389758970f64,
    0.00527157587272f64,
    0.00513822754514f64,
    0.00498396877629f64,
    0.00481094690600f64,
    0.00460395301471f64,
    0.00438018617447f64,
    0.00412516423270f64,
    0.00384564081246f64,
    0.00354012465507f64,
    0.00320918858098f64,
    0.00284467578623f64,
    0.00245085400321f64,
    0.00202741761850f64,
    0.00157846825768f64,
    0.00109023290512f64,
    0.00058322642480f64,
    0.00002760451905f64,
    -0.00054642808664f64,
    -0.00115681355227f64,
    -0.00180394725893f64,
    -0.00248267236449f64,
    -0.00319337783900f64,
    -0.00394011240522f64,
    -0.00472225962400f64,
    -0.00553372111088f64,
    -0.00637922932685f64,
    -0.00726158168517f64,
    -0.00817982333726f64,
    -0.00913253296085f64,
    -0.01011502154986f64,
    -0.01113155480321f64,
    -0.01218499959508f64,
    0.01327182200351f64,
    0.01439046660792f64,
    0.01554055533423f64,
    0.01673247129989f64,
    0.01794333813443f64,
    0.01918724313698f64,
    0.02045317933555f64,
    0.02174675502535f64,
    0.02306801692862f64,
    0.02441609920285f64,
    0.02578758475467f64,
    0.02718594296329f64,
    0.02860721736385f64,
    0.03005026574279f64,
    0.03150176087389f64,
    0.03297540810337f64,
    0.03446209487686f64,
    0.03596975605542f64,
    0.03748128504252f64,
    0.03900536794745f64,
    0.04053491705584f64,
    0.04206490946367f64,
    0.04360975421304f64,
    0.04514884056413f64,
    0.04668430272642f64,
    0.04821657200672f64,
    0.04973857556014f64,
    0.05125561555216f64,
    0.05276307465207f64,
    0.05424527683589f64,
    0.05571736482138f64,
    0.05716164501299f64,
    0.05859156836260f64,
    0.05998374801761f64,
    0.06134551717207f64,
    0.06268578081172f64,
    0.06397158980681f64,
    0.06522471064380f64,
    0.06643675122104f64,
    0.06760759851228f64,
    0.06870438283512f64,
    0.06976302447127f64,
    0.07076287107266f64,
    0.07170026731102f64,
    0.07256825833083f64,
    0.07336202550803f64,
    0.07410036424342f64,
    0.07474525581194f64,
    0.07531373362019f64,
    0.07580083586584f64,
    0.07619924793396f64,
    0.07649921704119f64,
    0.07670934904245f64,
    0.07681739756964f64,
    0.07682300113923f64,
    0.07672049241746f64,
    0.07650507183194f64,
    0.07617483218536f64,
    0.07573057565061f64,
    0.07515762552870f64,
    0.07446643947564f64,
    0.07364060057620f64,
    0.07267746427299f64,
    0.07158263647903f64,
    0.07035330735093f64,
    0.06896640131951f64,
    0.06745250215166f64,
    0.06576906686508f64,
    0.06394448059633f64,
    0.06196027790387f64,
    0.05981665708090f64,
    0.05751526919867f64,
    0.05504600343009f64,
    0.05240938217366f64,
    0.04959786763445f64,
    0.04663033051701f64,
    0.04347687821958f64,
    0.04014582784127f64,
    0.03664181168133f64,
    0.03295839306691f64,
    0.02908240060125f64,
    0.02503075618909f64,
    0.02079970728622f64,
    0.01637012582228f64,
    0.01176238327857f64,
    0.00696368621617f64,
    0.00197656014503f64,
    -0.00320868968304f64,
    -0.00857117491366f64,
    -0.01412888273558f64,
    -0.01988341292573f64,
    -0.02582272888064f64,
    -0.03195312745332f64,
    -0.03827765720822f64,
    -0.04478068215856f64,
    -0.05148041767934f64,
    -0.05837053268336f64,
    -0.06544098531359f64,
    -0.07269433008129f64,
    -0.08013729344279f64,
    -0.08775475365593f64,
    -0.09555333528914f64,
    -0.10353295311463f64,
    -0.11168269317730f64,
    -0.12000779846800f64,
    -0.12850028503878f64,
    -0.13715517611934f64,
    -0.14597664911870f64,
    -0.15496070710605f64,
    -0.16409588556669f64,
    -0.17338081721706f64,
    -0.18281725485142f64,
    -0.19239667457267f64,
    -0.20212501768103f64,
    -0.21197358538056f64,
    -0.22196526964149f64,
    -0.23206908706791f64,
    -0.24230168845974f64,
    -0.25264803095722f64,
    -0.26310532994603f64,
    -0.27366340405625f64,
    -0.28432141891085f64,
    -0.29507167170646f64,
    -0.30590985751916f64,
    -0.31682789136456f64,
    -0.32781137272105f64,
    -0.33887226938665f64,
    -0.34999141229310f64,
    0.36115899031355f64,
    0.37237955463061f64,
    0.38363500139043f64,
    0.39492117615675f64,
    0.40623176767625f64,
    0.41756968968409f64,
    0.42891199207373f64,
    0.44025537543665f64,
    0.45159965356824f64,
    0.46293080852757f64,
    0.47424532146115f64,
    0.48552530911099f64,
    0.49677082545707f64,
    0.50798175000434f64,
    0.51912349702391f64,
    0.53022408956855f64,
    0.54125534487322f64,
    0.55220512585061f64,
    0.56307891401370f64,
    0.57385241316923f64,
    0.58454032354679f64,
    0.59511230862496f64,
    0.60557835389180f64,
    0.61591099320291f64,
    0.62612426956055f64,
    0.63619801077286f64,
    0.64612696959461f64,
    0.65590163024671f64,
    0.66551398801627f64,
    0.67496631901712f64,
    0.68423532934598f64,
    0.69332823767032f64,
    0.70223887193539f64,
    0.71094104263095f64,
    0.71944626349561f64,
    0.72774489002994f64,
    0.73582117582769f64,
    0.74368278636488f64,
    0.75131374561237f64,
    0.75870807608242f64,
    0.76586748650939f64,
    0.77277808813327f64,
    0.77942875190216f64,
    0.78583531203920f64,
    0.79197358416424f64,
    0.79784664137700f64,
    0.80344857518505f64,
    0.80876950044491f64,
    0.81381912706217f64,
    0.81857760046468f64,
    0.82304198905409f64,
    0.82722753473360f64,
    0.83110384571520f64,
    0.83469373618402f64,
    0.83797173378865f64,
    0.84095413924722f64,
    0.84362382812005f64,
    0.84598184698206f64,
    0.84803157770763f64,
    0.84978051984268f64,
    0.85119715249343f64,
    0.85230470352147f64,
    0.85310209497017f64,
    0.85357205739107f64,
    0.85373856005937f64,
    0.85357205739107f64,
    0.85310209497017f64,
    0.85230470352147f64,
    0.85119715249343f64,
    0.84978051984268f64,
    0.84803157770763f64,
    0.84598184698206f64,
    0.84362382812005f64,
    0.84095413924722f64,
    0.83797173378865f64,
    0.83469373618402f64,
    0.83110384571520f64,
    0.82722753473360f64,
    0.82304198905409f64,
    0.81857760046468f64,
    0.81381912706217f64,
    0.80876950044491f64,
    0.80344857518505f64,
    0.79784664137700f64,
    0.79197358416424f64,
    0.78583531203920f64,
    0.77942875190216f64,
    0.77277808813327f64,
    0.76586748650939f64,
    0.75870807608242f64,
    0.75131374561237f64,
    0.74368278636488f64,
    0.73582117582769f64,
    0.72774489002994f64,
    0.71944626349561f64,
    0.71094104263095f64,
    0.70223887193539f64,
    0.69332823767032f64,
    0.68423532934598f64,
    0.67496631901712f64,
    0.66551398801627f64,
    0.65590163024671f64,
    0.64612696959461f64,
    0.63619801077286f64,
    0.62612426956055f64,
    0.61591099320291f64,
    0.60557835389180f64,
    0.59511230862496f64,
    0.58454032354679f64,
    0.57385241316923f64,
    0.56307891401370f64,
    0.55220512585061f64,
    0.54125534487322f64,
    0.53022408956855f64,
    0.51912349702391f64,
    0.50798175000434f64,
    0.49677082545707f64,
    0.48552530911099f64,
    0.47424532146115f64,
    0.46293080852757f64,
    0.45159965356824f64,
    0.44025537543665f64,
    0.42891199207373f64,
    0.41756968968409f64,
    0.40623176767625f64,
    0.39492117615675f64,
    0.38363500139043f64,
    0.37237955463061f64,
    -0.36115899031355f64,
    -0.34999141229310f64,
    -0.33887226938665f64,
    -0.32781137272105f64,
    -0.31682789136456f64,
    -0.30590985751916f64,
    -0.29507167170646f64,
    -0.28432141891085f64,
    -0.27366340405625f64,
    -0.26310532994603f64,
    -0.25264803095722f64,
    -0.24230168845974f64,
    -0.23206908706791f64,
    -0.22196526964149f64,
    -0.21197358538056f64,
    -0.20212501768103f64,
    -0.19239667457267f64,
    -0.18281725485142f64,
    -0.17338081721706f64,
    -0.16409588556669f64,
    -0.15496070710605f64,
    -0.14597664911870f64,
    -0.13715517611934f64,
    -0.12850028503878f64,
    -0.12000779846800f64,
    -0.11168269317730f64,
    -0.10353295311463f64,
    -0.09555333528914f64,
    -0.08775475365593f64,
    -0.08013729344279f64,
    -0.07269433008129f64,
    -0.06544098531359f64,
    -0.05837053268336f64,
    -0.05148041767934f64,
    -0.04478068215856f64,
    -0.03827765720822f64,
    -0.03195312745332f64,
    -0.02582272888064f64,
    -0.01988341292573f64,
    -0.01412888273558f64,
    -0.00857117491366f64,
    -0.00320868968304f64,
    0.00197656014503f64,
    0.00696368621617f64,
    0.01176238327857f64,
    0.01637012582228f64,
    0.02079970728622f64,
    0.02503075618909f64,
    0.02908240060125f64,
    0.03295839306691f64,
    0.03664181168133f64,
    0.04014582784127f64,
    0.04347687821958f64,
    0.04663033051701f64,
    0.04959786763445f64,
    0.05240938217366f64,
    0.05504600343009f64,
    0.05751526919867f64,
    0.05981665708090f64,
    0.06196027790387f64,
    0.06394448059633f64,
    0.06576906686508f64,
    0.06745250215166f64,
    0.06896640131951f64,
    0.07035330735093f64,
    0.07158263647903f64,
    0.07267746427299f64,
    0.07364060057620f64,
    0.07446643947564f64,
    0.07515762552870f64,
    0.07573057565061f64,
    0.07617483218536f64,
    0.07650507183194f64,
    0.07672049241746f64,
    0.07682300113923f64,
    0.07681739756964f64,
    0.07670934904245f64,
    0.07649921704119f64,
    0.07619924793396f64,
    0.07580083586584f64,
    0.07531373362019f64,
    0.07474525581194f64,
    0.07410036424342f64,
    0.07336202550803f64,
    0.07256825833083f64,
    0.07170026731102f64,
    0.07076287107266f64,
    0.06976302447127f64,
    0.06870438283512f64,
    0.06760759851228f64,
    0.06643675122104f64,
    0.06522471064380f64,
    0.06397158980681f64,
    0.06268578081172f64,
    0.06134551717207f64,
    0.05998374801761f64,
    0.05859156836260f64,
    0.05716164501299f64,
    0.05571736482138f64,
    0.05424527683589f64,
    0.05276307465207f64,
    0.05125561555216f64,
    0.04973857556014f64,
    0.04821657200672f64,
    0.04668430272642f64,
    0.04514884056413f64,
    0.04360975421304f64,
    0.04206490946367f64,
    0.04053491705584f64,
    0.03900536794745f64,
    0.03748128504252f64,
    0.03596975605542f64,
    0.03446209487686f64,
    0.03297540810337f64,
    0.03150176087389f64,
    0.03005026574279f64,
    0.02860721736385f64,
    0.02718594296329f64,
    0.02578758475467f64,
    0.02441609920285f64,
    0.02306801692862f64,
    0.02174675502535f64,
    0.02045317933555f64,
    0.01918724313698f64,
    0.01794333813443f64,
    0.01673247129989f64,
    0.01554055533423f64,
    0.01439046660792f64,
    -0.01327182200351f64,
    -0.01218499959508f64,
    -0.01113155480321f64,
    -0.01011502154986f64,
    -0.00913253296085f64,
    -0.00817982333726f64,
    -0.00726158168517f64,
    -0.00637922932685f64,
    -0.00553372111088f64,
    -0.00472225962400f64,
    -0.00394011240522f64,
    -0.00319337783900f64,
    -0.00248267236449f64,
    -0.00180394725893f64,
    -0.00115681355227f64,
    -0.00054642808664f64,
    0.00002760451905f64,
    0.00058322642480f64,
    0.00109023290512f64,
    0.00157846825768f64,
    0.00202741761850f64,
    0.00245085400321f64,
    0.00284467578623f64,
    0.00320918858098f64,
    0.00354012465507f64,
    0.00384564081246f64,
    0.00412516423270f64,
    0.00438018617447f64,
    0.00460395301471f64,
    0.00481094690600f64,
    0.00498396877629f64,
    0.00513822754514f64,
    0.00527157587272f64,
    0.00538389758970f64,
    0.00547537830770f64,
    0.00554043639400f64,
    0.00559171286630f64,
    0.00562661141932f64,
    0.00563891995151f64,
    0.00564551969164f64,
    0.00562206432097f64,
    0.00559380230045f64,
    0.00554757145088f64,
    0.00548760401507f64,
    0.00541967759307f64,
    0.00534716811982f64,
    0.00524611661324f64,
    0.00514073539032f64,
    0.00503930226013f64,
    0.00491376035745f64,
    0.00479325608498f64,
    0.00466064606118f64,
    0.00452098527825f64,
    0.00437307196781f64,
    0.00422642692270f64,
    0.00408197531935f64,
    0.00392074323703f64,
    0.00376039229104f64,
    0.00360082681231f64,
    0.00344188741828f64,
    0.00327396134847f64,
    0.00311254206525f64,
    0.00294694477165f64,
    0.00278704643465f64,
    0.00262017586902f64,
    0.00246256169126f64,
    0.00230172547746f64,
    0.00214615835557f64,
    0.00198411407369f64,
    0.00183482654224f64,
    0.00168680832531f64,
    0.00154432198471f64,
    0.00139024948272f64,
    0.00125778846475f64,
    0.00112501551307f64,
    0.00098859883015f64,
    0.00086084433262f64,
    0.00074580258865f64,
    0.00062393761391f64,
    0.00051073884952f64,
    0.00040265402160f64,
    0.00029495311041f64,
    0.00020430170688f64,
    0.00010943831274f64,
    0.00001349497418f64,
    -0.00006173344072f64,
    -0.00014463809349f64,
    -0.00020983373440f64,
    -0.00028969811748f64,
    -0.00035011758756f64,
    -0.00040951214522f64,
    -0.00046063254803f64,
    -0.00051455722108f64,
    -0.00055645763906f64,
    -0.00059461189330f64,
    -0.00063415949025f64,
    -0.00066504150893f64,
    -0.00069179375372f64,
    -0.00072153919876f64,
    -0.00073193571525f64,
    -0.00075300014201f64,
    -0.00076307935757f64,
    -0.00077579773310f64,
    -0.00078014496257f64,
    -0.00078036647100f64,
    -0.00077798694927f64,
    -0.00078343322877f64,
    -0.00077248485949f64,
    -0.00076813719270f64,
    -0.00074905980532f64,
    -0.00074409418541f64,
    -0.00072550431222f64,
    -0.00071577364744f64,
    -0.00069416146273f64,
    -0.00067776907764f64,
    -0.00065403333621f64,
    -0.00063124935319f64,
    -0.00061327473938f64,
    -0.00058709304852f64,
    -0.00056778025613f64,
    -0.00054665656337f64,
    -0.00052265642972f64,
    -0.00050407143497f64,
    -0.00048937912498f64,
    -0.00048752279712f64,
    -0.00049475180896f64,
    -0.00056176925738f64,
    -0.00055252865047f64,
];
#[no_mangle]
pub static mut normal_cross_freq: [ia_filter_bank_params_struct; 16] = [
    {
        let mut init = ia_filter_bank_params_struct {
            f_cross_norm: 2.0f32 / 1024.0f32,
            gamma: 0.0000373252f32,
            delta: 0.9913600345f32,
        };
        init
    },
    {
        let mut init = ia_filter_bank_params_struct {
            f_cross_norm: 3.0f32 / 1024.0f32,
            gamma: 0.0000836207f32,
            delta: 0.9870680830f32,
        };
        init
    },
    {
        let mut init = ia_filter_bank_params_struct {
            f_cross_norm: 4.0f32 / 1024.0f32,
            gamma: 0.0001480220f32,
            delta: 0.9827947083f32,
        };
        init
    },
    {
        let mut init = ia_filter_bank_params_struct {
            f_cross_norm: 5.0f32 / 1024.0f32,
            gamma: 0.0002302960f32,
            delta: 0.9785398263f32,
        };
        init
    },
    {
        let mut init = ia_filter_bank_params_struct {
            f_cross_norm: 6.0f32 / 1024.0f32,
            gamma: 0.0003302134f32,
            delta: 0.9743033527f32,
        };
        init
    },
    {
        let mut init = ia_filter_bank_params_struct {
            f_cross_norm: 2.0f32 / 256.0f32,
            gamma: 0.0005820761f32,
            delta: 0.9658852897f32,
        };
        init
    },
    {
        let mut init = ia_filter_bank_params_struct {
            f_cross_norm: 3.0f32 / 256.0f32,
            gamma: 0.0012877837f32,
            delta: 0.9492662926f32,
        };
        init
    },
    {
        let mut init = ia_filter_bank_params_struct {
            f_cross_norm: 2.0f32 / 128.0f32,
            gamma: 0.0022515827f32,
            delta: 0.9329321561f32,
        };
        init
    },
    {
        let mut init = ia_filter_bank_params_struct {
            f_cross_norm: 3.0f32 / 128.0f32,
            gamma: 0.0049030350f32,
            delta: 0.9010958535f32,
        };
        init
    },
    {
        let mut init = ia_filter_bank_params_struct {
            f_cross_norm: 2.0f32 / 64.0f32,
            gamma: 0.0084426929f32,
            delta: 0.8703307793f32,
        };
        init
    },
    {
        let mut init = ia_filter_bank_params_struct {
            f_cross_norm: 3.0f32 / 64.0f32,
            gamma: 0.0178631928f32,
            delta: 0.8118317459f32,
        };
        init
    },
    {
        let mut init = ia_filter_bank_params_struct {
            f_cross_norm: 2.0f32 / 32.0f32,
            gamma: 0.0299545822f32,
            delta: 0.7570763753f32,
        };
        init
    },
    {
        let mut init = ia_filter_bank_params_struct {
            f_cross_norm: 3.0f32 / 32.0f32,
            gamma: 0.0604985076f32,
            delta: 0.6574551915f32,
        };
        init
    },
    {
        let mut init = ia_filter_bank_params_struct {
            f_cross_norm: 2.0f32 / 16.0f32,
            gamma: 0.0976310729f32,
            delta: 0.5690355937f32,
        };
        init
    },
    {
        let mut init = ia_filter_bank_params_struct {
            f_cross_norm: 3.0f32 / 16.0f32,
            gamma: 0.1866943331f32,
            delta: 0.4181633458f32,
        };
        init
    },
    {
        let mut init = ia_filter_bank_params_struct {
            f_cross_norm: 2.0f32 / 8.0f32,
            gamma: 0.2928932188f32,
            delta: 0.2928932188f32,
        };
        init
    },
];
