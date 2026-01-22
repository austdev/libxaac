extern "C" {
    static ixheaacd_sine_win_128: [WORD32; 128];
    static ixheaacd_sine_win_1024: [WORD32; 1024];
    static ixheaacd_sine_win_64: [WORD32; 64];
    static ixheaacd_sine_win_768: [WORD32; 768];
    static ixheaacd_sine_win_192: [WORD32; 192];
    static ixheaacd_sine_win_96: [WORD32; 96];
    static ixheaacd_sine_win_256: [WORD32; 256];
    static ixheaacd_kbd_win120: [WORD32; 120];
    static ixheaacd_kbd_win128: [WORD32; 128];
    static ixheaacd_kbd_win960: [WORD32; 960];
    static ixheaacd_kbd_win1024: [WORD32; 1024];
    static ixheaacd_kbd_win4: [WORD32; 4];
    static ixheaacd_kbd_win16: [WORD32; 16];
    static ixheaacd_kbd_win_64: [WORD32; 64];
    static ixheaacd_kbd_win768: [WORD32; 768];
    static ixheaacd_kbd_win192: [WORD32; 192];
    static ixheaacd_kbd_win96: [WORD32; 96];
    static ixheaacd_kbd_win48: [WORD32; 48];
}
pub type WORD32 = core::ffi::c_int;
pub const WIN_LEN_1024: core::ffi::c_int = 1024;
pub const WIN_LEN_768: core::ffi::c_int = 768;
pub const WIN_LEN_192: core::ffi::c_int = 192;
pub const WIN_LEN_128: core::ffi::c_int = 128;
pub const WIN_LEN_960: core::ffi::c_int = 960;
pub const WIN_LEN_256: core::ffi::c_int = 256;
pub const WIN_LEN_120: core::ffi::c_int = 120;
pub const WIN_LEN_96: core::ffi::c_int = 96;
pub const WIN_LEN_64: core::ffi::c_int = 64;
pub const WIN_LEN_48: core::ffi::c_int = 48;
pub const WIN_LEN_16: core::ffi::c_int = 16;
pub const WIN_LEN_4: core::ffi::c_int = 4;
pub const WIN_SEL_0: core::ffi::c_int = 0;
pub const WIN_SEL_1: core::ffi::c_int = 1;
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_calc_window(
    mut win: *mut *mut WORD32,
    mut win_sz: WORD32,
    mut win_sel: WORD32,
    mut ec_flag: WORD32,
) -> WORD32 {
    match win_sel {
        WIN_SEL_0 => {
            match win_sz {
                WIN_LEN_128 => {
                    *win = ixheaacd_sine_win_128.as_ptr() as *mut WORD32;
                }
                WIN_LEN_1024 => {
                    *win = ixheaacd_sine_win_1024.as_ptr() as *mut WORD32;
                }
                WIN_LEN_64 => {
                    *win = ixheaacd_sine_win_64.as_ptr() as *mut WORD32;
                }
                WIN_LEN_768 => {
                    *win = ixheaacd_sine_win_768.as_ptr() as *mut WORD32;
                }
                WIN_LEN_192 => {
                    *win = ixheaacd_sine_win_192.as_ptr() as *mut WORD32;
                }
                WIN_LEN_96 => {
                    *win = ixheaacd_sine_win_96.as_ptr() as *mut WORD32;
                }
                WIN_LEN_256 => {
                    *win = ixheaacd_sine_win_256.as_ptr() as *mut WORD32;
                }
                _ => {
                    if ec_flag != 0 {
                        *win = ixheaacd_sine_win_1024.as_ptr() as *mut WORD32;
                    } else {
                        return -(1 as WORD32)
                    }
                }
            }
        }
        WIN_SEL_1 => {
            match win_sz {
                WIN_LEN_120 => {
                    *win = ixheaacd_kbd_win120.as_ptr() as *mut WORD32;
                }
                WIN_LEN_128 => {
                    *win = ixheaacd_kbd_win128.as_ptr() as *mut WORD32;
                }
                WIN_LEN_960 => {
                    *win = ixheaacd_kbd_win960.as_ptr() as *mut WORD32;
                }
                WIN_LEN_1024 => {
                    *win = ixheaacd_kbd_win1024.as_ptr() as *mut WORD32;
                }
                WIN_LEN_4 => {
                    *win = ixheaacd_kbd_win4.as_ptr() as *mut WORD32;
                }
                WIN_LEN_16 => {
                    *win = ixheaacd_kbd_win16.as_ptr() as *mut WORD32;
                }
                WIN_LEN_64 => {
                    *win = ixheaacd_kbd_win_64.as_ptr() as *mut WORD32;
                }
                WIN_LEN_768 => {
                    *win = ixheaacd_kbd_win768.as_ptr() as *mut WORD32;
                }
                WIN_LEN_192 => {
                    *win = ixheaacd_kbd_win192.as_ptr() as *mut WORD32;
                }
                WIN_LEN_96 => {
                    *win = ixheaacd_kbd_win96.as_ptr() as *mut WORD32;
                }
                WIN_LEN_48 => {
                    *win = ixheaacd_kbd_win48.as_ptr() as *mut WORD32;
                }
                _ => {
                    if ec_flag != 0 {
                        *win = ixheaacd_kbd_win1024.as_ptr() as *mut WORD32;
                    } else {
                        return -(1 as WORD32)
                    }
                }
            }
        }
        _ => return -(1 as WORD32),
    }
    return 0 as WORD32;
}
