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

 #[allow(dead_code)]
 impl OffsetLengths {

     /// Borrow as C struct (zero-cost, no copy)
     unsafe fn as_c_ref(&self) -> &crate::gen_ixheaacd_ref::offset_lengths {
         unsafe { &*(self as *const Self as *const crate::gen_ixheaacd_ref::offset_lengths) }
     }

     /// Borrow mutably as C struct (zero-cost, no copy)
    #[allow(invalid_reference_casting)]
     unsafe fn as_c_mut(&self) -> &mut crate::gen_ixheaacd_ref::offset_lengths {
         unsafe {
            let ptr = self as *const Self as *const crate::gen_ixheaacd_ref::offset_lengths;
            &mut *(ptr as *mut crate::gen_ixheaacd_ref::offset_lengths)
            }
     }

     /// Create from C struct reference (zero-cost, no copy)
     unsafe fn from_c_ref(c: &crate::gen_ixheaacd_ref::offset_lengths) -> &Self {
         unsafe { &*(c as *const crate::gen_ixheaacd_ref::offset_lengths as *const Self) }
     }

     /// Create from mutable C struct reference (zero-cost, no copy)
     unsafe fn from_c_mut(c: &mut crate::gen_ixheaacd_ref::offset_lengths) -> &mut Self {
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

/// Combines Forward Aliasing Cancellation (FAC) data by adding two sources with Q-format alignment.
///
/// # Q-Format Handling
/// - If `shift2 > shift1`: Right-shift src2 before adding
/// - If `shift2 < shift1`: Left-shift src2 with saturation before adding
/// - Result has `shift1` precision
///
/// # C signature
/// ```c
/// VOID ixheaacd_combine_fac(WORD32 *src1, WORD32 *src2, WORD32 *dest, WORD32 len,
///                           WORD8 output_q, WORD8 fac_q);
/// ```
pub fn combine_fac(
    src1: &[i32],      // First input buffer (overlap data)
    src2: &[i32],      // Second input buffer (FAC data)
    dest: &mut [i32],  // Output buffer for combined result
    shift1: i8,        // Q-format of output/src1
    shift2: i8,        // Q-format of FAC data (src2)
)
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

/// Performs overlap-add synthesis for long IMDCT blocks with different Q-formats
/// for current and overlap data.
///
/// # Returns
/// Output Q-format (min of shift1, shift2)
///
/// # Algorithm
/// Processes `vlen/2` samples:
/// - `dest[i] = src1[i] * win_fwd[i] + src2[i] * win_rev[i]` (Q-aligned)
/// - `dest[vlen-i-1] = -src1[i] * win_rev[i] + src2[vlen-i-1] * win_fwd[i]` (mirrored)
///
/// # C signature
/// ```c
/// WORD8 ixheaacd_windowing_long1(WORD32 *src1, WORD32 *src2,
///                                const WORD32 *win_fwd, const WORD32 *win_rev,
///                                WORD32 *dest, WORD32 vlen, WORD8 shift1,
///                                WORD8 shift2);
/// ```
pub fn windowing_long1(
    src1: &[i32],      // Current IMDCT output (first half)
    src2: &[i32],      // Current IMDCT output (second half)
    win_fwd: &[i32],   // Forward window coefficients (vlen/2 samples)
    win_rev: &[i32],   // Reverse window coefficients (vlen/2 samples, descending)
    dest: &mut [i32],  // Output buffer (vlen samples)
    vlen: i32,         // Vector length (total samples to process)
    shift1: i8,        // Q-format of src1 (current IMDCT)
    shift2: i8,        // Q-format of src2 (overlap buffer)
) -> i8
{
    let vlen_u = vlen as usize;
    assert!(vlen >= 0 && vlen % 2 == 0, "vlen must be non-negative and even");
    assert!(src1.len() >= vlen_u / 2, "src1 must have at least vlen/2 samples");
    assert!(src2.len() >= vlen_u, "src2 must have at least vlen samples");
    assert!(win_fwd.len() >= vlen_u / 2, "win_fwd must have at least vlen/2 samples");
    assert!(win_rev.len() >= vlen_u / 2, "win_rev must have at least vlen/2 samples");
    assert!(dest.len() >= vlen_u, "dest must have at least vlen samples");

    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_windowing_long1(
            src1.as_ptr() as *mut i32,
            src2.as_ptr() as *mut i32,
            win_fwd.as_ptr(),
            win_rev.as_ptr(),
            dest.as_mut_ptr(),
            vlen,
            shift1,
            shift2,
        )
    }
}

/// Handles long block windowing when transitioning from short to long blocks,
/// incorporating FAC data.
///
/// # Returns
/// Output Q-format
///
/// # Frame Regions
/// Controlled by `ixheaacd_drc_offset`:
/// 1. **Flat left** `[0 .. n_flat_ls + lfac)`: Copy overlap buffer
/// 2. **Transition** `[n_flat_ls + lfac .. n_flat_ls + n_trans_ls)`: Windowed IMDCT + FAC
/// 3. **Flat middle** `[n_flat_ls + n_trans_ls .. n_flat_ls + 3*lfac)`: Direct IMDCT + FAC
/// 4. **Right half** `[n_flat_ls + 3*lfac .. n_long)`: Negated IMDCT only
///
/// # C signature
/// ```c
/// WORD8 ixheaacd_windowing_long2(WORD32 *src1, const WORD32 *win_fwd,
///                                WORD32 *fac_data_out, WORD32 *over_lap,
///                                WORD32 *p_out_buffer,
///                                offset_lengths *ixheaacd_drc_offset,
///                                WORD8 shiftp, WORD8 shift_olap, WORD8 fac_q);
/// ```
pub fn windowing_long2(
    src1: &[i32],                        // Current IMDCT output (long block)
    win_fwd: &[i32],                     // Forward window coefficients
    fac_data_out: &[i32],                // FAC transition data
    over_lap: &[i32],                    // Overlap buffer from previous frame
    p_out_buffer: &mut [i32],            // Output buffer (n_long samples)
    ixheaacd_drc_offset: &OffsetLengths,  // Frame geometry (lfac, n_flat_ls, n_trans_ls, n_long)
    shift1: i8,                          // Q-format of current IMDCT (src1)
    shift2: i8,                          // Q-format of overlap buffer
    shift3: i8,                          // Q-format of FAC data
) -> i8
{
    let n_long = ixheaacd_drc_offset.n_long as usize;
    assert!(p_out_buffer.len() >= n_long, "output buffer must have at least n_long samples");
    assert!(over_lap.len() >= (ixheaacd_drc_offset.n_flat_ls + ixheaacd_drc_offset.lfac) as usize, "overlap buffer too small");
    assert!(src1.len() >= n_long / 2, "src1 must have at least n_long/2 samples");

    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_windowing_long2(
            src1.as_ptr() as *mut i32,
            win_fwd.as_ptr(),
            fac_data_out.as_ptr() as *mut i32,
            over_lap.as_ptr() as *mut i32,
            p_out_buffer.as_mut_ptr(),
            ixheaacd_drc_offset.as_c_mut(),
            shift1,
            shift2,
            shift3,
        )
    }
}

/// Standard overlap-add for long blocks with flat and transition regions.
///
/// # Returns
/// Output Q-format
///
/// # Frame Regions
/// 1. **Flat left** `[0 .. n_flat_ls)`: Copy overlap buffer
/// 2. **First transition** `[n_flat_ls .. n_long/2)`: `src1[i] * win_fwd + overlap[i] * win_rev`
/// 3. **Second transition** `[n_long/2 .. n_flat_ls + n_trans_ls)`: `-src1[n_long-i-1] * win_fwd + overlap[i] * win_rev`
/// 4. **Right** `[n_flat_ls + n_trans_ls .. n_long)`: Negated IMDCT only
///
/// # C signature
/// ```c
/// WORD8 ixheaacd_windowing_long3(WORD32 *src1, const WORD32 *win_fwd,
///                                WORD32 *over_lap, WORD32 *p_out_buffer,
///                                const WORD32 *win_rev,
///                                offset_lengths *ixheaacd_drc_offset,
///                                WORD8 shiftp, WORD8 shift_olap);
/// ```
pub fn windowing_long3(
    src1: &[i32],                        // Current IMDCT output
    win_fwd: &[i32],                     // Forward window coefficients
    over_lap: &[i32],                    // Overlap buffer from previous frame
    p_out_buffer: &mut [i32],            // Output buffer
    win_rev: &[i32],                     // Reverse window coefficients
    ixheaacd_drc_offset: &OffsetLengths,  // Frame geometry
    shift1: i8,                          // Q-format of current IMDCT
    shift2: i8,                          // Q-format of overlap buffer
) -> i8
{
    {
        let off = ixheaacd_drc_offset;
        assert!(src1.len() >= off.n_long as usize, "src1 must have at least n_long samples");
        assert!(over_lap.len() >= (off.n_flat_ls + off.n_trans_ls) as usize, "overlap buffer too small");
        assert!(p_out_buffer.len() >= off.n_long as usize, "output buffer must have at least n_long samples");
    }
    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_windowing_long3(
            src1.as_ptr() as *mut i32,
            win_fwd.as_ptr(),
            over_lap.as_ptr() as *mut i32,
            p_out_buffer.as_mut_ptr(),
            win_rev.as_ptr(),
            ixheaacd_drc_offset.as_c_mut(),
            shift1,
            shift2,
        )
    }
}

/// Initializes overlap buffer for short block processing, handling FAC region.
///
/// # Processing
/// - Adjusts first `lfac` samples of `fp` for Q-format alignment
/// - If `n_short > lfac`: Fills `[lfac .. n_short)` with windowed IMDCT
/// - Zeros out `[n_short .. n_flat_ls + lfac)`
///
/// # C signature
/// ```c
/// VOID ixheaacd_windowing_short1(WORD32 *src1, WORD32 *src2, WORD32 *fp,
///                                offset_lengths *ixheaacd_drc_offset,
///                                WORD8 shiftp, WORD8 shift_olap);
/// ```
pub fn windowing_short1(
    src1: &[i32],                        // Current IMDCT output (short block)
    src2: &[i32],                        // Window coefficients
    fp: &mut [i32],                      // In/Out overlap buffer (modified in-place)
    ixheaacd_drc_offset: &OffsetLengths,  // Frame geometry (lfac, n_flat_ls, n_short)
    shiftp: i8,                          // Q-format of current IMDCT
    shift_olap: i8,                      // Q-format of overlap buffer (fp)
)
{
    {
        let off = ixheaacd_drc_offset;
        assert_eq!(src1.len(), off.n_short as usize, "src1 must have n_short samples");
        assert_eq!(src2.len(), off.n_short as usize, "src2 must have n_short samples");
        assert!(fp.len() >= (off.n_flat_ls + off.lfac) as usize, "fp must have at least n_flat_ls + lfac samples");
    }
    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_windowing_short1(
            src1.as_ptr() as *mut i32,
            src2.as_ptr() as *mut i32,
            fp.as_mut_ptr(),
            ixheaacd_drc_offset.as_c_mut(),
            shiftp,
            shift_olap,
        );
    }
}

/// Performs overlap-add for short blocks with window application.
///
/// # Processing
/// - `fp[i] = src1[i] * win_fwd[i] + fp[i] * win_rev[i]` (Q-aligned)
/// - `fp[n_short-i-1] = -src1[i] * win_rev[i] + fp[n_short-i-1] * win_fwd[i]`
/// - Zeros out `[n_short .. n_flat_ls + n_short)`
///
/// # C signature
/// ```c
/// VOID ixheaacd_windowing_short2(WORD32 *src1, WORD32 *win_fwd, WORD32 *fp,
///                                offset_lengths *ixheaacd_drc_offset,
///                                WORD8 shiftp, WORD8 shift_olap);
/// ```
pub fn windowing_short2(
    src1: &[i32],                        // Current IMDCT output (short block)
    win_fwd: &[i32],                     // Window coefficients (n_short samples)
    fp: &mut [i32],                      // In/Out overlap buffer
    ixheaacd_drc_offset: &OffsetLengths,  // Frame geometry
    shiftp: i8,                          // Q-format of current IMDCT
    shift_olap: i8,                      // Q-format of overlap buffer
)
{
    {
        let off = ixheaacd_drc_offset;
        assert!(src1.len() >= (off.n_short as usize) / 2, "src1 must have at least n_short/2 samples");
        assert!(win_fwd.len() >= off.n_short as usize, "win_fwd must have at least n_short samples");
        assert!(fp.len() >= (off.n_flat_ls + off.n_short) as usize, "fp must have at least n_flat_ls + n_short samples");
    }
    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_windowing_short2(
            src1.as_ptr() as *mut i32,
            win_fwd.as_ptr() as *mut i32,
            fp.as_mut_ptr(),
            ixheaacd_drc_offset.as_c_mut(),
            shiftp,
            shift_olap,
        );
    }
}

/// Final windowing stage for short blocks in EIGHT_SHORT_SEQUENCE mode.
///
/// # Returns
/// Output Q-format (min of shiftp, shift_olap)
///
/// # Processing
/// Processes `n_short/2` samples using second half of src1:
/// - `fp[i] = -src1[n_short/2-i-1] * win_rev + fp[i]` (Q-aligned)
/// - `fp[n_short-i-1] = -src1[n_short/2-i-1] * win_fwd + fp[n_short-i-1]`
///
/// # C signature
/// ```c
/// WORD8 ixheaacd_windowing_short3(WORD32 *src1, WORD32 *win_rev, WORD32 *fp,
///                                 WORD32 nshort, WORD8 shiftp, WORD8 shift_olap);
/// ```
pub fn windowing_short3(
    src1: &[i32],      // Current IMDCT output (second half used: `[n_short/2 .. n_short)`)
    win_fwd: &[i32],   // Reverse window coefficients
    fp: &mut [i32],    // In/Out overlap buffer
    shiftp: i8,        // Q-format of current IMDCT
    shift_olap: i8,    // Q-format of overlap buffer
) -> i8
{
    let n_short = src1.len();
    assert_eq!(n_short, win_fwd.len(), "forward window must have same length");
    assert_eq!(n_short, fp.len(), "overlap buffer must have same length");
    unsafe {
        let win_rev = win_fwd.as_ptr().add(n_short - 1);
        crate::gen_ixheaacd_ref::ixheaacd_windowing_short3(
            src1.as_ptr() as *mut i32,
            win_rev as *mut i32,
            fp.as_mut_ptr(),
            n_short as i32,
            shiftp,
            shift_olap,
        )
    }
}

/// Windowing function performs overlap-add synthesis with windowing for short IMDCT blocks in AAC's
///  EIGHT_SHORT_SEQUENCE mode, ensuring smooth transitions between consecutive audio frames.
///
/// # Algorithm
/// 
/// if (shift_olap > output_q):
///   Scale DOWN overlap buffer to match output_q
///   return output_q
/// else:
///    Scale DOWN output to match shift_olap
///    return shift_olap
///
/// **Stage 1: First Half Windowing** `[0 .. n_short/2)`
/// 
/// - Reads second half of src1: `src1[n_short/2 + i]`
/// - Applies forward/reverse windows
/// - Updates first half and mirror positions of `fp`
/// - `fp[i] = src1[n_short/2 + i] * win_fwd[i] + fp[i]` (Q-aligned)
/// 
/// **Stage 2: Second Half Processing** `[n_short/2 .. n_short)` - Controlled by `windowed_flag`
/// 
/// **If `windowed_flag == true`:**
/// - Applies backward window using `win_fwd1` (reverse coefficients)
/// - `fp[i + n_short/2] = -src1[n_short - i - 1] * win_fwd1 + fp[i + n_short/2]`
/// 
/// **If `windowed_flag == false`:**
/// - Direct negated copy without windowing
/// - `fp[i + n_short/2] = -src1[n_short - i - 1] + fp[i + n_short/2]`
/// 
/// # C signature
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
    // windowing_short1() tests
    // ============================================================================
    // Tests cover all code paths in ixheaacd_windowing_short1():
    // - Branch 1: shift_olap > shiftp && n_short > lfac
    // - Branch 2: shift_olap > shiftp && n_short <= lfac
    // - Branch 3: shift_olap <= shiftp && n_short > lfac
    // - Branch 4: shift_olap <= shiftp && n_short <= lfac
    // ============================================================================

    // Helper function to create simple sine-like window coefficients
    fn create_test_window_len32() -> Vec<i32> {
        vec![
              13176960,  118530360,  223600288,  328129056,  431869696,  534568800,  635979456,  735858880, 
             833964544,  930062272, 1023918080, 1115308543, 1204012415, 1289815167, 1372508287, 1451898623,
            1527788543, 1599999871, 1668354303, 1732692863, 1792854655, 1848697855, 1900087039, 1946900095,
            1989020799, 2026350591, 2058798975, 2086288895, 2108751615, 2126133375, 2138393343, 2145503615]
    }

    #[test]
    fn test_windowing_short1_branch1_olap_nshort() {
        // Branch 1: shift_olap > shiftp && n_short > lfac
        let offset = OffsetLengths {
            lfac: 24,
            n_flat_ls: 104,
            n_trans_ls: 48,
            n_long: 256,
            n_short: 32,
        };
        let src1 = vec![134760976, 192459648, 235105440, 86985200, 147657648, 184622976, 138333184, 109944192, 46901072, 97105280, 163566848, 114981248, 77520880, 10512288, 14358304, -114643568,
                -184019744, -157282624, -236088256, -106367824, -217254992, -100738448, 85402512, -105615168, 133200032, -163217168, -167557232, 294838336, 288447472, 175529072, 151728032, -17404576];
        let src2 = create_test_window_len32();
        let mut fp = vec![-206367421; (offset.n_flat_ls + offset.lfac) as usize];

        windowing_short1(&src1, &src2, &mut fp, &offset, 12, 14);

        let exp = vec![-51591856, -51591856, -51591856, -51591856, -51591856, -51591856, -51591856, -51591856, -51591856, -51591856, -51591856, -51591856, -51591856, -51591856, -51591856, -51591856, -51591856, -51591856, -51591856, -51591856, -51591856, -51591856, -51591856, -51591856, -101831409, -130530228, -176998598, -143449992, -85416335, -232768023, -191644966, -134636724, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(fp, exp);
    }

    #[test]
    fn test_windowing_short1_branch2_olap_lfac() {
        // Branch 2: shift_olap > shiftp && n_short <= lfac
        let offset = OffsetLengths {
            lfac: 32,
            n_flat_ls: 100,
            n_trans_ls: 48,
            n_long: 256,
            n_short: 32,
        };
        let src1 = vec![112770912, 221991872, 155566144, 101234304, 112886528, -8768352, -35112000, -109608256, -149689984, -247050848, -252940448, -226440160, -214432576, -260521152, -121165184, -95656064, 57978112, 128177856, 41645280, 156887200, 97545120, -12161216, -51263168, -51276128, -32786144, -57784416, -49523424, -19171456, 5140672, -20268256, -5941088, -12329696];
        let src2 = create_test_window_len32();
        let mut fp = vec![8000; (offset.n_flat_ls + offset.lfac) as usize];

        windowing_short1(&src1, &src2, &mut fp, &offset, 12, 16);

        let exp = vec![500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(fp, exp);
    }

    #[test]
    fn test_windowing_short1_branch3_shiftp_nshort() {
        // Branch 3: shift_olap <= shiftp && n_short > lfac
        let offset = OffsetLengths {
            lfac: 16,
            n_flat_ls: 104,
            n_trans_ls: 48,
            n_long: 256,
            n_short: 32,
        };
        let src1 = vec![-2231040, -1453504, -8470208, -6010560, -37089856, -63401792, -99061664, -49948160, -39689344, -36031552, -45139328, -22368544, 22745568, 33116832, 35384800, 2671072, -26372288, -35004416, -23028672, -3573056, -29113216, -45877920, -55658880, -59907872, -60211552, -77955136, -38060672, -21001088, 3338240, -24066816, -17012992, -15744480];
        let src2 = create_test_window_len32();
        let mut fp = vec![3000; (offset.n_flat_ls + offset.lfac) as usize];

        windowing_short1(&src1, &src2, &mut fp, &offset, 16, 14);

        let exp = vec![3000, 3000, 3000, 3000, 3000, 3000, 3000, 3000, 3000, 3000, 3000, 3000, 3000, 3000, 3000, 3000, -475072, -6590933, -6432018, -4588054, 4668667, 9714739, 7970152, 8995550, 11565621, 23368473, 15195871, 9008235, 1475538, 2096499, 361837, 557245, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(fp, exp);
    }

    #[test]
    fn test_windowing_short1_branch4_shiftp_lfac() {
        // Branch 4: shift_olap <= shiftp && n_short <= lfac
        let offset = OffsetLengths {
            lfac: 64,
            n_flat_ls: 112,
            n_trans_ls: 32,
            n_long: 256,
            n_short: 32,
        };
        let src1 = vec![112770912, 221991872, 155566144, 101234304, 112886528, -8768352, -35112000, -109608256, -149689984, -247050848, -252940448, -226440160, -214432576, -260521152, -121165184, -95656064, 57978112, 128177856, 41645280, 156887200, 97545120, -12161216, -51263168, -51276128, -32786144, -57784416, -49523424, -19171456, 5140672, -20268256, -5941088, -12329696];
        let src2 = create_test_window_len32();
        let mut fp = vec![68186432; (offset.n_flat_ls + offset.lfac) as usize];

        windowing_short1(&src1, &src2, &mut fp, &offset, 18, 14);

        let exp = vec![68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(fp, exp);
    }

    // ============================================================================
    // windowing_short2() tests
    // ============================================================================
    // Tests cover all code paths in ixheaacd_windowing_short2():
    // - Branch 1: shift_olap > shiftp
    // - Branch 2: shift_olap <= shiftp
    // ============================================================================

    #[test]
    fn test_windowing_short2_branch1_olap_gt_shiftp() {
        // Branch 1: shift_olap > shiftp
        let offset = OffsetLengths {
            lfac: 16,
            n_flat_ls: 112,
            n_trans_ls: 32,
            n_long: 256,
            n_short: 32,
        };
        let src1 = vec![-2231040, -1453504, -8470208, -6010560, -37089856, -63401792, -99061664, -49948160, -39689344, -36031552, -45139328, -22368544, 22745568, 33116832, 35384800, 2671072, -26372288, -35004416, -23028672, -3573056, -29113216, -45877920, -55658880, -59907872, -60211552, -77955136, -38060672, -21001088, 3338240, -24066816, -17012992, -15744480];
        let win_fwd = create_test_window_len32();
        let mut fp = vec![-4606648; (offset.n_flat_ls + offset.n_short) as usize];

        windowing_short2(&src1, &win_fwd, &mut fp, &offset, 14, 16);

        let exp = vec![-1164291, -1227015, -2022148, -2049287, -8577801, -16886583, -30423914, -18181968, -16457252, -16624037, -22513815, -12578718, 11823357, 18995818, 21757219, 986563, -2678918, -27099784, -26419778, -18997910, 18076546, 38309846, 31381830, 35534957, 45867854, 93132829, 60496805, 35801337, 5726182, 8266083, 1383785, 2221915, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(fp, exp);
    }

    #[test]
    fn test_windowing_short2_branch2_olap_le_shiftp() {
        // Branch 2: shift_olap <= shiftp
        let offset = OffsetLengths {
            lfac: 16,
            n_flat_ls: 112,
            n_trans_ls: 32,
            n_long: 256,
            n_short: 32,
        };
        let src1 = vec![134760976, 192459648, 235105440, 86985200, 147657648, 184622976, 138333184, 109944192, 46901072, 97105280, 163566848, 114981248, 77520880, 10512288, 14358304, -114643568,
                -184019744, -157282624, -236088256, -106367824, -217254992, -100738448, 85402512, -105615168, 133200032, -163217168, -167557232, 294838336, 288447472, 175529072, 151728032, -17404576];
        let win_fwd = create_test_window_len32();
        let mut fp = vec![77680672; (offset.n_flat_ls + offset.n_short) as usize];

        windowing_short2(&src1, &win_fwd, &mut fp, &offset, 15, 14);

        let exp = vec![78022494, 82663254, 89148195, 82925160, 90314426, 97451604, 93782696, 90785407, 79531884, 89759479, 105866983, 94710820, 84407997, 63506111, 62464974, 16509658, 93299907, 44298694, 42572895, 12278814, -7652872, -33366586, -9316198, 8906737, -24297569, -42259905, -69162406, -56103025, -30838795, -108295746, -91534899, -66841714, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(fp, exp);
    }

    #[test]
    fn test_windowing_short2_equal_shifts() {
        // Edge case: shift_olap == shiftp
        let offset = OffsetLengths {
            lfac: 16,
            n_flat_ls: 112,
            n_trans_ls: 32,
            n_long: 256,
            n_short: 32,
        };
        let src1 = vec![112770912, 221991872, 155566144, 101234304, 112886528, -8768352, -35112000, -109608256, -149689984, -247050848, -252940448, -226440160, -214432576, -260521152, -121165184, -95656064, 57978112, 128177856, 41645280, 156887200, 97545120, -12161216, -51263168, -51276128, -32786144, -57784416, -49523424, -19171456, 5140672, -20268256, -5941088, -12329696];
        let win_fwd = create_test_window_len32();
        let mut fp = vec![68186432; (offset.n_flat_ls + offset.n_short) as usize];

        windowing_short2(&src1, &win_fwd, &mut fp, &offset, 14, 14);

        let exp = vec![68815524, 80150639, 83706379, 82424917, 88945434, 63187846, 53941786, 25596480, 3686188, -46665091, -61902306, -60676730, -65208106, -103500210, -26636729, -16162386, 114153150, 133854672, 243349688, 211244031, 224459545, 250259313, 248120977, 162188201, 124885091, 53324918, 25379756, -95957079, -88989760, -146919800, -217288630, -112248544, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(fp, exp);
    }

    // ============================================================================
    // windowing_short3() tests
    // ============================================================================
    // Tests cover all code paths in ixheaacd_windowing_short3():
    // - Branch 1: shift_olap > shiftp (returns shiftp)
    // - Branch 2: shift_olap <= shiftp (returns shift_olap)
    // ============================================================================

    // Helper function to create simple sine-like window coefficients
    fn create_test_window_len12() -> Vec<i32> {
        vec![13176960, 302060768, 585449152, 858185984, 1115308543, 1352137343, 1564364543, 1748129023, 1900087039, 2017472895, 2098151679, 2140654591]
    }

    #[test]
    fn test_windowing_short3_branch1_olap_gt_shiftp() {
        // Branch 1: shift_olap > shiftp, returns shiftp
        let src1 = vec![249141632, 241070784, 305427840, 231461440, 167675776, 119436992, 11057152, -54612032, -149492736, -159054784, -125548352, -13541312];
        let win_fwd = create_test_window_len12();
        let mut fp = vec![0; src1.len()];

        let result_q = windowing_short3(&src1, &win_fwd, &mut fp, 14, 16);

        let exp = vec![-119057180, -163823931, -217448539, -270241630, -196240300, -181490712, -156869043, -125201561, -122056293, -63101251, -23584941, -732866];
        assert_eq!(fp, exp);
        assert_eq!(result_q, 14);
    }

    #[test]
    fn test_windowing_short3_branch2_olap_le_shiftp() {
        // Branch 2: shift_olap <= shiftp, returns shift_olap
        let src1 = vec![139012976, -43757040, -70552272, 1001600, -27581184, 209319392, -222776656, 17346800, 47883904, -90583664, 57344384, 10769696];
        let win_fwd = create_test_window_len12();
        let mut fp = vec![0; src1.len()];

        let result_q = windowing_short3(&src1, &win_fwd, &mut fp, 17, 14);

        let exp = vec![-26081719, 3368448, -117621, 7803054, 4452475, -12658244, -10940982, 2840685, 3524297, -34133, 484939, -160548];
        assert_eq!(fp, exp);
        assert_eq!(result_q, 14);
    }

    #[test]
    fn test_windowing_short3_equal_shifts() {
        // Edge case: shift_olap == shiftp
        let src1 = vec![139012976, -43757040, -70552272, 1001600, -27581184, 209319392, -222776656, 17346800, 47883904, -90583664, 57344384, 10769696];
        let win_fwd = create_test_window_len12();
        let mut fp = vec![0; src1.len()];

        let result_q = windowing_short3(&src1, &win_fwd, &mut fp, 15, 15);

        let exp = vec![-208653751, 26947589, -940963, 62424436, 35619806, -101265950, -87527855, 22725481, 28194380, -273058, 3879514, -1284384];
        assert_eq!(fp, exp);
        assert_eq!(result_q, 15);
    }

    #[test]
    fn test_windowing_short3_negative_values() {
        // Test with negative src1 values
        let src1 = vec![68698176, -64474464, -86033120, -17652736, 22920896, -164053840, -132149856, -86296016, 32181376, -56454480, -73557968, -100885984];
        let win_fwd = create_test_window_len12();
        let mut fp = vec![0; src1.len()];

        let result_q = windowing_short3(&src1, &win_fwd, &mut fp, 14, 16);

        let exp = vec![163532144, -22394358, 16584022, 76121844, 52484535, -50044149, -43254984, 33485200, 34380898, 4812506, -3224008, 1006634];
        assert_eq!(fp, exp);
        assert_eq!(result_q, 14);
    }

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

    // ============================================================================
    // Branch 1: shift_olap > output_q, flag=1 (windowed)
    // ============================================================================

    #[test]
    fn test_windowing_short4_branch1_flag1() {
        let src1: Vec<i32> = vec![232644448, 87804592, -6177088, -2147483648, 251254752, 219674032, 125828992, 184318592, 106729056, 203009648, 281430368, 171565216];
        let win_fwd = create_test_window_len12();
        let mut fp = vec![21309976; 2 * src1.len()];
        
        let result_q = windowing_short4(
            &src1, &win_fwd, &mut fp, &win_fwd,
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
        let win_fwd = create_test_window_len12();
        let mut fp = vec![4352798; 2 * src1.len()];
        
        let result_q = windowing_short4(
            &src1, &win_fwd, &mut fp, &win_fwd,
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
        let win_fwd = create_test_window_len12();
        let mut fp = vec![-6177088; 2 * src1.len()];
        
        let result_q = windowing_short4(
            &src1, &win_fwd, &mut fp, &win_fwd,
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
        let win_fwd = create_test_window_len12();
        let mut fp = vec![162292782; 2 * src1.len()];
        
        let result_q = windowing_short4(
            &src1, &win_fwd, &mut fp, &win_fwd,
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
        let win_fwd = create_test_window_len12();
        let mut fp = vec![100; 2 * src1.len()];
        
        let result_q = windowing_short4(
            &src1, &win_fwd, &mut fp, &win_fwd,
            true, 17, 15, 15
        );
        
        let exp = vec![101, 135, 168, 199, 229, 257, -83, -104, -122, -135, -145, -150, -150, -145, -135, -122, -104, -83, -58, -30, 0, 31, 64, 98];
        assert_eq!(fp, exp);
        assert_eq!(result_q, 15);
    }

    #[test]
    fn test_windowing_short4_zero_src() {
        let src1 = vec![0; 12];
        let win_fwd = create_test_window_len12();
        let mut fp = vec![100; 2 * src1.len()];
        
        let result_q = windowing_short4(
            &src1, &win_fwd, &mut fp, &win_fwd,
            true, 18, 16, 14
        );
        
        let exp = vec![100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25];
        assert_eq!(fp, exp);
        assert_eq!(result_q, 14);
    }

    #[test]
    fn test_windowing_short4_zero_fp() {
        let src1 = vec![1000; 12];
        let win_fwd = create_test_window_len12();
        let mut fp = vec![0; 2 * src1.len()];
        
        let result_q = windowing_short4(
            &src1, &win_fwd, &mut fp, &win_fwd,
            true, 17, 16, 14
        );
        
        let exp = vec![0, 17, 34, 49, 64, 78, -92, -102, -111, -118, -123, -125, -125, -123, -118, -111, -102, -92, -79, -65, -50, -35, -18, -1];
        assert_eq!(fp, exp);
        assert_eq!(result_q, 14);
    }

    #[test]
    fn test_windowing_short4_large_shift_difference() {
        let src1 = vec![1000; 12];
        let win_fwd = create_test_window_len12();
        let mut fp = vec![100; 2 * src1.len()];
        
        // Large shift difference: shift_olap - output_q = 8
        let result_q = windowing_short4(
            &src1, &win_fwd, &mut fp, &win_fwd,
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