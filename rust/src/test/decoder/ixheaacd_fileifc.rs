extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn fclose(__stream: *mut FILE) -> core::ffi::c_int;
    fn fopen(
        __filename: *const core::ffi::c_char,
        __modes: *const core::ffi::c_char,
    ) -> *mut FILE;
    fn fread(
        __ptr: *mut core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> core::ffi::c_ulong;
}
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
pub struct FileWrapper {
    pub isMp4File: core::ffi::c_uint,
    pub inputFile: *mut FILE,
}
pub type FileWrapperPtr = *mut FileWrapper;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn FileWrapper_Open(
    mut fileName: *mut core::ffi::c_char,
) -> FileWrapperPtr {
    let mut transport: FileWrapperPtr = calloc(
        1 as size_t,
        ::core::mem::size_of::<FileWrapper>() as size_t,
    ) as FileWrapperPtr;
    (*transport).isMp4File = 0 as core::ffi::c_uint;
    (*transport).inputFile = 0 as *mut FILE;
    (*transport).inputFile = fopen(
        fileName as *const core::ffi::c_char,
        b"rb\0" as *const u8 as *const core::ffi::c_char,
    ) as *mut FILE;
    if ((*transport).inputFile).is_null() {
        free(transport as *mut core::ffi::c_void);
        return 0 as FileWrapperPtr;
    } else {
        return transport
    };
}
#[no_mangle]
pub unsafe extern "C" fn FileWrapper_Read(
    mut transport: FileWrapperPtr,
    mut buffer: *mut core::ffi::c_uchar,
    mut bufSize: core::ffi::c_int,
    mut length: *mut core::ffi::c_uint,
) -> core::ffi::c_int {
    *length = fread(
        buffer as *mut core::ffi::c_void,
        1 as size_t,
        bufSize as size_t,
        (*transport).inputFile,
    ) as core::ffi::c_uint;
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FileWrapper_Close(
    mut transport: FileWrapperPtr,
) -> core::ffi::c_uint {
    if transport.is_null() {
        return 0 as core::ffi::c_uint;
    }
    if !((*transport).inputFile).is_null() {
        fclose((*transport).inputFile);
    }
    free(transport as *mut core::ffi::c_void);
    return 0 as core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn FileWrapper_IsMp4File(
    mut transport: FileWrapperPtr,
) -> core::ffi::c_uint {
    return (*transport).isMp4File;
}
