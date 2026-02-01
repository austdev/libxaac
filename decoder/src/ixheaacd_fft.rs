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

}

impl FixedPointOps for i32 {
    #[inline]
    fn mult_shl(self, other: Self) -> Self {
        (((self as i64 * other as i64) >> 32) as i32) << 1
    }

}

/// Valid mixed-radix FFT lengths (3^k * 2^m)
const MIXED_RADIX_LENGTHS: [usize; 5] = [48, 96, 192, 384, 768];

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
    assert_eq!(xr.len(), xi.len(), "Real and imaginary arrays must have equal length");
    let nlength = xr.len();
    assert!(nlength <= 512, "Power-of-2 FFT supports lengths up to 512");
    assert!(nlength.is_power_of_two(), "Power-of-2 FFT requires power-of-2 length");

    let mut preshift_out = preshift;

    #[cfg(feature = "fallback")]
    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_complex_fft_p2_dec(
            xr.as_mut_ptr(),
            xi.as_mut_ptr(),
            nlength as i32,
            fft_mode.sign(),
            &mut preshift_out,
        );
    }

    #[cfg(not(feature = "fallback"))]
    {
        let _ = (xr, xi, nlength, fft_mode);
        todo!("Pure Rust implementation pending")
    }

    preshift_out
}

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
/// Supports lengths like 48, 96, 192, 384, 768 (3^k * 2^m).
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
    assert_eq!(xr.len(), xi.len(), "Real and imaginary arrays must have equal length");
    let nlength = xr.len();
    assert!(
        MIXED_RADIX_LENGTHS.contains(&nlength),
        "Mixed-radix FFT requires length in {:?}, got {}",
        MIXED_RADIX_LENGTHS,
        nlength
    );

    let mut preshift_out = preshift;

    #[cfg(feature = "fallback")]
    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_complex_fft_p3(
            xr.as_mut_ptr(),
            xi.as_mut_ptr(),
            nlength as i32,
            fft_mode.sign(),
            &mut preshift_out,
        );
    }

    #[cfg(not(feature = "fallback"))]
    {
        let _ = (xr, xi, nlength, fft_mode);
        todo!("Pure Rust implementation pending")
    }

    preshift_out
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
