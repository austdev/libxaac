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

/// Frequency scale denominator for pitch calculations
pub const FSCALE_DENOM: usize = 12800;

/// Maximum frequency scale factor
pub const FAC_FSCALE_MAX: usize = 24000;

/// Minimum pitch lag
pub const TMIN: usize = 34;

/// Maximum pitch lag base value
pub const TMAX: usize = 27 + 6 * TMIN;

/// Maximum pitch lag
pub const MAX_PITCH: usize =
    TMAX + 6 * ((FAC_FSCALE_MAX * TMIN + FSCALE_DENOM / 2) / FSCALE_DENOM - TMIN);

/// Filter delay for BPF
pub const FILTER_DELAY: usize = 12;

/// FIR low-pass filter coefficients for bass post-filter
/// Symmetric filter with FILTER_DELAY+1 = 13 taps
pub const FIR_LP_FILT: [f32; FILTER_DELAY + 1] = [
    0.088250, 0.086410, 0.081074, 0.072768, 0.062294, 0.050623, 0.038774,
    0.027692, 0.018130, 0.010578, 0.005221, 0.001946, 0.000385
];

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

#[cfg(not(feature = "legacy-build"))]
/// Applies bass post-filter to reduce inter-harmonic noise in voiced speech.
///
/// The filter exploits pitch periodicity by computing a noise signal from the
/// difference between current samples and pitch-delayed samples, then applies
/// a low-pass FIR filter to smooth the noise signal before subtracting it.
///
fn bass_post_filter(
    synth_buf: &[f32],     // full synthesis buffer (indexed with synth_base as base)
    synth_base: usize,     // base index (synth = synth_buf + synth_base)
    pitch: &[i32],         // pitch lag values per subframe
    pitch_gain: &mut [f32],// pitch gain values (clamped in place)
    synth_out: &mut [f32], // output buffer for filtered signal
    len_fr: usize,         // frame length
    len2: usize,           // secondary length parameter for lookahead
    bpf_prev: &mut [f32],  // BPF state buffer (FILTER_DELAY + LEN_SUBFR elements)
    ec_flag: bool,         // error concealment mode
) -> LpcResult<()>
{
    // synth_buf[synth_base] corresponds to synth[0] in C code
    // Access synth_buf[synth_base + offset] for synth[offset]

    // Copy input to output: synth_out[..len_fr] = synth[−LEN_SUBFR..len_fr−LEN_SUBFR]
    let copy_start = synth_base - LEN_SUBFR;
    synth_out[..len_fr].copy_from_slice(&synth_buf[copy_start..copy_start + len_fr]);

    // Zero-pad if len_fr is not multiple of 64
    if len_fr % 64 != 0 {
        let pad_len = LEN_SUBFR - (len_fr % 64);
        synth_out[len_fr..len_fr + pad_len].fill(0.0);
    }

    let mut noise_buf = vec![0.0f32; FILTER_DELAY + 2 * LEN_SUBFR];

    let mut sf = 0usize;
    let mut num_subfr = 0usize;

    while num_subfr < len_fr {
        let mut pitch_lag = pitch[sf] as usize;
        let mut gain = pitch_gain[sf];

        // Validate pitch_lag: ((pitch_lag >> 1) + 96 - num_subfr) > MAX_PITCH
        let check_val = (pitch_lag / 2 + 96).saturating_sub(num_subfr);
        if check_val > MAX_PITCH {
            if ec_flag {
                pitch_lag = (MAX_PITCH + num_subfr).saturating_sub(96) * 2;
            } else {
                return Err(LpcError::InvalidPitchLag);
            }
        }

        // Clamp gain to [0, 1]
        gain = gain.clamp(0.0, 1.0);

        // Calculate normalized correlation
        // In C: x = &synth_sig[num_subfr - 96]; y = &synth_sig[num_subfr - pitch_lag/2 - 96];
        // With synth_sig = synth (at synth_base), these become:
        // x_idx = synth_base + num_subfr - 96
        // y_idx = synth_base + num_subfr - pitch_lag/2 - 96
        let x_base = synth_base + num_subfr - 96;
        let y_base = synth_base + num_subfr - pitch_lag / 2 - 96;

        let (mut x_energy, mut xy_corr, mut y_energy) = (0.01f32, 0.01f32, 0.01f32);
        for i in 0..(LEN_SUBFR + 96) {
            let x_val = synth_buf[x_base + i];
            let y_val = synth_buf[y_base + i];
            x_energy += x_val * x_val;
            xy_corr += x_val * y_val;
            y_energy += y_val * y_val;
        }

        let norm_corr = xy_corr / (x_energy * y_energy).sqrt();

        // Halve pitch_lag if high correlation
        if norm_corr > 0.95 {
            pitch_lag /= 2;
        }

        // Calculate lookahead length: lg = len_fr + len2 - pitch_lag - num_subfr
        let mut lg = (len_fr + len2).saturating_sub(pitch_lag + num_subfr);
        lg = lg.min(LEN_SUBFR);

        // Validate pitch_lag again
        if pitch_lag > MAX_PITCH {
            if ec_flag {
                pitch_lag = MAX_PITCH;
            } else {
                return Err(LpcError::InvalidPitchLag);
            }
        }

        // Compute noise signal
        if gain > 0.0 {
            // Adjust gain based on energy ratio
            if lg > 0 {
                let sig_base = synth_base + num_subfr;
                let mut tmp = 0.01f32;
                for i in 0..lg {
                    let val = synth_buf[sig_base + i];
                    tmp += val * val;
                }
                let mut energy = 0.01f32;
                for i in 0..lg {
                    let val = synth_buf[sig_base + i + pitch_lag];
                    energy += val * val;
                }
                let energy_ratio = (tmp / energy).sqrt();
                if energy_ratio < gain {
                    gain = energy_ratio;
                }
            }

            let alpha = 0.5 * gain;
            let sig_base = synth_base + num_subfr;

            // Three-tap comb filter for lg samples (with lookahead)
            for i in 0..lg {
                noise_buf[FILTER_DELAY + LEN_SUBFR + i] = alpha * (
                    synth_buf[sig_base + i] -
                    0.5 * synth_buf[sig_base + i - pitch_lag] -
                    0.5 * synth_buf[sig_base + i + pitch_lag]
                );
            }
            // Two-tap comb filter for remaining samples (no lookahead)
            for i in lg..LEN_SUBFR {
                noise_buf[FILTER_DELAY + LEN_SUBFR + i] = alpha * (
                    synth_buf[sig_base + i] -
                    synth_buf[sig_base + i - pitch_lag]
                );
            }
        } else {
            // Zero noise signal when gain is zero
            noise_buf[FILTER_DELAY + LEN_SUBFR .. FILTER_DELAY + LEN_SUBFR + LEN_SUBFR].fill(0.0);
        }

        // Copy previous state into noise_buf start
        noise_buf[..FILTER_DELAY + LEN_SUBFR].copy_from_slice(bpf_prev);
        // Update bpf_prev with current noise data for next iteration
        bpf_prev.copy_from_slice(&noise_buf[LEN_SUBFR .. LEN_SUBFR + FILTER_DELAY + LEN_SUBFR]);

        // Apply symmetric FIR low-pass filter and subtract from output
        for i in 0..LEN_SUBFR {
            let mut tmp = FIR_LP_FILT[0] * noise_buf[FILTER_DELAY + i];
            for j in 1..=FILTER_DELAY {
                tmp += FIR_LP_FILT[j] * (noise_buf[FILTER_DELAY + i - j] + noise_buf[FILTER_DELAY + i + j]);
            }
            synth_out[num_subfr + i] -= tmp;
        }

        num_subfr += LEN_SUBFR;
        sf += 1;
    }

    Ok(())
}

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
    st: &mut crate::gen_ixheaacd_ref::ia_usac_lpd_decoder, // LPD decoder state
) -> LpcResult<()>
{
    #[cfg(feature = "legacy-build")]
    #[allow(invalid_reference_casting)]
    unsafe {
        let result = crate::gen_ixheaacd_ref::ixheaacd_lpd_bpf_fix(
            usac_data as *const crate::gen_ixheaacd_ref::ia_usac_data_struct as *mut crate::gen_ixheaacd_ref::ia_usac_data_struct,
            if is_short_flag { 1 } else { 0 },
            out_buffer.as_mut_ptr(),
            st as crate::gen_ixheaacd_ref::ia_usac_lpd_decoder_handle,
        );
        return if result == 0 { Ok(()) } else { Err(LpcError::InvalidPitchLag) };
    };

    #[cfg(not(feature = "legacy-build"))]
    {
        // Calculate derived values
        let len_fr = usac_data.ccfl as usize;
        let lpd_sbf_len = (NUM_FRAMES * usac_data.num_subfrm as usize) / 2;
        let num_subfr_by2 = lpd_sbf_len - 1;
        let synth_delay = num_subfr_by2 * LEN_SUBFR;
        let ec_flag = usac_data.ec_flag != 0;

        // Initialize synthesis buffer
        let mut synth_buf = vec![0.0f32; MAX_PITCH + SYNTH_DELAY_LMAX + LEN_SUPERFRAME];

        // Copy st.synth_prev to beginning of synth_buf
        let synth_prev_len = MAX_PITCH + synth_delay;
        synth_buf[..synth_prev_len].copy_from_slice(&st.synth_prev[..synth_prev_len]);

        // Copy out_buffer into synth_buf starting at MAX_PITCH - LEN_SUBFR
        let copy_start = MAX_PITCH - LEN_SUBFR;
        let copy_len = synth_delay + len_fr + LEN_SUBFR;
        synth_buf[copy_start..copy_start + copy_len].copy_from_slice(&out_buffer[..copy_len]);

        // Initialize pitch and pitch_gain arrays
        let mut pitch = [64i32; NUM_SUBFR_SUPERFRAME_BY2 + 3];
        let mut pitch_gain = [0.0f32; NUM_SUBFR_SUPERFRAME_BY2 + 3];

        // Copy previous pitch/gain values
        for i in 0..num_subfr_by2 {
            pitch[i] = st.pitch_prev[i];
            pitch_gain[i] = st.gain_prev[i];
        }

        // Handle mode_prev == 0 case (extend pitch values)
        if st.mode_prev == 0 {
            pitch[num_subfr_by2] = pitch[num_subfr_by2 - 1];
            pitch_gain[num_subfr_by2] = pitch_gain[num_subfr_by2 - 1];
            if !is_short_flag {
                pitch[num_subfr_by2 + 1] = pitch[num_subfr_by2];
                pitch_gain[num_subfr_by2 + 1] = pitch_gain[num_subfr_by2];
            }
        }

        // Recalculate pitch gains using correlation
        for i in 0..(num_subfr_by2 + 2) {
            let mut tp = pitch[i] as usize;
            let i_subfr = i * LEN_SUBFR;

            // Validate pitch lag
            // C code: if ((i * LEN_SUBFR + MAX_PITCH) < tp)
            if i_subfr + MAX_PITCH < tp {
                if !ec_flag {
                    return Err(LpcError::InvalidPitchLag);
                }
                tp = MAX_PITCH.saturating_sub(i_subfr);
            // C code checks for buffer overflow conditions
            } else if (i_subfr + MAX_PITCH).saturating_sub(tp) >= 1883 ||
                      i_subfr + LEN_SUBFR > LEN_SUPERFRAME ||
                      (i_subfr + LEN_SUBFR).saturating_sub(tp) > LEN_SUPERFRAME {
                if !ec_flag {
                    return Err(LpcError::InvalidPitchLag);
                }
                tp = i_subfr + MAX_PITCH - 1882;
            }

            if pitch_gain[i] > 0.0 {
                let (mut synth_corr, mut synth_energy) = (0.0f32, 1e-6f32);
                for k in 0..LEN_SUBFR {
                    // synth points to synth_buf + MAX_PITCH
                    let delayed = synth_buf[MAX_PITCH + i_subfr - tp + k];
                    synth_corr += synth_buf[MAX_PITCH + i_subfr + k] * delayed;
                    synth_energy += delayed * delayed;
                }
                pitch_gain[i] = synth_corr / synth_energy;
            }
        }

        // Call bass_post_filter
        // synth points to synth_buf + MAX_PITCH, so synth_base = MAX_PITCH

        let mut signal_out = vec![0.0f32; LEN_SUPERFRAME];

        bass_post_filter(
            &synth_buf,
            MAX_PITCH,  // synth_base: synth = synth_buf + MAX_PITCH
            &pitch,
            &mut pitch_gain,
            &mut signal_out,
            (lpd_sbf_len + 3) * LEN_SUBFR,
            len_fr.saturating_sub((lpd_sbf_len + 4) * LEN_SUBFR),
            &mut st.bpf_prev,
            ec_flag,
        )?;

        // Copy result back to out_buffer
        let output_len = (lpd_sbf_len + 2) * LEN_SUBFR + LEN_SUBFR;
        out_buffer[..output_len].copy_from_slice(&signal_out[..output_len]);

        Ok(())
    }
}


// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use approx::relative_ne;
    use testdata::TestFile;

    #[test]
    fn test_constants() {
        assert_eq!(LEN_SUPERFRAME, crate::gen_ixheaacd_ref::LEN_SUPERFRAME as usize);
        assert_eq!(LEN_SUBFR, crate::gen_ixheaacd_ref::LEN_SUBFR as usize);
        assert_eq!(LEN_FRAME, crate::gen_ixheaacd_ref::LEN_FRAME as usize);
        assert_eq!(NUM_FRAMES, crate::gen_ixheaacd_ref::NUM_FRAMES as usize);
        assert_eq!(NUM_SUBFR_SUPERFRAME, crate::gen_ixheaacd_ref::NUM_SUBFR_SUPERFRAME as usize);
        assert_eq!(NUM_SUBFR_SUPERFRAME_BY2, crate::gen_ixheaacd_ref::NUM_SUBFR_SUPERFRAME_BY2 as usize);
        assert_eq!(SYNTH_DELAY_LMAX, (NUM_SUBFR_SUPERFRAME_BY2 - 1) * LEN_SUBFR);
        assert_eq!(ORDER, crate::gen_ixheaacd_ref::ORDER as usize);
        assert_eq!(FSCALE_DENOM, crate::gen_ixheaacd_ref::FSCALE_DENOM as usize);
        assert_eq!(FAC_FSCALE_MAX, crate::gen_ixheaacd_ref::FAC_FSCALE_MAX as usize);
        assert_eq!(TMIN, crate::gen_ixheaacd_ref::TMIN as usize);
        assert_eq!(TMAX, crate::gen_ixheaacd_ref::TMAX as usize);
        assert_eq!(MAX_PITCH, crate::gen_ixheaacd_ref::MAX_PITCH as usize);
        assert_eq!(FILTER_DELAY, crate::gen_ixheaacd_ref::FILTER_DELAY as usize);
    }

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
        let mut lpd_st = create_test_lpd_data(&test_data.lpd_data);

        let mut buffer = test_data.input.clone();
        let result = lpd_bpf_fix(&usac_data, false, &mut buffer, &mut lpd_st);

        assert!(result.is_ok());
        assert_eq!(buffer.len(), test_data.output.len());

        let mut failed = 0;
        for i in 0..buffer.len() {
            if relative_ne!(buffer[i], test_data.output[i], epsilon = 1e-6) {
                eprintln!("Mismatch at index {}: {:.6} instead of {:.6}", i, buffer[i], test_data.output[i]);
                failed += 1;
                if failed > 10 {
                    break;
                }
            }
            if failed > 0 {
                panic!();
            }
        }
    }

}
