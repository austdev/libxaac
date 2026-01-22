extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(
        __nptr: *const core::ffi::c_char,
        __endptr: *mut *mut core::ffi::c_char,
        __base: core::ffi::c_int,
    ) -> core::ffi::c_long;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn printf(__format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn fgets(
        __s: *mut core::ffi::c_char,
        __n: core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut core::ffi::c_char;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn strncmp(
        __s1: *const core::ffi::c_char,
        __s2: *const core::ffi::c_char,
        __n: size_t,
    ) -> core::ffi::c_int;
}
pub type pCHAR8 = *mut core::ffi::c_char;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type size_t = usize;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: core::ffi::c_int,
    pub _IO_read_ptr: *mut core::ffi::c_char,
    pub _IO_read_end: *mut core::ffi::c_char,
    pub _IO_read_base: *mut core::ffi::c_char,
    pub _IO_write_base: *mut core::ffi::c_char,
    pub _IO_write_ptr: *mut core::ffi::c_char,
    pub _IO_write_end: *mut core::ffi::c_char,
    pub _IO_buf_base: *mut core::ffi::c_char,
    pub _IO_buf_end: *mut core::ffi::c_char,
    pub _IO_save_base: *mut core::ffi::c_char,
    pub _IO_backup_base: *mut core::ffi::c_char,
    pub _IO_save_end: *mut core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: core::ffi::c_int,
    pub _flags2: core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: core::ffi::c_ushort,
    pub _vtable_offset: core::ffi::c_schar,
    pub _shortbuf: [core::ffi::c_char; 1],
    pub _lock: *mut core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: core::ffi::c_int,
    pub _unused2: [core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct metadata_info {
    pub ia_mp4_stsz_size: *mut UWORD32,
    pub ia_mp4_stsz_entries: UWORD32,
    pub fill_once: UWORD32,
    pub movie_time_scale: UWORD32,
    pub media_time_scale: UWORD32,
    pub dec_info_init: UWORD32,
    pub g_track_count: UWORD32,
    pub useEditlist: [UWORD32; 50],
    pub startOffsetInSamples: [UWORD32; 50],
    pub playTimeInSamples: [UWORD32; 50],
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const NULL_0: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const core::ffi::c_char) -> core::ffi::c_int {
    return strtol(__nptr, NULL as *mut *mut core::ffi::c_char, 10 as core::ffi::c_int)
        as core::ffi::c_int;
}
pub const MAX_TRACKS_PER_LAYER: core::ffi::c_int = 50 as core::ffi::c_int;
pub const IA_MAX_CMDLINE_LENGTH: core::ffi::c_int = 100 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn metadata_mp4_stsz_size_free(mut meta_info: *mut metadata_info) {
    if !((*meta_info).ia_mp4_stsz_size).is_null() {
        free((*meta_info).ia_mp4_stsz_size as *mut core::ffi::c_void);
        (*meta_info).ia_mp4_stsz_size = 0 as *mut UWORD32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_read_metadata_info(
    mut g_pf_metadata: *mut FILE,
    mut meta_info: *mut metadata_info,
) -> core::ffi::c_int {
    let mut cmd: [core::ffi::c_char; 100] = [0; 100];
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut l: WORD32 = 0;
    l = 0 as core::ffi::c_int as WORD32;
    k = l;
    j = k;
    i = j;
    metadata_mp4_stsz_size_free(meta_info);
    while !(fgets(cmd.as_mut_ptr(), IA_MAX_CMDLINE_LENGTH, g_pf_metadata)).is_null() {
        if strncmp(
            cmd.as_mut_ptr() as *const core::ffi::c_char,
            b"-dec_info_init:\0" as *const u8 as *const core::ffi::c_char,
            15 as size_t,
        ) == 0
        {
            let mut pb_arg_val: pCHAR8 = cmd
                .as_mut_ptr()
                .offset(15 as core::ffi::c_int as isize);
            let mut dec_info_init: UWORD32 = atoi(pb_arg_val as *const core::ffi::c_char)
                as UWORD32;
            (*meta_info).dec_info_init = dec_info_init;
        } else if strncmp(
            cmd.as_mut_ptr() as *const core::ffi::c_char,
            b"-g_track_count:\0" as *const u8 as *const core::ffi::c_char,
            15 as size_t,
        ) == 0
        {
            let mut pb_arg_val_0: pCHAR8 = cmd
                .as_mut_ptr()
                .offset(15 as core::ffi::c_int as isize);
            let mut g_track_count: UWORD32 = atoi(
                pb_arg_val_0 as *const core::ffi::c_char,
            ) as UWORD32;
            (*meta_info).g_track_count = g_track_count;
        } else if strncmp(
            cmd.as_mut_ptr() as *const core::ffi::c_char,
            b"-movie_time_scale:\0" as *const u8 as *const core::ffi::c_char,
            18 as size_t,
        ) == 0
        {
            let mut pb_arg_val_1: pCHAR8 = cmd
                .as_mut_ptr()
                .offset(18 as core::ffi::c_int as isize);
            let mut movie_time_scale: UWORD32 = atoi(
                pb_arg_val_1 as *const core::ffi::c_char,
            ) as UWORD32;
            (*meta_info).movie_time_scale = movie_time_scale;
        } else if strncmp(
            cmd.as_mut_ptr() as *const core::ffi::c_char,
            b"-media_time_scale:\0" as *const u8 as *const core::ffi::c_char,
            18 as size_t,
        ) == 0
        {
            let mut pb_arg_val_2: pCHAR8 = cmd
                .as_mut_ptr()
                .offset(18 as core::ffi::c_int as isize);
            let mut media_time_scale: UWORD32 = atoi(
                pb_arg_val_2 as *const core::ffi::c_char,
            ) as UWORD32;
            (*meta_info).media_time_scale = media_time_scale;
        } else if strncmp(
            cmd.as_mut_ptr() as *const core::ffi::c_char,
            b"-ia_mp4_stsz_entries:\0" as *const u8 as *const core::ffi::c_char,
            21 as size_t,
        ) == 0
        {
            let mut pb_arg_val_3: pCHAR8 = cmd
                .as_mut_ptr()
                .offset(21 as core::ffi::c_int as isize);
            let mut ia_mp4_stsz_entries: UWORD32 = atoi(
                pb_arg_val_3 as *const core::ffi::c_char,
            ) as UWORD32;
            (*meta_info).ia_mp4_stsz_entries = ia_mp4_stsz_entries;
            metadata_mp4_stsz_size_free(meta_info);
            (*meta_info).ia_mp4_stsz_size = malloc(
                (::core::mem::size_of::<UWORD32>() as size_t)
                    .wrapping_mul(ia_mp4_stsz_entries as size_t),
            ) as *mut UWORD32;
            memset(
                (*meta_info).ia_mp4_stsz_size as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (::core::mem::size_of::<UWORD32>() as size_t)
                    .wrapping_mul(ia_mp4_stsz_entries as size_t),
            );
        } else if strncmp(
            cmd.as_mut_ptr() as *const core::ffi::c_char,
            b"-playTimeInSamples:\0" as *const u8 as *const core::ffi::c_char,
            19 as size_t,
        ) == 0
        {
            let mut pb_arg_val_4: pCHAR8 = cmd
                .as_mut_ptr()
                .offset(19 as core::ffi::c_int as isize);
            let mut playTimeInSamples: UWORD32 = atoi(
                pb_arg_val_4 as *const core::ffi::c_char,
            ) as UWORD32;
            (*meta_info).playTimeInSamples[i as usize] = playTimeInSamples;
            i += 1;
        } else if strncmp(
            cmd.as_mut_ptr() as *const core::ffi::c_char,
            b"-startOffsetInSamples:\0" as *const u8 as *const core::ffi::c_char,
            22 as size_t,
        ) == 0
        {
            let mut pb_arg_val_5: pCHAR8 = cmd
                .as_mut_ptr()
                .offset(22 as core::ffi::c_int as isize);
            let mut startOffsetInSamples: UWORD32 = atoi(
                pb_arg_val_5 as *const core::ffi::c_char,
            ) as UWORD32;
            (*meta_info).startOffsetInSamples[j as usize] = startOffsetInSamples;
            j += 1;
        } else if strncmp(
            cmd.as_mut_ptr() as *const core::ffi::c_char,
            b"-useEditlist:\0" as *const u8 as *const core::ffi::c_char,
            13 as size_t,
        ) == 0
        {
            let mut pb_arg_val_6: pCHAR8 = cmd
                .as_mut_ptr()
                .offset(13 as core::ffi::c_int as isize);
            let mut useEditlist: UWORD32 = atoi(pb_arg_val_6 as *const core::ffi::c_char)
                as UWORD32;
            (*meta_info).useEditlist[k as usize] = useEditlist;
            k += 1;
        } else if strncmp(
            cmd.as_mut_ptr() as *const core::ffi::c_char,
            b"-ia_mp4_stsz_size:\0" as *const u8 as *const core::ffi::c_char,
            18 as size_t,
        ) == 0
        {
            let mut pb_arg_val_7: pCHAR8 = cmd
                .as_mut_ptr()
                .offset(18 as core::ffi::c_int as isize);
            let mut ia_mp4_stsz_size: UWORD32 = atoi(
                pb_arg_val_7 as *const core::ffi::c_char,
            ) as UWORD32;
            *((*meta_info).ia_mp4_stsz_size).offset(l as isize) = ia_mp4_stsz_size;
            l += 1;
        } else {
            printf(b"Command not found\0" as *const u8 as *const core::ffi::c_char);
            return -(1 as core::ffi::c_int);
        }
    }
    while i < MAX_TRACKS_PER_LAYER {
        (*meta_info).playTimeInSamples[i as usize] = 0 as UWORD32;
        i += 1;
    }
    while j < MAX_TRACKS_PER_LAYER {
        (*meta_info).startOffsetInSamples[j as usize] = 0 as UWORD32;
        j += 1;
    }
    while k < MAX_TRACKS_PER_LAYER {
        (*meta_info).useEditlist[k as usize] = 0 as UWORD32;
        k += 1;
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_metadata_dec_info_init(
    mut meta_info: metadata_info,
) -> core::ffi::c_int {
    return meta_info.dec_info_init as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_metadata_dec_exec(
    mut meta_info: metadata_info,
    mut frame: core::ffi::c_int,
) -> core::ffi::c_int {
    if frame < meta_info.ia_mp4_stsz_entries as core::ffi::c_int {
        return *(meta_info.ia_mp4_stsz_size).offset(frame as isize) as core::ffi::c_int
    } else {
        return 0 as core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_movie_time_scale(
    mut meta_info: metadata_info,
) -> core::ffi::c_int {
    return meta_info.movie_time_scale as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_media_time_scale(
    mut meta_info: metadata_info,
) -> core::ffi::c_int {
    return meta_info.media_time_scale as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_g_track_count(
    mut meta_info: metadata_info,
) -> core::ffi::c_int {
    return meta_info.g_track_count as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_use_edit_list(
    mut meta_info: metadata_info,
) -> core::ffi::c_int {
    return meta_info.useEditlist[0 as core::ffi::c_int as usize] as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_start_offset_in_samples(
    mut meta_info: metadata_info,
) -> core::ffi::c_int {
    return (meta_info.startOffsetInSamples[0 as core::ffi::c_int as usize])
        .wrapping_add(2048 as UWORD32) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_play_time_in_samples(
    mut meta_info: metadata_info,
) -> core::ffi::c_int {
    return meta_info.playTimeInSamples[0 as core::ffi::c_int as usize]
        as core::ffi::c_int;
}
