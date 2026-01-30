pub type WORD16 = core::ffi::c_short;
pub type WORD32 = core::ffi::c_int;
pub type FLOAT32 = core::ffi::c_float;
#[no_mangle]
pub static mut ia_ec_interpolation_fac: [WORD16; 4] = [
    0x4000 as core::ffi::c_int as WORD16,
    0x4c1b as core::ffi::c_int as WORD16,
    0x5a82 as core::ffi::c_int as WORD16,
    0x6ba2 as core::ffi::c_int as WORD16,
];
#[no_mangle]
pub static mut ia_ec_fade_factors: [FLOAT32; 9] = [
    1.00000f32,
    0.875f32,
    0.750f32,
    0.625f32,
    0.500f32,
    0.375f32,
    0.250f32,
    0.125f32,
    0.00000f32,
];
#[no_mangle]
pub static mut ia_ec_fade_factors_fix: [WORD32; 9] = [
    1073741824 as core::ffi::c_int,
    939524096 as core::ffi::c_int,
    805306368 as core::ffi::c_int,
    671088640 as core::ffi::c_int,
    536870912 as core::ffi::c_int,
    402653184 as core::ffi::c_int,
    268435456 as core::ffi::c_int,
    134217728 as core::ffi::c_int,
    0 as core::ffi::c_int,
];
#[no_mangle]
pub static mut ixheaacd_exc_fade_fac: [FLOAT32; 8] = [
    0.8f32,
    0.7f32,
    0.6f32,
    0.5f32,
    0.4f32,
    0.3f32,
    0.2f32,
    0.1f32,
];
