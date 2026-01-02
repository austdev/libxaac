
pub mod ixheaacd;

// next mod is needed for migration process only 
mod gen_ixheaacd_ref;


#[cfg(all(feature = "fallback", feature = "intergation"))]
compile_error!("features \"fallback\" and \"intergation\" cannot be enabled at the same time");

#[cfg(feature = "intergation")]
mod integration_test {

    use crate::gen_ixheaacd_ref::*;
    use std::slice;

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_scale_down(
            dest: *mut WORD32,
            src: *mut WORD32,
            len: WORD32,
            shift1: WORD8,
            shift2: WORD8,
        ) {
        if dest.is_null() || src.is_null() || len < 0 {
            return;
        }
        let len = len as usize;
        unsafe {
            let dest_slice = slice::from_raw_parts_mut(dest, len);
            let src_slice = slice::from_raw_parts(src, len);
            super::ixheaacd::scale_down(dest_slice, src_slice, shift1, shift2);
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_scale_down_adj(
            dest: *mut WORD32,
            src: *mut WORD32,
            len: WORD32,
            shift1: WORD8,
            shift2: WORD8,
        ) {
        if dest.is_null() || src.is_null() || len < 0 {
            return;
        }
        let len = len as usize;
        unsafe {
            let dest_slice = slice::from_raw_parts_mut(dest, len);
            let src_slice = slice::from_raw_parts(src, len);
            super::ixheaacd::scale_down_adj(dest_slice, src_slice, shift1, shift2);
        }
    }

}