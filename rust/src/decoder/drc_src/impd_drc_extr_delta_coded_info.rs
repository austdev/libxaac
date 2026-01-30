extern "C" {
    static ia_drc_gain_tbls_prof_0_1: [ia_delta_gain_code_table_struct; 25];
    static ia_drc_gain_tbls_prof_2: [ia_delta_gain_code_table_struct; 49];
}
pub type WORD32 = core::ffi::c_int;
pub type FLOAT32 = core::ffi::c_float;
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
#[no_mangle]
pub unsafe extern "C" fn impd_init_tbls(
    num_gain_max_values: WORD32,
    mut str_tables: *mut ia_tables_struct,
) -> VOID {
    impd_gen_delta_time_code_tbl(
        num_gain_max_values,
        ((*str_tables).delta_time_code_table).as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn impd_get_delta_gain_code_tbl(
    gain_coding_profile: WORD32,
    mut delta_time_code_tbl: *mut *const ia_delta_gain_code_table_struct,
    mut num_entries: *mut WORD32,
) {
    if gain_coding_profile == GAIN_CODING_PROFILE_CLIPPING {
        *delta_time_code_tbl = ia_drc_gain_tbls_prof_2.as_ptr();
        *num_entries = NUM_GAIN_TBL_PROF_2_ENTRIES as WORD32;
    } else {
        *delta_time_code_tbl = ia_drc_gain_tbls_prof_0_1.as_ptr();
        *num_entries = NUM_GAIN_TBL_PROF_0_1_ENTRIES as WORD32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn impd_gen_delta_time_code_tbl(
    num_gain_max_values: WORD32,
    mut delta_time_code_tbl_item: *mut ia_delta_time_code_table_entry_struct,
) {
    let mut n: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut Z: WORD32 = 1 as WORD32;
    while (1 as core::ffi::c_int) << Z < 2 as WORD32 * num_gain_max_values {
        Z += 1;
    }
    (*delta_time_code_tbl_item.offset(0 as core::ffi::c_int as isize)).size = -(1
        as core::ffi::c_int) as WORD32;
    (*delta_time_code_tbl_item.offset(0 as core::ffi::c_int as isize)).code = -(1
        as core::ffi::c_int) as WORD32;
    (*delta_time_code_tbl_item.offset(0 as core::ffi::c_int as isize)).value = -(1
        as core::ffi::c_int) as WORD32;
    (*delta_time_code_tbl_item.offset(1 as core::ffi::c_int as isize)).size = 2
        as core::ffi::c_int as WORD32;
    (*delta_time_code_tbl_item.offset(1 as core::ffi::c_int as isize)).code = 0
        as core::ffi::c_int as WORD32;
    (*delta_time_code_tbl_item.offset(1 as core::ffi::c_int as isize)).value = 1
        as core::ffi::c_int as WORD32;
    n = 0 as core::ffi::c_int as WORD32;
    while n < 4 as core::ffi::c_int {
        (*delta_time_code_tbl_item
            .offset((n as core::ffi::c_int + 2 as core::ffi::c_int) as isize))
            .size = 4 as core::ffi::c_int as WORD32;
        (*delta_time_code_tbl_item
            .offset((n as core::ffi::c_int + 2 as core::ffi::c_int) as isize))
            .code = 0x4 as WORD32 + n;
        (*delta_time_code_tbl_item
            .offset((n as core::ffi::c_int + 2 as core::ffi::c_int) as isize))
            .value = (n as core::ffi::c_int + 2 as core::ffi::c_int) as WORD32;
        n += 1;
    }
    n = 0 as core::ffi::c_int as WORD32;
    while n < 8 as core::ffi::c_int {
        (*delta_time_code_tbl_item
            .offset((n as core::ffi::c_int + 6 as core::ffi::c_int) as isize))
            .size = 5 as core::ffi::c_int as WORD32;
        (*delta_time_code_tbl_item
            .offset((n as core::ffi::c_int + 6 as core::ffi::c_int) as isize))
            .code = 0x10 as WORD32 + n;
        (*delta_time_code_tbl_item
            .offset((n as core::ffi::c_int + 6 as core::ffi::c_int) as isize))
            .value = (n as core::ffi::c_int + 6 as core::ffi::c_int) as WORD32;
        n += 1;
    }
    k = (2 as core::ffi::c_int * num_gain_max_values as core::ffi::c_int
        - 14 as core::ffi::c_int + 1 as core::ffi::c_int) as WORD32;
    n = 0 as core::ffi::c_int as WORD32;
    while n < k {
        (*delta_time_code_tbl_item
            .offset((n as core::ffi::c_int + 14 as core::ffi::c_int) as isize))
            .size = 2 as WORD32 + Z;
        (*delta_time_code_tbl_item
            .offset((n as core::ffi::c_int + 14 as core::ffi::c_int) as isize))
            .code = ((0x3 as WORD32) << Z) + n;
        (*delta_time_code_tbl_item
            .offset((n as core::ffi::c_int + 14 as core::ffi::c_int) as isize))
            .value = (n as core::ffi::c_int + 14 as core::ffi::c_int) as WORD32;
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn impd_get_delta_tmin(sampling_rate: WORD32) -> WORD32 {
    let mut lowerBound: WORD32 = (0.5f32
        + 0.0005f32 * sampling_rate as core::ffi::c_float) as WORD32;
    let mut result: WORD32 = 1 as WORD32;
    while result <= lowerBound {
        result = result << 1 as core::ffi::c_int;
    }
    return result;
}
pub const GAIN_CODING_PROFILE_CLIPPING: core::ffi::c_int = 2 as core::ffi::c_int;
pub const NUM_GAIN_TBL_PROF_0_1_ENTRIES: core::ffi::c_int = 25 as core::ffi::c_int;
pub const NUM_GAIN_TBL_PROF_2_ENTRIES: core::ffi::c_int = 49 as core::ffi::c_int;
