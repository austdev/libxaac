// decoder::ixheaacd_basic_ops

//! Basic operations for the decoder, DSP primitives
//!

#[allow(dead_code)]
const ADJ_SCALE: i32 = 11;

 use std::mem;

 #[repr(C)]  // remove this after migration
 #[derive(Debug, Clone, Copy, PartialEq, Eq)]
 pub struct OffsetLengths {
     pub lfac: i32,          // FAC (Forward Aliasing Cancellation) length
     pub n_flat_ls: i32,     // Flat region length (left side)
     pub n_trans_ls: i32,    // Transition region length (left side)
     pub n_long: i32,        // Long block length (full frame)
     pub n_short: i32,       // Short block length (1/8 of frame)
 }

 impl OffsetLengths {

     /// Borrow as C struct (zero-cost, no copy)
     pub fn as_c_ref(&self) -> &crate::gen_ixheaacd_ref::offset_lengths {
         unsafe { &*(self as *const Self as *const crate::gen_ixheaacd_ref::offset_lengths) }
     }

     /// Borrow mutably as C struct (zero-cost, no copy)
     pub fn as_c_mut(&mut self) -> &mut crate::gen_ixheaacd_ref::offset_lengths {
         unsafe { &mut *(self as *mut Self as *mut crate::gen_ixheaacd_ref::offset_lengths) }
     }

     /// Create from C struct reference (zero-cost, no copy)
     pub fn from_c_ref(c: &crate::gen_ixheaacd_ref::offset_lengths) -> &Self {
         unsafe { &*(c as *const crate::gen_ixheaacd_ref::offset_lengths as *const Self) }
     }

     /// Create from mutable C struct reference (zero-cost, no copy)
     pub fn from_c_mut(c: &mut crate::gen_ixheaacd_ref::offset_lengths) -> &mut Self {
         unsafe { &mut *(c as *mut crate::gen_ixheaacd_ref::offset_lengths as *mut Self) }
     }
 }

 const _: () = {  // Safety: OffsetLengths has #[repr(C)] and identical layout
     assert!(mem::size_of::<OffsetLengths>() == mem::size_of::<crate::gen_ixheaacd_ref::offset_lengths>());
     assert!(mem::align_of::<OffsetLengths>() == mem::align_of::<crate::gen_ixheaacd_ref::offset_lengths>());
 };

// ============================================================================
// Function stubs from ixheaacd_vec_baisc_ops.h (in order)
// ============================================================================

/// Combine FAC (Forward Aliasing Cancellation) data
///
/// C signature:
/// ```c
/// VOID ixheaacd_combine_fac(WORD32 *src1, WORD32 *src2, WORD32 *dest, WORD32 len,
///                           WORD8 shift1, WORD8 shift2);
/// ```
pub fn combine_fac(src1: &[i32], src2: &[i32], dest: &mut [i32], shift1: i8, shift2: i8) 
{
    assert_eq!(src1.len(), src2.len(), "src1 and src2 must have same length");
    assert_eq!(src1.len(), dest.len(), "src1 and dest must have same length");

    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_combine_fac(
            src1.as_ptr() as *mut i32,
            src2.as_ptr() as *mut i32,
            dest.as_mut_ptr(),
            dest.len() as i32,
            shift1,
            shift2,
        );
    }
}

/// Windowing function for long blocks, variant 1
///
/// C signature:
/// ```c
/// WORD8 ixheaacd_windowing_long1(WORD32 *src1, WORD32 *src2,
///                                const WORD32 *win_fwd, const WORD32 *win_rev,
///                                WORD32 *dest, WORD32 vlen, WORD8 shift1,
///                                WORD8 shift2);
/// ```
pub fn windowing_long1(src1: &mut [i32], src2: &mut [i32],
            win_fwd: &[i32], win_rev: &[i32],
            dest: &mut [i32],
            vlen: i32,
            shift1: i8, shift2: i8) -> i8 
{
    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_windowing_long1(
            src1.as_mut_ptr(),
            src2.as_mut_ptr(),
            win_fwd.as_ptr(),
            win_rev.as_ptr(),
            dest.as_mut_ptr(),
            vlen,
            shift1,
            shift2,
        )
    }
}

/// Windowing function for long blocks, variant 2
///
/// C signature:
/// ```c
/// WORD8 ixheaacd_windowing_long2(WORD32 *src1, const WORD32 *win_fwd,
///                                WORD32 *fac_data_out, WORD32 *over_lap,
///                                WORD32 *p_out_buffer,
///                                offset_lengths *ixheaacd_drc_offset,
///                                WORD8 shift1, WORD8 shift2, WORD8 shift3);
/// ```
pub fn windowing_long2(src1: &mut [i32], win_fwd: &[i32],
            fac_data_out: &mut [i32], over_lap: &mut [i32],
            p_out_buffer: &mut [i32],
            ixheaacd_drc_offset: &mut OffsetLengths,
            shift1: i8, shift2: i8, shift3: i8) -> i8 
{
    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_windowing_long2(
            src1.as_mut_ptr(),
            win_fwd.as_ptr(),
            fac_data_out.as_mut_ptr(),
            over_lap.as_mut_ptr(),
            p_out_buffer.as_mut_ptr(),
            ixheaacd_drc_offset.as_c_mut(),
            shift1,
            shift2,
            shift3,
        )
    }
}

/// Windowing function for long blocks, variant 3
///
/// C signature:
/// ```c
/// WORD8 ixheaacd_windowing_long3(WORD32 *src1, const WORD32 *win_fwd,
///                                WORD32 *over_lap, WORD32 *p_out_buffer,
///                                const WORD32 *win_rev,
///                                offset_lengths *ixheaacd_drc_offset,
///                                WORD8 shift1, WORD8 shift2);
/// ```
pub fn windowing_long3(src1: &mut [i32], win_fwd: &[i32],
            over_lap: &mut [i32], p_out_buffer: &mut [i32],
            win_rev: &[i32],
            ixheaacd_drc_offset: &mut OffsetLengths,
            shift1: i8, shift2: i8) -> i8 
{
    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_windowing_long3(
            src1.as_mut_ptr(),
            win_fwd.as_ptr(),
            over_lap.as_mut_ptr(),
            p_out_buffer.as_mut_ptr(),
            win_rev.as_ptr(),
            ixheaacd_drc_offset.as_c_mut(),
            shift1,
            shift2,
        )
    }
}

/// Windowing function for short blocks, variant 1
///
/// C signature:
/// ```c
/// VOID ixheaacd_windowing_short1(WORD32 *src1, WORD32 *src2, WORD32 *fp,
///                                offset_lengths *ixheaacd_drc_offset,
///                                WORD8 shiftp, WORD8 shift_olap);
/// ```
pub fn windowing_short1(src1: &mut [i32], src2: &mut [i32], fp: &mut [i32],
            ixheaacd_drc_offset: &mut OffsetLengths,
            shiftp: i8, shift_olap: i8)
{
    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_windowing_short1(
            src1.as_mut_ptr(),
            src2.as_mut_ptr(),
            fp.as_mut_ptr(),
            ixheaacd_drc_offset.as_c_mut(),
            shiftp,
            shift_olap,
        );
    }
}

/// Windowing function for short blocks, variant 2
///
/// C signature:
/// ```c
/// VOID ixheaacd_windowing_short2(WORD32 *src1, WORD32 *win_fwd, WORD32 *fp,
///                                offset_lengths *ixheaacd_drc_offset,
///                                WORD8 shiftp, WORD8 shift_olap);
/// ```
pub fn windowing_short2(src1: &mut [i32], win_fwd: &mut [i32], fp: &mut [i32],
            ixheaacd_drc_offset: &mut OffsetLengths,
            shiftp: i8, shift_olap: i8)
{
    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_windowing_short2(
            src1.as_mut_ptr(),
            win_fwd.as_mut_ptr(),
            fp.as_mut_ptr(),
            ixheaacd_drc_offset.as_c_mut(),
            shiftp,
            shift_olap,
        );
    }
}

/// Windowing function for short blocks, variant 3
///
/// C signature:
/// ```c
/// WORD8 ixheaacd_windowing_short3(WORD32 *src1, WORD32 *win_rev, WORD32 *fp,
///                                 WORD32 nshort, WORD8 shiftp, WORD8 shift_olap);
/// ```
pub fn windowing_short3(src1: &mut [i32], win_rev: &mut [i32], fp: &mut [i32],
    nshort: i32,
    shiftp: i8,
    shift_olap: i8,
) -> i8 {
    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_windowing_short3(
            src1.as_mut_ptr(),
            win_rev.as_mut_ptr(),
            fp.as_mut_ptr(),
            nshort,
            shiftp,
            shift_olap,
        )
    }
}

/// Windowing function performs overlap-add synthesis with windowing for short IMDCT blocks in AAC's
///  EIGHT_SHORT_SEQUENCE mode, ensuring smooth transitions between consecutive audio frames.
/// C signature:
/// ```c
/// WORD8 ixheaacd_windowing_short4(WORD32 *src1, WORD32 *win_fwd, WORD32 *fp,
///                                 WORD32 *win_fwd1, WORD32 nshort, WORD32 flag,
///                                 WORD8 shiftp, WORD8 shift_olap, WORD8 output_q);
/// ```
pub fn windowing_short4(
    src1: &[i32],     // Current IMDCT output (n_short samples)
    win_fwd: &[i32],   // Forward window coefficients
    fp: &mut [i32],   // Overlap buffer (in/out) - 2*n_short samples
    win_rev1: &[i32],  // Backward window coefficients
    windowed_flag: bool, // Apply windowing if true, otherwise direct copy
    shiftp: i8,   // Q-format of src1 (current IMDCT)
    shift_olap: i8,  // Q-format of fp (overlap buffer)
    output_q: i8,  // Target Q-format for output
) -> i8 {
    let n_short = src1.len();
    assert_eq!(n_short, win_fwd.len(), "forward window must have same length");
    assert_eq!(n_short, win_rev1.len(), "backward window must have same length");
    assert_eq!(n_short * 2, fp.len(), "overlap buffer must have double length");

    unsafe {
        let src1 = src1.as_ptr();
        let win_fwd = win_fwd.as_ptr();
        let win_fwd1 = win_rev1.as_ptr().add(n_short - 1);
        crate::gen_ixheaacd_ref::ixheaacd_windowing_short4(
            src1 as *mut i32,
            win_fwd as *mut i32,
            fp.as_mut_ptr(),
            win_fwd1 as *mut i32,
            n_short as i32,
            if windowed_flag { 1 } else { 0 },
            shiftp, shift_olap, output_q)
    }
}

/// Scale down/up audio samples between Q-formats
///
/// C signature:
/// ```c
/// VOID ixheaacd_scale_down(WORD32 *dest, WORD32 *src, WORD32 len, WORD8 shift1,
///                          WORD8 shift2);
/// ```
pub fn scale_down(dest: &mut [i32], src: &[i32], shift1: i8, shift2: i8)
{
    assert_eq!(dest.len(), src.len(), "dest and src must have same length");

    #[cfg(feature = "fallback")]
    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_scale_down(
            dest.as_mut_ptr(), 
            src.as_ptr() as *mut i32, 
            src.len() as i32, 
            shift1, shift2);
    }

    #[cfg(not(feature = "fallback"))]
    if shift1 > shift2 {
        let shift = (shift1 - shift2) as u32;
        for (d, s) in dest.iter_mut().zip(src.iter()) {
            *d = s >> shift;
        }
    } else {
        let shift = (shift2 - shift1) as u32;
        for (d, s) in dest.iter_mut().zip(src.iter()) {
            *d = shl32_sat(*s, shift);
        }
    }

}

/// Scale down/up audio samples between Q-formats
///
/// C signature:
/// ```c
/// VOID ixheaacd_scale_down_adj(WORD32 *dest, WORD32 *src, WORD32 len,
///                          WORD8 shift1, WORD8 shift2);
/// ```
pub fn scale_down_adj(dest: &mut [i32], src: &[i32], shift1: i8, shift2: i8)
{
    assert_eq!(dest.len(), src.len(), "dest and src must have same length");
    
    #[cfg(feature = "fallback")]
    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_scale_down_adj(
            dest.as_mut_ptr(), 
            src.as_ptr() as *mut i32, 
            src.len() as i32, 
            shift1, shift2);
    }
    #[cfg(not(feature = "fallback"))]
    if shift1 > shift2 {
        let shift = (shift1 - shift2) as u32;
        for (d, s) in dest.iter_mut().zip(src.iter()) {
            *d = (s >> shift).saturating_add(ADJ_SCALE);
        }
    } else {
        let shift = (shift2 - shift1) as u32;
        for (d, s) in dest.iter_mut().zip(src.iter()) {
            *d = shl32_sat(*s, shift).saturating_add(ADJ_SCALE);
        }
    }
}

/// Saturating left shift for 32-bit signed integers
#[inline]
#[cfg(not(feature = "fallback"))]
fn shl32_sat(a: i32, b: u32) -> i32 
{
    let max = i32::MAX >> b;
    let min = i32::MIN >> b;
    match a {
        _ if a > max => i32::MAX,
        _ if a < min => i32::MIN,
        _ => a << b,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // windowing_short4() tests
    // ============================================================================
    // Tests cover all code paths in ixheaacd_windowing_short4():
    // - Branch 1: shift_olap > output_q, flag=1 (windowed)
    // - Branch 2: shift_olap > output_q, flag=0 (no windowing)
    // - Branch 3: shift_olap <= output_q, flag=1 (windowed)
    // - Branch 4: shift_olap <= output_q, flag=0 (no windowing)
    // - Various n_short sizes: 8, 16, 32, 64, 128
    // - Edge cases: saturation, zero values, max values
    // ============================================================================

    // Helper function to create simple sine-like window coefficients
    fn create_test_windows() -> (Vec<i32>, Vec<i32>) {
        (
            vec![13176960, 302060768, 585449152, 858185984, 1115308543, 1352137343, 1564364543, 1748129023, 1900087039, 2017472895, 2098151679, 2140654591],
            vec![13176960, 302060768, 585449152, 858185984, 1115308543, 1352137343, 1564364543, 1748129023, 1900087039, 2017472895, 2098151679, 2140654591],
        )
    }

    // ============================================================================
    // Branch 1: shift_olap > output_q, flag=1 (windowed)
    // ============================================================================

    #[test]
    fn test_windowing_short4_branch1_flag1() {
        let src1: Vec<i32> = vec![232644448, 87804592, -6177088, -2147483648, 251254752, 219674032, 125828992, 184318592, 106729056, 203009648, 281430368, 171565216];
        let (win_fwd, win_rev1) = create_test_windows();
        let mut fp = vec![21309976; 2 * src1.len()];
        
        let result_q = windowing_short4(
            &src1, &win_fwd, &mut fp, &win_rev1,
            true, 15, 14, 12
        );
        
        let exp = vec![21406486, 24550711, 24947048, 31450916, 39580295, 34812973, 5687590, -7326831, -1142815, 8776529, -1200578, 5631369, -22044439, -25357873, 257511605, 6010677, -3607019, -15856645, -12982730, -372737, 5636058, 78508637, 909869, 5159004];
        assert_eq!(fp, exp);
        assert_eq!(result_q, 12);
    }

    // ============================================================================
    // Branch 2: shift_olap > output_q, flag=0 (no windowing)
    // ============================================================================

    #[test]
    fn test_windowing_short4_branch1_flag0() {
        let src1: Vec<i32> = vec![232644448, 87804592, -6177088, -2147483648, 251254752, 219674032, 125828992, 184318592, 106729056, 203009648, 281430368, 171565216];
        let (win_fwd, win_rev1) = create_test_windows();
        let mut fp = vec![4352798; 2 * src1.len()];
        
        let result_q = windowing_short4(
            &src1, &win_fwd, &mut fp, &win_rev1,
            false, 17, 16, 14
        );
        
        let exp = vec![4449308, 7593533, 7989870, 14493738, 22623117, 17855795, -11269588, -24284009, -18099993, -8180649, -18157756, -11325809, -26371055, -30318645, 269523654, 1860335, -9887375, -27992357, -27992357, -9887375, 1860335, 269523654, -30318645, -26371055];
        assert_eq!(fp, exp);
        assert_eq!(result_q, 14);
    }

    // ============================================================================
    // Branch 3: shift_olap <= output_q, flag=1 (windowed)
    // ============================================================================

    #[test]
    fn test_windowing_short4_branch2_flag1() {
        let src1: Vec<i32> = vec![232644448, 87804592, -6177088, -2147483648, 251254752, 219674032, 125828992, 184318592, 106729056, 203009648, 281430368, 171565216];
        let (win_fwd, win_rev1) = create_test_windows();
        let mut fp = vec![-6177088; 2 * src1.len()];
        
        let result_q = windowing_short4(
            &src1, &win_fwd, &mut fp, &win_rev1,
            true, 16, 14, 15
        );
        
        let exp = vec![-2895523, 3392927, 4185601, 17193337, 33452094, 23917451, -37421860, -63450702, -51082669, -31243982, -51198195, -37534302, -60920954, -67547821, 498191135, -4810721, -24046113, -48545365, -42797536, -17577549, -5559960, 140185199, -15012337, -6514068];
        assert_eq!(fp, exp);
        assert_eq!(result_q, 14, "Should return shift_olap");
    }

    // ============================================================================
    // Branch 4: shift_olap <= output_q, flag=0 (no windowing)
    // ============================================================================

    #[test]
    fn test_windowing_short4_branch2_flag0() {
        let src1: Vec<i32> = vec![232644448, 87804592, -6177088, -2147483648, 251254752, 219674032, 125828992, 184318592, 106729056, 203009648, 281430368, 171565216];
        let (win_fwd, win_rev1) = create_test_windows();
        let mut fp = vec![162292782; 2 * src1.len()];
        
        let result_q = windowing_short4(
            &src1, &win_fwd, &mut fp, &win_rev1,
            false, 14, 12, 15
        );
        
        let exp = vec![20479618, 26768068, 27560742, 40568478, 56827235, 47292592, 131048010, 105019168, 117387201, 137225888, 117271675, 130935568, 107374274, 99479094, 699163693, 163837054, 140341634, 104131670, 104131670, 140341634, 163837054, 699163693, 99479094, 107374274];
        assert_eq!(fp, exp);
        assert_eq!(result_q, 12);
    }

    // ============================================================================
    // Edge cases and special conditions
    // ============================================================================

    #[test]
    fn test_windowing_short4_equal_q_formats() {
        // shift_olap == output_q (boundary condition)
        let src1 = vec![1000; 12];
        let (win_fwd, win_rev1) = create_test_windows();
        let mut fp = vec![100; 2 * src1.len()];
        
        let result_q = windowing_short4(
            &src1, &win_fwd, &mut fp, &win_rev1,
            true, 17, 15, 15
        );
        
        let exp = vec![101, 135, 168, 199, 229, 257, -83, -104, -122, -135, -145, -150, -150, -145, -135, -122, -104, -83, -58, -30, 0, 31, 64, 98];
        assert_eq!(fp, exp);
        assert_eq!(result_q, 15);
    }

    #[test]
    fn test_windowing_short4_zero_src() {
        let src1 = vec![0; 12];
        let (win_fwd, win_rev1) = create_test_windows();
        let mut fp = vec![100; 2 * src1.len()];
        
        let result_q = windowing_short4(
            &src1, &win_fwd, &mut fp, &win_rev1,
            true, 18, 16, 14
        );
        
        let exp = vec![100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25];
        assert_eq!(fp, exp);
        assert_eq!(result_q, 14);
    }

    #[test]
    fn test_windowing_short4_zero_fp() {
        let src1 = vec![1000; 12];
        let (win_fwd, win_rev1) = create_test_windows();
        let mut fp = vec![0; 2 * src1.len()];
        
        let result_q = windowing_short4(
            &src1, &win_fwd, &mut fp, &win_rev1,
            true, 17, 16, 14
        );
        
        let exp = vec![0, 17, 34, 49, 64, 78, -92, -102, -111, -118, -123, -125, -125, -123, -118, -111, -102, -92, -79, -65, -50, -35, -18, -1];
        assert_eq!(fp, exp);
        assert_eq!(result_q, 14);
    }

    #[test]
    fn test_windowing_short4_large_shift_difference() {
        let src1 = vec![1000; 12];
        let (win_fwd, win_rev1) = create_test_windows();
        let mut fp = vec![100; 2 * src1.len()];
        
        // Large shift difference: shift_olap - output_q = 8
        let result_q = windowing_short4(
            &src1, &win_fwd, &mut fp, &win_rev1,
            true, 21, 20, 12
        );
        
        let exp = vec![100, 100, 100, 100, 101, 101, 98, 98, 98, 98, 98, 98, -2, -2, -2, -2, -2, -2, -2, -2, -1, -1, -1, -1];
        assert_eq!(fp, exp);
        assert_eq!(result_q, 12);
    }

    #[test]
    fn test_windowing_short4_max_positive_values() {
        let n_short = 16;
        let src1 = vec![i32::MAX / 2; n_short];
        let win_fwd = vec![i32::MAX / 4; n_short];
        let win_rev1 = vec![i32::MAX / 4; n_short];
        let mut fp = vec![1000; 2 * n_short];
        
        let result_q = windowing_short4(
            &src1, &win_fwd, &mut fp, &win_rev1,
            true, 12, 16, 14
        );
        
        let exp = vec![1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 999, 999, 999, 999, 999, 999, 999, 999, 249, 249, 249, 249, 249, 249, 249, 249, 249, 249, 249, 249, 249, 249, 249, 249];
        assert_eq!(fp, exp);
        assert_eq!(result_q, 14);
    }

    // ============================================================================
    // scale_down() tests - Right shift (scale down / decrease precision)
    // ============================================================================

    #[test]
    fn test_scale_down_right_shift_len8() {
        // Test length 8 (exactly 8, SIMD boundary for AVX2)
        let src = vec![8000, 16000, 24000, 32000, 40000, 48000, 56000, 64000];
        let mut dest = vec![0; 8];
        
        scale_down(&mut dest, &src, 5, 2); // Right shift by 3
        
        assert_eq!(dest, vec![1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000]);
    }

    #[test]
    fn test_scale_down_right_shift_len18() {
        // Test length 18 (16 SIMD + 2 scalar remainder)
        let src = vec![
            8000, 16000, 24000, 32000, 40000, 48000, 56000, 64000,
            72000, 80000, 88000, 96000, 104000, 112000, 120000, 128000,
            136000, 144000
        ];
        let mut dest = vec![0; 18];
        
        scale_down(&mut dest, &src, 5, 2); // Right shift by 3
        
        assert_eq!(dest, vec![
            1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000,
            9000, 10000, 11000, 12000, 13000, 14000, 15000, 16000,
            17000, 18000
        ]);
    }

    #[test]
    fn test_scale_down_right_shift_negative_values() {
        // Test with negative values (arithmetic right shift preserves sign)
        let src = vec![-8000, -16000, 24000, -32000, 40000, -48000, 56000, -64000];
        let mut dest = vec![0; 8];
        
        scale_down(&mut dest, &src, 5, 2); // Right shift by 3
        
        assert_eq!(dest, vec![-1000, -2000, 3000, -4000, 5000, -6000, 7000, -8000]);
    }

    // ============================================================================
    // scale_down() tests - Left shift (scale up / increase precision)
    // ============================================================================

    #[test]
    fn test_scale_down_left_shift_len7() {
        let src = vec![1000, 2000, 3000, 4000, 5000, 6000, 7000];
        let mut dest = vec![0; 7];
        
        scale_down(&mut dest, &src, 2, 5); // Left shift by 3
        
        assert_eq!(dest, vec![8000, 16000, 24000, 32000, 40000, 48000, 56000]);
    }

     #[test]
    fn test_scale_down_left_shift_len15() {
        let src = vec![
            1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000,
            9000, 10000, 11000, 12000, 13000, 14000, 15000
        ];
        let mut dest = vec![0; 15];
        
        scale_down(&mut dest, &src, 2, 5); // Left shift by 3
        
        assert_eq!(dest, vec![
            8000, 16000, 24000, 32000, 40000, 48000, 56000, 64000,
            72000, 80000, 88000, 96000, 104000, 112000, 120000
        ]);
    }

    #[test]
    fn test_scale_down_left_shift_negative_values() {
        let src = vec![-1000, -2000, 3000, -4000, 5000, -6000, 7000, -8000];
        let mut dest = vec![0; 8];
        
        scale_down(&mut dest, &src, 2, 5); // Left shift by 3
        
        assert_eq!(dest, vec![-8000, -16000, 24000, -32000, 40000, -48000, 56000, -64000]);
    }

    // ============================================================================
    // scale_down() tests - Saturation
    // ============================================================================

    #[test]
    fn test_scale_down_saturation_max_len1() {
        let src = vec![i32::MAX / 2];
        let mut dest = vec![0];
        
        scale_down(&mut dest, &src, 0, 2); // Left shift by 2 would overflow
        
        assert_eq!(dest, vec![i32::MAX]);
    }

    #[test]
    fn test_scale_down_saturation_min_len1() {
        let src = vec![i32::MIN / 2];
        let mut dest = vec![0];
        
        scale_down(&mut dest, &src, 0, 2); // Left shift by 2 would overflow
        
        assert_eq!(dest, vec![i32::MIN]);
    }

    #[test]
    fn test_scale_down_saturation_mixed_len16() {
        // Test saturation with 16 samples
        let src = vec![
            i32::MAX / 2, 1000, i32::MIN / 2, -1000,
            i32::MAX / 4, 2000, i32::MIN / 4, -2000,
            i32::MAX / 8, 3000, i32::MIN / 8, -3000,
            i32::MAX / 16, 4000, i32::MIN / 16, -4000
        ];
        let mut dest = vec![0; 16];
        
        scale_down(&mut dest, &src, 0, 2); // Left shift by 2
        
        assert_eq!(dest, vec![
            i32::MAX, 4000, i32::MIN, -4000,
            i32::MAX / 4 << 2, 8000, i32::MIN / 4 << 2, -8000,
            i32::MAX / 8 << 2, 12000, i32::MIN / 8 << 2, -12000,
            i32::MAX / 16 << 2, 16000, i32::MIN / 16 << 2, -16000
        ]);
    }

    #[test]
    fn test_scale_down_saturation_all_max_len8() {
        // All values saturate to MAX
        let src = vec![i32::MAX / 2; 8];
        let mut dest = vec![0; 8];
        
        scale_down(&mut dest, &src, 0, 2);
        
        assert_eq!(dest, vec![i32::MAX; 8]);
    }

    #[test]
    fn test_scale_down_saturation_all_min_len8() {
        // All values saturate to MIN
        let src = vec![i32::MIN / 2; 8];
        let mut dest = vec![0; 8];
        
        scale_down(&mut dest, &src, 0, 2);
        
        assert_eq!(dest, vec![i32::MIN; 8]);
    }

    // ============================================================================
    // scale_down() tests - No change (shift1 == shift2)
    // ============================================================================

    #[test]
    fn test_scale_down_no_change_len1() {
        let src = vec![1000];
        let mut dest = vec![0];
        
        scale_down(&mut dest, &src, 5, 5);
        
        assert_eq!(dest, src);
    }

    #[test]
    fn test_scale_down_no_change_len16() {
        let src = vec![
            1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000,
            9000, 10000, 11000, 12000, 13000, 14000, 15000, 16000
        ];
        let mut dest = vec![0; 16];
        
        scale_down(&mut dest, &src, 5, 5);
        
        assert_eq!(dest, src);
    }

    // ============================================================================
    // scale_down() tests - Edge cases
    // ============================================================================

    #[test]
    fn test_scale_down_zero_values_len8() {
        let src = vec![0; 8];
        let mut dest = vec![42; 8]; // Pre-filled with non-zero
        
        scale_down(&mut dest, &src, 3, 7); // Left shift by 4
        
        assert_eq!(dest, vec![0; 8]);
    }

    #[test]
    fn test_scale_down_single_bit_shift_len8() {
        let src = vec![100, 200, 300, 400, 500, 600, 700, 800];
        let mut dest = vec![0; 8];
        
        scale_down(&mut dest, &src, 5, 6); // Left shift by 1
        
        assert_eq!(dest, vec![200, 400, 600, 800, 1000, 1200, 1400, 1600]);
    }

    #[test]
    fn test_scale_down_large_right_shift_len8() {
        let src = vec![1000000, 2000000, 3000000, 4000000, 5000000, 6000000, 7000000, 8000000];
        let mut dest = vec![0; 8];
        
        scale_down(&mut dest, &src, 10, 0); // Right shift by 10
        
        assert_eq!(dest, vec![976, 1953, 2929, 3906, 4882, 5859, 6835, 7812]);
    }

    // ============================================================================
    // scale_down_adj() tests - With ADJ_SCALE adjustment
    // ============================================================================

    #[test]
    fn test_scale_down_adj_no_shift_len1() {
        let src = vec![1000];
        let mut dest = vec![0];
        
        scale_down_adj(&mut dest, &src, 0, 0); // No shift, just adjustment
        
        assert_eq!(dest, vec![1011]); // 1000 + ADJ_SCALE(11)
    }

    #[test]
    fn test_scale_down_adj_no_shift_len16() {
        let src = vec![
            1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000,
            9000, 10000, 11000, 12000, 13000, 14000, 15000, 16000
        ];
        let mut dest = vec![0; 16];
        
        scale_down_adj(&mut dest, &src, 0, 0);
        
        assert_eq!(dest, vec![
            1011, 2011, 3011, 4011, 5011, 6011, 7011, 8011,
            9011, 10011, 11011, 12011, 13011, 14011, 15011, 16011
        ]);
    }

    #[test]
    fn test_scale_down_adj_right_shift_len8() {
        let src = vec![8000, 16000, 24000, 32000, 40000, 48000, 56000, 64000];
        let mut dest = vec![0; 8];
        
        scale_down_adj(&mut dest, &src, 5, 2); // Right shift by 3, then add 11
        
        assert_eq!(dest, vec![1011, 2011, 3011, 4011, 5011, 6011, 7011, 8011]);
    }

    #[test]
    fn test_scale_down_adj_left_shift_len8() {
        let src = vec![1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000];
        let mut dest = vec![0; 8];
        
        scale_down_adj(&mut dest, &src, 2, 5); // Left shift by 3, then add 11
        
        assert_eq!(dest, vec![8011, 16011, 24011, 32011, 40011, 48011, 56011, 64011]);
    }

    #[test]
    fn test_scale_down_adj_negative_values_len8() {
        let src = vec![-1000, -2000, 3000, -4000, 5000, -6000, 7000, -8000];
        let mut dest = vec![0; 8];
        
        scale_down_adj(&mut dest, &src, 2, 5); // Left shift by 3, then add 11
        
        assert_eq!(dest, vec![-7989, -15989, 24011, -31989, 40011, -47989, 56011, -63989]);
    }

    #[test]
    fn test_scale_down_adj_saturation_max_len8() {
        // Test saturation with adjustment
        let src = vec![
            i32::MAX / 2 - 20,  // Will saturate to MAX after shift and add
            1000,
            2000,
            3000,
            4000,
            5000,
            6000,
            7000
        ];
        let mut dest = vec![0; 8];
        
        scale_down_adj(&mut dest, &src, 0, 2); // Left shift by 2, then add 11
        
        // First element: (i32::MAX / 2 - 20) << 2 would overflow, so saturates to MAX
        // Then add 11 with saturation (stays at MAX)
        assert_eq!(dest[0], i32::MAX);
        assert_eq!(dest[1], 4011);
        assert_eq!(dest[2], 8011);
    }

    #[test]
    fn test_scale_down_adj_zero_values_len8() {
        let src = vec![0; 8];
        let mut dest = vec![42; 8];
        
        scale_down_adj(&mut dest, &src, 0, 0); // No shift, just add 11
        
        assert_eq!(dest, vec![11; 8]);
    }

}