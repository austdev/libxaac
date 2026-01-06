# xHE-AAC Decoder Architecture Diagram

## Overview
This document provides a medium-detail architectural diagram of the xHE-AAC decoder, structured to minimize connections while preserving all architectural details for Rust migration.

---

## High-Level Decoder Pipeline

```
Input Bitstream
      ↓
Container/Format Parsing (ADTS/LATM/Raw)
      ↓
Header Decoding & Configuration
      ↓
Bitstream Parsing & Element Decoding
      ↓
Core AAC Decoding (Spectral → Time Domain)
      ↓
Enhancement Modules (SBR/PS/MPS)
      ↓
DRC & Post-Processing
      ↓
Output PCM Audio
```

---

## Layer 1: Input & Container Parsing Layer

### Bitstream Input & Format Detection
**Components:** `ixheaacd_bitbuffer.h`, `ixheaacd_adts.h`, `ixheaacd_latmdemux.h`

```
┌─────────────────────────────────────────────────────────────────┐
│                    BITSTREAM INPUT                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌─────────────────┐  ┌─────────────────┐  ┌────────────────┐  │
│  │  ADTS Format    │  │  LATM/LOAS      │  │  Raw AAC       │  │
│  │  Parser         │  │  Parser         │  │  Stream        │  │
│  ├─────────────────┤  ├─────────────────┤  ├────────────────┤  │
│  │ • Sync word     │  │ • Audio Mux     │  │ • Direct       │  │
│  │   detection     │  │ • Stream Mux    │  │   bitstream    │  │
│  │ • Frame header  │  │ • Multi-layer   │  │   access       │  │
│  │ • CRC validation│  │ • Config blocks │  │                │  │
│  │ • Frame size    │  │ • Sub-frames    │  │                │  │
│  └─────────────────┘  └─────────────────┘  └────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │           Bit Buffer Management                            │  │
│  │  • Buffering, bit reading, byte alignment                 │  │
│  │  • Support for various bit widths                         │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

**Key Structures:**
- `ia_bit_buf_struct` - Bitstream buffer state
- `ia_adts_header_struct` - ADTS header data
- `ixheaacd_latm_struct` - LATM/LOAS configuration

---

## Layer 2: Header Decoding & Configuration

### Audio Specific Configuration
**Components:** `ixheaacd_headerdecode.h`, `ixheaacd_config.h`

```
┌─────────────────────────────────────────────────────────────────┐
│              HEADER DECODE & CONFIGURATION                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │  Audio Specific Config (ASC) Parser                      │    │
│  │  • Audio Object Type (AAC-LC, HE-AAC, xHE-AAC, etc.)   │    │
│  │  • Sampling frequency                                    │    │
│  │  • Channel configuration                                 │    │
│  │  • Extension flags (SBR, PS, MPS)                       │    │
│  └─────────────────────────────────────────────────────────┘    │
│                                                                   │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │  Program Config Element (PCE)                            │    │
│  │  • Front/Side/Back channels                             │    │
│  │  • LFE channels                                          │    │
│  │  • Coupled channels                                      │    │
│  │  • Element tags and routing                             │    │
│  └─────────────────────────────────────────────────────────┘    │
│                                                                   │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │  USAC Configuration (for xHE-AAC)                        │    │
│  │  • Core/Extension config                                │    │
│  │  • ACELP parameters                                      │    │
│  │  • Enhanced SBR settings                                 │    │
│  └─────────────────────────────────────────────────────────┘    │
│                                                                   │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │  ELD Configuration (Enhanced Low Delay)                  │    │
│  │  • Frame size (480/512)                                  │    │
│  │  • Low delay settings                                    │    │
│  └─────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────┘
```

**Key Structures:**
- `ia_audio_specific_config_struct`
- `ia_program_config_struct`
- `ia_usac_config_struct`
- `ia_eld_specific_config_struct`

---


## Layer 3: Core AAC Decoder - Bitstream Parsing

### Element Stream Parsing
**Components:** `ixheaacd_channel.h`, `ixheaacd_block.h`, `ixheaacd_channelinfo.h`

```
┌─────────────────────────────────────────────────────────────────┐
│                ELEMENT STREAM PARSING                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Channel Elements (ID detection and routing):                    │
│                                                                   │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌──────────┐  │
│  │  ID_SCE    │  │  ID_CPE    │  │  ID_LFE    │  │ ID_CCE   │  │
│  │  Single    │  │  Channel   │  │  Low Freq  │  │ Coupling │  │
│  │  Channel   │  │  Pair      │  │  Effects   │  │ Channel  │  │
│  │  Element   │  │  Element   │  │  Element   │  │ Element  │  │
│  └────────────┘  └────────────┘  └────────────┘  └──────────┘  │
│                                                                   │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐                 │
│  │  ID_DSE    │  │  ID_PCE    │  │  ID_FIL    │                 │
│  │  Data      │  │  Program   │  │  Fill      │                 │
│  │  Stream    │  │  Config    │  │  Element   │                 │
│  │  Element   │  │  Element   │  │  (Extension│                 │
│  └────────────┘  └────────────┘  │   Payload) │                 │
│                                   └────────────┘                 │
│                                                                   │
│  Per-Channel Decoding Pipeline:                                  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 1. Individual Channel Info (ICS)                          │  │
│  │    • ics_info: Window sequence, grouping                  │  │
│  │    • Section data                                         │  │
│  │    • Scale factors                                        │  │
│  │    • Spectral data                                        │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 2. Window Sequence Types                                  │  │
│  │    • ONLY_LONG_SEQUENCE (1024/960 samples)               │  │
│  │    • LONG_START_SEQUENCE (transition)                     │  │
│  │    • EIGHT_SHORT_SEQUENCE (128 samples × 8)              │  │
│  │    • LONG_STOP_SEQUENCE (transition)                      │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

**Key Structures:**
- `ia_aac_dec_channel_info_struct` - Channel decoding state
- `ia_ics_info_struct` - Individual channel stream info

---

## Layer 4: Core AAC Decoder - Spectral Decoding

### Huffman Decoding & Dequantization
**Components:** `ixheaacd_block.h`, `ixheaacd_spectrum_dec.c`, `ixheaacd_hufftables.c`

```
┌─────────────────────────────────────────────────────────────────┐
│               SPECTRAL DATA DECODING                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Section Data Parsing                                       │  │
│  │  • Codebook selection per section                         │  │
│  │  • Scale factor bands (SFB) grouping                      │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Scale Factor Decoding                                      │  │
│  │  • DPCM decoding of scale factors                         │  │
│  │  • Intensity stereo parameters                            │  │
│  │  • PNS (Perceptual Noise Substitution) flags             │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Huffman Decoding                                           │  │
│  │  • 11 spectral codebooks (CB 1-11)                        │  │
│  │  • ESC codebook handling (large values)                   │  │
│  │  • Sign bit processing                                     │  │
│  │  • Zero/Noise codebooks                                    │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Inverse Quantization                                       │  │
│  │  • x^(4/3) computation via lookup tables                  │  │
│  │  • Scale factor application                               │  │
│  │  Formula: X_rescaled = sign * |X|^(4/3) * 2^(scale/4)    │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Layer 5: Spectral Processing Tools

### Spectral Domain Processing
**Components:** `ixheaacd_pns.h`, `ixheaacd_tns.h`, `ixheaacd_stereo.h`, `ixheaacd_lt_predict.h`

```
┌─────────────────────────────────────────────────────────────────┐
│            SPECTRAL PROCESSING & PREDICTION                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ PNS - Perceptual Noise Substitution                        │  │
│  │  • Replace noise-like spectral data with noise generator  │  │
│  │  • Save bits for perceptually irrelevant components       │  │
│  │  • Random vector generation with proper scaling           │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ TNS - Temporal Noise Shaping                               │  │
│  │  • Apply prediction filter in frequency domain            │  │
│  │  • Control temporal envelope of quantization noise        │  │
│  │  • AR filter per window/group                             │  │
│  │  • Up to 20 coefficients                                  │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ M/S Stereo (Mid/Side) Processing                           │  │
│  │  • Joint stereo coding                                     │  │
│  │  • MS mask per scale factor band                          │  │
│  │  • Convert M/S to L/R: L=M+S, R=M-S                       │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Intensity Stereo                                           │  │
│  │  • Share spectral data between channels                   │  │
│  │  • Reconstruct from intensity parameters                  │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ LTP - Long Term Prediction (AAC Main profile)             │  │
│  │  • Predict from previous frame                            │  │
│  │  • Time-domain prediction in frequency domain             │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

**Key Structures:**
- `ia_pns_correlation_info_struct`
- `ia_pns_info_struct`
- `ia_aac_dec_tns_info` (TNS coefficients and order)

---


## Layer 6: Time Domain Conversion (IMDCT)

### Inverse Modified Discrete Cosine Transform
**Components:** `ixheaacd_aac_imdct.h`, `ixheaacd_imdct.c`, `ixheaacd_Windowing.c`

```
┌─────────────────────────────────────────────────────────────────┐
│                   IMDCT & WINDOWING                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Spectral Domain (Frequency) → Time Domain Conversion            │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Pre-Processing                                             │  │
│  │  • Pre-twiddle computation                                │  │
│  │  • Spectral rearrangement                                 │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ FFT-based IMDCT                                            │  │
│  │  • N-point IMDCT via N/4-point complex FFT                │  │
│  │  • Supported sizes: 128, 256, 512, 1024, 2048             │  │
│  │  • Special sizes: 480, 960 (for framesize_480)            │  │
│  │  • LD/ELD optimized paths                                 │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Post-Processing                                            │  │
│  │  • Post-twiddle computation                               │  │
│  │  • FFT output rearrangement                               │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Windowing Functions                                        │  │
│  │  • Sine window (standard AAC)                             │  │
│  │  • KBD window (Kaiser-Bessel derived)                     │  │
│  │  • Low-overlap window (LD/ELD)                            │  │
│  │  • Window switching handling                              │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Overlap-Add                                                │  │
│  │  • Combine with previous frame overlap buffer             │  │
│  │  • 50% overlap (512 samples for 1024 frame)               │  │
│  │  • Store current frame's second half for next frame       │  │
│  │  • Handle window transitions (long↔short)                 │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  Window Transition Handling:                                     │
│   • LONG → LONG: Standard overlap-add                           │
│   • LONG → SHORT: Long-start window                             │
│   • SHORT → LONG: Long-stop window                              │
│   • SHORT → SHORT: 8 short windows per frame                    │
└─────────────────────────────────────────────────────────────────┘
```

**Key Functions:**
- `ixheaacd_imdct_process()` - Main IMDCT entry point
- `ixheaacd_inverse_transform()` - Core transform
- `ixheaacd_over_lap_add1/2()` - Overlap-add processing

---

## Layer 7: USAC/xHE-AAC Specific Modules

### ACELP (Speech Coding) & TCX (Transform Coding)
**Components:** `ixheaacd_acelp_*.c`, `ixheaacd_tcx_*.c`, `ixheaacd_lpd_*.c`

```
┌─────────────────────────────────────────────────────────────────┐
│                USAC CORE CODER (xHE-AAC)                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  USAC supports dual coding modes per frame/channel:              │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ FD - Frequency Domain (AAC-like)                           │  │
│  │  • Standard MDCT-based coding                             │  │
│  │  • Same as AAC-LC pipeline                                │  │
│  │  • Codebooks 0-11                                         │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ LPD - Linear Prediction Domain (ACELP/TCX)                │  │
│  │                                                            │  │
│  │  ┌────────────────────────────────────────────────────┐   │  │
│  │  │ ACELP - Algebraic CELP (Speech coding)             │   │  │
│  │  │  • LPC analysis and synthesis                      │   │  │
│  │  │  • Pitch prediction                                │   │  │
│  │  │  • Algebraic codebook excitation                   │   │  │
│  │  │  • Adaptive and fixed codebooks                    │   │  │
│  │  │  • Frame sizes: 64, 128, 192, 256 samples          │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  │                                                            │  │
│  │  ┌────────────────────────────────────────────────────┐   │  │
│  │  │ TCX - Transform Coded Excitation                   │   │  │
│  │  │  • MDCT of LPC residual                            │   │  │
│  │  │  • Variable length: 256, 512, 1024 samples         │   │  │
│  │  │  • AVQ (Algebraic Vector Quantization)             │   │  │
│  │  │  • Arithmetic coding                               │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  │                                                            │  │
│  │  ┌────────────────────────────────────────────────────┐   │  │
│  │  │ FAC - Forward Aliasing Cancellation                │   │  │
│  │  │  • Transition between FD and LPD                   │   │  │
│  │  │  • Remove aliasing artifacts                       │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Arithmetic Decoder (for TCX)                               │  │
│  │  • Context-based arithmetic decoding                      │  │
│  │  • Cumulative frequency tables                            │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

**Key Structures:**
- `ia_usac_data_struct` - USAC decoder state
- `ia_usac_td_config_handle` - Time domain config
- `ia_td_frame_data_struct` - Frame data

---


## Layer 8: SBR - Spectral Band Replication

### High Frequency Reconstruction
**Components:** `ixheaacd_sbrdecoder.h`, `ixheaacd_sbr_dec.h`, `ixheaacd_env_extr.h`, `ixheaacd_qmf_dec.h`

```
┌─────────────────────────────────────────────────────────────────┐
│                  SBR DECODER (HE-AAC/HE-AACv2)                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Input: Core AAC time-domain samples (low frequency only)        │
│  Output: Full bandwidth audio (up to 48 kHz)                     │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 1. QMF Analysis Bank (64 or 32 bands)                     │  │
│  │    • Convert time-domain to QMF domain                    │  │
│  │    • 64-band analysis filterbank                          │  │
│  │    • Complex QMF samples                                  │  │
│  │    • 32 or 64 time slots per frame                        │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 2. SBR Bitstream Parsing                                   │  │
│  │    ┌───────────────────────────────────────────────────┐  │  │
│  │    │ Header Data (infrequent)                          │  │  │
│  │    │  • Frequency range                                │  │  │
│  │    │  • Limiter bands                                  │  │  │
│  │    │  • Noise bands                                    │  │  │
│  │    └───────────────────────────────────────────────────┘  │  │
│  │    ┌───────────────────────────────────────────────────┐  │  │
│  │    │ Frame Data (every frame)                          │  │  │
│  │    │  • Envelope scalefactors                          │  │  │
│  │    │  • Noise floor scalefactors                       │  │  │
│  │    │  • Time/frequency grid                            │  │  │
│  │    │  • Inverse filtering levels                       │  │  │
│  │    │  • Additional harmonics flags                     │  │  │
│  │    └───────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 3. High Frequency Generation (HFG)                         │  │
│  │    • Transposer: Copy low freq to high freq              │  │
│  │    • Patching algorithm                                   │  │
│  │    • LPP (Low Power Profile) transposition               │  │
│  │    • Harmonic transposition                               │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 4. Envelope Adjustment                                     │  │
│  │    • Apply envelope scalefactors                          │  │
│  │    • Smooth envelope over time/frequency                  │  │
│  │    • Interpolation between envelopes                      │  │
│  │    • Limiter processing (prevent overshooting)            │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 5. Noise Floor Addition                                    │  │
│  │    • Add noise in specified bands                         │  │
│  │    • Controlled by noise floor parameters                 │  │
│  │    • Perceptually shaped noise                            │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 6. Additional Sinusoids                                    │  │
│  │    • Add tonal components (if signaled)                   │  │
│  │    • Improve tonal quality                                │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 7. QMF Synthesis Bank                                      │  │
│  │    • Convert QMF domain back to time domain               │  │
│  │    • 64-band synthesis filterbank                         │  │
│  │    • Produces full-bandwidth output                       │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  Enhanced SBR (ESBR) Extensions:                                 │
│  • Higher quality HFG algorithms                                 │
│  • Improved patching                                             │
│  • Inter-TES (Inter-channel Temporal Envelope Shaping)          │
│  • PVC (Predictive Vector Coding)                                │
└─────────────────────────────────────────────────────────────────┘
```

**Key Structures:**
- `ia_sbr_dec_struct` - SBR decoder instance
- `ia_sbr_header_data_struct` - SBR configuration
- `ia_sbr_frame_info_data_struct` - Per-frame SBR data
- `ia_qmf_dec_data_struct` - QMF filterbank state

**Key Components:**
- `ixheaacd_env_extr.c` - Envelope extraction
- `ixheaacd_env_calc.c` - Envelope calculation
- `ixheaacd_lpp_tran.c` - LPP transposer
- `ixheaacd_qmf_dec.c` - QMF filterbank
- `ixheaacd_esbr_*.c` - Enhanced SBR

---

## Layer 9: PS - Parametric Stereo

### Stereo Reconstruction from Mono
**Components:** `ixheaacd_ps_dec.h`, `ixheaacd_hybrid.h`

```
┌─────────────────────────────────────────────────────────────────┐
│              PS DECODER (HE-AACv2)                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Input: Mono QMF domain signal + PS parameters                   │
│  Output: Stereo QMF domain signal                                │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 1. Hybrid Analysis (for low frequencies)                   │  │
│  │    • Further split low QMF bands into sub-bands           │  │
│  │    • 8 or 12 hybrid bands from first 3 QMF bands          │  │
│  │    • Improved frequency resolution                         │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 2. PS Parameter Parsing                                    │  │
│  │    • IID - Inter-channel Intensity Difference             │  │
│  │    • ICC - Inter-channel Coherence                        │  │
│  │    • IPD - Inter-channel Phase Difference                 │  │
│  │    • OPD - Overall Phase Difference                       │  │
│  │    • 20/34 frequency bands                                │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 3. Stereo Processing                                       │  │
│  │    • Apply mixing matrices per band                       │  │
│  │    • Decorrelation of coherence                           │  │
│  │    • Phase rotation                                        │  │
│  │    • Generate L/R from mono + parameters                  │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 4. Hybrid Synthesis (low frequencies)                      │  │
│  │    • Recombine hybrid bands back to QMF bands             │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  Output: Stereo QMF signal ready for QMF synthesis               │
└─────────────────────────────────────────────────────────────────┘
```

**Key Structures:**
- `ia_ps_dec_struct` - PS decoder state
- `ia_hybrid_struct` - Hybrid filterbank state

---


## Layer 10: MPS - MPEG Surround (Spatial Audio)

### Multi-channel Audio Reconstruction
**Components:** `ixheaacd_mps_dec.h`, `ixheaacd_ld_mps_dec.h`

```
┌─────────────────────────────────────────────────────────────────┐
│              MPS DECODER (Multi-channel)                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Input: Downmix channels + spatial parameters                    │
│  Output: Multi-channel audio (5.1, 7.1, etc.)                   │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 1. Tree Configuration                                      │  │
│  │    • Tree 5151: Standard 5.1 config                       │  │
│  │    • Tree 5152: Alternative 5.1                           │  │
│  │    • Tree 525: 5.2.5 config                               │  │
│  │    • Tree 7271/7272: 7.2 configs                          │  │
│  │    • Tree 7571/7572: 7.5 configs                          │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 2. QMF/Hybrid Analysis (if not already in QMF domain)     │  │
│  │    • Convert downmix to QMF domain                        │  │
│  │    • Hybrid filterbank for low frequencies                │  │
│  │    • 71 parameter bands                                   │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 3. Spatial Parameter Parsing                               │  │
│  │    • CLD - Channel Level Difference                       │  │
│  │    • ICC - Inter-Channel Coherence                        │  │
│  │    • CPC - Channel Prediction Coefficients                │  │
│  │    • Time/frequency grid                                  │  │
│  │    • Smooth/fine coding                                   │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 4. Parameter Processing                                    │  │
│  │    • Smooth parameters over time                          │  │
│  │    • Interpolate between updates                          │  │
│  │    • Convert to M1/M2 matrices                            │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 5. Decorrelation                                           │  │
│  │    • Generate decorrelated signals                        │  │
│  │    • Allpass-based decorrelators                          │  │
│  │    • Lattice filters                                       │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 6. Multi-channel Reconstruction (M1/M2 Application)        │  │
│  │    • Apply M1 matrix (pre-decorrelation mixing)           │  │
│  │    • Apply M2 matrix (post-decorrelation mixing)          │  │
│  │    • Tree-based processing                                │  │
│  │    • Per-band matrix application                          │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 7. Residual Coding (if present)                            │  │
│  │    • Decode residual signals                              │  │
│  │    • Add to reconstructed channels                        │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 8. Temporal Processing                                     │  │
│  │    • Temporal envelope shaping                            │  │
│  │    • Transient handling                                    │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 9. QMF/Hybrid Synthesis                                    │  │
│  │    • Convert back to time domain                          │  │
│  │    • Per-channel synthesis                                │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  Variants:                                                        │
│  • Standard MPS: QMF-based processing                            │
│  • LD-MPS: Low-delay version for ELD-AAC                         │
│  • HEAAC-MPS: Integration with SBR                               │
└─────────────────────────────────────────────────────────────────┘
```

**Key Structures:**
- `ia_mps_dec_state_struct` - MPS decoder state
- `ia_heaac_mps_state_struct` - HE-AAC+MPS state
- `ia_mps_spatial_bs_frame_struct` - Spatial bitstream frame

**Key Components:**
- `ixheaacd_mps_parse.c` - Parameter parsing
- `ixheaacd_mps_decorr.c` - Decorrelation
- `ixheaacd_mps_apply_m1.c` / `ixheaacd_mps_apply_m2.c` - Matrix application
- `ixheaacd_mps_calc_m1m2_*.c` - M1/M2 calculation (tree-specific)
- `ixheaacd_mps_temp_process.c` - Temporal processing

---

## Layer 11: DRC - Dynamic Range Control

### Loudness Management & Dynamic Range Compression
**Components:** `decoder/drc_src/*`, `ixheaacd_drc_dec.h`

```
┌─────────────────────────────────────────────────────────────────┐
│              DRC / LOUDNESS CONTROL                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Two DRC systems supported:                                       │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ A. Legacy DRC (MPEG-2/4 AAC)                               │  │
│  │                                                             │  │
│  │    ┌────────────────────────────────────────────────────┐  │  │
│  │    │ 1. DRC Parameters                                  │  │  │
│  │    │    • prog_ref_level                                │  │  │
│  │    │    • dyn_rng_sgn / dyn_rng_ctl                     │  │  │
│  │    │    • Per-channel or program level                  │  │  │
│  │    └────────────────────────────────────────────────────┘  │  │
│  │                                                             │  │
│  │    ┌────────────────────────────────────────────────────┐  │  │
│  │    │ 2. Gain Calculation                                │  │  │
│  │    │    • Boost factor (user configurable)              │  │  │
│  │    │    • Cut factor (user configurable)                │  │  │
│  │    │    • Target level (user configurable)              │  │  │
│  │    └────────────────────────────────────────────────────┘  │  │
│  │                                                             │  │
│  │    ┌────────────────────────────────────────────────────┐  │  │
│  │    │ 3. Apply Gain                                      │  │  │
│  │    │    • Per-sample or per-block scaling               │  │  │
│  │    │    • Smooth transitions                            │  │  │
│  │    └────────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ B. MPEG-D DRC (ISO/IEC 23003-4)                            │  │
│  │                                                             │  │
│  │    ┌────────────────────────────────────────────────────┐  │  │
│  │    │ 1. DRC Configuration                               │  │  │
│  │    │    • Multiple DRC sets                             │  │  │
│  │    │    • Downmix instructions                          │  │  │
│  │    │    • Effect types                                  │  │  │
│  │    └────────────────────────────────────────────────────┘  │  │
│  │                                                             │  │
│  │    ┌────────────────────────────────────────────────────┐  │  │
│  │    │ 2. Gain Sequence Decoding                          │  │  │
│  │    │    • Time-varying gain curves                      │  │  │
│  │    │    • Interpolation/extrapolation                   │  │  │
│  │    │    • Multi-band DRC                                │  │  │
│  │    └────────────────────────────────────────────────────┘  │  │
│  │                                                             │  │
│  │    ┌────────────────────────────────────────────────────┐  │  │
│  │    │ 3. DRC Set Selection                               │  │  │
│  │    │    • User preference based                         │  │  │
│  │    │    • Effect type matching                          │  │  │
│  │    │    • Loudness normalization                        │  │  │
│  │    └────────────────────────────────────────────────────┘  │  │
│  │                                                             │  │
│  │    ┌────────────────────────────────────────────────────┐  │  │
│  │    │ 4. Multi-band Processing                           │  │  │
│  │    │    • QMF/Stft domain processing                    │  │  │
│  │    │    • Per-band gain application                     │  │  │
│  │    │    • Parametric EQ support                         │  │  │
│  │    └────────────────────────────────────────────────────┘  │  │
│  │                                                             │  │
│  │    ┌────────────────────────────────────────────────────┐  │  │
│  │    │ 5. Loudness Normalization                          │  │  │
│  │    │    • Target loudness level                         │  │  │
│  │    │    • Album mode / track mode                       │  │  │
│  │    │    • Measurement according to ITU-R BS.1770        │  │  │
│  │    └────────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

**Key Structures:**
- `ia_drc_dec_struct` - Legacy DRC decoder
- `ia_drc_config` - MPEG-D DRC configuration
- `ia_drc_gain_dec_struct` - Gain decoder state

**DRC Source Files:**
- `impd_drc_dec.c` - Main DRC processing
- `impd_drc_gain_dec.c` - Gain curve decoding
- `impd_drc_selection_process.c` - DRC set selection
- `impd_drc_filter_bank.c` - Multi-band filterbank

---


## Layer 12: Post-Processing & Output

### Peak Limiting and Format Conversion
**Components:** `ixheaacd_peak_limiter.c`, `ixheaacd_peak_limiter_struct_def.h`

```
┌─────────────────────────────────────────────────────────────────┐
│              POST-PROCESSING & OUTPUT                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 1. Peak Limiter (Optional)                                 │  │
│  │    • Prevent clipping/overshooting                        │  │
│  │    • Lookahead-based limiting                             │  │
│  │    • Attack time: configurable                            │  │
│  │    • Release time: configurable                           │  │
│  │    • Threshold: 0 dBFS                                    │  │
│  │    • Per-channel processing                               │  │
│  │    • Introduces additional delay                          │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 2. Scale/Gain Adjustment                                   │  │
│  │    • Q-shift normalization                                │  │
│  │    • Convert internal representation to output format     │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 3. Format Conversion                                       │  │
│  │    • 16-bit PCM (WORD16)                                  │  │
│  │    • 24-bit PCM (WORD32, upper 24 bits)                   │  │
│  │    • 32-bit float (FLOAT32)                               │  │
│  │    • Interleaved or deinterleaved output                  │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 4. Channel Mapping/Downmix (Optional)                      │  │
│  │    • Multi-channel to stereo downmix                      │  │
│  │    • Stereo to mono                                       │  │
│  │    • Channel reordering                                   │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ OUTPUT: PCM Audio Buffer                                   │  │
│  │  • Sample rate: 8k to 96kHz (with SBR)                    │  │
│  │  • Channels: 1 to 24 (depending on config)                │  │
│  │  • Frame size: 1024, 960, 480, 512, or 2048 samples       │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Layer 13: Error Concealment

### Robust Error Handling
**Components:** `ixheaacd_ec_*.c`, `ixheaacd_aac_ec.c`

```
┌─────────────────────────────────────────────────────────────────┐
│              ERROR CONCEALMENT                                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Handles bitstream errors, corrupted frames, and packet loss     │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Error Detection                                            │  │
│  │  • CRC validation failures                                │  │
│  │  • Bitstream syntax errors                                │  │
│  │  • Invalid codebook indices                               │  │
│  │  • Range violations                                       │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Concealment Strategies                                     │  │
│  │                                                            │  │
│  │  ┌────────────────────────────────────────────────────┐   │  │
│  │  │ 1. Spectral Domain Concealment                     │   │  │
│  │  │    • Copy previous frame spectrum                  │   │  │
│  │  │    • Apply attenuation/fade                        │   │  │
│  │  │    • Maintain energy levels                        │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  │                                                            │  │
│  │  ┌────────────────────────────────────────────────────┐   │  │
│  │  │ 2. Time Domain Concealment                         │   │  │
│  │  │    • Overlap-add with previous good frame          │   │  │
│  │  │    • Fade-out strategy                             │   │  │
│  │  │    • Zero insertion for severe errors              │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  │                                                            │  │
│  │  ┌────────────────────────────────────────────────────┐   │  │
│  │  │ 3. State Recovery                                  │   │  │
│  │  │    • Reset prediction states                       │   │  │
│  │  │    • Clear TNS filter memory                       │   │  │
│  │  │    • Reinitialize overlap buffers                  │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Frame Status Tracking                                      │  │
│  │  • Good frame: full decoding                             │  │
│  │  • Bad frame: apply concealment                          │  │
│  │  • Consecutive error counting                            │  │
│  │  • Gradual quality degradation                           │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

**Key Structures:**
- `ia_aac_err_config_struct` - Error concealment configuration
- `ia_ec_state_struct` - EC state tracking

---

## Complete Decoder State Management

### Main Decoder Structures
**Components:** `ixheaacd_struct_def.h`, `ixheaacd_struct.h`

```
┌─────────────────────────────────────────────────────────────────┐
│              DECODER STATE HIERARCHY                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ia_exhaacplus_dec_api_struct (Top-level API structure)         │
│  │                                                               │
│  ├─ ia_aac_dec_state_struct (Main decoder state)               │
│  │  │                                                            │
│  │  ├─ ia_aac_dec_config_struct (Configuration)                │
│  │  │  • Sample rate, channels, object type                    │
│  │  │  • DRC settings, output format                           │
│  │  │  • Feature flags (SBR, PS, MPS)                          │
│  │  │                                                            │
│  │  ├─ ia_bit_buf_struct (Bitstream buffer)                    │
│  │  │  • Input buffer management                               │
│  │  │                                                            │
│  │  ├─ ia_aac_decoder_struct[] (Per-element decoders)          │
│  │  │  • Up to MAX_BS_ELEMENT instances                        │
│  │  │  • Each handles one audio element (SCE/CPE/LFE)          │
│  │  │  │                                                         │
│  │  │  ├─ ia_aac_dec_channel_info_struct[] (Per-channel)      │
│  │  │  │  • ICS info, scale factors, spectral data            │
│  │  │  │  • Window sequence, grouping                          │
│  │  │  │                                                         │
│  │  │  └─ ia_aac_dec_overlap_info[] (Overlap buffers)         │
│  │  │     • Previous frame data for overlap-add               │
│  │  │                                                            │
│  │  ├─ ia_handle_sbr_dec_inst_struct[] (SBR decoders)         │
│  │  │  • Per-element SBR instances                             │
│  │  │  • QMF filterbank states                                 │
│  │  │  • SBR configuration and frame data                      │
│  │  │                                                            │
│  │  ├─ ia_ps_dec_struct (PS decoder, if present)               │
│  │  │  • Hybrid filterbank                                     │
│  │  │  • Stereo parameters                                     │
│  │  │                                                            │
│  │  ├─ ia_mps_dec_state_struct (MPS decoder, if present)       │
│  │  │  • Spatial parameters                                    │
│  │  │  • Tree configuration                                    │
│  │  │  • QMF states                                            │
│  │  │                                                            │
│  │  ├─ ia_drc_dec_struct (DRC processor)                       │
│  │  │  • Gain computation                                      │
│  │  │  • Configuration                                         │
│  │  │                                                            │
│  │  └─ ia_peak_limiter_struct (Peak limiter)                   │
│  │     • Limiter state, delay buffer                           │
│  │                                                               │
│  ├─ ia_aac_dec_tables_struct (Lookup tables)                   │
│  │  • Huffman tables                                            │
│  │  • Scale factor tables                                       │
│  │  • Window tables                                             │
│  │  • Quantization tables                                       │
│  │                                                               │
│  ├─ ia_sbr_tables_struct (SBR tables)                          │
│  │  • QMF filter coefficients                                   │
│  │  • SBR codebooks                                             │
│  │                                                               │
│  └─ ixheaacd_misc_tables (Common tables)                        │
│     • FFT twiddle factors                                        │
│     • Trigonometric tables                                       │
└─────────────────────────────────────────────────────────────────┘
```

---


## Memory Architecture

### Memory Pools and Buffer Management

```
┌─────────────────────────────────────────────────────────────────┐
│              MEMORY ARCHITECTURE                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 1. PERSISTENT MEMORY                                       │  │
│  │    • Allocated once during initialization                 │  │
│  │    • Maintained across frames                             │  │
│  │    • Contains:                                            │  │
│  │      - Decoder state structures                           │  │
│  │      - Overlap buffers                                    │  │
│  │      - Filter states (QMF, hybrid, etc.)                  │  │
│  │      - Previous frame data                                │  │
│  │      - SBR/PS/MPS states                                  │  │
│  │    • Size: ~100KB - 500KB depending on config             │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 2. SCRATCH MEMORY                                          │  │
│  │    • Temporary working memory                             │  │
│  │    • Reused across function calls                         │  │
│  │    • Contains:                                            │  │
│  │      - IMDCT scratch buffers                              │  │
│  │      - FFT working memory                                 │  │
│  │      - QMF temporary buffers                              │  │
│  │      - Intermediate spectral data                         │  │
│  │    • Size: ~64KB - 128KB                                  │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 3. TABLE MEMORY (Read-only)                                │  │
│  │    • Lookup tables, ROM data                              │  │
│  │    • Shared across all decoder instances                  │  │
│  │    • Contains:                                            │  │
│  │      - Huffman codebooks                                  │  │
│  │      - Quantization tables (x^4/3)                        │  │
│  │      - Scale factor tables                                │  │
│  │      - Window coefficients                                │  │
│  │      - FFT twiddle factors                                │  │
│  │      - QMF prototype filters                              │  │
│  │    • Size: ~200KB - 400KB                                 │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ 4. INPUT/OUTPUT BUFFERS                                    │  │
│  │    • Managed by application                               │  │
│  │    • Bitstream input buffer                               │  │
│  │    • PCM output buffer                                    │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

**Memory Management Functions:**
- `ixheaacd_dec_mem_api()` - Query memory requirements
- `ixheaacd_allocate_mem_persistent()` - Allocate persistent memory
- `ixheaacd_fill_aac_mem_tables()` - Setup memory tables

---

## Decoder Data Flow (Complete Frame Processing)

### Frame-by-Frame Processing Pipeline

```
┌═════════════════════════════════════════════════════════════════┐
║                    FRAME DECODE PIPELINE                         ║
╠═════════════════════════════════════════════════════════════════╣
║                                                                   ║
║  INPUT: Compressed AAC bitstream frame                           ║
║    ↓                                                              ║
║  ┌─────────────────────────────────────────────────────────┐    ║
║  │ Phase 1: CONTAINER & SYNC                               │    ║
║  │  • Find sync word (if ADTS/LATM)                        │    ║
║  │  • Parse container header                               │    ║
║  │  • Extract raw AAC frame                                │    ║
║  │  • Validate CRC (if present)                            │    ║
║  └─────────────────────────────────────────────────────────┘    ║
║    ↓                                                              ║
║  ┌─────────────────────────────────────────────────────────┐    ║
║  │ Phase 2: HEADER DECODE (first frame only)              │    ║
║  │  • Parse Audio Specific Config                          │    ║
║  │  • Determine object type, sample rate, channels         │    ║
║  │  • Initialize decoder structures                        │    ║
║  │  • Setup SBR/PS/MPS if present                          │    ║
║  └─────────────────────────────────────────────────────────┘    ║
║    ↓                                                              ║
║  ┌─────────────────────────────────────────────────────────┐    ║
║  │ Phase 3: ELEMENT STREAM PARSING                         │    ║
║  │  • Loop through elements until ID_END                   │    ║
║  │  • For each element (SCE/CPE/LFE/CCE/FIL):              │    ║
║  │    - Parse element-specific syntax                      │    ║
║  │    - Extract SBR/DRC/MPS data from fill elements        │    ║
║  └─────────────────────────────────────────────────────────┘    ║
║    ↓                                                              ║
║  ┌─────────────────────────────────────────────────────────┐    ║
║  │ Phase 4: CORE AAC DECODE (per channel)                  │    ║
║  │                                                          │    ║
║  │  For each channel in element:                           │    ║
║  │    ↓                                                     │    ║
║  │  ┌──────────────────────────────────────────────────┐   │    ║
║  │  │ 4a. ICS Info Parse                               │   │    ║
║  │  │     • Window sequence, grouping                  │   │    ║
║  │  └──────────────────────────────────────────────────┘   │    ║
║  │    ↓                                                     │    ║
║  │  ┌──────────────────────────────────────────────────┐   │    ║
║  │  │ 4b. Section Data                                 │   │    ║
║  │  │     • Codebook selection per SFB                 │   │    ║
║  │  └──────────────────────────────────────────────────┘   │    ║
║  │    ↓                                                     │    ║
║  │  ┌──────────────────────────────────────────────────┐   │    ║
║  │  │ 4c. Scale Factors                                │   │    ║
║  │  │     • DPCM decode                                │   │    ║
║  │  └──────────────────────────────────────────────────┘   │    ║
║  │    ↓                                                     │    ║
║  │  ┌──────────────────────────────────────────────────┐   │    ║
║  │  │ 4d. Spectral Data (Huffman decode)               │   │    ║
║  │  │     • Quantized spectral coefficients            │   │    ║
║  │  └──────────────────────────────────────────────────┘   │    ║
║  │    ↓                                                     │    ║
║  │  ┌──────────────────────────────────────────────────┐   │    ║
║  │  │ 4e. Inverse Quantization                         │   │    ║
║  │  │     • x^(4/3) and scale factor apply             │   │    ║
║  │  └──────────────────────────────────────────────────┘   │    ║
║  │    ↓                                                     │    ║
║  │  ┌──────────────────────────────────────────────────┐   │    ║
║  │  │ 4f. Spectral Processing                          │   │    ║
║  │  │     • TNS (if present)                           │   │    ║
║  │  │     • M/S stereo (if CPE)                        │   │    ║
║  │  │     • Intensity stereo                           │   │    ║
║  │  │     • PNS                                        │   │    ║
║  │  └──────────────────────────────────────────────────┘   │    ║
║  │    ↓                                                     │    ║
║  │  ┌──────────────────────────────────────────────────┐   │    ║
║  │  │ 4g. IMDCT + Windowing                            │   │    ║
║  │  │     • Frequency → Time domain                    │   │    ║
║  │  │     • Overlap-add with previous frame            │   │    ║
║  │  └──────────────────────────────────────────────────┘   │    ║
║  │                                                          │    ║
║  │  Result: Time-domain core AAC audio                     │    ║
║  └─────────────────────────────────────────────────────────┘    ║
║    ↓                                                              ║
║  ┌─────────────────────────────────────────────────────────┐    ║
║  │ Phase 5: SBR PROCESSING (if present)                    │    ║
║  │  • QMF analysis of core AAC output                      │    ║
║  │  • Parse SBR bitstream data                             │    ║
║  │  • High frequency generation                            │    ║
║  │  • Envelope adjustment                                  │    ║
║  │  • Noise floor addition                                 │    ║
║  │  • QMF synthesis                                        │    ║
║  │  Result: Full-bandwidth audio                           │    ║
║  └─────────────────────────────────────────────────────────┘    ║
║    ↓                                                              ║
║  ┌─────────────────────────────────────────────────────────┐    ║
║  │ Phase 6: PS PROCESSING (if present, HE-AACv2)           │    ║
║  │  • Input: Mono QMF signal                               │    ║
║  │  • Hybrid analysis                                      │    ║
║  │  • Apply stereo parameters                              │    ║
║  │  • Decorrelation                                        │    ║
║  │  • Hybrid synthesis                                     │    ║
║  │  Result: Stereo QMF signal                              │    ║
║  │  • QMF synthesis to time domain                         │    ║
║  └─────────────────────────────────────────────────────────┘    ║
║    ↓                                                              ║
║  ┌─────────────────────────────────────────────────────────┐    ║
║  │ Phase 7: MPS PROCESSING (if present, multi-channel)     │    ║
║  │  • QMF analysis of downmix                              │    ║
║  │  • Parse spatial parameters                             │    ║
║  │  • Decorrelation                                        │    ║
║  │  • M1/M2 matrix application                             │    ║
║  │  • Temporal processing                                  │    ║
║  │  • QMF synthesis per channel                            │    ║
║  │  Result: Multi-channel output                           │    ║
║  └─────────────────────────────────────────────────────────┘    ║
║    ↓                                                              ║
║  ┌─────────────────────────────────────────────────────────┐    ║
║  │ Phase 8: DRC APPLICATION                                │    ║
║  │  • Parse DRC metadata (if present)                      │    ║
║  │  • Compute gain values                                  │    ║
║  │  • Apply dynamic range compression                      │    ║
║  │  • Loudness normalization (if enabled)                  │    ║
║  └─────────────────────────────────────────────────────────┘    ║
║    ↓                                                              ║
║  ┌─────────────────────────────────────────────────────────┐    ║
║  │ Phase 9: POST-PROCESSING                                │    ║
║  │  • Peak limiter (if enabled)                            │    ║
║  │  • Scale/gain adjustment                                │    ║
║  │  • Format conversion (16/24/32-bit)                     │    ║
║  │  • Channel mapping/downmix                              │    ║
║  └─────────────────────────────────────────────────────────┘    ║
║    ↓                                                              ║
║  OUTPUT: PCM audio samples                                       ║
╚═════════════════════════════════════════════════════════════════╝
```

---


## Module Dependencies & Interaction Map

### Minimized Connection Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    MODULE DEPENDENCY GRAPH                       │
│           (Organized to minimize cross-dependencies)             │
└─────────────────────────────────────────────────────────────────┘

LAYER 0: Foundation (No dependencies)
├─ Basic Types & Constants
│  ├─ ixheaac_type_def.h
│  ├─ ixheaac_constants.h
│  └─ ixheaac_error_standards.h
│
└─ Basic Operations
   ├─ ixheaac_basic_ops*.h (arithmetic operations)
   └─ ixheaacd_intrinsics.h (platform-specific)

LAYER 1: Core Utilities (Depends on Layer 0 only)
├─ Bitstream I/O
│  └─ ixheaacd_bitbuffer.h/.c
│
├─ Memory Management
│  └─ ixheaacd_memory_standards.h
│
└─ Lookup Tables (ROM data)
   ├─ ixheaacd_aac_rom.c/.h
   ├─ ixheaacd_common_rom.c/.h
   ├─ ixheaacd_sbr_rom.c/.h
   └─ ixheaacd_mps_rom.c/.h

LAYER 2: DSP Primitives (Depends on Layers 0-1)
├─ FFT/IMDCT
│  ├─ ixheaacd_fft.c/.h
│  ├─ ixheaacd_aac_imdct.c/.h
│  └─ ixheaacd_imdct.c
│
├─ Filterbanks
│  ├─ ixheaacd_qmf_dec.c/.h (QMF analysis/synthesis)
│  └─ ixheaacd_hybrid.c/.h (Hybrid filterbank)
│
└─ Windows
   └─ ixheaacd_Windowing.c/.h

LAYER 3: Container Parsing (Depends on Layers 0-1)
├─ Format Detection & Parsing
   ├─ ixheaacd_adts.c/.h (ADTS format)
   ├─ ixheaacd_latmdemux.c/.h (LATM/LOAS)
   └─ ixheaacd_adts_crc_check.c/.h

LAYER 4: Configuration (Depends on Layers 0-3)
└─ Header Decoding
   ├─ ixheaacd_headerdecode.c/.h
   ├─ ixheaacd_config.h
   └─ ixheaacd_aac_config.h

LAYER 5: Core AAC Decoding (Depends on Layers 0-4)
├─ Channel Stream Parsing
│  ├─ ixheaacd_channel.c/.h
│  ├─ ixheaacd_channelinfo.h
│  └─ ixheaacd_block.c/.h
│
├─ Huffman Decoding
│  ├─ ixheaacd_hufftables.c
│  ├─ ixheaacd_spectrum_dec.c
│  └─ ixheaacd_huff_code_reorder.c
│
├─ Spectral Tools
│  ├─ ixheaacd_pns.h (PNS)
│  ├─ ixheaacd_tns.c/.h (TNS)
│  ├─ ixheaacd_stereo.c/.h (M/S, Intensity)
│  └─ ixheaacd_lt_predict.c/.h (LTP)
│
└─ AAC Decoder Core
   ├─ ixheaacd_aacdec.h
   └─ ixheaacd_aacdecoder.c

LAYER 6: USAC Extensions (Depends on Layers 0-5)
├─ LPD/ACELP
│  ├─ ixheaacd_acelp_*.c
│  ├─ ixheaacd_lpc.c/.h
│  └─ ixheaacd_lpd_*.c
│
├─ TCX
│  ├─ ixheaacd_tcx_*.c
│  └─ ixheaacd_arith_dec.c/.h (Arithmetic coding)
│
└─ FAC
   └─ ixheaacd_fwd_alias_cnx.c

LAYER 7: Enhancement Modules (Parallel, Depend on Layers 0-5)
├─ SBR Module (Standalone)
│  ├─ ixheaacd_sbrdecoder.c/.h
│  ├─ ixheaacd_sbr_dec.c/.h
│  ├─ ixheaacd_env_extr.c/.h
│  ├─ ixheaacd_env_calc.c/.h
│  ├─ ixheaacd_lpp_tran.c/.h
│  ├─ ixheaacd_freq_sca.c/.h
│  └─ ixheaacd_esbr_*.c (Enhanced SBR)
│
├─ PS Module (Depends on SBR)
│  ├─ ixheaacd_ps_dec.c/.h
│  └─ ixheaacd_ps_bitdec.c/.h
│
└─ MPS Module (Standalone)
   ├─ ixheaacd_mps_dec.c/.h
   ├─ ixheaacd_ld_mps_dec.c/.h
   ├─ ixheaacd_mps_parse.c
   ├─ ixheaacd_mps_decorr.c
   ├─ ixheaacd_mps_apply_m1.c
   ├─ ixheaacd_mps_apply_m2.c
   └─ ixheaacd_mps_calc_m1m2_*.c

LAYER 8: DRC Module (Parallel, Depends on Layers 0-1)
└─ DRC Subsystem
   ├─ decoder/drc_src/impd_drc_dec.c/.h
   ├─ decoder/drc_src/impd_drc_gain_dec.c/.h
   ├─ decoder/drc_src/impd_drc_selection_process.c/.h
   └─ decoder/drc_src/impd_drc_*.c (various DRC components)

LAYER 9: Post-Processing (Depends on all above)
├─ Peak Limiter
│  └─ ixheaacd_peak_limiter.c
│
└─ Error Concealment
   ├─ ixheaacd_aac_ec.c
   └─ ixheaacd_ec_*.c

LAYER 10: Top-Level API (Orchestrates all layers)
├─ Main Decoder
│  ├─ ixheaacd_api.c
│  ├─ ixheaacd_decode_main.c
│  ├─ ixheaacd_main.h
│  └─ ixheaacd_struct_def.h
│
└─ Initialization
   ├─ ixheaacd_create.c/.h
   └─ ixheaacd_process.c/.h

PLATFORM-SPECIFIC (Can override generic implementations)
├─ ARMv7 (decoder/armv7/*.s)
├─ ARMv8 (decoder/armv8/*.s)
├─ x86/x86_64 (decoder/x86/*.c, decoder/x86_64/*.c)
└─ Generic (decoder/generic/*.c)
```

---

## Key Interface Boundaries for Rust Migration

### Critical API Boundaries

```
┌─────────────────────────────────────────────────────────────────┐
│           PUBLIC API (C → Application)                           │
├─────────────────────────────────────────────────────────────────┤
│  Entry Points:                                                   │
│  • ixheaacd_dec_api() - Main API dispatcher                     │
│  • Commands: INIT, SET_CONFIG, EXECUTE, GET_INFO, etc.          │
│                                                                   │
│  Key Structures (exposed to application):                        │
│  • ia_exhaacplus_dec_api_struct                                 │
│  • Configuration parameters                                      │
│  • Memory allocation requirements                                │
└─────────────────────────────────────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────────────────────────────────────┐
│           DECODER CORE (Rust Implementation)                     │
├─────────────────────────────────────────────────────────────────┤
│  Main Modules (can be Rust crates):                              │
│                                                                   │
│  1. bitstream_parser                                             │
│     • Container parsing (ADTS/LATM)                              │
│     • Bitstream I/O                                              │
│                                                                   │
│  2. aac_core                                                     │
│     • Channel decoding                                           │
│     • Huffman decode                                             │
│     • Spectral processing                                        │
│     • IMDCT                                                      │
│                                                                   │
│  3. usac_core (optional)                                         │
│     • ACELP/TCX decoding                                         │
│     • LPD tools                                                  │
│                                                                   │
│  4. sbr_module                                                   │
│     • SBR decoder                                                │
│     • QMF filterbank                                             │
│                                                                   │
│  5. ps_module                                                    │
│     • Parametric stereo                                          │
│                                                                   │
│  6. mps_module                                                   │
│     • MPEG Surround                                              │
│                                                                   │
│  7. drc_module                                                   │
│     • Dynamic range control                                      │
│                                                                   │
│  8. dsp_primitives                                               │
│     • FFT/IMDCT                                                  │
│     • Filterbanks                                                │
│     • Basic math operations                                      │
│                                                                   │
│  9. tables (const data)                                          │
│     • Lookup tables                                              │
│     • ROM data                                                   │
└─────────────────────────────────────────────────────────────────┘
```

---

## Rust Migration Strategy Recommendations

### Module Priority & Complexity Assessment

```
Phase 1: Foundation (Low Risk, High Reuse)
═══════════════════════════════════════════
Priority: START HERE
Complexity: LOW
Risk: LOW

Modules:
├─ Basic types and constants
├─ Bitstream reader/writer
├─ Memory management abstractions
├─ Lookup tables (as const arrays)
└─ Basic operations (as trait implementations)

Rust Benefits:
• Type safety for bitstream operations
• Zero-cost abstractions
• Const generics for table sizes
• Memory safety guarantees

Estimated Effort: 2-3 weeks

Phase 2: DSP Primitives (Medium Complexity)
═══════════════════════════════════════════
Priority: SECOND
Complexity: MEDIUM
Risk: LOW-MEDIUM

Modules:
├─ FFT implementations
├─ IMDCT
├─ QMF filterbank
├─ Window functions
└─ Hybrid filterbank

Rust Benefits:
• SIMD support via std::simd or external crates
• Generic implementations over sample types
• Safe array indexing
• Benchmark-driven optimization

Estimated Effort: 4-6 weeks

Phase 3: Core AAC Decoder (High Complexity)
═══════════════════════════════════════════
Priority: THIRD
Complexity: HIGH
Risk: MEDIUM

Modules:
├─ Container parsers (ADTS/LATM)
├─ Header decoding
├─ Channel element parsing
├─ Huffman decoder
├─ Spectral tools (TNS, PNS, M/S)
├─ Dequantization
└─ IMDCT + overlap-add

Rust Benefits:
• Enum-based element type handling
• Pattern matching for syntax parsing
• Immutable default for state
• Strong typing prevents state confusion

Estimated Effort: 8-12 weeks

Phase 4: Enhancement Modules (Parallel Development)
═══════════════════════════════════════════════════
Priority: FOURTH (can be parallel)
Complexity: HIGH
Risk: MEDIUM-HIGH

Modules:
├─ SBR decoder (6-8 weeks)
├─ PS decoder (3-4 weeks)
├─ MPS decoder (8-10 weeks)
└─ DRC processor (4-6 weeks)

Rust Benefits:
• Module isolation (each can be a separate crate)
• Trait-based interfaces
• Optional features via cargo features
• Independent testing and benchmarking

Estimated Effort: 20-28 weeks (parallel)

Phase 5: USAC Extensions (Optional, High Complexity)
════════════════════════════════════════════════════
Priority: FIFTH (if xHE-AAC support needed)
Complexity: VERY HIGH
Risk: HIGH

Modules:
├─ ACELP decoder
├─ TCX decoder
├─ Arithmetic decoder
├─ FAC tools
└─ LPD framework

Rust Benefits:
• Complex state machines via enums
• Safe fixed-point arithmetic
• Type-safe mode switching

Estimated Effort: 12-16 weeks

Phase 6: Integration & Optimization
═══════════════════════════════════
Priority: FINAL
Complexity: MEDIUM
Risk: LOW

Tasks:
├─ Top-level API design
├─ C FFI layer (if needed)
├─ Platform-specific optimizations
├─ SIMD optimization
├─ Multi-threading support
├─ Comprehensive testing
└─ Benchmarking & profiling

Estimated Effort: 6-8 weeks
```

---


## Rust Implementation Patterns

### Recommended Rust Architecture

```rust
// Example crate structure for Rust migration

workspace/
├── Cargo.toml (workspace root)
│
├── xheaac_types/           // Phase 1: Foundation
│   ├── src/
│   │   ├── lib.rs
│   │   ├── constants.rs
│   │   ├── error.rs
│   │   └── basic_ops.rs
│   └── Cargo.toml
│
├── xheaac_bitstream/       // Phase 1: Bitstream I/O
│   ├── src/
│   │   ├── lib.rs
│   │   ├── reader.rs
│   │   ├── writer.rs
│   │   └── buffer.rs
│   └── Cargo.toml
│
├── xheaac_tables/          // Phase 1: Lookup tables
│   ├── src/
│   │   ├── lib.rs
│   │   ├── huffman.rs
│   │   ├── quantization.rs
│   │   ├── windows.rs
│   │   └── trig.rs
│   └── Cargo.toml (build.rs to generate tables)
│
├── xheaac_dsp/             // Phase 2: DSP primitives
│   ├── src/
│   │   ├── lib.rs
│   │   ├── fft.rs
│   │   ├── imdct.rs
│   │   ├── qmf.rs
│   │   ├── hybrid.rs
│   │   └── windows.rs
│   └── Cargo.toml (optional: depends on rustfft)
│
├── xheaac_container/       // Phase 3: Container parsing
│   ├── src/
│   │   ├── lib.rs
│   │   ├── adts.rs
│   │   ├── latm.rs
│   │   └── raw.rs
│   └── Cargo.toml
│
├── xheaac_core/            // Phase 3: Core AAC
│   ├── src/
│   │   ├── lib.rs
│   │   ├── decoder.rs
│   │   ├── channel.rs
│   │   ├── huffman.rs
│   │   ├── spectrum.rs
│   │   ├── tns.rs
│   │   ├── pns.rs
│   │   ├── stereo.rs
│   │   └── imdct_process.rs
│   └── Cargo.toml
│
├── xheaac_sbr/             // Phase 4: SBR
│   ├── src/
│   │   ├── lib.rs
│   │   ├── decoder.rs
│   │   ├── env_extract.rs
│   │   ├── env_calc.rs
│   │   ├── hfg.rs
│   │   └── esbr.rs
│   └── Cargo.toml
│
├── xheaac_ps/              // Phase 4: PS
│   ├── src/
│   │   ├── lib.rs
│   │   └── decoder.rs
│   └── Cargo.toml
│
├── xheaac_mps/             // Phase 4: MPS
│   ├── src/
│   │   ├── lib.rs
│   │   ├── decoder.rs
│   │   ├── parser.rs
│   │   ├── decorr.rs
│   │   └── matrix.rs
│   └── Cargo.toml
│
├── xheaac_drc/             // Phase 4: DRC
│   ├── src/
│   │   ├── lib.rs
│   │   ├── legacy.rs
│   │   └── mpeg_d.rs
│   └── Cargo.toml
│
├── xheaac_usac/            // Phase 5: USAC (optional)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── acelp.rs
│   │   ├── tcx.rs
│   │   ├── lpd.rs
│   │   └── arith.rs
│   └── Cargo.toml
│
└── xheaac/                 // Phase 6: Top-level API
    ├── src/
    │   ├── lib.rs
    │   ├── decoder.rs
    │   ├── config.rs
    │   ├── api.rs
    │   └── ffi.rs (C FFI if needed)
    ├── Cargo.toml
    └── examples/
        └── decode.rs
```

### Key Rust Design Patterns

#### 1. Type-Safe State Machine
```rust
pub enum DecoderState {
    Uninitialized,
    HeaderParsed(AudioConfig),
    Ready {
        config: AudioConfig,
        persistent: PersistentState,
    },
    Decoding {
        config: AudioConfig,
        persistent: PersistentState,
        frame_count: u64,
    },
    Error(DecoderError),
}

impl Decoder {
    pub fn init(&mut self) -> Result<(), DecoderError> {
        match &self.state {
            DecoderState::Uninitialized => {
                // Transition to HeaderParsed or Ready
            }
            _ => Err(DecoderError::InvalidState),
        }
    }
}
```

#### 2. Zero-Copy Bitstream Reading
```rust
pub struct BitReader<'a> {
    data: &'a [u8],
    bit_pos: usize,
}

impl<'a> BitReader<'a> {
    pub fn read_bits(&mut self, n: u8) -> Result<u32, BitstreamError> {
        // Safe, bounds-checked bit reading
    }
}
```

#### 3. Const Generic Tables
```rust
pub struct HuffmanTable<const N: usize> {
    codes: [u16; N],
    lengths: [u8; N],
    values: [i16; N],
}

pub const HUFFMAN_TABLE_1: HuffmanTable<162> = /* ... */;
```

#### 4. Trait-Based Processing
```rust
pub trait SpectralProcessor {
    fn process(&mut self, spectrum: &mut [f32], ctx: &ChannelContext);
}

impl SpectralProcessor for TnsDecoder {
    fn process(&mut self, spectrum: &mut [f32], ctx: &ChannelContext) {
        // TNS processing
    }
}

impl SpectralProcessor for PnsDecoder {
    fn process(&mut self, spectrum: &mut [f32], ctx: &ChannelContext) {
        // PNS processing
    }
}
```

#### 5. Error Handling
```rust
#[derive(Debug, thiserror::Error)]
pub enum DecoderError {
    #[error("Bitstream error: {0}")]
    Bitstream(#[from] BitstreamError),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    
    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(&'static str),
    
    #[error("Memory allocation failed")]
    OutOfMemory,
}
```

#### 6. SIMD Optimization (when stable)
```rust
#[cfg(target_feature = "avx2")]
fn imdct_avx2(input: &[f32], output: &mut [f32]) {
    use std::arch::x86_64::*;
    // SIMD implementation
}

#[cfg(not(target_feature = "avx2"))]
fn imdct_generic(input: &[f32], output: &mut [f32]) {
    // Generic implementation
}
```

---

## Testing Strategy for Rust Migration

### Test Pyramid

```
                    /\
                   /  \
                  / E2E \           End-to-End Tests
                 /  Tests \         • Full decode chains
                /          \        • Reference file comparison
               /____________\
              /              \
             /  Integration   \     Integration Tests
            /     Tests        \    • Module interactions
           /                    \   • Format compatibility
          /______________________\
         /                        \
        /      Unit Tests          \ Unit Tests
       /                            \• Individual functions
      /______________________________\• Algorithm correctness
```

#### Test Categories

**1. Unit Tests (70% of tests)**
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_huffman_decode_codebook_1() {
        let bitstream = &[0b10110011, 0b01010101];
        let mut reader = BitReader::new(bitstream);
        let result = decode_huffman(&mut reader, Codebook::CB1);
        assert_eq!(result, Ok(QuantPair { w: 1, x: -1 }));
    }
    
    #[test]
    fn test_imdct_1024() {
        let input = generate_test_spectrum();
        let mut output = vec![0.0; 1024];
        imdct(&input, &mut output);
        assert_approx_eq!(output, reference_output(), epsilon = 1e-5);
    }
}
```

**2. Integration Tests (20% of tests)**
```rust
#[test]
fn test_aac_lc_stereo_decode() {
    let bitstream = include_bytes!("test_data/stereo_48k.aac");
    let mut decoder = Decoder::new();
    decoder.init(bitstream)?;
    
    let frames = decoder.decode_all()?;
    assert_eq!(frames.len(), 100);
    assert_eq!(frames[0].channels, 2);
    assert_eq!(frames[0].sample_rate, 48000);
}
```

**3. Conformance Tests (10% of tests)**
- Test vectors from ISO/IEC standards
- Comparison with reference decoder output
- Bitexact matching for deterministic parts

---

## Performance Considerations

### Optimization Priorities

```
Critical Path (Optimize First):
1. IMDCT/FFT (20-30% of decode time)
   → SIMD, lookup tables, efficient windowing
   
2. Huffman Decoding (15-20% of decode time)
   → Table lookups, branch prediction
   
3. QMF Filterbank (15-25% with SBR)
   → Polyphase implementation, SIMD
   
4. Scale Factor Application (10-15%)
   → Vectorized operations
   
5. M/S Stereo (5-10% for stereo content)
   → In-place operations

Less Critical:
- Bitstream parsing (usually <5%)
- Parameter decoding (usually <5%)
- DRC application (usually <5%)
```

### Rust Performance Tips

1. **Use `&[T]` instead of `Vec<T>` when possible**
   - Avoid allocations in hot paths
   
2. **Leverage const functions and const generics**
   - Compile-time computation
   
3. **Profile-guided optimization**
   - Use `cargo pgo` or similar tools
   
4. **Inline hot functions**
   ```rust
   #[inline(always)]
   fn critical_function() { }
   ```

5. **Use `unsafe` judiciously for SIMD**
   - Only in well-tested, isolated modules
   - Extensive testing and documentation

---


## Component File Mapping Reference

### Quick Reference: C Files → Rust Modules

```
┌──────────────────────────────────────────────────────────────────┐
│                    FILE → MODULE MAPPING                          │
├──────────────────────────────────────────────────────────────────┤

BITSTREAM & CONTAINERS → xheaac_bitstream + xheaac_container
─────────────────────────────────────────────────────────────────
  ixheaacd_bitbuffer.c/h          → bitstream/reader.rs
  ixheaacd_adts.c/h               → container/adts.rs
  ixheaacd_latmdemux.c/h          → container/latm.rs
  ixheaacd_adts_crc_check.c/h     → container/crc.rs

CONFIGURATION → xheaac_core/config
─────────────────────────────────────────────────────────────────
  ixheaacd_headerdecode.c/h       → core/config/header.rs
  ixheaacd_config.h               → core/config/mod.rs
  ixheaacd_aac_config.h           → core/config/aac.rs

CORE AAC DECODER → xheaac_core
─────────────────────────────────────────────────────────────────
  ixheaacd_aacdecoder.c           → core/decoder.rs
  ixheaacd_channel.c/h            → core/channel.rs
  ixheaacd_channelinfo.h          → core/channel_info.rs
  ixheaacd_block.c/h              → core/block.rs
  ixheaacd_spectrum_dec.c         → core/spectrum.rs
  ixheaacd_hufftables.c           → tables/huffman.rs
  ixheaacd_huff_code_reorder.c    → core/huffman_decode.rs

SPECTRAL PROCESSING → xheaac_core/spectral
─────────────────────────────────────────────────────────────────
  ixheaacd_pns_js_thumb.c         → core/spectral/pns.rs
  ixheaacd_tns.c/h                → core/spectral/tns.rs
  ixheaacd_stereo.c/h             → core/spectral/stereo.rs
  ixheaacd_lt_predict.c/h         → core/spectral/ltp.rs

IMDCT & WINDOWS → xheaac_dsp
─────────────────────────────────────────────────────────────────
  ixheaacd_aac_imdct.c/h          → dsp/imdct.rs
  ixheaacd_imdct.c                → dsp/imdct_impl.rs
  ixheaacd_Windowing.c/h          → dsp/windows.rs
  ixheaacd_fft.c                  → dsp/fft.rs
  ixheaacd_dsp_fft32x32s.c        → dsp/fft32.rs

SBR DECODER → xheaac_sbr
─────────────────────────────────────────────────────────────────
  ixheaacd_sbrdecoder.c/h         → sbr/decoder.rs
  ixheaacd_sbr_dec.c/h            → sbr/process.rs
  ixheaacd_env_extr.c/h           → sbr/envelope.rs
  ixheaacd_env_calc.c/h           → sbr/env_calc.rs
  ixheaacd_lpp_tran.c/h           → sbr/transposer.rs
  ixheaacd_qmf_dec.c/h            → dsp/qmf.rs
  ixheaacd_freq_sca.c/h           → sbr/freq_scale.rs
  ixheaacd_esbr_*.c               → sbr/enhanced.rs

PS DECODER → xheaac_ps
─────────────────────────────────────────────────────────────────
  ixheaacd_ps_dec.c/h             → ps/decoder.rs
  ixheaacd_ps_bitdec.c/h          → ps/bitstream.rs
  ixheaacd_hybrid.c/h             → dsp/hybrid.rs

MPS DECODER → xheaac_mps
─────────────────────────────────────────────────────────────────
  ixheaacd_mps_dec.c/h            → mps/decoder.rs
  ixheaacd_ld_mps_dec.c/h         → mps/ld_mps.rs
  ixheaacd_mps_parse.c            → mps/parser.rs
  ixheaacd_mps_decorr.c           → mps/decorrelation.rs
  ixheaacd_mps_apply_m1.c         → mps/matrix_m1.rs
  ixheaacd_mps_apply_m2.c         → mps/matrix_m2.rs
  ixheaacd_mps_calc_m1m2_*.c      → mps/trees/
  ixheaacd_mps_polyphase.c        → dsp/polyphase.rs

USAC/xHE-AAC → xheaac_usac
─────────────────────────────────────────────────────────────────
  ixheaacd_acelp_*.c              → usac/acelp/
  ixheaacd_lpc.c/h                → usac/lpc.rs
  ixheaacd_lpd_*.c                → usac/lpd.rs
  ixheaacd_tcx_*.c                → usac/tcx.rs
  ixheaacd_arith_dec.c/h          → usac/arithmetic.rs
  ixheaacd_fwd_alias_cnx.c        → usac/fac.rs

DRC → xheaac_drc
─────────────────────────────────────────────────────────────────
  decoder/drc_src/impd_drc_dec.*  → drc/decoder.rs
  decoder/drc_src/impd_drc_gain_* → drc/gain.rs
  decoder/drc_src/impd_drc_sel_*  → drc/selection.rs
  decoder/drc_src/impd_drc_*      → drc/ (various)

POST-PROCESSING → xheaac/post
─────────────────────────────────────────────────────────────────
  ixheaacd_peak_limiter.c         → post/limiter.rs
  ixheaacd_aac_ec.c               → core/error_conceal.rs

LOOKUP TABLES → xheaac_tables
─────────────────────────────────────────────────────────────────
  ixheaacd_aac_rom.c/h            → tables/aac_rom.rs
  ixheaacd_common_rom.c/h         → tables/common.rs
  ixheaacd_sbr_rom.c/h            → tables/sbr_rom.rs
  ixheaacd_mps_rom.c              → tables/mps_rom.rs
  common/ixheaac_esbr_rom.c       → tables/esbr_rom.rs

TOP-LEVEL API → xheaac
─────────────────────────────────────────────────────────────────
  ixheaacd_api.c                  → api.rs
  ixheaacd_decode_main.c          → decoder.rs
  ixheaacd_create.c/h             → init.rs
  ixheaacd_process.c/h            → process.rs
  ixheaacd_struct_def.h           → types.rs

└──────────────────────────────────────────────────────────────────┘
```

---

## Critical Data Structures Summary

### State Management

```rust
// Top-level decoder state
pub struct Decoder {
    config: DecoderConfig,
    state: DecoderState,
    aac_decoders: Vec<AacElementDecoder>,
    sbr_decoders: Vec<Option<SbrDecoder>>,
    ps_decoder: Option<PsDecoder>,
    mps_decoder: Option<MpsDecoder>,
    drc_processor: DrcProcessor,
    peak_limiter: Option<PeakLimiter>,
    bitstream: BitstreamReader,
    scratch: ScratchBuffers,
    tables: &'static DecoderTables,
}

// Per-element AAC decoder
pub struct AacElementDecoder {
    element_type: ElementType,  // SCE, CPE, LFE
    channels: Vec<ChannelDecoder>,
    overlap_buffers: Vec<OverlapBuffer>,
}

// Per-channel decoder state
pub struct ChannelDecoder {
    ics_info: IcsInfo,
    section_data: SectionData,
    scale_factors: Vec<i16>,
    spectrum: Vec<f32>,
    tns_info: Option<TnsInfo>,
    pns_info: Option<PnsInfo>,
}

// Configuration
pub struct DecoderConfig {
    sample_rate: u32,
    channels: u8,
    object_type: AudioObjectType,
    frame_length: u16,
    sbr_present: bool,
    ps_present: bool,
    mps_present: bool,
    output_format: OutputFormat,
    drc_config: DrcConfig,
}
```

---

## Summary Statistics

### Codebase Metrics for Rust Migration

```
┌──────────────────────────────────────────────────────────────────┐
│                    MIGRATION COMPLEXITY                           │
├──────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Total C Source Files:        301                                │
│  Total C Header Files:        330                                │
│  Total Lines of C Code:       ~246,000                           │
│  Assembly Files (optional):   80 (can use Rust SIMD instead)     │
│                                                                   │
│  Estimated Rust Modules:      ~80-100 modules                    │
│  Estimated Rust LOC:          ~180,000-200,000 lines             │
│    (20-25% reduction due to:                                     │
│     - Type safety eliminates defensive code                      │
│     - Iterator chains replace manual loops                       │
│     - Match expressions replace if-else chains                   │
│     - Macro usage for repetitive patterns)                       │
│                                                                   │
│  Major Components:                                               │
│    1. Core AAC:         ~60,000 LOC  →  ~45,000 Rust LOC        │
│    2. SBR:              ~40,000 LOC  →  ~30,000 Rust LOC        │
│    3. MPS:              ~50,000 LOC  →  ~38,000 Rust LOC        │
│    4. USAC/ACELP:       ~35,000 LOC  →  ~27,000 Rust LOC        │
│    5. DRC:              ~25,000 LOC  →  ~20,000 Rust LOC        │
│    6. DSP/IMDCT:        ~20,000 LOC  →  ~15,000 Rust LOC        │
│    7. Infrastructure:   ~16,000 LOC  →  ~12,000 Rust LOC        │
│                                                                   │
│  Estimated Timeline (Full-time developer):                       │
│    - Foundation (Phase 1):          3 weeks                      │
│    - DSP Primitives (Phase 2):      6 weeks                      │
│    - Core AAC (Phase 3):           12 weeks                      │
│    - Enhancements (Phase 4):       24 weeks (parallel)           │
│    - USAC (Phase 5):               16 weeks                      │
│    - Integration (Phase 6):         8 weeks                      │
│    ─────────────────────────────────────                         │
│    Total (serial):                 69 weeks (~16 months)         │
│    Total (with parallelization):   ~45 weeks (~11 months)        │
│                                                                   │
│  Team Recommendation:                                            │
│    - 3-4 developers with Rust + DSP experience                   │
│    - 6-9 month timeline with proper testing                      │
│                                                                   │
└──────────────────────────────────────────────────────────────────┘
```

---

## Key Advantages of Rust Migration

### 1. **Memory Safety**
   - Eliminate buffer overflows, use-after-free
   - Guaranteed at compile time
   - No runtime overhead

### 2. **Fearless Concurrency**
   - Safe multi-threaded decoding
   - Parallel frame processing
   - Data race prevention

### 3. **Zero-Cost Abstractions**
   - Iterator chains compile to optimal code
   - Generic code monomorphization
   - Inlining and optimization

### 4. **Strong Type System**
   - Enums for state machines
   - Impossible states unrepresentable
   - Self-documenting code

### 5. **Modern Tooling**
   - Cargo for build and dependencies
   - Built-in testing framework
   - Documentation generation
   - Excellent IDE support

### 6. **Performance**
   - Comparable to C when optimized
   - SIMD support via portable_simd
   - Profile-guided optimization
   - Link-time optimization

### 7. **Maintainability**
   - Clear ownership semantics
   - Explicit error handling
   - No hidden control flow
   - Module system

---

## Next Steps

### Immediate Actions

1. **Set up Rust workspace structure**
   - Initialize cargo workspace
   - Define crate dependencies
   - Setup CI/CD pipeline

2. **Port foundation modules (Phase 1)**
   - Basic types and constants
   - Bitstream reader
   - Lookup tables as const data
   - Basic operations

3. **Create test infrastructure**
   - Conformance test vectors
   - Unit test framework
   - Benchmark harness

4. **Port DSP primitives (Phase 2)**
   - FFT/IMDCT implementations
   - QMF filterbank
   - Window functions
   - Verify bit-exact or near bit-exact

5. **Establish baseline performance**
   - Benchmark against C implementation
   - Identify optimization opportunities
   - Document performance characteristics

### Long-term Goals

- **Full AAC-LC support** (Phase 3)
- **HE-AAC support** (Phase 4 - SBR + PS)
- **Multi-channel support** (Phase 4 - MPS)
- **xHE-AAC support** (Phase 5 - USAC)
- **Production-ready API** (Phase 6)
- **Platform optimizations** (ARM NEON, x86 AVX)
- **C FFI for backward compatibility**

---

## References & Resources

### Standards Documents
- ISO/IEC 14496-3: MPEG-4 Audio (AAC)
- ISO/IEC 14496-26: Audio Conformance
- ISO/IEC 23003-1: MPEG Surround (MPS)
- ISO/IEC 23003-3: MPEG-D USAC (xHE-AAC)
- ISO/IEC 23003-4: MPEG-D DRC

### Rust Resources
- The Rust Programming Language Book
- Rust DSP Working Group
- portable_simd (SIMD support)
- rustfft (FFT library)
- dasp (Digital audio signal processing)

### Testing Resources
- ISO AAC conformance bitstreams
- AAC test vectors
- Reference decoder outputs

---

**END OF DECODER ARCHITECTURE DOCUMENT**

Generated for Rust Migration Planning
Focus: Decoder Path Only
Detail Level: Medium
Connection Minimization: Optimized via layered architecture

---
