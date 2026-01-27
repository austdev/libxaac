// decoder::ixheaacd::imdct
//! IMDCT (Inverse Modified Discrete Cosine Transform) for USAC decoder

use super::OffsetLengths;

/// Error type for IMDCT operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImdctError {
    InvalidLength,
    InvalidWindowSequence,
    WindowCalculationFailed,
    BpfError,
}

pub type ImdctResult<T> = Result<T, ImdctError>;

/// Low-level FFT-based IMDCT computation.
///
/// Transforms spectral coefficients in-place to time-domain samples.
///
/// # C signature
/// ```c
/// VOID ixheaacd_acelp_imdct(WORD32 *imdct_in, WORD32 npoints, WORD8 *qshift, WORD32 *scratch);
/// ```
pub fn acelp_imdct(
    imdct_in: &mut [i32],  // Input/output: spectral coefficients -> time samples (in-place)
    qshift: &mut i8,       // Quantization shift factor (modified on output)
    scratch: &mut [i32],   // Scratch buffer (min 1024 elements)
) {
    let npoints = imdct_in.len() * 2;

    #[cfg(feature = "fallback")]
    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_acelp_imdct(
            imdct_in.as_mut_ptr(),
            npoints as i32,
            qshift as *mut i8,
            scratch.as_mut_ptr(),
        );
    }

    #[cfg(not(feature = "fallback"))]
    {
        let _ = (imdct_in, npoints, qshift, scratch);
        todo!("Pure Rust implementation pending")
    }
}

/// IMDCT for long window sequences.
///
/// Handles ONLY_LONG_SEQUENCE, LONG_START_SEQUENCE, STOP_START_SEQUENCE, LONG_STOP_SEQUENCE.
///
/// # C signature
/// ```c
/// IA_ERRORCODE ixheaacd_fd_imdct_long(
///     ia_usac_data_struct *usac_data, WORD32 i_ch, WORD32 *fac_idata,
///     offset_lengths *ixheaacd_drc_offset, WORD8 fac_q);
/// ```
pub fn fd_imdct_long(
    usac_data: &mut crate::gen_ixheaacd_ref::ia_usac_data_struct, // Main USAC decoder state
    i_ch: i32,             // Channel index
    fac_idata: &mut [i32], // FAC data buffer (2*FAC_LENGTH+16 = 272 elements)
    offset: &OffsetLengths,// Window offset/length parameters
    fac_q: i8,             // FAC quantization factor
) -> ImdctResult<()>
{
    #[cfg(feature = "fallback")]
    unsafe {
        let mut c_offset = offset.as_c_struct();
        let result = crate::gen_ixheaacd_ref::ixheaacd_fd_imdct_long(
            usac_data,
            i_ch,
            fac_idata.as_mut_ptr(),
            &mut c_offset,
            fac_q,
        );
        return if result == 0 { Ok(()) } else { Err(ImdctError::InvalidLength) };
    }

    #[cfg(not(feature = "fallback"))]
    {
        let _ = (usac_data, i_ch, fac_idata, offset, fac_q);
        todo!("Pure Rust implementation - see docs/claude_imdct_analysis.md")
    }
}

/// IMDCT for short window sequences (8 consecutive short windows).
///
/// Handles EIGHT_SHORT_SEQUENCE with overlap-add of 8 short transforms.
///
/// # C signature
/// ```c
/// IA_ERRORCODE ixheaacd_fd_imdct_short(
///     ia_usac_data_struct *usac_data, WORD32 i_ch, WORD32 *fac_data_out,
///     offset_lengths *ixheaacd_drc_offset, WORD8 fac_q);
/// ```
pub fn fd_imdct_short(
    usac_data: &mut crate::gen_ixheaacd_ref::ia_usac_data_struct, // Main USAC decoder state
    i_ch: i32,                // Channel index
    fac_data_out: &mut [i32], // FAC data output buffer
    offset: &OffsetLengths,   // Window offset/length parameters
    fac_q: i8,                // FAC quantization factor
) -> ImdctResult<()>
{
    #[cfg(feature = "fallback")]
    unsafe {
        let mut c_offset = offset.as_c_struct();
        let result = crate::gen_ixheaacd_ref::ixheaacd_fd_imdct_short(
            usac_data,
            i_ch,
            fac_data_out.as_mut_ptr(),
            &mut c_offset,
            fac_q,
        );
        return if result == 0 { Ok(()) } else { Err(ImdctError::InvalidLength) };
    }

    #[cfg(not(feature = "fallback"))]
    {
        let _ = (usac_data, i_ch, fac_data_out, offset, fac_q);
        todo!("Pure Rust implementation - see docs/claude_imdct_analysis.md")
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

    #[derive(Debug, Deserialize)]
    struct AcelpImdctTestData {
        imdct_in: Vec<i32>,
        qshift_in: i8,
        output: Vec<i32>,
        qshift_out: i8,
    }

    
    #[testdata::files]
    #[test]
    fn test_acelp_imdct(
        #[glob = "tests/fixtures/acelp_imdct_*.json"] test_data: &TestFile,
    ) {
        let test_data: AcelpImdctTestData =
            serde_json::from_slice(&test_data.raw_read())
            .expect("Failed to parse test data JSON");

        let mut imdct_in = test_data.imdct_in.clone();
        let mut qshift = test_data.qshift_in;
        let mut scratch = vec![0i32; 1024];

        acelp_imdct(&mut imdct_in, &mut qshift, &mut scratch);

        assert_eq!(qshift, test_data.qshift_out, "qshift output mismatch");
        assert_eq!(imdct_in.len(), test_data.output.len(), "Output length mismatch");

        let mut failed = 0;
        for i in 0..imdct_in.len() {
            if imdct_in[i] != test_data.output[i] {
                eprintln!("Mismatch at index {}: {} instead of {}", i, imdct_in[i], test_data.output[i]);
                failed += 1;
                if failed > 10 { 
                    break;
                }
            }
        }
        if failed > 0 {
            panic!();
        }
    }
}
