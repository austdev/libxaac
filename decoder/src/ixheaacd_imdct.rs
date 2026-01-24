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
    npoints: i32,          // Total points = 2 * IMDCT length (e.g., 2048 for 1024-point IMDCT)
    qshift: &mut i8,       // Quantization shift factor (modified on output)
    scratch: &mut [i32],   // Scratch buffer (min 1024 elements)
) {
    #[cfg(feature = "fallback")]
    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_acelp_imdct(
            imdct_in.as_mut_ptr(),
            npoints,
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
