// decoder::ixheaacd::lpc (Linear Predictive Coding)
//
// The file contains functions for:
//
//  1. LSF/LSP conversions - Line Spectral Frequencies/Pairs (stable LPC representation)
//  2. LPC coefficient interpolation - smooth transitions between frames
//  3. ACELP decoding - Code-Excited Linear Prediction
//  4. Bass Post-Filter - uses pitch periodicity (related to LPC residual)


// =============================================================================
// Constants from ixheaacd_cnst.h
// =============================================================================

/// Superframe length in samples
pub const LEN_SUPERFRAME: usize = 1024;

/// Single frame length
pub const LEN_FRAME: usize = 256;

/// Number of frames per superframe
pub const NUM_FRAMES: usize = 4;

/// LPC order (number of coefficients)
pub const ORDER: usize = 16;

/// Subframe length in samples
pub const LEN_SUBFR: usize = 64;

/// Total subframes per superframe
pub const NUM_SUBFR_SUPERFRAME: usize = 16;

/// Half of total subframes
pub const NUM_SUBFR_SUPERFRAME_BY2: usize = NUM_SUBFR_SUPERFRAME / 2;

/// Maximum synthesis delay
pub const SYNTH_DELAY_LMAX: usize = (NUM_SUBFR_SUPERFRAME_BY2 - 1) * LEN_SUBFR;

/// Maximum pitch lag
pub const MAX_PITCH: usize = 289; // Calculated from TMAX formula

/// Filter delay for BPF
pub const FILTER_DELAY: usize = 12;

// =============================================================================
// LPD Bass Post-Filter
// =============================================================================

/// Error type for LPC operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LpcError {
    /// Invalid pitch lag or buffer index
    InvalidPitchLag,
    /// Null pointer passed to function
    NullPointer,
    /// Buffer overflow detected
    BufferOverflow,
}

/// Result type for LPC operations
pub type LpcResult<T> = Result<T, LpcError>;

/// Applies Bass Post-Filter to synthesized audio during LPD-to-FD transition.
///
/// The Bass Post-Filter (BPF) reduces inter-harmonic noise in voiced speech
/// by exploiting pitch periodicity. This function is called when transitioning
/// from LPD (Linear Prediction Domain) coding back to frequency domain.
///
pub fn lpd_bpf_fix(
    usac_data: &crate::gen_ixheaacd_ref::ia_usac_data_struct, // The main USAC decoder state
    is_short_flag: bool, // True if previous frame used short windows
    out_buffer: &mut [f32], // Audio sample buffer (input/output)
    st: &crate::gen_ixheaacd_ref::ia_usac_lpd_decoder, // LPD decoder state
) -> LpcResult<()> 
{
    #[cfg(feature = "fallback")]
    #[allow(invalid_reference_casting)]
    let result = unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_lpd_bpf_fix(
            usac_data as *const crate::gen_ixheaacd_ref::ia_usac_data_struct as *mut crate::gen_ixheaacd_ref::ia_usac_data_struct,
            if is_short_flag { 1 } else { 0 },
            out_buffer.as_mut_ptr(),
            st as *const crate::gen_ixheaacd_ref::ia_usac_lpd_decoder as crate::gen_ixheaacd_ref::ia_usac_lpd_decoder_handle,
        )
    };

    if result == 0 {
        Ok(())
    } else {
        Err(LpcError::InvalidPitchLag)
    }
}


// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use testdata::TestFile;
    use serde::Deserialize;
    use approx::assert_relative_eq;

    // -------------------------------------------------------------------------
    // Test Data Structures (for JSON deserialization)
    // -------------------------------------------------------------------------

    #[derive(Debug, Deserialize)]
    struct UsacDataTestInput {
        ccfl: i32,
        num_subfrm: i32,
        ec_flag: i32,
    }

    #[derive(Debug, Deserialize)]
    struct LpdDataTestInput {
        mode_prev: i32,
        synth_prev: Vec<f32>,
        pitch_prev: Vec<i32>,
        gain_prev: Vec<f32>,
        bpf_prev: Vec<f32>,
    }

    #[derive(Debug, Deserialize)]
    struct LpdBpfFixTestData {
        usac_data: UsacDataTestInput,
        lpd_data: LpdDataTestInput,
        input: Vec<f32>,
        output: Vec<f32>,
    }

    // -------------------------------------------------------------------------
    // Unit Tests
    // -------------------------------------------------------------------------

    fn create_test_usac_data(input: &UsacDataTestInput) -> Box<crate::gen_ixheaacd_ref::ia_usac_data_struct> {
        unsafe {
            let mut big = Box::<crate::gen_ixheaacd_ref::ia_usac_data_struct>::new_uninit();
            let data = big.as_mut_ptr();
            std::ptr::write_bytes(data, 0, 1);
            (*data).num_subfrm = input.num_subfrm;
            (*data).ec_flag = input.ec_flag;
            (*data).ccfl = input.ccfl;
            big.assume_init()
        }
    }

    fn create_test_lpd_data(input: &LpdDataTestInput) -> Box<crate::gen_ixheaacd_ref::ia_usac_lpd_decoder> {
        let mut lpd = unsafe {
            let mut buff = Box::<crate::gen_ixheaacd_ref::ia_usac_lpd_decoder>::new_uninit();
            let raw = buff.as_mut_ptr();
            std::ptr::write_bytes(raw, 0, 1);
            buff.assume_init()
        };

        // Copy synth_prev (up to array size)
        let synth_len = input.synth_prev.len().min(lpd.synth_prev.len());
        lpd.synth_prev[..synth_len].copy_from_slice(&input.synth_prev[..synth_len]);

        // Copy pitch_prev (up to array size)
        let pitch_len = input.pitch_prev.len().min(lpd.pitch_prev.len());
        lpd.pitch_prev[..pitch_len].copy_from_slice(&input.pitch_prev[..pitch_len]);

        // Copy gain_prev (up to array size)
        let gain_len = input.gain_prev.len().min(lpd.gain_prev.len());
        lpd.gain_prev[..gain_len].copy_from_slice(&input.gain_prev[..gain_len]);

        // Copy bpf_prev (up to array size)
        let bpf_len = input.bpf_prev.len().min(lpd.bpf_prev.len());
        lpd.bpf_prev[..bpf_len].copy_from_slice(&input.bpf_prev[..bpf_len]);

        lpd.mode_prev = input.mode_prev;
        lpd
    }

    #[testdata::files] //(rebuild = "src/ixheaacd_lpc.rs")]
    #[test]
    fn test_lpd_bpf_fix(
        #[glob = "tests/fixtures/lpd_bpf_fix_*.json"] test_data: &TestFile,
    ) {
        let test_data: LpdBpfFixTestData = 
            serde_json::from_slice(&test_data.raw_read()).
            expect("Failed to parse test data JSON");

        let usac_data = create_test_usac_data(&test_data.usac_data);
        let lpd_st = create_test_lpd_data(&test_data.lpd_data);

        let mut buffer = test_data.input.clone();
        let result = lpd_bpf_fix(&usac_data, false, &mut buffer, &lpd_st);

        assert!(result.is_ok());
        assert_eq!(buffer.len(), test_data.output.len());
        for i in 0..buffer.len() {
            assert_relative_eq!(buffer[i], test_data.output[i], epsilon = 1e-6);
        }
    }
}
