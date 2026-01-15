// decoder::ixheaacd::windowing

//! Window calculation for IMDCT operations
//!

use super::rom;

/// Window type selection
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WindowType {
    Sine = 0, // WIN_SEL_0
    Kbd = 1, // WIN_SEL_1
    // ? WIN_SEL_2
}

/// Window length constants matching C defines
pub const WIN_LEN_4: i32 = 4;
pub const WIN_LEN_16: i32 = 16;
pub const WIN_LEN_48: i32 = 48;
pub const WIN_LEN_64: i32 = 64;
pub const WIN_LEN_96: i32 = 96;
pub const WIN_LEN_120: i32 = 120;
pub const WIN_LEN_128: i32 = 128;
pub const WIN_LEN_192: i32 = 192;
pub const WIN_LEN_256: i32 = 256;
pub const WIN_LEN_768: i32 = 768;
pub const WIN_LEN_960: i32 = 960;
pub const WIN_LEN_1024: i32 = 1024;

/// Calculate window coefficients for IMDCT operations.
///
/// Returns a reference to the appropriate window table based on window size and type.
///
/// # C signature
/// ```c
/// WORD32 ixheaacd_calc_window(WORD32 **win, WORD32 win_sz, WORD32 win_sel, WORD32 ec_flag);
/// ```
pub fn calc_window(
    win_sz: i32, // Window length (4, 16, 48, 64, 96, 120, 128, 192, 256, 768, 960, 1024)
    win_type: WindowType, // Window type selection (Sine or Kbd)
    ec_flag: bool, // Error concealment flag: if true, returns default window for invalid sizes
) -> Option<&'static [i32]>  // Reference to window coefficients on success
{
    #[cfg(feature = "fallback")]
    unsafe {
        let mut win_ptr: *mut i32 = std::ptr::null_mut();
        let result = crate::gen_ixheaacd_ref::ixheaacd_calc_window(
            &mut win_ptr,
            win_sz,
            win_type as i32,
            ec_flag as i32,
        );
        if result == 0 && !win_ptr.is_null() {
            Some(std::slice::from_raw_parts(win_ptr, win_sz as usize))
        } else {
            None
        }
    }

    #[cfg(not(feature = "fallback"))]
    {
        match win_type {
            WindowType::Sine => calc_sine_window(win_sz, ec_flag),
            WindowType::Kbd => calc_kbd_window(win_sz, ec_flag),
        }
    }
}

#[cfg(not(feature = "fallback"))]
fn calc_sine_window(win_sz: i32, ec_flag: bool) -> Option<&'static [i32]> {
    match win_sz {
        WIN_LEN_64 => Some(&rom::SINE_WIN_64),
        WIN_LEN_96 => Some(&rom::SINE_WIN_96),
        WIN_LEN_128 => Some(&rom::SINE_WIN_128),
        WIN_LEN_192 => Some(&rom::SINE_WIN_192),
        WIN_LEN_256 => Some(&rom::SINE_WIN_256),
        WIN_LEN_768 => Some(&rom::SINE_WIN_768),
        WIN_LEN_1024 => Some(&rom::SINE_WIN_1024),
        _ if ec_flag => Some(&rom::SINE_WIN_1024),
        _ => None,
    }
}

#[cfg(not(feature = "fallback"))]
fn calc_kbd_window(win_sz: i32, ec_flag: bool) -> Option<&'static [i32]> {
    match win_sz {
        WIN_LEN_4 => Some(&rom::KBD_WIN4),
        WIN_LEN_16 => Some(&rom::KBD_WIN16),
        WIN_LEN_48 => Some(&rom::KBD_WIN48),
        WIN_LEN_64 => Some(&rom::KBD_WIN64),
        WIN_LEN_96 => Some(&rom::KBD_WIN96),
        WIN_LEN_120 => Some(&rom::KBD_WIN120),
        WIN_LEN_128 => Some(&rom::KBD_WIN128),
        WIN_LEN_192 => Some(&rom::KBD_WIN192),
        WIN_LEN_768 => Some(&rom::KBD_WIN768),
        WIN_LEN_960 => Some(&rom::KBD_WIN960),
        WIN_LEN_1024 => Some(&rom::KBD_WIN1024),
        _ if ec_flag => Some(&rom::KBD_WIN1024),
        _ => None,
    }
}
