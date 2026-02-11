// decoder::ixheaacd::fft

//! FFT (Fast Fourier Transform) functions for AAC decoder
//!
//! This module implements complex FFT operations for both floating-point and
//! fixed-point arithmetic. These functions are used primarily in the IMDCT
//! computations for audio decoding.
//!
//! The implementation uses radix-4 and radix-2 butterfly operations with
//! mixed-radix support for non-power-of-2 lengths (specifically lengths
//! containing factors of 3).

use std::simd::{cmp::SimdOrd, i32x8, i64x4, Simd, simd_swizzle};
use std::intrinsics::simd::{simd_saturating_add, simd_saturating_sub};

/// FFT transform direction
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum FftMode {
    Forward = -1,
    Inverse = 1,
}

impl FftMode {
    /// Returns the sign multiplier for twiddle factor application
    pub fn sign(&self) -> i32 {
        *self as i32
    }
}

/// Extension trait for fixed-point DSP operations on i32
trait FixedPointOps {
    /// Q31 fixed-point multiply: ((a * b) >> 32) << 1
    fn mult_shl(self, other: Self) -> Self;
    /// Q31 fixed-point multiply with saturation: sat32((a * b) >> 31)
    fn mult32_sat(self, other: u32) -> Self;
}

impl FixedPointOps for i32 {
    #[inline]
    fn mult_shl(self, other: Self) -> Self {
        (((self as i64 * other as i64) >> 32) as i32) << 1
    }

    #[inline]
    fn mult32_sat(self, other: u32) -> Self {
        ((self as i64 * (other as i32) as i64) >> 31).clamp(i32::MIN as i64, i32::MAX as i64) as i32
    }
}

/// Count leading zeros normalization (ixheaac_norm32)
#[inline]
fn norm32(a: u32) -> i16 {
    (if (a & 0x80000000) != 0 { !a } else { a }).leading_zeros() as i16 - 1
}

/// Bit-reversal permutation index (DIG_REV macro)
#[inline]
fn dig_rev(i: usize, m: i16) -> usize {
    let mut v = i as u32;
    v = ((v & 0x33333333) << 2) | ((v & !0x33333333) >> 2);
    v = ((v & 0x0F0F0F0F) << 4) | ((v & !0x0F0F0F0F) >> 4);
    v = ((v & 0x00FF00FF) << 8) | ((v & !0x00FF00FF) >> 8);
    (v >> m) as usize
}

/// Radix-4 butterfly with configurable cross-term signs
fn radix4_butterfly(
    mut x0r: i32, mut x0i: i32, // DC component
    mut x1r: i32, mut x1i: i32, // Quarter-turn component
    mut x2r: i32, mut x2i: i32, // Half-turn component
    mut x3r: i32, mut x3i: i32, // Three-quarter component
    alt_x1x3: bool,             // Alternate x1/x3 signs (4th twiddle range)
    forward: bool,              // Forward FFT direction
) -> [i32; 8] {
    x0r = x0r.saturating_add(x2r);
    x0i = x0i.saturating_add(x2i);
    x2r = x0r.saturating_sub(x2r.saturating_mul(2));
    x2i = x0i.saturating_sub(x2i.saturating_mul(2));

    if alt_x1x3 {
        x1r = x1r.saturating_add(x3r);
        x1i = x1i.saturating_sub(x3i);
        x3r = x1r.saturating_sub(x3r.saturating_mul(2));
        x3i = x1i.saturating_add(x3i.saturating_mul(2));
    } else {
        x1r = x1r.saturating_add(x3r);
        x1i = x1i.saturating_add(x3i);
        x3r = x1r.saturating_sub(x3r.saturating_mul(2));
        x3i = x1i.saturating_sub(x3i.saturating_mul(2));
    }

    x0r = x0r.saturating_add(x1r);
    x0i = x0i.saturating_add(x1i);
    x1r = x0r.saturating_sub(x1r.saturating_mul(2));
    x1i = x0i.saturating_sub(x1i.saturating_mul(2));

    if forward {
        x2r = x2r.saturating_add(x3i);
        x2i = x2i.saturating_sub(x3r);
        x3i = x2r.saturating_sub(x3i.saturating_mul(2));
        x3r = x2i.saturating_add(x3r.saturating_mul(2));
    } else {
        x2r = x2r.saturating_sub(x3i);
        x2i = x2i.saturating_add(x3r);
        x3i = x2r.saturating_add(x3i.saturating_mul(2));
        x3r = x2i.saturating_sub(x3r.saturating_mul(2));
    }

    [x0r, x0i, x2r, x2i, x1r, x1i, x3i, x3r]
}


// =============================================================================
// SIMD Radix-4 Butterfly Implementation
// =============================================================================
/* 
/// SIMD helper: saturating add for i32x8
#[inline]
fn simd_saturating_add(a: i32x8, b: i32x8) -> i32x8 {
    // Widen to i64, add, clamp, narrow back
    let a_lo = i64x4::from_array([a[0] as i64, a[1] as i64, a[2] as i64, a[3] as i64]);
    let a_hi = i64x4::from_array([a[4] as i64, a[5] as i64, a[6] as i64, a[7] as i64]);
    let b_lo = i64x4::from_array([b[0] as i64, b[1] as i64, b[2] as i64, b[3] as i64]);
    let b_hi = i64x4::from_array([b[4] as i64, b[5] as i64, b[6] as i64, b[7] as i64]);

    let sum_lo = a_lo + b_lo;
    let sum_hi = a_hi + b_hi;

    let min = Simd::splat(i32::MIN as i64);
    let max = Simd::splat(i32::MAX as i64);

    let clamped_lo = sum_lo.simd_min(max).simd_max(min);
    let clamped_hi = sum_hi.simd_min(max).simd_max(min);

    i32x8::from_array([
        clamped_lo[0] as i32, clamped_lo[1] as i32, clamped_lo[2] as i32, clamped_lo[3] as i32,
        clamped_hi[0] as i32, clamped_hi[1] as i32, clamped_hi[2] as i32, clamped_hi[3] as i32,
    ])
}

/// SIMD helper: saturating sub for i32x8
#[inline]
fn simd_saturating_sub(a: i32x8, b: i32x8) -> i32x8 {
    let a_lo = i64x4::from_array([a[0] as i64, a[1] as i64, a[2] as i64, a[3] as i64]);
    let a_hi = i64x4::from_array([a[4] as i64, a[5] as i64, a[6] as i64, a[7] as i64]);
    let b_lo = i64x4::from_array([b[0] as i64, b[1] as i64, b[2] as i64, b[3] as i64]);
    let b_hi = i64x4::from_array([b[4] as i64, b[5] as i64, b[6] as i64, b[7] as i64]);

    let diff_lo = a_lo - b_lo;
    let diff_hi = a_hi - b_hi;

    let min = Simd::splat(i32::MIN as i64);
    let max = Simd::splat(i32::MAX as i64);

    let clamped_lo = diff_lo.simd_min(max).simd_max(min);
    let clamped_hi = diff_hi.simd_min(max).simd_max(min);

    i32x8::from_array([
        clamped_lo[0] as i32, clamped_lo[1] as i32, clamped_lo[2] as i32, clamped_lo[3] as i32,
        clamped_hi[0] as i32, clamped_hi[1] as i32, clamped_hi[2] as i32, clamped_hi[3] as i32,
    ])
}
*/
/// Vectorized radix-4 butterfly for forward FFT (alt_x1x3 = false).
///
/// Input layout:  [x0r, x0i, x1r, x1i, x2r, x2i, x3r, x3i]
/// Output layout: [y0r, y0i, y2r, y2i, y1r, y1i, y3i, y3r]
///
/// This matches the scalar `radix4_butterfly` output ordering.
#[inline]
fn radix4_bfly_simd_fwd(x: i32x8) -> i32x8 {
    // Extract components: x = [x0r, x0i, x1r, x1i, x2r, x2i, x3r, x3i]
    //                          0     1     2     3     4     5     6     7
unsafe {
    // Stage 1: t0 = x0 + x2, t2 = x0 - x2, t1 = x1 + x3, t3 = x1 - x3
    // Rearrange for parallel add/sub: [x0r, x0i, x1r, x1i, x0r, x0i, x1r, x1i]
    let x_02_13 = simd_swizzle!(x, [0, 1, 2, 3, 0, 1, 2, 3]);
    let x_24_36 = simd_swizzle!(x, [4, 5, 6, 7, 4, 5, 6, 7]);

    // [t0r, t0i, t1r, t1i, t2r, t2i, t3r, t3i]
    //  sum  sum  sum  sum  diff diff diff diff
    let sums = simd_saturating_add(x_02_13, x_24_36);
    let diffs = simd_saturating_sub(x_02_13, x_24_36);

    // Combine: [t0r, t0i, t1r, t1i, t2r, t2i, t3r, t3i]
    let t = simd_swizzle!(sums, diffs, [0, 1, 2, 3, 12, 13, 14, 15]);

    // Stage 2: y0 = t0 + t1, y1 = t0 - t1
    // [t0r, t0i, t0r, t0i, t2r, t2i, t2r, t2i]
    let t_0022 = simd_swizzle!(t, [0, 1, 0, 1, 4, 5, 4, 5]);
    // [t1r, t1i, t1r, t1i, t3r, t3i, t3r, t3i]
    let t_1133 = simd_swizzle!(t, [2, 3, 2, 3, 6, 7, 6, 7]);

    // [y0r, y0i, y1r, y1i, t2+t3r, t2+t3i, t2-t3r, t2-t3i]
    let upper_sums = simd_saturating_add(t_0022, t_1133);
    let upper_diffs = simd_saturating_sub(t_0022, t_1133);

    // y0, y1 from upper_sums/upper_diffs
    // y0 = sums[0,1], y1 = diffs[2,3]
    let y01 = simd_swizzle!(upper_sums, upper_diffs, [0, 1, 10, 11, 4, 5, 6, 7]);

    // Stage 3: Forward rotation - y2 = t2 + j*t3, y3 = t2 - j*t3
    // j*t3 = (-t3i, t3r), so:
    // y2r = t2r - t3i, y2i = t2i + t3r
    // y3i = t2r + t3i (from y2r - 2*(-t3i) = y2r + 2*t3i... but cleaner to compute directly)
    // y3r = t2i - t3r (swapped in output as y3i, y3r)

    let t2r = t[4];
    let t2i = t[5];
    let t3r = t[6];
    let t3i = t[7];

    let y2r = t2r.saturating_add(t3i);  // forward: add
    let y2i = t2i.saturating_sub(t3r);
    let y3i = y2r.saturating_sub(t3i.saturating_mul(2)); // = t2r - t3i
    let y3r = y2i.saturating_add(t3r.saturating_mul(2)); // = t2i + t3r

    // Output: [y0r, y0i, y2r, y2i, y1r, y1i, y3i, y3r]
    i32x8::from_array([y01[0], y01[1], y2r, y2i, y01[2], y01[3], y3i, y3r])
}
}

/// Vectorized radix-4 butterfly for inverse FFT (alt_x1x3 = false).
///
/// Input layout:  [x0r, x0i, x1r, x1i, x2r, x2i, x3r, x3i]
/// Output layout: [y0r, y0i, y2r, y2i, y1r, y1i, y3i, y3r]
#[inline]
fn radix4_bfly_simd_inv(x: i32x8) -> i32x8 {
    unsafe {
    // Stage 1: t0 = x0 + x2, t2 = x0 - x2, t1 = x1 + x3, t3 = x1 - x3
    let x_02_13 = simd_swizzle!(x, [0, 1, 2, 3, 0, 1, 2, 3]);
    let x_24_36 = simd_swizzle!(x, [4, 5, 6, 7, 4, 5, 6, 7]);

    let sums = simd_saturating_add(x_02_13, x_24_36);
    let diffs = simd_saturating_sub(x_02_13, x_24_36);

    let t = simd_swizzle!(sums, diffs, [0, 1, 2, 3, 12, 13, 14, 15]);

    // Stage 2: y0 = t0 + t1, y1 = t0 - t1
    let t_0022 = simd_swizzle!(t, [0, 1, 0, 1, 4, 5, 4, 5]);
    let t_1133 = simd_swizzle!(t, [2, 3, 2, 3, 6, 7, 6, 7]);

    let upper_sums = simd_saturating_add(t_0022, t_1133);
    let upper_diffs = simd_saturating_sub(t_0022, t_1133);

    let y01 = simd_swizzle!(upper_sums, upper_diffs, [0, 1, 10, 11, 4, 5, 6, 7]);

    // Stage 3: Inverse rotation - y2 = t2 - j*t3, y3 = t2 + j*t3
    // -j*t3 = (t3i, -t3r), so:
    // y2r = t2r + t3i... wait, that's wrong for inverse
    // Let me recalculate from the scalar code:
    // inverse: x2r = x2r - x3i, x2i = x2i + x3r
    //          x3i = x2r + x3i*2 (= x2r_old - x3i + 2*x3i = x2r_old + x3i)
    //          x3r = x2i - x3r*2 (= x2i_old + x3r - 2*x3r = x2i_old - x3r)

    let t2r = t[4];
    let t2i = t[5];
    let t3r = t[6];
    let t3i = t[7];

    let y2r = t2r.saturating_sub(t3i);  // inverse: sub
    let y2i = t2i.saturating_add(t3r);
    let y3i = y2r.saturating_add(t3i.saturating_mul(2)); // = t2r + t3i
    let y3r = y2i.saturating_sub(t3r.saturating_mul(2)); // = t2i - t3r

    i32x8::from_array([y01[0], y01[1], y2r, y2i, y01[2], y01[3], y3i, y3r])
    }
}

/// Dispatch to appropriate SIMD butterfly based on FFT direction.
/// For alt_x1x3=true cases, falls back to scalar implementation.
#[inline]
fn radix4_butterfly_simd(
    x: i32x8,       // [x0r, x0i, x1r, x1i, x2r, x2i, x3r, x3i]
    alt_x1x3: bool, // Use scalar fallback if true (Range 4 only)
    forward: bool,  // FFT direction
) -> i32x8 {
    if alt_x1x3 {
        // Fallback to scalar for alt_x1x3 case (rare, Range 4 only)
        let bfly = radix4_butterfly(
            x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7],
            true, forward,
        );
        i32x8::from_array(bfly)
    } else if forward {
        radix4_bfly_simd_fwd(x)
    } else {
        radix4_bfly_simd_inv(x)
    }
}

// Twiddle multiplication types for different angle quadrants.
// Forward: standard complex multiply  (r*cos - i*sin, r*sin + i*cos)
// Inverse: conjugate complex multiply (r*cos + i*sin, i*cos - r*sin)
// Types C-F handle quadrant wrapping when twiddle indices exceed table bounds.

/// Type A: standard complex multiply (forward x1/x2/x3 range 1)
#[inline]
fn tw_fwd(r: i32, i: i32, wh: u32, wl: u32) -> (i32, i32) {
    (r.mult32_sat(wl).saturating_sub(i.mult32_sat(wh)),
     r.mult32_sat(wh).saturating_add(i.mult32_sat(wl)))
}

/// Type B: conjugate complex multiply (inverse x1/x2/x3 range 1)
#[inline]
fn tw_inv(r: i32, i: i32, wh: u32, wl: u32) -> (i32, i32) {
    (r.mult32_sat(wl).saturating_add(i.mult32_sat(wh)),
     r.mult32_sat(wh).wrapping_neg().saturating_add(i.mult32_sat(wl)))
}

/// Type C: quadrant-wrapped (forward x3 ranges 2-3, x2 ranges 3-4)
#[inline]
fn tw_fwd_wrap(r: i32, i: i32, wh: u32, wl: u32) -> (i32, i32) {
    (r.mult32_sat(wh).saturating_add(i.mult32_sat(wl)),
     i.mult32_sat(wh).saturating_sub(r.mult32_sat(wl)))
}

/// Type D: quadrant-wrapped conjugate (inverse x3 ranges 2-3, x2 ranges 3-4)
#[inline]
fn tw_inv_wrap(r: i32, i: i32, wh: u32, wl: u32) -> (i32, i32) {
    (r.mult32_sat(wh).saturating_sub(i.mult32_sat(wl)),
     r.mult32_sat(wl).saturating_add(i.mult32_sat(wh)))
}

/// Type E: double-wrapped (forward x3 range 4)
#[inline]
fn tw_fwd_wrap2(r: i32, i: i32, wh: u32, wl: u32) -> (i32, i32) {
    (i.mult32_sat(wh).saturating_sub(r.mult32_sat(wl)),
     r.mult32_sat(wh).saturating_add(i.mult32_sat(wl)))
}

/// Type F: double-wrapped conjugate (inverse x3 range 4)
#[inline]
fn tw_inv_wrap2(r: i32, i: i32, wh: u32, wl: u32) -> (i32, i32) {
    (r.mult32_sat(wl).saturating_add(i.mult32_sat(wh)).wrapping_neg(),
     r.mult32_sat(wh).wrapping_neg().saturating_add(i.mult32_sat(wl)))
}

/// Valid mixed-radix FFT lengths (3^k * 2^m)
const MIXED_RADIX_LENGTHS: [usize; 4] = [48, 96, 192, 384];

/// Specialized 32-point FFT for MPEG Surround (MPS) synthesis filter bank.
///
/// Operates on interleaved real/imaginary pairs and uses hardcoded twiddle
/// factors for efficiency. Input/output uses interleaved format where each
/// array contains `npoints * 2` elements: [r0, i0, r1, i1, ...].
///
/// # C signature
/// ```c
/// VOID ixheaacd_mps_synth_calc_fft(FLOAT32 *ptr_xr, FLOAT32 *ptr_xi, WORD32 npoints);
/// ```
pub fn mps_synth_calc_fft(
    ptr_xr: &mut [f32], // Real part (interleaved): npoints * 2 elements
    ptr_xi: &mut [f32], // Imaginary part (interleaved): npoints * 2 elements
) {
    assert_eq!(ptr_xr.len(), ptr_xi.len(), "Real and imaginary arrays must have equal length");
    assert_eq!(ptr_xr.len(), 64, "MPS synth FFT requires exactly 64 elements (32-point FFT)");

    let npoints = (ptr_xr.len() / 2) as i32;

    #[cfg(feature = "fallback")]
    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_mps_synth_calc_fft(
            ptr_xr.as_mut_ptr(),
            ptr_xi.as_mut_ptr(),
            npoints,
        );
    }

    #[cfg(not(feature = "fallback"))]
    {
        let _ = (ptr_xr, ptr_xi, npoints);
        todo!("Pure Rust implementation pending")
    }
}

/// General-purpose floating-point complex FFT for MPS processing.
///
/// Supports power-of-2 lengths up to 128 points. Uses radix-4 Cooley-Tukey
/// algorithm with decimation-in-time.
///
/// # C signature
/// ```c
/// VOID ixheaacd_mps_complex_fft(FLOAT32 *xr, FLOAT32 *xi, WORD32 nlength);
/// ```
pub fn mps_complex_fft(
    xr: &mut [f32], // Real part: nlength elements
    xi: &mut [f32], // Imaginary part: nlength elements
) {
    assert_eq!(xr.len(), xi.len(), "Real and imaginary arrays must have equal length");
    let nlength = xr.len();
    assert!(nlength <= 128, "MPS complex FFT supports lengths up to 128");
    assert!(nlength.is_power_of_two(), "MPS complex FFT requires power-of-2 length");

    #[cfg(feature = "fallback")]
    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_mps_complex_fft(
            xr.as_mut_ptr(),
            xi.as_mut_ptr(),
            nlength as i32,
        );
    }

    #[cfg(not(feature = "fallback"))]
    {
        let _ = (xr, xi, nlength);
        todo!("Pure Rust implementation pending")
    }
}

/// Fixed-point power-of-2 complex FFT with automatic scaling.
///
/// Used as the primary FFT engine for the decoder. Implements radix-4
/// Cooley-Tukey with saturating arithmetic.
///
/// # C signature
/// ```c
/// VOID ixheaacd_complex_fft_p2_dec(WORD32 *xr, WORD32 *xi, WORD32 nlength,
///                                   WORD32 fft_mode, WORD32 *preshift);
/// ```
pub fn complex_fft_p2(
    xr: &mut [i32],        // Real part: nlength elements
    xi: &mut [i32],        // Imaginary part: nlength elements
    fft_mode: FftMode, // Forward (-1) or Inverse (+1)
    preshift: i32,         // Initial scaling factor
) -> i32  // Updated preshift value after transform.
{
    let npoints = xr.len();
    assert!(npoints <= 512, "Power-of-2 FFT supports lengths up to 512");
    assert!(npoints.is_power_of_two(), "Power-of-2 FFT requires power-of-2 length");
    assert_eq!(xr.len(), xi.len(), "Real and imaginary arrays must have equal length");

    #[cfg(feature = "fallback")]
    unsafe {
        let mut preshift_out = preshift;
        crate::gen_ixheaacd_ref::ixheaacd_complex_fft_p2_dec(
            xr.as_mut_ptr(),
            xi.as_mut_ptr(),
            npoints as i32,
            fft_mode.sign(),
            &mut preshift_out,
        );
        return preshift_out;
    }

    use crate::ixheaacd::rom::TWIDDLE_TABLE_FFT_32X32;

    let forward = fft_mode == FftMode::Forward;
    let dig_rev_shift = norm32(npoints as u32) + 1 - 16;
    let mut n_stages = 30 - norm32(npoints as u32);
    let not_power_4 = (n_stages & 1) != 0;
    n_stages >>= 1;

    // Compute bit-count n and shift
    let mut n = 0u32;
    let mut npts = npoints >> 1;
    while npts != 0 {
        n += 1;
        npts >>= 1;
    }
    let mut shift = if n % 2 == 0 { (n + 4) / 2 } else { (n + 3) / 2 };

    // Scale input into interleaved working buffer
    let mut ptr_x = [0i32; 1024]; // buffer on the stack!
    {
        let scale = Simd::splat(1i32 << shift);
        let mut i = 0;
        while i + 8 <= npoints {
            let xr_vec = i32x8::from_slice(&xr[i..i + 8]);
            let xi_vec = i32x8::from_slice(&xi[i..i + 8]);

            let (lo1, lo2) = xr_vec.interleave(xi_vec);
            (lo1 / scale).copy_to_slice(&mut ptr_x[2*i .. 2*i + 8]);
            (lo2 / scale).copy_to_slice(&mut ptr_x[2*i + 8 .. 2*i + 16]);
            i += 8;
        }
    }

    let mut y = [0i32; 1024]; // buffer on the stack!

    // Initial radix-4 pass with bit-reversal
    {
        let mut yi = 0usize;
        for i in (0..npoints).step_by(4) {
            let mut h2 = dig_rev(i, dig_rev_shift);
            if not_power_4 {
                h2 = (h2 + 1) & !1;
            }
            let half = npoints >> 1;

            // Pack scattered inputs into SIMD vector
            let x = i32x8::from_array([
                ptr_x[h2],              ptr_x[h2 + 1],              // x0
                ptr_x[h2 + half],       ptr_x[h2 + half + 1],       // x1
                ptr_x[h2 + 2 * half],   ptr_x[h2 + 2 * half + 1],   // x2
                ptr_x[h2 + 3 * half],   ptr_x[h2 + 3 * half + 1],   // x3
            ]);

            let bfly = radix4_butterfly_simd(x, false, forward);
            bfly.copy_to_slice(&mut y[yi..yi + 8]);
            yi += 8;
        }
    }

    // Main radix-4 stages
    let mut del: usize = 4;
    let mut nodespacing: usize = 64;
    let mut in_loop_cnt = npoints >> 4;
    let twiddles = &TWIDDLE_TABLE_FFT_32X32;
    let tw_apply: fn(i32, i32, u32, u32) -> (i32, i32) = if forward { tw_fwd } else { tw_inv };
    let tw_apply_wrap: fn(i32, i32, u32, u32) -> (i32, i32) = if forward { tw_fwd_wrap } else { tw_inv_wrap };
    let tw_apply_wrap2: fn(i32, i32, u32, u32) -> (i32, i32) = if forward { tw_fwd_wrap2 } else { tw_inv_wrap2 };

    for _ in 1..n_stages {
        // Range 0: no twiddles (j=0)
        {
            let mut base = 0usize;
            let stride = del << 1;
            for _ in 0..in_loop_cnt {
                // Pack strided inputs into SIMD vector
                let x = i32x8::from_array([
                    y[base],               y[base + 1],               // x0
                    y[base + stride],      y[base + stride + 1],      // x1
                    y[base + 2 * stride],  y[base + 2 * stride + 1],  // x2
                    y[base + 3 * stride],  y[base + 3 * stride + 1],  // x3
                ]);

                let bfly = radix4_butterfly_simd(x, false, forward);

                // Scatter outputs back to strided positions
                y[base] = bfly[0];
                y[base + 1] = bfly[1];
                y[base + stride] = bfly[2];
                y[base + stride + 1] = bfly[3];
                y[base + 2 * stride] = bfly[4];
                y[base + 2 * stride + 1] = bfly[5];
                y[base + 3 * stride] = bfly[6];
                y[base + 3 * stride + 1] = bfly[7];
                base += 4 * stride;
            }
        }

        let sec_loop_cnt = {
            let v = (nodespacing * del) as i32;
            (v / 4) + (v / 8) - (v / 16) + (v / 32) - (v / 64) + (v / 128) - (v / 256)
        } as usize;

        // Range 1: standard twiddles
        let mut col = 1usize; // column offset in interleaved y
        let mut j = nodespacing;
        let stride = del << 1;
        while j <= sec_loop_cnt {
            let w1h = twiddles[2 * j];
            let w1l = twiddles[2 * j + 1];
            let w2h = twiddles[2 * (j << 1)];
            let w2l = twiddles[2 * (j << 1) + 1];
            let w3h = twiddles[2 * j + 2 * (j << 1)];
            let w3l = twiddles[2 * j + 2 * (j << 1) + 1];

            let mut base = col * 2;
            for _k in 0..in_loop_cnt {
                let (x1r, x1i) = tw_apply(y[base + stride], y[base + stride + 1], w1h, w1l);
                let (x2r, x2i) = tw_apply(y[base + 2 * stride], y[base + 2 * stride + 1], w2h, w2l);
                let (x3r, x3i) = tw_apply(y[base + 3 * stride], y[base + 3 * stride + 1], w3h, w3l);

                let x = i32x8::from_array([
                    y[base], y[base + 1], x1r, x1i, x2r, x2i, x3r, x3i
                ]);
                let bfly = radix4_butterfly_simd(x, false, forward);

                y[base] = bfly[0];
                y[base + 1] = bfly[1];
                y[base + stride] = bfly[2];
                y[base + stride + 1] = bfly[3];
                y[base + 2 * stride] = bfly[4];
                y[base + 2 * stride + 1] = bfly[5];
                y[base + 3 * stride] = bfly[6];
                y[base + 3 * stride + 1] = bfly[7];
                base += 4 * stride;
            }
            col += 1;
            j += nodespacing;
        }

        // Range 2: x3 wrapped
        while j <= (nodespacing * del) >> 1 {
            let w1h = twiddles[2 * j];
            let w1l = twiddles[2 * j + 1];
            let w2h = twiddles[2 * (j << 1)];
            let w2l = twiddles[2 * (j << 1) + 1];
            let w3h = twiddles[2 * j + 2 * (j << 1) - 512];
            let w3l = twiddles[2 * j + 2 * (j << 1) - 511];

            let mut base = col * 2;
            for _k in 0..in_loop_cnt {
                let (x1r, x1i) = tw_apply(y[base + stride], y[base + stride + 1], w1h, w1l);
                let (x2r, x2i) = tw_apply(y[base + 2 * stride], y[base + 2 * stride + 1], w2h, w2l);
                let (x3r, x3i) = tw_apply_wrap(y[base + 3 * stride], y[base + 3 * stride + 1], w3h, w3l);

                let x = i32x8::from_array([
                    y[base], y[base + 1], x1r, x1i, x2r, x2i, x3r, x3i
                ]);
                let bfly = radix4_butterfly_simd(x, false, forward);

                y[base] = bfly[0];
                y[base + 1] = bfly[1];
                y[base + stride] = bfly[2];
                y[base + stride + 1] = bfly[3];
                y[base + 2 * stride] = bfly[4];
                y[base + 2 * stride + 1] = bfly[5];
                y[base + 3 * stride] = bfly[6];
                y[base + 3 * stride + 1] = bfly[7];
                base += 4 * stride;
            }
            col += 1;
            j += nodespacing;
        }

        // Range 3: x2 + x3 wrapped
        while j <= sec_loop_cnt * 2 {
            let w1h = twiddles[2 * j];
            let w1l = twiddles[2 * j + 1];
            let w2h = twiddles[2 * (j << 1) - 512];
            let w2l = twiddles[2 * (j << 1) - 511];
            let w3h = twiddles[2 * j + 2 * (j << 1) - 512];
            let w3l = twiddles[2 * j + 2 * (j << 1) - 511];

            let mut base = col * 2;
            for _k in 0..in_loop_cnt {
                let (x1r, x1i) = tw_apply(y[base + stride], y[base + stride + 1], w1h, w1l);
                let (x2r, x2i) = tw_apply_wrap(y[base + 2 * stride], y[base + 2 * stride + 1], w2h, w2l);
                let (x3r, x3i) = tw_apply_wrap(y[base + 3 * stride], y[base + 3 * stride + 1], w3h, w3l);

                let x = i32x8::from_array([
                    y[base], y[base + 1], x1r, x1i, x2r, x2i, x3r, x3i
                ]);
                let bfly = radix4_butterfly_simd(x, false, forward);

                y[base] = bfly[0];
                y[base + 1] = bfly[1];
                y[base + stride] = bfly[2];
                y[base + stride + 1] = bfly[3];
                y[base + 2 * stride] = bfly[4];
                y[base + 2 * stride + 1] = bfly[5];
                y[base + 3 * stride] = bfly[6];
                y[base + 3 * stride + 1] = bfly[7];
                base += 4 * stride;
            }
            col += 1;
            j += nodespacing;
        }

        // Range 4: x2 wrapped, x3 double-wrapped, alt butterfly
        while j < nodespacing * del {
            let w1h = twiddles[2 * j];
            let w1l = twiddles[2 * j + 1];
            let w2h = twiddles[2 * (j << 1) - 512];
            let w2l = twiddles[2 * (j << 1) - 511];
            let w3h = twiddles[2 * j + 2 * (j << 1) - 1024];
            let w3l = twiddles[2 * j + 2 * (j << 1) - 1023];

            let mut base = col * 2;
            for _k in 0..in_loop_cnt {
                let (x1r, x1i) = tw_apply(y[base + stride], y[base + stride + 1], w1h, w1l);
                let (x2r, x2i) = tw_apply_wrap(y[base + 2 * stride], y[base + 2 * stride + 1], w2h, w2l);
                let (x3r, x3i) = tw_apply_wrap2(y[base + 3 * stride], y[base + 3 * stride + 1], w3h, w3l);

                // Range 4 uses alt_x1x3=true, falls back to scalar in radix4_butterfly_simd
                let x = i32x8::from_array([
                    y[base], y[base + 1], x1r, x1i, x2r, x2i, x3r, x3i
                ]);
                let bfly = radix4_butterfly_simd(x, true, forward);

                y[base] = bfly[0];
                y[base + 1] = bfly[1];
                y[base + stride] = bfly[2];
                y[base + stride + 1] = bfly[3];
                y[base + 2 * stride] = bfly[4];
                y[base + 2 * stride + 1] = bfly[5];
                y[base + 3 * stride] = bfly[6];
                y[base + 3 * stride + 1] = bfly[7];
                base += 4 * stride;
            }
            col += 1;
            j += nodespacing;
        }

        nodespacing >>= 2;
        del <<= 2;
        in_loop_cnt >>= 2;
    }

    // Optional radix-2 final stage
    if not_power_4 {
        let nodespacing2 = nodespacing << 1;
        shift += 1;
        let stride = del << 1;

        // First half
        let mut yi = 0usize;
        let mut tw_idx = 0usize;
        for _j in 0..del / 2 {
            let w1h = twiddles[tw_idx];
            let w1l = twiddles[tw_idx + 1];
            tw_idx += nodespacing2 * 2;

            let x0r = y[yi];
            let x0i = y[yi + 1];
            let x1r = y[yi + stride];
            let x1i = y[yi + stride + 1];

            let (t1r, t1i) = tw_apply(x1r, x1i, w1h, w1l);

            y[yi + stride] = x0r / 2 - t1r / 2;
            y[yi + stride + 1] = x0i / 2 - t1i / 2;
            y[yi] = x0r / 2 + t1r / 2;
            y[yi + 1] = x0i / 2 + t1i / 2;
            yi += 2;
        }

        // Second half
        let mut tw_idx = 0usize;
        for _j in 0..del / 2 {
            let w1h = twiddles[tw_idx];
            let w1l = twiddles[tw_idx + 1];
            tw_idx += nodespacing2 * 2;

            let x0r = y[yi];
            let x0i = y[yi + 1];
            let x1r = y[yi + stride];
            let x1i = y[yi + stride + 1];

            let (t1r, t1i) = tw_apply_wrap(x1r, x1i, w1h, w1l);

            y[yi + stride] = x0r / 2 - t1r / 2;
            y[yi + stride + 1] = x0i / 2 - t1i / 2;
            y[yi] = x0r / 2 + t1r / 2;
            y[yi + 1] = x0i / 2 + t1i / 2;
            yi += 2;
        }
    }

    // Deinterleave output
    for i in 0..npoints {
        xr[i] = y[2 * i];
        xi[i] = y[2 * i + 1];
    }

    shift as i32 - preshift
}
/*
fn radix4_butterfly_x2(
    x0: i32x8,  // [x0r_a, x0i_a, x0r_b, x0i_b, ...]
    x1: i32x8,
    x2: i32x8,
    x3: i32x8,
    forward: bool,
) -> (i32x8, i32x8, i32x8, i32x8) {
    // Stage 1: x0 +/- x2, x1 +/- x3
    let t0 = x0.saturating_add(x2);
    let t2 = x0.saturating_sub(x2);
    let t1 = x1.saturating_add(x3);
    let t3 = x1.saturating_sub(x3);

    // Stage 2: combine with rotation for x3
    let y0 = t0.saturating_add(t1);
    let y1 = t0.saturating_sub(t1);

    // Rotation for t3: swap real/imag and negate appropriately
    let t3_rot = if forward {
        // j * t3 = (-t3i, t3r)
        let t3_swapped = simd_swizzle!(t3, [1, 0, 3, 2, 5, 4, 7, 6]);
        let signs = i32x8::from_array([-1, 1, -1, 1, -1, 1, -1, 1]);
        t3_swapped * signs
    } else {
        // -j * t3 = (t3i, -t3r)
        let t3_swapped = simd_swizzle!(t3, [1, 0, 3, 2, 5, 4, 7, 6]);
        let signs = i32x8::from_array([1, -1, 1, -1, 1, -1, 1, -1]);
        t3_swapped * signs
    };

    let y2 = t2.saturating_add(t3_rot);
    let y3 = t2.saturating_sub(t3_rot);

    (y0, y2, y1, y3)
}
*/
/// Radix-3 DFT butterfly for mixed-radix FFT.
///
/// Implements the 3-point DFT kernel used in mixed-radix FFT for lengths
/// containing factors of 3.
///
/// # C signature
/// ```c
/// static void ixheaacd_complex_3point_fft(WORD32 *inp, WORD32 *op, WORD32 sign_dir);
/// ```
pub fn complex_3point_fft(
    inp: &[i32],           // Input: 6 elements (3 complex pairs)
    op: &mut [i32],        // Output: 6 elements (3 complex pairs)
    sign_dir: FftMode, // Forward (-1) or Inverse (+1)
) {
    assert_eq!(inp.len(), 6, "3-point FFT input must have exactly 6 elements");
    assert_eq!(op.len(), 6, "3-point FFT output must have exactly 6 elements");

    // Note: The C function is static inline, so we implement it directly here
    // rather than calling through FFI. For fallback mode, we still provide
    // the implementation inline.

    let sinmu = -1859775393_i32 * sign_dir.sign();

    let temp_real = inp[0].saturating_add(inp[2]);
    let temp_imag = inp[1].saturating_add(inp[3]);

    let add_r = inp[2].saturating_add(inp[4]);
    let add_i = inp[3].saturating_add(inp[5]);

    let sub_r = inp[2].saturating_sub(inp[4]);
    let sub_i = inp[3].saturating_sub(inp[5]);

    let p1 = add_r >> 1;
    let p4 = add_i >> 1;
    let p2 = sub_i.mult_shl(sinmu);
    let p3 = sub_r.mult_shl(sinmu);

    let temp = inp[0].wrapping_sub(p1);

    op[0] = temp_real.saturating_add(inp[4]);
    op[1] = temp_imag.saturating_add(inp[5]);
    op[2] = temp.saturating_add(p2);
    op[3] = inp[1].saturating_sub(p3).saturating_sub(p4);
    op[4] = temp.saturating_sub(p2);
    op[5] = inp[1].saturating_add(p3).saturating_sub(p4);

}

/// Mixed-radix FFT for lengths that are products of powers of 2 and 3.
///
/// Supports lengths like 48, 96, 192, 384 (3^k * 2^m).
///
/// # C signature
/// ```c
/// VOID ixheaacd_complex_fft_p3(WORD32 *xr, WORD32 *xi, WORD32 nlength,
///                               WORD32 fft_mode, WORD32 *preshift);
/// ```
pub fn complex_fft_p3(
    xr: &mut [i32],        // Real part: nlength elements
    xi: &mut [i32],        // Imaginary part: nlength elements
    fft_mode: FftMode, // Forward (-1) or Inverse (+1)
    preshift: i32,         // Initial scaling factor
) -> i32  // Updated preshift value after transform.
{
    let nlength = xr.len();
    assert!(
        MIXED_RADIX_LENGTHS.contains(&nlength),
        "Mixed-radix FFT requires length in {:?}, got {}",
        MIXED_RADIX_LENGTHS,
        nlength
    );
    assert_eq!(xr.len(), xi.len(), "Real and imaginary arrays must have equal length");

    #[cfg(feature = "fallback")]
    unsafe {
        let mut preshift_out = preshift;
        crate::gen_ixheaacd_ref::ixheaacd_complex_fft_p3(
            xr.as_mut_ptr(),
            xi.as_mut_ptr(),
            nlength as i32,
            fft_mode.sign(),
            &mut preshift_out,
        );
        return preshift_out;
    }

    use crate::ixheaacd::rom::{TWIDDLE_TABLE_3PI, TWIDDLE_TABLE_3PR};

    let mut mpass = nlength;
    let mut cnfac = 0usize;
    while mpass % 3 == 0 {
        mpass /= 3;
        cnfac += 1;
    }

    // Run power-of-2 sub-FFTs on strided sub-sequences
    let mut xr_3 = [0i32; 384];
    let mut xi_3 = [0i32; 384];
    for i in 0..3 * cnfac {
        for j in 0..mpass {
            xr_3[j] = xr[3 * j + i];
            xi_3[j] = xi[3 * j + i];
        }
        complex_fft_p2(&mut xr_3[..mpass], &mut xi_3[..mpass], fft_mode, 0);
        for j in 0..mpass {
            xr[3 * j + i] = xr_3[j];
            xi[3 * j + i] = xi_3[j];
        }
    }

    // Compute shift from mpass (matching C behavior)
    let mut n = 0u32;
    let mut npts = mpass >> 1;
    while npts != 0 {
        n += 1;
        npts >>= 1;
    }
    let shift = if n % 2 == 0 { (n + 4) / 2 } else { (n + 5) / 2 } as i32;

    // Scale and interleave into working buffer
    let mut ptr_x = [0i32; 768];
    for i in 0..nlength {
        ptr_x[2 * i] = xr[i] >> 1;
        ptr_x[2 * i + 1] = xi[i] >> 1;
    }

    // Apply twiddle factors from 3PR/3PI tables
    let tw_step = 3 * (128 / mpass - 1) + 1;
    if fft_mode == FftMode::Forward {
        let mut tw_pos = 0usize;
        for i in (0..nlength).step_by(3) {
            tw_pos += 1;
            let (w1r, w1i) = (TWIDDLE_TABLE_3PR[tw_pos], TWIDDLE_TABLE_3PI[tw_pos]);
            let r = ptr_x[2 * i + 2];
            let im = ptr_x[2 * i + 3];
            ptr_x[2 * i + 2] = r.mult32_sat(w1r).saturating_sub(im.mult32_sat(w1i));
            ptr_x[2 * i + 3] = r.mult32_sat(w1i).saturating_add(im.mult32_sat(w1r));

            tw_pos += 1;
            let (w2r, w2i) = (TWIDDLE_TABLE_3PR[tw_pos], TWIDDLE_TABLE_3PI[tw_pos]);
            let r = ptr_x[2 * i + 4];
            let im = ptr_x[2 * i + 5];
            ptr_x[2 * i + 4] = r.mult32_sat(w2r).saturating_sub(im.mult32_sat(w2i));
            ptr_x[2 * i + 5] = r.mult32_sat(w2i).saturating_add(im.mult32_sat(w2r));

            tw_pos += tw_step;
        }
    } else {
        let mut tw_pos = 0usize;
        for i in (0..nlength).step_by(3) {
            tw_pos += 1;
            let (w1r, w1i) = (TWIDDLE_TABLE_3PR[tw_pos], TWIDDLE_TABLE_3PI[tw_pos]);
            let r = ptr_x[2 * i + 2];
            let im = ptr_x[2 * i + 3];
            ptr_x[2 * i + 2] = r.mult32_sat(w1r).saturating_add(im.mult32_sat(w1i));
            ptr_x[2 * i + 3] = im.mult32_sat(w1r).saturating_sub(r.mult32_sat(w1i));

            tw_pos += 1;
            let (w2r, w2i) = (TWIDDLE_TABLE_3PR[tw_pos], TWIDDLE_TABLE_3PI[tw_pos]);
            let r = ptr_x[2 * i + 4];
            let im = ptr_x[2 * i + 5];
            ptr_x[2 * i + 4] = r.mult32_sat(w2r).saturating_add(im.mult32_sat(w2i));
            ptr_x[2 * i + 5] = im.mult32_sat(w2r).saturating_sub(r.mult32_sat(w2i));

            tw_pos += tw_step;
        }
    }

    // Run 3-point FFTs
    let mut y = [0i32; 768];
    for i in 0..mpass {
        complex_3point_fft(&ptr_x[6 * i..6 * i + 6], &mut y[6 * i..6 * i + 6], fft_mode);
    }

    // Reorder output
    for i in 0..mpass {
        xr[i] = y[6 * i];
        xi[i] = y[6 * i + 1];
        xr[mpass + i] = y[6 * i + 2];
        xi[mpass + i] = y[6 * i + 3];
        xr[2 * mpass + i] = y[6 * i + 4];
        xi[2 * mpass + i] = y[6 * i + 5];
    }

    shift - preshift + 1
}

/// Main FFT dispatcher that routes to the appropriate implementation.
///
/// Automatically selects between power-of-2 (p2) and mixed-radix (p3)
/// implementations based on the FFT length.
///
/// # C signature
/// ```c
/// VOID ixheaacd_complex_fft(WORD32 *data_r, WORD32 *data_i, WORD32 nlength,
///                            WORD32 fft_mode, WORD32 *preshift);
/// ```
pub fn complex_fft(
    data_r: &mut [i32],    // Real part: nlength elements
    data_i: &mut [i32],    // Imaginary part: nlength elements
    fft_mode: FftMode, // Forward (-1) or Inverse (+1)
    preshift: i32,         // Initial scaling factor
) -> i32  // Updated preshift value after transform.
{
    assert_eq!(data_r.len(), data_i.len(), "Real and imaginary arrays must have equal length");
    let nlength = data_r.len();
    assert!(
        nlength.is_power_of_two() || MIXED_RADIX_LENGTHS.contains(&nlength),
        "FFT length must be power-of-2 (up to 512) or mixed-radix {:?}, got {}",
        MIXED_RADIX_LENGTHS,
        nlength
    );

    // Dispatch logic matching C implementation
    if nlength.is_power_of_two() {
        complex_fft_p2(data_r, data_i, fft_mode, preshift)
    } else {
        complex_fft_p3(data_r, data_i, fft_mode, preshift)
    }
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use testdata::TestFile;

    #[test]
    fn test_fft_mode_sign() {
        assert_eq!(FftMode::Forward.sign(), -1);
        assert_eq!(FftMode::Inverse.sign(), 1);
    }

    // -------------------------------------------------------------------------
    // mps_synth_calc_fft tests
    // -------------------------------------------------------------------------

    #[derive(Debug, Deserialize)]
    struct MpsSynthCalcFftTestData {
        ptr_xr_in: Vec<f32>,
        ptr_xi_in: Vec<f32>,
        ptr_xr_out: Vec<f32>,
        ptr_xi_out: Vec<f32>,
    }

    #[testdata::files]
    #[test]
    fn test_mps_synth_calc_fft(
        #[glob = "tests/fixtures/mps_synth_calc_fft_*.json"] test_data: &TestFile,
    ) {
        let test_data: MpsSynthCalcFftTestData =
            serde_json::from_slice(&test_data.raw_read())
            .expect("Failed to parse test data JSON");

        let mut ptr_xr = test_data.ptr_xr_in.clone();
        let mut ptr_xi = test_data.ptr_xi_in.clone();

        mps_synth_calc_fft(&mut ptr_xr, &mut ptr_xi);

        assert_eq!(ptr_xr.len(), test_data.ptr_xr_out.len(), "xr output length mismatch");
        assert_eq!(ptr_xi.len(), test_data.ptr_xi_out.len(), "xi output length mismatch");

        let mut failed = 0;
        for i in 0..ptr_xr.len() {
            if (ptr_xr[i] - test_data.ptr_xr_out[i]).abs() > 1e-6 {
                eprintln!("xr mismatch at index {}: {} instead of {}", i, ptr_xr[i], test_data.ptr_xr_out[i]);
                failed += 1;
                if failed > 10 { break; }
            }
        }
        for i in 0..ptr_xi.len() {
            if (ptr_xi[i] - test_data.ptr_xi_out[i]).abs() > 1e-6 {
                eprintln!("xi mismatch at index {}: {} instead of {}", i, ptr_xi[i], test_data.ptr_xi_out[i]);
                failed += 1;
                if failed > 20 { break; }
            }
        }
        if failed > 0 {
            panic!("Test failed with {} mismatches", failed);
        }
    }

    // -------------------------------------------------------------------------
    // mps_complex_fft tests
    // -------------------------------------------------------------------------

    #[derive(Debug, Deserialize)]
    struct MpsComplexFftTestData {
        xr_in: Vec<f32>,
        xi_in: Vec<f32>,
        xr_out: Vec<f32>,
        xi_out: Vec<f32>,
    }

    #[testdata::files]
    #[test]
    fn test_mps_complex_fft(
        #[glob = "tests/fixtures/mps_complex_fft_*.json"] test_data: &TestFile,
    ) {
        let test_data: MpsComplexFftTestData =
            serde_json::from_slice(&test_data.raw_read())
            .expect("Failed to parse test data JSON");

        let mut xr = test_data.xr_in.clone();
        let mut xi = test_data.xi_in.clone();

        mps_complex_fft(&mut xr, &mut xi);

        assert_eq!(xr.len(), test_data.xr_out.len(), "xr output length mismatch");
        assert_eq!(xi.len(), test_data.xi_out.len(), "xi output length mismatch");

        let mut failed = 0;
        for i in 0..xr.len() {
            if (xr[i] - test_data.xr_out[i]).abs() > 1e-6 {
                eprintln!("xr mismatch at index {}: {} instead of {}", i, xr[i], test_data.xr_out[i]);
                failed += 1;
                if failed > 10 { break; }
            }
        }
        for i in 0..xi.len() {
            if (xi[i] - test_data.xi_out[i]).abs() > 1e-6 {
                eprintln!("xi mismatch at index {}: {} instead of {}", i, xi[i], test_data.xi_out[i]);
                failed += 1;
                if failed > 20 { break; }
            }
        }
        if failed > 0 {
            panic!("Test failed with {} mismatches", failed);
        }
    }

    // -------------------------------------------------------------------------
    // complex_fft_p2 tests
    // -------------------------------------------------------------------------

    #[derive(Debug, Deserialize)]
    struct ComplexFftP2TestData {
        xr_in: Vec<i32>,
        xi_in: Vec<i32>,
        fft_mode: i32,
        preshift_in: i32,
        xr_out: Vec<i32>,
        xi_out: Vec<i32>,
        preshift_out: i32,
    }

    #[testdata::files]
    #[test]
    fn test_complex_fft_p2(
        #[glob = "tests/fixtures/complex_fft_p2_*.json"] test_data: &TestFile,
    ) {
        let test_data: ComplexFftP2TestData =
            serde_json::from_slice(&test_data.raw_read())
            .expect("Failed to parse test data JSON");

        let mut xr = test_data.xr_in.clone();
        let mut xi = test_data.xi_in.clone();
        let fft_mode = if test_data.fft_mode < 0 { FftMode::Forward } else { FftMode::Inverse };

        let preshift_out = complex_fft_p2(&mut xr, &mut xi, fft_mode, test_data.preshift_in);

        assert_eq!(preshift_out, test_data.preshift_out, "preshift output mismatch");
        assert_eq!(xr.len(), test_data.xr_out.len(), "xr output length mismatch");
        assert_eq!(xi.len(), test_data.xi_out.len(), "xi output length mismatch");

        let mut failed = 0;
        for i in 0..xr.len() {
            if xr[i] != test_data.xr_out[i] {
                eprintln!("xr mismatch at index {}: {} instead of {}", i, xr[i], test_data.xr_out[i]);
                failed += 1;
                if failed > 10 { break; }
            }
        }
        for i in 0..xi.len() {
            if xi[i] != test_data.xi_out[i] {
                eprintln!("xi mismatch at index {}: {} instead of {}", i, xi[i], test_data.xi_out[i]);
                failed += 1;
                if failed > 20 { break; }
            }
        }
        if failed > 0 {
            panic!("Test failed with {} mismatches", failed);
        }
    }

    // -------------------------------------------------------------------------
    // complex_fft_p3 tests
    // -------------------------------------------------------------------------

    #[derive(Debug, Deserialize)]
    struct ComplexFftP3TestData {
        xr_in: Vec<i32>,
        xi_in: Vec<i32>,
        fft_mode: i32,
        preshift_in: i32,
        xr_out: Vec<i32>,
        xi_out: Vec<i32>,
        preshift_out: i32,
    }

    #[testdata::files]
    #[test]
    fn test_complex_fft_p3(
        #[glob = "tests/fixtures/complex_fft_p3_*.json"] test_data: &TestFile,
    ) {
        let test_data: ComplexFftP3TestData =
            serde_json::from_slice(&test_data.raw_read())
            .expect("Failed to parse test data JSON");

        let mut xr = test_data.xr_in.clone();
        let mut xi = test_data.xi_in.clone();
        let fft_mode = if test_data.fft_mode < 0 { FftMode::Forward } else { FftMode::Inverse };

        let preshift_out = complex_fft_p3(&mut xr, &mut xi, fft_mode, test_data.preshift_in);

        assert_eq!(preshift_out, test_data.preshift_out, "preshift output mismatch");
        assert_eq!(xr.len(), test_data.xr_out.len(), "xr output length mismatch");
        assert_eq!(xi.len(), test_data.xi_out.len(), "xi output length mismatch");

        let mut failed = 0;
        for i in 0..xr.len() {
            if xr[i] != test_data.xr_out[i] {
                eprintln!("xr mismatch at index {}: {} instead of {}", i, xr[i], test_data.xr_out[i]);
                failed += 1;
                if failed > 10 { break; }
            }
        }
        for i in 0..xi.len() {
            if xi[i] != test_data.xi_out[i] {
                eprintln!("xi mismatch at index {}: {} instead of {}", i, xi[i], test_data.xi_out[i]);
                failed += 1;
                if failed > 20 { break; }
            }
        }
        if failed > 0 {
            panic!("Test failed with {} mismatches", failed);
        }
    }
}
