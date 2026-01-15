// decoder::ixheaacd::basic_ops

//! Basic operations for the decoder, DSP primitives
//!

#[allow(dead_code)]
const ADJ_SCALE: i32 = 11;

// ============================================================================
// Fixed-point DSP operations extension trait
// ============================================================================

/// Extension trait for fixed-point DSP operations on i32
#[cfg(not(feature = "fallback"))]
trait FixedPointOps {
    /// Q31 fixed-point multiply: (a * b) >> 31
    fn mult_shift_q31(self, other: Self) -> Self;
    /// Saturating left shift: clamps to i32::MIN/MAX on overflow
    fn saturating_shl(self, shift: u32) -> Self;
}

#[cfg(not(feature = "fallback"))]
impl FixedPointOps for i32 {
    #[inline]
    fn mult_shift_q31(self, other: Self) -> Self {
        ((self as i64 * other as i64) >> 31) as i32
    }

    #[inline]
    fn saturating_shl(self, shift: u32) -> Self {
        if self > (i32::MAX >> shift) {
            i32::MAX
        } else if self < (i32::MIN >> shift) {
            i32::MIN
        } else {
            self << shift
        }
    }
}

 #[derive(Clone, Copy)]
 pub struct OffsetLengths {
     pub lfac: usize,          // FAC (Forward Aliasing Cancellation) length
     pub n_flat_ls: usize,     // Flat region length (left side)
     pub n_trans_ls: usize,    // Transition region length (left side)
     pub n_long: usize,        // Long block length (full frame)
     pub n_short: usize,       // Short block length (1/8 of frame)
 }

 #[allow(dead_code)]
 impl OffsetLengths {

    /// Copy as C struct
    fn as_c_struct(&self) -> crate::gen_ixheaacd_ref::offset_lengths {
        crate::gen_ixheaacd_ref::offset_lengths {
            lfac: self.lfac as std::os::raw::c_int,
            n_flat_ls: self.n_flat_ls as std::os::raw::c_int,
            n_trans_ls: self.n_trans_ls as std::os::raw::c_int,
            n_long: self.n_long as std::os::raw::c_int,
            n_short: self.n_short as std::os::raw::c_int,
        }
    }

     /// Create from C struct 
    pub unsafe fn from_c_struct(c: *const crate::gen_ixheaacd_ref::offset_lengths) -> Self {
        unsafe { 
            Self {
                lfac: (*c).lfac as usize,
                n_flat_ls: (*c).n_flat_ls as usize,
                n_trans_ls: (*c).n_trans_ls as usize,
                n_long: (*c).n_long as usize,
                n_short: (*c).n_short as usize,
            }
        }
     }

}



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

    #[cfg(feature = "fallback")]
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

    #[cfg(not(feature = "fallback"))]
    {
        if shift2 > shift1 {
            // Right-shift src2 before adding
            let shift = (shift2 - shift1) as u32;
            for ((d, &s1), &s2) in dest.iter_mut().zip(src1.iter()).zip(src2.iter()) {
                *d = s1.saturating_add(s2 >> shift);
            }
        } else {
            // Left-shift src2 with saturation before adding
            let shift = (shift1 - shift2) as u32;
            for ((d, &s1), &s2) in dest.iter_mut().zip(src1.iter()).zip(src2.iter()) {
                *d = s1.saturating_add(s2.saturating_shl(shift));
            }
        }
    }
}

/// Performs overlap-add synthesis for long IMDCT blocks with different Q-formats
/// for current and overlap data.
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
    win_fwd: &[i32],   // Forward window coefficients
    win_rev: &[i32],   // Reverse window coefficients
    dest: &mut [i32],  // Output buffer (vlen samples)
    shift1: i8,        // Q-format of src1 (current IMDCT)
    shift2: i8,        // Q-format of src2 (overlap buffer)
) -> i8  // Returns Q-format of output (min of shift1, shift2)
{
    let vlen = dest.len();
    assert!(src1.len() >= vlen / 2, "src1 must have at least vlen/2 samples");
    assert_eq!(src2.len(), vlen, "src2 must have at least vlen samples");
    assert_eq!(win_fwd.len(), vlen, "win_fwd must have at least vlen/2 samples");
    assert_eq!(win_rev.len(), vlen, "win_rev must have at least vlen/2 samples");

    #[cfg(feature = "fallback")]
    unsafe {
        let win_rev = win_rev.as_ptr().add(win_rev.len() - 1);
        return crate::gen_ixheaacd_ref::ixheaacd_windowing_long1(
            src1.as_ptr() as *mut i32,
            src2.as_ptr() as *mut i32,
            win_fwd.as_ptr(),
            win_rev as *mut i32,
            dest.as_mut_ptr(),
            vlen as i32,
            shift1,
            shift2,
        );
    }

    #[cfg(not(feature = "fallback"))]
    {
        let half = vlen / 2;

        if shift1 > shift2 {
            let shift = (shift1 - shift2) as u32;
            for i in 0..half {
                let mirror = vlen - i - 1;
                dest[i] = (src1[i].mult_shift_q31(win_fwd[i]) >> shift)
                    .saturating_add(src2[i].mult_shift_q31(win_rev[mirror]));
                dest[mirror] = (src1[i].saturating_neg().mult_shift_q31(win_rev[mirror]) >> shift)
                    .saturating_add(src2[mirror].mult_shift_q31(win_fwd[i]));
            }
            shift2
        } else {
            let shift = (shift2 - shift1) as u32;
            for i in 0..half {
                let mirror = vlen - i - 1;
                dest[i] = src1[i].mult_shift_q31(win_fwd[i])
                    .saturating_add(src2[i].mult_shift_q31(win_rev[mirror]) >> shift);
                dest[mirror] = src1[i].saturating_neg().mult_shift_q31(win_rev[mirror])
                    .saturating_add(src2[mirror].mult_shift_q31(win_fwd[i]) >> shift);
            }
            shift1
        }
    }
}

/// Handles long block windowing when transitioning from short to long blocks,
/// incorporating FAC data (Forward Aliasing Cancellation).
///
/// # Frame Regions
/// Controlled by `offset`:
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
///                                offset_lengths *offset,
///                                WORD8 shiftp, WORD8 shift_olap, WORD8 fac_q);
/// ```
pub fn windowing_long2(
    src1: &[i32],                        // Current IMDCT output (long block)
    win_fwd: &[i32],                     // Forward window coefficients (n_trans_ls + lfac)
    fac_data_out: &[i32],                // FAC transition data (input only, length: lfac * 2)
    over_lap: &[i32],                    // Overlap buffer from previous frame (n_flat_ls + lfac)
    dest: &mut [i32],            // Output buffer (n_long samples)
    offset: &OffsetLengths,  // Frame geometry (lfac, n_flat_ls, n_trans_ls, n_long)
    shiftp: i8,                          // Q-format of current IMDCT (src1)
    shift_olap: i8,                      // Q-format of overlap buffer
    fac_q: i8,                           // Q-format of FAC data
) -> i8  // Returns Q-format of output buffer
{
    let n_long = offset.n_long as usize;
    let n_trans_ls = offset.n_trans_ls;
    let lfac = offset.lfac;
    let n_flat_ls = offset.n_flat_ls;

    assert_eq!(src1.len(), n_long, "source buffer must have at least n_long samples");
    assert_eq!(dest.len(), n_long, "output buffer must have at least n_long samples");
    assert!(fac_data_out.len() >= lfac * 2, "fac_data buffer too small");
    assert!(over_lap.len() >= lfac + n_flat_ls, "overlap buffer too small");
    assert!(win_fwd.len() >= n_trans_ls, "win_fwd too small");

    #[cfg(feature = "fallback")]
    unsafe {
        let mut ixheaacd_drc_offset = offset.as_c_struct();
        return crate::gen_ixheaacd_ref::ixheaacd_windowing_long2(
            src1.as_ptr() as *mut i32,
            win_fwd.as_ptr(),
            fac_data_out.as_ptr() as *mut i32,
            over_lap.as_ptr() as *mut i32,
            dest.as_mut_ptr(),
            &mut ixheaacd_drc_offset,
            shiftp,
            shift_olap,
            fac_q,
        );
    }

    #[cfg(not(feature = "fallback"))]
    {
        // Region boundaries
        let region1_end = n_flat_ls + lfac;
        let region2_end = n_flat_ls + n_trans_ls;
        let region3_end = n_flat_ls + 3 * lfac;

        // src1 base index for computing source indices
        let base = n_long / 2 + n_flat_ls + lfac;

        // win_fwd is used starting at lfac offset in C code
        let win_fwd = &win_fwd[lfac..];

        if shiftp > fac_q {
            if shift_olap > fac_q {
                // Branch 1: shiftp > fac_q && shift_olap > fac_q → return fac_q
                let shift_o = (shift_olap - fac_q) as u32;
                let shift_p = (shiftp - fac_q) as u32;

                // Region 1: Copy overlap with shift
                for i in 0..region1_end {
                    dest[i] = over_lap[i] >> shift_o;
                }
                // Region 2: Windowed IMDCT + FAC
                for (i, (fac, &win)) in (region1_end..region2_end).zip(fac_data_out.iter().zip(win_fwd.iter())) {
                    dest[i] = (src1[base - i - 1].saturating_neg().mult_shift_q31(win) >> shift_p)
                        .saturating_add(*fac);
                }
                // Region 3: Negated IMDCT + FAC (no window)
                let fac_offset = region2_end - region1_end;
                for (i, fac) in (region2_end..region3_end).zip(fac_data_out[fac_offset..].iter()) {
                    dest[i] = (src1[base - i - 1].saturating_neg() >> shift_p).saturating_add(*fac);
                }
                // Region 4: Negated IMDCT only
                for i in region3_end..n_long {
                    dest[i] = src1[base - i - 1].saturating_neg() >> shift_p;
                }
                fac_q
            } else {
                // Branch 2: shiftp > fac_q && shift_olap <= fac_q → return shift_olap
                let shift_p = (shiftp - shift_olap) as u32;
                let shift_f = (fac_q - shift_olap) as u32;

                // Region 1: Copy overlap directly
                dest[..region1_end].copy_from_slice(&over_lap[..region1_end]);
                // Region 2: Windowed IMDCT + FAC
                for (i, (fac, &win)) in (region1_end..region2_end).zip(fac_data_out.iter().zip(win_fwd.iter())) {
                    dest[i] = (src1[base - i - 1].saturating_neg().mult_shift_q31(win) >> shift_p)
                        .saturating_add(*fac >> shift_f);
                }
                // Region 3: Negated IMDCT + FAC
                let fac_offset = region2_end - region1_end;
                for (i, fac) in (region2_end..region3_end).zip(fac_data_out[fac_offset..].iter()) {
                    dest[i] = (src1[base - i - 1].saturating_neg() >> shift_p)
                        .saturating_add(*fac >> shift_f);
                }
                // Region 4: Negated IMDCT only
                for i in region3_end..n_long {
                    dest[i] = src1[base - i - 1].saturating_neg() >> shift_p;
                }
                shift_olap
            }
        } else {
            // shiftp <= fac_q
            if shift_olap > shiftp {
                // Branch 3: shiftp <= fac_q && shift_olap > shiftp → return shiftp
                let shift_o = (shift_olap - shiftp) as u32;
                let shift_f = (fac_q - shiftp) as u32;

                // Region 1: Copy overlap with shift
                for i in 0..region1_end {
                    dest[i] = over_lap[i] >> shift_o;
                }
                // Region 2: Windowed IMDCT + FAC
                for (i, (fac, &win)) in (region1_end..region2_end).zip(fac_data_out.iter().zip(win_fwd.iter())) {
                    dest[i] = src1[base - i - 1].saturating_neg().mult_shift_q31(win).saturating_add(*fac >> shift_f);
                }
                // Region 3: Negated IMDCT + FAC
                let fac_offset = region2_end - region1_end;
                for (i, fac) in (region2_end..region3_end).zip(fac_data_out[fac_offset..].iter()) {
                    dest[i] = src1[base - i - 1].saturating_neg().saturating_add(*fac >> shift_f);
                }
                // Region 4: Negated IMDCT only
                for i in region3_end..n_long {
                    dest[i] = src1[base - i - 1].saturating_neg();
                }
                shiftp
            } else {
                // Branch 4: shiftp <= fac_q && shift_olap <= shiftp → return shift_olap
                let shift_p = (shiftp - shift_olap) as u32;
                let shift_f = (fac_q - shift_olap) as u32;

                // Region 1: Copy overlap directly
                dest[..region1_end].copy_from_slice(&over_lap[..region1_end]);
                // Region 2: Windowed IMDCT + FAC
                for (i, (fac, &win)) in (region1_end..region2_end).zip(fac_data_out.iter().zip(win_fwd.iter())) {
                    dest[i] = (src1[base - i - 1].saturating_neg().mult_shift_q31(win) >> shift_p)
                        .saturating_add(*fac >> shift_f);
                }
                // Region 3: Negated IMDCT + FAC
                let fac_offset = region2_end - region1_end;
                for (i, fac) in (region2_end..region3_end).zip(fac_data_out[fac_offset..].iter()) {
                    dest[i] = (src1[base - i - 1].saturating_neg() >> shift_p).saturating_add(*fac >> shift_f);
                }
                // Region 4: Negated IMDCT only
                for i in region3_end..n_long {
                    dest[i] = src1[base - i - 1].saturating_neg() >> shift_p;
                }
                shift_olap
            }
        }
    }
}

/// Standard overlap-add for long blocks with flat and transition regions.
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
///                                offset_lengths *offset,
///                                WORD8 shiftp, WORD8 shift_olap);
/// ```
pub fn windowing_long3(
    src1: &[i32],                        // Current IMDCT output
    win_fwd: &[i32],                     // Forward window coefficients
    over_lap: &[i32],                    // Overlap buffer from previous frame
    dest: &mut [i32],            // Output buffer
    win_rev: &[i32],                     // Reverse window coefficients
    offset: &OffsetLengths,  // Frame geometry
    shiftp: i8,                          // Q-format of current IMDCT
    shift_olap: i8,                      // Q-format of overlap buffer
) -> i8  // Returns Q-format of output buffer
{
    let n_long = offset.n_long;
    let n_flat_ls = offset.n_flat_ls;
    let n_trans_ls = offset.n_trans_ls;
    assert_eq!(src1.len(), n_long, "source buffer must have at least n_long samples");
    assert_eq!(dest.len(), n_long, "output buffer must have at least n_long samples");
    assert!(over_lap.len() >= n_flat_ls + n_trans_ls, "overlap buffer too small");
    assert!(win_fwd.len() >= n_trans_ls, "win_fwd too small");
    assert!(win_rev.len() >= n_trans_ls, "win_rev too small");

    #[cfg(feature = "fallback")]
    unsafe {
        let mut ixheaacd_drc_offset = offset.as_c_struct();
        let win_rev = win_rev.as_ptr().add(n_trans_ls - 1);
        return crate::gen_ixheaacd_ref::ixheaacd_windowing_long3(
            src1.as_ptr() as *mut i32,
            win_fwd.as_ptr(),
            over_lap.as_ptr() as *mut i32,
            dest.as_mut_ptr(),
            win_rev as *mut i32,
            &mut ixheaacd_drc_offset,
            shiftp,
            shift_olap,
        );
    }

    #[cfg(not(feature = "fallback"))]
    {
        // Region boundaries
        let half = n_long / 2;
        let region3_end = n_flat_ls + n_trans_ls;

        if shiftp > shift_olap {
            // Branch 1: shiftp > shift_olap → return shift_olap
            let shift = (shiftp - shift_olap) as u32;

            // Region 1: Copy overlap directly
            dest[..n_flat_ls].copy_from_slice(&over_lap[..n_flat_ls]);

            // Region 2: [n_flat_ls, n_long/2) - src1[i] * win_fwd + overlap[i] * win_rev
            for i in n_flat_ls..half {
                let win_idx = i - n_flat_ls;
                let win_rev_idx = n_trans_ls - 1 - win_idx;
                dest[i] = (src1[i].mult_shift_q31(win_fwd[win_idx]) >> shift)
                    .saturating_add(over_lap[i].mult_shift_q31(win_rev[win_rev_idx]));
            }

            // Region 3: [n_long/2, n_flat_ls + n_trans_ls) - -src1[n_long-i-1] * win_fwd + overlap[i] * win_rev
            for i in half..region3_end {
                let win_idx = i - n_flat_ls;
                let win_rev_idx = n_trans_ls - 1 - win_idx;
                dest[i] = (src1[n_long - i - 1].saturating_neg().mult_shift_q31(win_fwd[win_idx]) >> shift)
                    .saturating_add(over_lap[i].mult_shift_q31(win_rev[win_rev_idx]));
            }

            // Region 4: [n_flat_ls + n_trans_ls, n_long) - negated IMDCT only
            for i in region3_end..n_long {
                dest[i] = src1[n_long - i - 1].saturating_neg() >> shift;
            }

            shift_olap
        } else {
            // Branch 2: shiftp <= shift_olap → return shiftp
            let shift = (shift_olap - shiftp) as u32;

            // Region 1: Copy overlap with shift
            for i in 0..n_flat_ls {
                dest[i] = over_lap[i] >> shift;
            }

            // Region 2: [n_flat_ls, n_long/2) - src1[i] * win_fwd + (overlap[i] * win_rev >> shift)
            for i in n_flat_ls..half {
                let win_idx = i - n_flat_ls;
                let win_rev_idx = n_trans_ls - 1 - win_idx;
                dest[i] = src1[i].mult_shift_q31(win_fwd[win_idx])
                    .saturating_add(over_lap[i].mult_shift_q31(win_rev[win_rev_idx]) >> shift);
            }

            // Region 3: [n_long/2, n_flat_ls + n_trans_ls) - -src1[n_long-i-1] * win_fwd + (overlap[i] * win_rev >> shift)
            for i in half..region3_end {
                let win_idx = i - n_flat_ls;
                let win_rev_idx = n_trans_ls - 1 - win_idx;
                dest[i] = src1[n_long - i - 1].saturating_neg().mult_shift_q31(win_fwd[win_idx])
                    .saturating_add(over_lap[i].mult_shift_q31(win_rev[win_rev_idx]) >> shift);
            }

            // Region 4: [n_flat_ls + n_trans_ls, n_long) - negated IMDCT only
            for i in region3_end..n_long {
                dest[i] = src1[n_long - i - 1].saturating_neg();
            }

            shiftp
        }
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
///                                offset_lengths *offset,
///                                WORD8 shiftp, WORD8 shift_olap);
/// ```
pub fn windowing_short1(
    src1: &[i32],                        // Current IMDCT output (short block)
    src2: &[i32],                        // Window coefficients
    fp: &mut [i32],                      // In/Out overlap buffer (modified in-place)
    offset: &OffsetLengths,  // Frame geometry (lfac, n_flat_ls, n_short)
    shiftp: i8,                          // Q-format of current IMDCT
    shift_olap: i8,                      // Q-format of overlap buffer (fp)
)
{
    let lfac = offset.lfac;
    let n_short = offset.n_short;
    let n_flat_ls = offset.n_flat_ls;

    assert_eq!(src1.len(), n_short, "src1 must have n_short samples");
    assert_eq!(src2.len(), n_short, "src2 must have n_short samples");
    assert!(fp.len() >= n_flat_ls + lfac, "fp must have at least n_flat_ls + lfac samples");

    #[cfg(feature = "fallback")]
    unsafe {
        let mut ixheaacd_drc_offset = offset.as_c_struct();
        crate::gen_ixheaacd_ref::ixheaacd_windowing_short1(
            src1.as_ptr() as *mut i32,
            src2.as_ptr() as *mut i32,
            fp.as_mut_ptr(),
            &mut ixheaacd_drc_offset,
            shiftp,
            shift_olap,
        );
    }

    #[cfg(not(feature = "fallback"))]
    {
        if shift_olap > shiftp {
            let shift = (shift_olap - shiftp) as u32;
            assert!(shift < 32, "wrong Q-format parameters");

            // Scale down first lfac samples
            for i in 0..lfac {
                fp[i] >>= shift;
            }
            // If n_short > lfac, compute windowed IMDCT for [lfac..n_short)
            if n_short > lfac {
                for i in lfac..n_short {
                    fp[i] = src1[n_short - i - 1].saturating_neg().mult_shift_q31(src2[i]);
                }
            }
        } else {
            let shift = (shiftp - shift_olap) as u32;
            assert!(shift < 32, "wrong Q-format parameters");

            // If n_short > lfac, compute windowed IMDCT with shift for [lfac..n_short)
            if n_short > lfac {
                for i in lfac..n_short {
                    fp[i] = src1[n_short - i - 1].saturating_neg().mult_shift_q31(src2[i]) >> shift;
                }
            }
        }
        // Zero out [n_short..n_flat_ls + lfac) or [lfac..n_flat_ls + lfac) if n_short <= lfac
        let zero_start = if n_short > lfac { n_short } else { lfac };
        for i in zero_start..(n_flat_ls + lfac) {
            fp[i] = 0;
        }
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
///                                offset_lengths *offset,
///                                WORD8 shiftp, WORD8 shift_olap);
/// ```
pub fn windowing_short2(
    src1: &[i32],                        // Current IMDCT output (short block)
    win_fwd: &[i32],                     // Window coefficients (n_short samples)
    fp: &mut [i32],                      // In/Out overlap buffer
    offset: &OffsetLengths,  // Frame geometry
    shiftp: i8,                          // Q-format of current IMDCT
    shift_olap: i8,                      // Q-format of overlap buffer
)
{
    let n_short = offset.n_short;
    let n_flat_ls = offset.n_flat_ls;

    assert!(src1.len() >= n_short / 2, "src1 must have at least n_short/2 samples");
    assert!(win_fwd.len() >= n_short, "win_fwd must have at least n_short samples");
    assert!(fp.len() >= n_flat_ls + n_short, "fp must have at least n_flat_ls + n_short samples");

    #[cfg(feature = "fallback")]
    unsafe {
        let mut ixheaacd_drc_offset = offset.as_c_struct();
        crate::gen_ixheaacd_ref::ixheaacd_windowing_short2(
            src1.as_ptr() as *mut i32,
            win_fwd.as_ptr() as *mut i32,
            fp.as_mut_ptr(),
            &mut ixheaacd_drc_offset,
            shiftp,
            shift_olap,
        );
    }

    #[cfg(not(feature = "fallback"))]
    {
        let half = n_short / 2;

        if shift_olap > shiftp {
            let shift = (shift_olap - shiftp) as u32;
            assert!(shift < 32, "wrong Q-format parameters");
            
            for i in 0..half {
                let mirror = n_short - i - 1;
                let win_fwd_i = win_fwd[i];
                let win_rev_i = win_fwd[mirror];

                fp[i] = src1[i].mult_shift_q31(win_fwd_i)
                    .saturating_add(fp[i].mult_shift_q31(win_rev_i) >> shift);
                fp[mirror] = src1[i].saturating_neg().mult_shift_q31(win_rev_i)
                    .saturating_add(fp[mirror].mult_shift_q31(win_fwd_i) >> shift);
            }
        } else {
            let shift = (shiftp - shift_olap) as u32;
            assert!(shift < 32, "wrong Q-format parameters");

            for i in 0..half {
                let mirror = n_short - i - 1;
                let win_fwd_i = win_fwd[i];
                let win_rev_i = win_fwd[mirror];

                fp[i] = (src1[i].mult_shift_q31(win_fwd_i) >> shift)
                    .saturating_add(fp[i].mult_shift_q31(win_rev_i));
                fp[mirror] = (src1[i].saturating_neg().mult_shift_q31(win_rev_i) >> shift)
                    .saturating_add(fp[mirror].mult_shift_q31(win_fwd_i));
            }
        }
        // Zero out [n_short..n_flat_ls + n_short)
        for i in n_short..(n_flat_ls + n_short) {
            fp[i] = 0;
        }
    }
}

/// Final windowing stage for short blocks in EIGHT_SHORT_SEQUENCE mode.
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
) -> i8  // Returns Q-format of output buffer (min of shiftp, shift_olap)
{
    let n_short = src1.len();
    assert_eq!(n_short, win_fwd.len(), "forward window must have same length");
    assert_eq!(n_short, fp.len(), "overlap buffer must have same length");

    #[cfg(feature = "fallback")]
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

    #[cfg(not(feature = "fallback"))]
    {
        let half = n_short / 2;

        if shift_olap > shiftp {
            let shift = (shift_olap - shiftp) as u32;
            assert!(shift < 32, "wrong Q-format parameters");

            for i in 0..half {
                let mirror = n_short - i - 1;
                let neg_src = src1[half - i - 1].saturating_neg();

                fp[i] = neg_src.mult_shift_q31(win_fwd[n_short - 1 - i])
                    .saturating_add(fp[i] >> shift);
                fp[mirror] = neg_src.mult_shift_q31(win_fwd[i])
                    .saturating_add(fp[mirror] >> shift);
            }
            shiftp
        } else {
            let shift = (shiftp - shift_olap) as u32;
            assert!(shift < 32, "wrong Q-format parameters");

            for i in 0..half {
                let mirror = n_short - i - 1;
                let neg_src = src1[half - i - 1].saturating_neg();

                fp[i] = (neg_src.mult_shift_q31(win_fwd[n_short - 1 - i]) >> shift)
                    .saturating_add(fp[i]);
                fp[mirror] = (neg_src.mult_shift_q31(win_fwd[i]) >> shift)
                    .saturating_add(fp[mirror]);
            }
            shift_olap
        }
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
) -> i8  // Returns Q-format of output buffer
{
    let n_short = src1.len();
    assert_eq!(n_short, win_fwd.len(), "forward window must have same length");
    assert_eq!(n_short, win_rev1.len(), "backward window must have same length");
    assert_eq!(n_short * 2, fp.len(), "overlap buffer must have double length");
    assert!(shiftp >= output_q, "output_q must be min(shiftp, shift_olap)");

    #[cfg(feature = "fallback")]
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

    #[cfg(not(feature = "fallback"))]
    {
        let half = n_short / 2;

        if shift_olap > output_q {
            let shift_p = (shiftp - output_q) as u32;
            let shift_o = (shift_olap - output_q) as u32;
            assert!(shift_o < 32, "wrong input/output Q-format");

            // Stage 1: First half windowing [0..half)
            for i in 0..half {
                let src = src1[half + i];
                let mirror = n_short - i - 1;

                fp[i] = (src.mult_shift_q31(win_fwd[i]) >> shift_p)
                        .saturating_add(fp[i]);
                fp[mirror] = (src.saturating_neg().mult_shift_q31(win_fwd[mirror]) >> shift_p)
                        .saturating_add(fp[mirror]);
            }

            // Stage 2: Second half processing [half..n_short)
            if windowed_flag {
                for j in 0..half {
                    let fp_idx1 = n_short + j;
                    let fp_idx2 = 2 * n_short - 1 - j;
                    let neg_src = src1[half - 1 - j].saturating_neg();

                    fp[fp_idx1] = (neg_src.mult_shift_q31(win_rev1[n_short - 1 - j]) >> shift_p)
                            .saturating_add(fp[fp_idx1] >> shift_o);
                    fp[fp_idx2] = (neg_src.mult_shift_q31(win_rev1[j]) >> shift_p)
                            .saturating_add(fp[fp_idx2] >> shift_o);
                }
            } else {
                for j in 0..half {
                    let fp_idx1 = n_short + j;
                    let fp_idx2 = 2 * n_short - 1 - j;
                    let neg_src = src1[half - 1 - j].saturating_neg();

                    fp[fp_idx1] = (neg_src >> shift_p).saturating_add(fp[fp_idx1] >> shift_o);
                    fp[fp_idx2] = (neg_src >> shift_p).saturating_add(fp[fp_idx2] >> shift_o);
                }
            }
            output_q
        } else { // shift_olap <= output_q
            let shift_p = (shiftp - shift_olap) as u32;
            let shift_o = (output_q - shift_olap) as u32;
            assert!(shift_o < 32, "wrong input/output Q-format");

            // Stage 1: First half windowing [0..half)
            for i in 0..half {
                let src= src1[half + i];
                let mirror = n_short - i - 1;

                fp[i] = (src.mult_shift_q31(win_fwd[i]) >> shift_p)
                        .saturating_add(fp[i] >> shift_o);
                fp[mirror] = (src.saturating_neg().mult_shift_q31(win_fwd[n_short - 1 - i]) >> shift_p)
                        .saturating_add(fp[mirror]);
            }

            // Stage 2: Second half processing [half..n_short)
            if windowed_flag {
                for j in 0..half {
                    let fp_idx1 = n_short + j;
                    let fp_idx2 = 2 * n_short - 1 - j;
                    let neg_src = src1[half - 1 - j].saturating_neg();

                    fp[fp_idx1] = (neg_src.mult_shift_q31(win_rev1[n_short - 1 - j]) >> shift_p)
                            .saturating_add(fp[fp_idx1]);
                    fp[fp_idx2] = (neg_src.mult_shift_q31(win_rev1[j]) >> shift_p)
                            .saturating_add(fp[fp_idx2]);
                }
            } else {
                for j in 0..half {
                    let fp_idx1 = n_short + j;
                    let fp_idx2 = 2 * n_short - 1 - j;
                    let neg_src = src1[half - 1 - j].saturating_neg();

                    fp[fp_idx1] = fp[fp_idx1].saturating_add(neg_src >> shift_p);
                    fp[fp_idx2] = fp[fp_idx2].saturating_add(neg_src >> shift_p);
                }
            }
            shift_olap
        }
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
            *d = s.saturating_shl(shift);
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
            *d = s.saturating_shl(shift).saturating_add(ADJ_SCALE);
        }
    }
}

/// Multiplies each element of a vector by a constant (FLOAT32 version)
///
/// # C signature
/// ```c
/// VOID ixheaacd_vec_cnst_mul(FLOAT32 a, FLOAT32 x[], FLOAT32 z[], WORD32 n);
/// ```
pub fn vec_cnst_mul(a: f32, x: &[f32], z: &mut [f32]) {
    assert_eq!(x.len(), z.len(), "x and z must have same length");
    for (zi, &xi) in z.iter_mut().zip(x.iter()) {
        *zi = a * xi;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create simple sine-like window coefficients
    fn create_test_window_len32() -> Vec<i32> {
        vec![
              13176960,  118530360,  223600288,  328129056,  431869696,  534568800,  635979456,  735858880, 
             833964544,  930062272, 1023918080, 1115308543, 1204012415, 1289815167, 1372508287, 1451898623,
            1527788543, 1599999871, 1668354303, 1732692863, 1792854655, 1848697855, 1900087039, 1946900095,
            1989020799, 2026350591, 2058798975, 2086288895, 2108751615, 2126133375, 2138393343, 2145503615]
    }

    // ============================================================================
    // combine_fac() tests
    // ============================================================================
    // Tests cover all code paths in ixheaacd_combine_fac():
    // - Branch 1: fac_q > output_q (right-shift src2 before adding)
    // - Branch 2: fac_q <= output_q (left-shift src2 with saturation before adding)
    // ============================================================================

    #[test]
    fn test_combine_fac_branch1_fac_gt_output() {
        // Branch 1: fac_q > output_q - right-shift src2 before adding
        let src1 = vec![-1000000, i32::MIN, i32::MIN/2, i32::MAX, i32::MAX /2, 100000, 200000, 300000, -400000, 500000, 600000, -700000, 800000];
        let src2 = vec![-1000000, i32::MIN/2, i32::MIN, i32::MAX/2, i32::MAX, 80000, 160000, 240000, 320000, 400000, -480000, 560000, -640000];
        let mut dest = vec![0; src1.len()];

        combine_fac(&src1, &src2, &mut dest, 12, 15); // fac_q=15 > output_q=12

        let exp = vec![-1125000, -2147483648, -1342177280, 2147483647, 1342177278, 110000, 220000, 330000, -360000, 550000, 540000, -630000, 720000];
        assert_eq!(dest, exp);
    }

    #[test]
    fn test_combine_fac_branch2_fac_le_output() {
        // Branch 2: fac_q <= output_q - left-shift src2 with saturation
        let src1 = vec![-1000000, i32::MIN, i32::MIN/2, i32::MAX/2, i32::MAX, 100000, 200000, 300000, 400000, 500000, -600000, 700000, -800000];
        let src2 = vec![-10000000, i32::MIN, i32::MIN/2, i32::MAX/2, i32::MAX, 10000, 20000, 30000, 40000, 50000, 60000, -70000, -80000];
        let mut dest = vec![0; src1.len()];

        combine_fac(&src1, &src2, &mut dest, 14, 12); // fac_q=12 <= output_q=15

        let exp = vec![-41000000, -2147483648, -2147483648, 2147483647, 2147483647, 140000, 280000, 420000, 560000, 700000, -360000, 420000, -1120000];
        assert_eq!(dest, exp);
    }

    #[test]
    fn test_combine_fac_equal_q() {
        // Edge case: fac_q == output_q (no shift)
        let src1 = vec![i32::MIN, i32::MIN/2, i32::MAX/2, i32::MAX, 100000, 200000, -300000, -400000, 500000, 600000, 700000, 800000];
        let src2 = vec![i32::MIN, i32::MIN/2, i32::MAX/2, i32::MAX, 10000, 20000, 30000, -40000, -50000, 60000, 70000, 80000];
        let mut dest = vec![0; src1.len()];

        combine_fac(&src1, &src2, &mut dest, 14, 14); // fac_q == output_q

        let exp = vec![-2147483648, -2147483648, 2147483646, 2147483647, 110000, 220000, -270000, -440000, 450000, 660000, 770000, 880000];
        assert_eq!(dest, exp);
    }

    // ============================================================================
    // windowing_long1() tests
    // ============================================================================
    // Tests cover all code paths in ixheaacd_windowing_long1():
    // - Branch 1: shift1 > shift2 (scale down src1, keep src2 precision)
    // - Branch 2: shift1 <= shift2 (keep src1 precision, scale down src2)
    // ============================================================================

    #[test]
    fn test_windowing_long1_branch1_shift1_gt_shift2() {
        // Branch 1: shift1 > shift2 - scale down src1, keep src2 precision
        let src1 = vec![
            -42000000, 1234567890, i32::MIN / 2, 999, -1000, 42, 876543210, -100000, i32::MAX, -500, 250, -1, 5000, -234567890, 100000000, 75000,
            i32::MIN, 200000, -1234567, 0, 500000000, -100, i32::MAX / 2, 1500000000, -50000, 9876543, -999999999, -1500000000, 42000000, -100000000, -500000000, -42,
        ];
        let src2 = vec![
            i32::MIN, 200000, -1234567, 0, 500000000, -100, i32::MAX / 2, 1500000000, -50000, 9876543, -999999999, -1500000000, 42000000, -100000000, -500000000, -42,
            -42000000, 1234567890, i32::MIN / 2, 999, -1000, 42, 876543210, -100000, i32::MAX, -500, 250, -1, 5000, -234567890, 100000000, 75000,
        ];
        let win_fwd = create_test_window_len32();
        let mut dest = vec![0; win_fwd.len()];

        let result_q = windowing_long1(&src1, &src2, &win_fwd, &win_fwd, 
                    &mut dest, 16, 14);

        let exp = vec![-2145568044, 17234646, -29172329, 38, 485751935, -94, 1078072583, 1389306411, 208445805, 8738680, -860866994, -1252294511, 33888315, -112910168, -356550901, 12646, -28409246, 770415488, -599349339, -449, -520, -34, 379625710, -486763859, 735882034, -206775164, 51, 241, 517, 241342993, -301815991, 10490778];
        assert_eq!(dest, exp);
        assert_eq!(result_q, 14);
    }

    #[test]
    fn test_windowing_long1_branch2_shift1_le_shift2() {
        // Branch 2: shift1 <= shift2 - keep src1 precision, scale down src2
        let src1 = vec![
            -42000000, 1234567890, i32::MIN / 2, 999, -1000, 42, 876543210, -100000, i32::MAX, -500, 250, -1, 5000, -234567890, 100000000, 75000,
            i32::MIN, 200000, -1234567, 0, 500000000, -100, i32::MAX / 2, 1500000000, -50000, 9876543, -999999999, -1500000000, 42000000, -100000000, -500000000, -42,
        ];
        let src2 = vec![
            i32::MIN, 200000, -1234567, 0, 500000000, -100, i32::MAX / 2, 1500000000, -50000, 9876543, -999999999, -1500000000, 42000000, -100000000, -500000000, -42,
            -42000000, 1234567890, i32::MIN / 2, 999, -1000, 42, 876543210, -100000, i32::MAX, -500, 250, -1, 5000, -234567890, 100000000, 75000,
        ];
        let win_fwd = create_test_window_len32();
        let mut dest = vec![0; win_fwd.len()];

        let result_q = windowing_long1(&src1, &src2, &win_fwd, &win_fwd, 
                    &mut dest, 14, 16);

        let exp = vec![-536633617, 68191762, -112105718, 152, 121437794, -14, 512882981, 347294477, 833953210, 2184466, -215216637, -313073629, 8474706, -160307662, -29219853, 50698, -7152335, 122754684, 21006085, -3895, -130, -211, 94906842, -1946909804, 184057339, -827100097, -26, 970, -791, 1056960767, -1227962086, 41961389];
        assert_eq!(dest, exp);
        assert_eq!(result_q, 14);
    }

    #[test]
    fn test_windowing_long1_equal_shifts() {
        // Edge case: shift1 == shift2
        let src1 = vec![
            -42000000, 1234567890, i32::MIN / 2, 999, -1000, 42, 876543210, -100000, i32::MAX, -500, 250, -1, 5000, -234567890, 100000000, 75000,
            i32::MIN, 200000, -1234567, 0, 500000000, -100, i32::MAX / 2, 1500000000, -50000, 9876543, -999999999, -1500000000, 42000000, -100000000, -500000000, -42,
        ];
        let src2 = vec![
            i32::MIN, 200000, -1234567, 0, 500000000, -100, i32::MAX / 2, 1500000000, -50000, 9876543, -999999999, -1500000000, 42000000, -100000000, -500000000, -42,
            -42000000, 1234567890, i32::MIN / 2, 999, -1000, 42, 876543210, -100000, i32::MAX, -500, 250, -1, 5000, -234567890, 100000000, 75000,
        ];
        let win_fwd = create_test_window_len32();
        let mut dest = vec![0; win_fwd.len()];

        let result_q = windowing_long1(&src1, &src2, &win_fwd, &win_fwd, 
                    &mut dest, 14, 14);

        let exp = vec![-2145761328, 68341127, -113022437, 152, 485751784, -86, 1272764452, 1389280711, 833919213, 8738518, -860866904, -1252294511, 33890418, -218574263, -308616603, 50676, -28449264, 714536138, -462674603, -3475, -520, -196, 379626042, -1946938930, 735951499, -827100208, 21, 970, -218, 1038643009, -1223822461, 41961734];
        assert_eq!(dest, exp);
        assert_eq!(result_q, 14);
    }

    #[test]
    fn test_windowing_long1_large_shift_difference() {
        // Test with large shift difference
        let src1 = vec![
            -42000000, 1234567890, i32::MIN / 2, 999, -1000, 42, 876543210, -100000, i32::MAX, -500, 250, -1, 5000, -234567890, 100000000, 75000,
            i32::MIN, 200000, -1234567, 0, 500000000, -100, i32::MAX / 2, 1500000000, -50000, 9876543, -999999999, -1500000000, 42000000, -100000000, -500000000, -42,
        ];
        let src2 = vec![
            i32::MIN, 200000, -1234567, 0, 500000000, -100, i32::MAX / 2, 1500000000, -50000, 9876543, -999999999, -1500000000, 42000000, -100000000, -500000000, -42,
            -42000000, 1234567890, i32::MIN / 2, 999, -1000, 42, 876543210, -100000, i32::MAX, -500, 250, -1, 5000, -234567890, 100000000, 75000,
        ];
        let win_fwd = create_test_window_len32();
        let mut dest = vec![0; win_fwd.len()];

        let result_q = windowing_long1(&src1, &src2, &win_fwd, &win_fwd, 
                    &mut dest, 18, 12);

        let exp = vec![-2145507642, 1263871, -2969171, 2, 485751982, -96, 1017231374, 1389314442, 12985365, 8738731, -860867022, -1252294511, 33887658, -79890138, -371530369, 762, -28396740, 787877784, -642060194, 496, -520, 16, 379625606, -30459149, 735860326, -12923588, 61, 14, 747, -7813262, -13688969, 656104];
        assert_eq!(dest, exp);
        assert_eq!(result_q, 12);
    }

    // ============================================================================
    // windowing_long2() tests
    // ============================================================================
    // Tests cover all 4 code paths in ixheaacd_windowing_long2():
    // - Branch 1: shiftp > fac_q && shift_olap > fac_q (return fac_q)
    // - Branch 2: shiftp > fac_q && shift_olap <= fac_q (return shift_olap)
    // - Branch 3: shiftp <= fac_q && shift_olap > shiftp (return shiftp)
    // - Branch 4: shiftp <= fac_q && shift_olap <= shiftp (return shift_olap)
    // ============================================================================

    // Helper to create offset_lengths for windowing_long2 tests with reduced sizes
    fn create_test_offset_long2() -> OffsetLengths {
        // Proportionally reduced: n_long=64, lfac=8, n_flat_ls=16, n_trans_ls=16
        OffsetLengths {
            lfac: 8, 
            n_flat_ls: 24,
            n_trans_ls: 16,
            n_long: 64,
            n_short: 8,
        }
    }

    fn create_test_src1_long2() -> Vec<i32> {
        vec![
            -42000000, 1234567890, i32::MIN / 2, 999, -1000, 42, 876543210, -100000, i32::MAX, -500, 250, -1, 5000, -234567890, 100000000, 75000,
            i32::MIN, 200000, -1234567, 0, 500000000, -100, i32::MAX / 2, 1500000000, -50000, 9876543, -999999999, -1500000000, 42000000, -100000000, -500000000, -42,
            -42000000, 1234567890, i32::MIN / 2, 999, -1000, 42, 876543210, -100000, i32::MAX, -500, 250, -1, 5000, -234567890, 100000000, 75000,
            i32::MIN, 200000, -1234567, 0, 500000000, -100, i32::MAX / 2, 1500000000, -50000, 9876543, -999999999, -1500000000, 42000000, -100000000, -500000000, -42,
        ]
    }

    fn create_test_overlap_long2() -> Vec<i32> {
        vec![
            i32::MIN, 200000, -1234567, 0, 500000000, -100, i32::MAX / 2, 1500000000, -50000, 9876543, -999999999, -1500000000, 42000000, -100000000, -500000000, -42,
            -42000000, 1234567890, i32::MIN / 2, 999, -1000, 42, 876543210, -100000, i32::MAX, -500, 250, -1, 5000, -234567890, 100000000, 75000,
       ]
    }

    fn create_test_overlap_long3() -> Vec<i32> {
        vec![
            -1500000000, i32::MAX, -234567890, 999, 500000000, i32::MIN / 2, -100, 75000, i32::MIN, 200000, -1234567, 0, 500000000, -100, i32::MAX / 2, 1500000000, -50000,
            9876543, -999999999, -1500000000, 42000000, -100000000, -500000000, -42, i32::MAX / 2, -999999999, 42000000, i32::MIN, 1234567890, -50000, 250, 876543210,
            -42000000, 1234567890, i32::MIN / 2, 999, -1000, 42, 876543210, -100000, i32::MAX, -500, 250, -1, 5000, -234567890, 100000000, 75000,
       ]
    }

    fn create_test_fac_data() -> Vec<i32> {
        vec![
            i32::MAX / 2, -999999999, 42000000, i32::MIN, 1234567890, -50000, 250, 876543210,
            -1500000000, i32::MAX, -234567890, 999, 500000000, i32::MIN / 2, -100, 75000,
        ]
    }

    #[test]
    fn test_windowing_long2_branch1_shiftp_gt_fac_olap_gt_fac() {
        // Branch 1: shiftp > fac_q && shift_olap > fac_q (return fac_q)
        let offset = create_test_offset_long2();
        let src1 = create_test_src1_long2();
        let win_fwd = create_test_window_len32();
        let fac_data = create_test_fac_data();
        let over_lap = create_test_overlap_long2();
        let mut p_out_buffer = vec![0; offset.n_long];

        let result_q = windowing_long2(
            &src1, &win_fwd, &fac_data, &over_lap, &mut p_out_buffer,
            &offset, 16, 14, 12  // shiftp=16 > fac_q=12, shift_olap=14 > fac_q=12
        );

        let exp = vec![-536870912, 50000, -308642, 0, 125000000, -25, 268435455, 375000000, -12500, 2469135, -250000000, -375000000, 10500000, -25000000, -125000000, -11, -10500000, 308641972, -268435456, 249, -250, 10, 219135802, -25000, 536870911, -125, 62, -1, 1250, -58641973, 25000000, 18750, 1073741824, -986465812, 44979993, -2147483648, 1287129949, 37488561, -394271, 876545322, -1593750000, 2080374783, -234567884, -31249001, 500000000, -1073664664, -12600, 134292727, -4688, -6250000, 14660493, -313, 0, -16, 31, -134217728, 6250, -54783951, -3, 62, -63, 67108864, -77160494, 2625000];
        assert_eq!(p_out_buffer, exp);
        assert_eq!(result_q, 12);
    }

    #[test]
    fn test_windowing_long2_branch2_shiftp_gt_fac_olap_le_fac() {
        // Branch 2: shiftp > fac_q && shift_olap <= fac_q (return shift_olap)
        let offset = create_test_offset_long2();
        let src1 = create_test_src1_long2();
        let win_fwd = create_test_window_len32();
        let fac_data = create_test_fac_data();
        let over_lap = create_test_overlap_long2();
        let mut p_out_buffer = vec![0; offset.n_long];

        let result_q = windowing_long2(
            &src1, &win_fwd, &fac_data, &over_lap, &mut p_out_buffer,
            &offset, 16, 10, 12  // shiftp=16 > fac_q=12, shift_olap=10 <= fac_q=12
        );

        let exp = vec![-2147483648, 200000, -1234567, 0, 500000000, -100, 1073741823, 1500000000, -50000, 9876543, -999999999, -1500000000, 42000000, -100000000, -500000000, -42, -42000000, 1234567890, -1073741824, 999, -1000, 42, 876543210, -100000, 2147483647, -500, 250, -1, 5000, -234567890, 100000000, 75000, 268435455, -246616454, 11244998, -537211740, 321782486, 9372140, -98569, 219136330, -398437500, 520093695, -58641972, -7812251, 125000000, -268416166, -3150, 33573181, -1172, -1562500, 3665123, -79, 0, -4, 7, -33554432, 1562, -13695988, -1, 15, -16, 16777216, -19290124, 656250];
        assert_eq!(p_out_buffer, exp);
        assert_eq!(result_q, 10);
    }

    #[test]
    fn test_windowing_long2_branch3_shiftp_le_fac_olap_gt_shiftp() {
        // Branch 3: shiftp <= fac_q && shift_olap > shiftp (return shiftp)
        let offset = create_test_offset_long2();
        let src1 = create_test_src1_long2();
        let win_fwd = create_test_window_len32();
        let fac_data = create_test_fac_data();
        let over_lap = create_test_overlap_long2();
        let mut p_out_buffer = vec![0; offset.n_long];

        let result_q = windowing_long2(
            &src1, &win_fwd, &fac_data, &over_lap, &mut p_out_buffer,
            &offset, 12, 15, 14  // shiftp=12 <= fac_q=14, shift_olap=15 > shiftp=12
        );

        let exp = vec![-268435456, 25000, -154321, 0, 62500000, -13, 134217727, 187500000, -6250, 1234567, -125000000, -187500000, 5250000, -12500000, -62500000, -6, -5250000, 154320986, -134217728, 124, -125, 5, 109567901, -12500, 268435455, -63, 31, -1, 625, -29320987, 12500000, 9375, 268435471, -33453003, 58179901, -558683865, 1149634928, 600604490, -6312274, 219169606, -1875000000, -536870912, -58641873, -499999751, 125000000, -267200889, -200025, 2147483647, -75000, -100000000, 234567890, -5000, 1, -250, 500, -2147483647, 100000, -876543210, -42, 1000, -999, 1073741824, -1234567890, 42000000];
        assert_eq!(p_out_buffer, exp);
        assert_eq!(result_q, 12);
    }

    #[test]
    fn test_windowing_long2_branch4_shiftp_le_fac_olap_le_shiftp() {
        // Branch 4: shiftp <= fac_q && shift_olap <= shiftp (return shift_olap)
        let offset = create_test_offset_long2();
        let src1 = create_test_src1_long2();
        let win_fwd = create_test_window_len32();
        let fac_data = create_test_fac_data();
        let over_lap = create_test_overlap_long2();
        let mut p_out_buffer = vec![0; offset.n_long];

        let result_q = windowing_long2(
            &src1, &win_fwd, &fac_data, &over_lap, &mut p_out_buffer,
            &offset, 12, 10, 14  // shiftp=12 <= fac_q=14, shift_olap=10 <= shiftp=12
        );

        let exp = vec![-2147483648, 200000, -1234567, 0, 500000000, -100, 1073741823, 1500000000, -50000, 9876543, -999999999, -1500000000, 42000000, -100000000, -500000000, -42, -42000000, 1234567890, -1073741824, 999, -1000, 42, 876543210, -100000, 2147483647, -500, 250, -1, 5000, -234567890, 100000000, 75000, 67108867, -8363251, 14544975, -139670967, 287408732, 150151122, -1578069, 54792401, -468750000, -134217729, -14660469, -124999938, 31250000, -66800223, -50007, 536875598, -18750, -25000000, 58641972, -1250, 0, -63, 125, -536870912, 25000, -219135803, -11, 250, -250, 268435456, -308641973, 10500000];
        assert_eq!(p_out_buffer, exp);
        assert_eq!(result_q, 10);
    }

    #[test]
    fn test_windowing_long2_all_equal_q() {
        // Edge case: all Q-formats equal
        let offset = create_test_offset_long2();
        let src1 = create_test_src1_long2();
        let win_fwd = create_test_window_len32();
        let fac_data = create_test_fac_data();
        let over_lap = create_test_overlap_long2();
        let mut p_out_buffer = vec![0; offset.n_long];

        let result_q = windowing_long2(
            &src1, &win_fwd, &fac_data, &over_lap, &mut p_out_buffer,
            &offset, 14, 14, 14  // All equal
        );

        let exp = vec![-2147483648, 200000, -1234567, 0, 500000000, -100, 1073741823, 1500000000, -50000, 9876543, -999999999, -1500000000, 42000000, -100000000, -500000000, -42, -42000000, 1234567890, -1073741824, 999, -1000, 42, 876543210, -100000, 2147483647, -500, 250, -1, 5000, -234567890, 100000000, 75000, 1073741839, -783453002, 89679901, -2147483648, 2075560846, 600566990, -6312086, 876577014, -2147483648, 1073741824, -234567790, -499999001, 500000000, -1072507257, -200100, 2147483647, -75000, -100000000, 234567890, -5000, 1, -250, 500, -2147483647, 100000, -876543210, -42, 1000, -999, 1073741824, -1234567890, 42000000];
        assert_eq!(p_out_buffer, exp);
        assert_eq!(result_q, 14);
    }

    // ============================================================================
    // windowing_long3() tests
    // ============================================================================
    // Tests cover all code paths in ixheaacd_windowing_long3():
    // - Branch 1: shiftp > shift_olap (memcpy flat, scale down src1) - return shift_olap
    // - Branch 2: shiftp <= shift_olap (scale down overlap) - return shiftp
    // ============================================================================

    #[test]
    fn test_windowing_long3_branch1_shiftp_gt_olap() {
        // Branch 1: shiftp > shift_olap (memcpy flat, scale down src1) - return shift_olap
        let offset = create_test_offset_long2();
        let src1 = create_test_src1_long2();
        let win_fwd = create_test_window_len32();
        let over_lap = create_test_overlap_long3();
        let mut p_out_buffer = vec![0; offset.n_long];

        let result_q = windowing_long3(
            &src1, &win_fwd, &over_lap, &mut p_out_buffer, &win_fwd,
            &offset, 16, 14  // shiftp=16 > shift_olap=14
        );

        let exp = vec![-1500000000, 2147483647, -234567890, 999, 500000000, -1073741824, -100, 75000, -2147483648, 200000, -1234567, 0, 500000000, -100, 1073741823, 1500000000, -50000, 9876543, -999999999, -1500000000, 42000000, -100000000, -500000000, -42, 725949233, -638987693, -804587, -1261311290, 643291857, -6247040, -37018769, 340401171, -14391758, 419755279, -255364425, -5453039, 210248086, 150154251, 46802718, 7837, -375000000, -268435456, 25, -125000000, 0, 308641, -50000, 536870911, -18750, -25000000, 58641972, -1250, 0, -63, 125, -536870912, 25000, -219135803, -11, 250, -250, 268435456, -308641973, 10500000];
        assert_eq!(p_out_buffer, exp);
        assert_eq!(result_q, 14);
    }

    #[test]
    fn test_windowing_long3_branch2_shiftp_le_olap() {
        // Branch 2: shiftp <= shift_olap (scale down overlap) - return shiftp
        let offset = create_test_offset_long2();
        let src1 = create_test_src1_long2();
        let win_fwd = create_test_window_len32();
        let over_lap = create_test_overlap_long3();
        let mut p_out_buffer = vec![0; offset.n_long];

        let result_q = windowing_long3(
            &src1, &win_fwd, &over_lap, &mut p_out_buffer, &win_fwd,
            &offset, 14, 16  // shiftp=14 <= shift_olap=16
        );

        let exp = vec![-375000000, 536870911, -58641973, 249, 125000000, -268435456, -25, 18750, -536870912, 50000, -308642, 0, 125000000, -25, 268435455, 375000000, -12500, 2469135, -250000000, -375000000, 10500000, -25000000, -125000000, -11, 181487020, -159235859, -97815520, -530198602, 168741472, -24898760, -148075480, 85100278, -3597925, 307951629, -19141199, -21812903, 840992917, 600616991, 5782864, 33650, -1500000000, -1073741823, 100, -500000000, 0, 1234567, -200000, 2147483647, -75000, -100000000, 234567890, -5000, 1, -250, 500, -2147483647, 100000, -876543210, -42, 1000, -999, 1073741824, -1234567890, 42000000];
        assert_eq!(p_out_buffer, exp);
        assert_eq!(result_q, 14);
    }

    #[test]
    fn test_windowing_long3_equal_shifts() {
        // Edge case: shiftp == shift_olap
        let offset = create_test_offset_long2();
        let src1 = create_test_src1_long2();
        let win_fwd = create_test_window_len32();
        let over_lap = create_test_overlap_long3();
        let mut p_out_buffer = vec![0; offset.n_long];

        let result_q = windowing_long3(
            &src1, &win_fwd, &over_lap, &mut p_out_buffer, &win_fwd,
            &offset, 14, 14  // Equal shifts
        );

        let exp = vec![-1500000000, 2147483647, -234567890, 999, 500000000, -1073741824, -100, 75000, -2147483648, 200000, -1234567, 0, 500000000, -100, 1073741823, 1500000000, -50000, 9876543, -999999999, -1500000000, 42000000, -100000000, -500000000, -42, 725949003, -638578841, -78896085, -1433207913, 649626664, -24916640, -148075399, 340401160, -14391746, 582165527, -219604499, -21812753, 840992803, 600616994, 42068466, 33190, -1500000000, -1073741823, 100, -500000000, 0, 1234567, -200000, 2147483647, -75000, -100000000, 234567890, -5000, 1, -250, 500, -2147483647, 100000, -876543210, -42, 1000, -999, 1073741824, -1234567890, 42000000];
        assert_eq!(p_out_buffer, exp);
        assert_eq!(result_q, 14);
    }

    #[test]
    fn test_windowing_long3_large_shift_difference() {
        // Test with large shift difference
        let offset = create_test_offset_long2();
        let src1 = create_test_src1_long2();
        let win_fwd = create_test_window_len32();
        let over_lap = create_test_overlap_long3();
        let mut p_out_buffer = vec![0; offset.n_long];

        let result_q = windowing_long3(
            &src1, &win_fwd, &over_lap, &mut p_out_buffer, &win_fwd,
            &offset, 18, 12  // Large difference: shiftp - shift_olap = 6
        );

        let exp = vec![-1500000000, 2147483647, -234567890, 999, 500000000, -1073741824, -100, 75000, -2147483648, 200000, -1234567, 0, 500000000, -100, 1073741823, 1500000000, -50000, 9876543, -999999999, -1500000000, 42000000, -100000000, -500000000, -42, 725949305, -639115459, 23599006, -1207593595, 641312230, -412790, -2313572, 340401174, -14391762, 369002076, -266539402, -340628, 13140361, 9384644, 48282171, -86, -23437500, -16777216, 1, -7812500, 0, 19290, -3125, 33554431, -1172, -1562500, 3665123, -79, 0, -4, 7, -33554432, 1562, -13695988, -1, 15, -16, 16777216, -19290124, 656250];
        assert_eq!(p_out_buffer, exp);
        assert_eq!(result_q, 12);
    }

    // ============================================================================
    // windowing_short1() tests
    // ============================================================================
    // Tests cover all code paths in ixheaacd_windowing_short1():
    // - Branch 1: shift_olap > shiftp && n_short > lfac
    // - Branch 2: shift_olap > shiftp && n_short <= lfac
    // - Branch 3: shift_olap <= shiftp && n_short > lfac
    // - Branch 4: shift_olap <= shiftp && n_short <= lfac
    // ============================================================================

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
        let mut fp = vec![-206367421; offset.n_flat_ls + offset.lfac];

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
        let mut fp = vec![8000; offset.n_flat_ls + offset.lfac];

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
        let mut fp = vec![3000; offset.n_flat_ls + offset.lfac];

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
        let mut fp = vec![68186432; offset.n_flat_ls + offset.lfac];

        windowing_short1(&src1, &src2, &mut fp, &offset, 18, 14);

        let exp = vec![68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 68186432, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(fp, exp);
    }

    // ============================================================================
    // windowing_short1() edge case tests - saturation and i32::MIN handling
    // ============================================================================

    #[test]
    fn test_windowing_short1_i32min_saturation() {
        // Test negate_sat behavior: i32::MIN should become i32::MAX
        let offset = OffsetLengths {
            lfac: 8,
            n_flat_ls: 24,
            n_trans_ls: 16,
            n_long: 64,
            n_short: 16,
        };
        // Place i32::MIN in positions that will be negated
        let src1 = vec![
            1000, 2000, 3000, 4000, 5000, 6000, 7000, i32::MIN,
            i32::MIN, 9000, 10000, 11000, 12000, 13000, 14000, 15000
        ];
        let src2 = vec![
            13176960, 118530360, 223600288, 328129056, 431869696, 534568800, 635979456, 735858880,
            833964544, 930062272, 1023918080, 1115308543, 1204012415, 1289815167, 1372508287, 1451898623
        ];
        let mut fp = vec![100; offset.n_flat_ls + offset.lfac];

        windowing_short1(&src1, &src2, &mut fp, &offset, 14, 16);

        // First 8 samples (lfac) should be scaled down from 100 >> 2 = 25
        for i in 0..8 {
            assert_eq!(fp[i], 25, "fp[{}] should be 25", i);
        }
        // Samples 8..16 have windowed IMDCT with negate_sat
        // fp[8] uses src1[16-8-1=7] which is i32::MIN, so negate_sat gives i32::MAX
        // mult_shift_q31(i32::MAX, src2[8]) should give a large value
        assert!(fp[8] > 0, "fp[8] with negated i32::MIN should be positive");
    }

    #[test]
    fn test_windowing_short1_max_values_branch3() {
        // Branch 3: shift_olap <= shiftp with max values
        let offset = OffsetLengths {
            lfac: 8,
            n_flat_ls: 24,
            n_trans_ls: 16,
            n_long: 64,
            n_short: 16,
        };
        let src1 = vec![i32::MAX / 2; 16];
        let src2 = vec![i32::MAX / 4; 16];
        let mut fp = vec![i32::MAX / 4; offset.n_flat_ls + offset.lfac];

        windowing_short1(&src1, &src2, &mut fp, &offset, 12, 16);

        // First lfac samples unchanged
        for i in 0..8 {
            assert_eq!(fp[i], i32::MAX / 64, "fp[{}] should remain unchanged", i);
        }
        // Remaining should be windowed with shift
        // negate_sat(i32::MAX) = -i32::MAX, then mult_shift_q31 then right shift
        for i in 8..16 {
            assert!(fp[i] < 0, "fp[{}] should be negative after negation", i);
        }
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
        let mut fp = vec![-4606648; offset.n_flat_ls + offset.n_short];

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
        let mut fp = vec![77680672; offset.n_flat_ls + offset.n_short];

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
        let mut fp = vec![68186432; offset.n_flat_ls + offset.n_short];

        windowing_short2(&src1, &win_fwd, &mut fp, &offset, 14, 14);

        let exp = vec![68815524, 80150639, 83706379, 82424917, 88945434, 63187846, 53941786, 25596480, 3686188, -46665091, -61902306, -60676730, -65208106, -103500210, -26636729, -16162386, 114153150, 133854672, 243349688, 211244031, 224459545, 250259313, 248120977, 162188201, 124885091, 53324918, 25379756, -95957079, -88989760, -146919800, -217288630, -112248544, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(fp, exp);
    }

    // ============================================================================
    // windowing_short2() edge case tests - saturation
    // ============================================================================

    #[test]
    fn test_windowing_short2_saturating_add_overflow() {
        // Test saturating_add when mult results would overflow
        let offset = OffsetLengths {
            lfac: 8,
            n_flat_ls: 24,
            n_trans_ls: 16,
            n_long: 64,
            n_short: 16,
        };
        let src1 = vec![i32::MAX / 2; 8];
        let win_fwd = vec![i32::MAX / 2; 16];
        let mut fp = vec![i32::MAX / 2; offset.n_flat_ls + offset.n_short];

        windowing_short2(&src1, &win_fwd, &mut fp, &offset, 14, 14);

        // Values should be saturated, not overflowed
        for i in 0..16 {
            assert!(fp[i] <= i32::MAX, "fp[{}] should not overflow", i);
            assert!(fp[i] >= i32::MIN, "fp[{}] should not underflow", i);
        }
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
    // windowing_short3() edge case tests - saturation
    // ============================================================================

    #[test]
    fn test_windowing_short3_saturating_add_overflow() {
        // Test saturating_add when mult results would overflow
        let src1 = vec![i32::MAX / 2; 12];
        let win_fwd = vec![i32::MAX / 2; 12];
        let mut fp = vec![i32::MAX / 2; 12];

        let result_q = windowing_short3(&src1, &win_fwd, &mut fp, 14, 14);

        // Values should be saturated, not overflowed
        for i in 0..12 {
            assert!(fp[i] <= i32::MAX, "fp[{}] should not overflow", i);
            assert!(fp[i] >= i32::MIN, "fp[{}] should not underflow", i);
        }
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
            false, 14, 12, 14
        );
        
        let exp = vec![40766216, 47054666, 47847340, 60855076, 77113833, 67579190, 131048010, 105019168, 117387201, 137225888, 117271675, 130935568, 107374274, 99479094, 699163693, 163837054, 140341634, 104131670, 104131670, 140341634, 163837054, 699163693, 99479094, 107374274];
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
            true, 12, 16, 12
        );
        
        let exp = vec![268436455, 268436455, 268436455, 268436455, 268436455, 268436455, 268436455, 268436455, -268434456, -268434456, -268434456, -268434456, -268434456, -268434456, -268434456, -268434456, -268435394, -268435394, -268435394, -268435394, -268435394, -268435394, -268435394, -268435394, -268435394, -268435394, -268435394, -268435394, -268435394, -268435394, -268435394, -268435394];
        assert_eq!(fp, exp);
        assert_eq!(result_q, 12);
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