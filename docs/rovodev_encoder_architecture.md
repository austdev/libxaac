# xHE-AAC Encoder Architecture & Rust Migration Guide

## Overview
This document provides a comprehensive architectural overview of the xHE-AAC encoder, organized into layers to minimize connections, along with detailed metrics for Rust migration planning.

---

## Encoder vs Decoder Comparison

| Metric | Encoder | Decoder |
|--------|---------|---------|
| **C Source Files** | 137 | 301 |
| **C Header Files** | 147 | 330 |
| **Total C Lines** | 99,118 | 246,189 |
| **Total H Lines** | 11,582 | 30,205 |
| **Average C File Size** | 723 lines | 818 lines |
| **Complexity** | Higher (analysis + encoding) | Lower (decoding only) |

---

## High-Level Encoder Pipeline

```
Input PCM Audio
      ↓
Pre-processing & Resampling
      ↓
Block Switching Decision (Attack Detection)
      ↓
Psychoacoustic Analysis (Perceptual Model)
      ↓
AAC Core Encoding (MDCT + Quantization)
      ↓
Rate Control & Bit Allocation
      ↓
Enhancement Encoding (SBR/PS/MPS)
      ↓
DRC Metadata Generation
      ↓
Bitstream Formatting & Multiplexing
      ↓
Output Bitstream (AAC/ADTS/LATM/USAC)
```

---

## Layered Architecture (Connection-Minimized)

### Layer 0: Foundation & Constants
**Purpose:** Type definitions, constants, basic operations  
**Dependencies:** None  
**Files:** ~15 files

```
┌─────────────────────────────────────────────────────────────────┐
│                    FOUNDATION LAYER                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Type Definitions & Constants                                    │
│  • ixheaac_type_def.h - Basic types (WORD32, FLOAT32, etc.)     │
│  • ixheaace_aac_constants.h - AAC constants                     │
│  • ixheaace_constants.h - General constants                     │
│  • iusace_cnst.h - USAC constants                               │
│                                                                   │
│  Basic Operations                                                │
│  • ixheaac_basic_ops*.h - Arithmetic operations                 │
│  • ixheaace_basic_ops_flt.h - Floating point ops                │
│                                                                   │
│  Error Codes                                                     │
│  • ixheaace_error_codes.h - Encoder error definitions           │
│  • ixheaac_error_standards.h - Standard error codes             │
└─────────────────────────────────────────────────────────────────┘
```

**Key Constants:**
- Frame lengths: 1024, 960, 512, 480, 768
- Sample rates: 8000 to 96000 Hz
- Channels: 1 to 24
- Bitrates: 8 kbps to 384 kbps per channel

---

### Layer 1: Lookup Tables & ROM Data
**Purpose:** Pre-computed tables for fast encoding  
**Dependencies:** Layer 0  
**Files:** ~8 files, ~21,000 lines

```
┌─────────────────────────────────────────────────────────────────┐
│                 LOOKUP TABLES & ROM DATA                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Main ROM Tables (21,000+ lines)                                 │
│  • iusace_rom.c (15,083 lines) - USAC tables                    │
│  • ixheaace_rom.c (3,964 lines) - AAC core tables               │
│  • ixheaace_sbr_rom.c (1,939 lines) - SBR tables                │
│  • ixheaace_common_rom.c - Shared tables                        │
│  • iusace_lpd_rom.c - LPD tables                                 │
│  • iusace_avq_rom.c - AVQ tables                                 │
│  • iusace_esbr_rom.c - Enhanced SBR tables                       │
│  • ixheaace_mps_rom.c - MPS tables                               │
│                                                                   │
│  Table Contents:                                                 │
│  ├─ Quantization tables (pow, sqrt)                             │
│  ├─ Huffman codebooks (11 spectral + TNS)                       │
│  ├─ Window coefficients (sine, KBD)                             │
│  ├─ Psychoacoustic tables (spreading, masking)                  │
│  ├─ FFT twiddle factors                                          │
│  ├─ QMF prototype filters                                        │
│  ├─ TNS coefficient tables                                       │
│  └─ SBR/PS tables                                                │
└─────────────────────────────────────────────────────────────────┘
```

**Migration Note:** Tables should be `const` arrays in Rust, potentially using `include_bytes!` for large data or code generation at build time.

---

### Layer 2: DSP Primitives
**Purpose:** Core signal processing building blocks  
**Dependencies:** Layers 0-1  
**Files:** ~12 files

```
┌─────────────────────────────────────────────────────────────────┐
│                    DSP PRIMITIVES                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ MDCT/FFT (Forward Transforms)                             │  │
│  │  • ixheaace_fft.c (2,485 lines) - FFT implementation      │  │
│  │  • ixheaace_fd_mdct.c - MDCT forward                      │  │
│  │  • iusace_fft.c - USAC FFT                                │  │
│  │  • iusace_tcx_mdct.c - TCX MDCT                           │  │
│  │  • ixheaace_radix2_fft.c - Radix-2 FFT                    │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Windowing                                                  │  │
│  │  • ixheaace_windowing.c - Apply windows                   │  │
│  │  • iusace_windowing.c - USAC windows                      │  │
│  │  • Window types: Sine, KBD, Low-overlap                   │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Filterbanks                                                │  │
│  │  • ixheaace_hybrid.c - Hybrid filterbank (PS)             │  │
│  │  • ixheaace_hybrid_init.c - Hybrid init                   │  │
│  │  • ixheaace_mps_polyphase.c - Polyphase filters           │  │
│  │  • ixheaace_mps_dct.c - DCT filterbank                    │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Resampling                                                 │  │
│  │  • ixheaace_resampler.c - IIR resampler                   │  │
│  │  • ixheaace_resampler_init.c - Init resamplers            │  │
│  │  • Ratios: 2:1, 4:1, 8:3, 1:3                             │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

**Performance Critical:** MDCT and FFT operations consume 20-30% of encoding time.

---


### Layer 3: Input Processing & Signal Analysis
**Purpose:** Pre-processing and perceptual analysis  
**Dependencies:** Layers 0-2  
**Files:** ~15 files

```
┌─────────────────────────────────────────────────────────────────┐
│              INPUT PROCESSING & ANALYSIS                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Block Switching Decision                                   │  │
│  │  • ixheaace_block_switch.c (1,254 lines)                  │  │
│  │  • iusace_block_switch.c - USAC block switch              │  │
│  │  • ixheaace_signal_classifier.c - Signal type detection   │  │
│  │                                                            │  │
│  │  Analysis:                                                 │  │
│  │  ├─ Attack detection (transient finder)                   │  │
│  │  ├─ Energy calculation per sub-block                      │  │
│  │  ├─ Threshold comparison                                  │  │
│  │  └─ Window sequence decision:                             │  │
│  │     • LONG_WINDOW (stationary)                            │  │
│  │     • START_WINDOW (attack detected)                      │  │
│  │     • SHORT_WINDOW (8 short windows)                      │  │
│  │     • STOP_WINDOW (return to stationary)                  │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Psychoacoustic Model (Perceptual Analysis)                │  │
│  │  • ixheaace_psy_mod.c (1,967 lines) - Main PSY model     │  │
│  │  • iusace_psy_mod.c (2,175 lines) - USAC PSY model       │  │
│  │  • ixheaace_psy_utils.c - PSY utilities                   │  │
│  │  • ixheaace_psy_configuration.c - PSY config              │  │
│  │  • ixheaace_psy_utils_spreading.c - Spreading function   │  │
│  │  • iusace_psy_utils.c - USAC PSY utils                    │  │
│  │                                                            │  │
│  │  Perceptual Model Components:                             │  │
│  │  ├─ FFT/MDCT of input signal                              │  │
│  │  ├─ Critical band analysis (Bark scale)                   │  │
│  │  ├─ Energy per band calculation                           │  │
│  │  ├─ Spreading function application                        │  │
│  │  ├─ Masking threshold calculation                         │  │
│  │  │  • Tonal masking                                       │  │
│  │  │  • Noise masking                                       │  │
│  │  │  • Absolute threshold of hearing                       │  │
│  │  ├─ Signal-to-Mask Ratio (SMR) computation               │  │
│  │  └─ Perceptual entropy estimation                         │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Stereo Pre-processing                                      │  │
│  │  • ixheaace_stereo_preproc.c (1,158 lines)               │  │
│  │                                                            │  │
│  │  Operations:                                               │  │
│  │  ├─ Inter-channel correlation analysis                    │  │
│  │  ├─ M/S stereo decision                                   │  │
│  │  ├─ Intensity stereo decision                             │  │
│  │  └─ Downmix for SBR/PS                                    │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

**Performance Critical:** Psychoacoustic model is 2nd most expensive operation (15-20% of encode time).

---

### Layer 4: AAC Core Encoder
**Purpose:** Main AAC encoding pipeline  
**Dependencies:** Layers 0-3  
**Files:** ~25 files

```
┌─────────────────────────────────────────────────────────────────┐
│                   AAC CORE ENCODER                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Main Encoding Loop                                         │  │
│  │  • ixheaace_enc_main.c (2,231 lines) - Main encoder      │  │
│  │  • ixheaace_fd_enc.c (1,419 lines) - FD encoder          │  │
│  │  • iusace_enc_main.c - USAC main encoder                  │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ MDCT & Windowing                                           │  │
│  │  • Apply analysis window                                   │  │
│  │  • Perform MDCT (1024/960/512/480 samples)                │  │
│  │  • Output: Spectral coefficients (frequency domain)       │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Rate Control & Bit Allocation                              │  │
│  │  • ixheaace_qc_main_hp.c (1,426 lines) - QC main         │  │
│  │  • ixheaace_qc_util.c - QC utilities                      │  │
│  │  • ixheaace_adjust_threshold.c (2,548 lines)              │  │
│  │                                                            │  │
│  │  Rate Control Process:                                     │  │
│  │  ├─ Calculate available bits for frame                    │  │
│  │  ├─ Distribute bits across channels                       │  │
│  │  ├─ Adjust quantization thresholds                        │  │
│  │  ├─ Iterative quantization + counting                     │  │
│  │  └─ Converge on target bitrate                            │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Spectral Processing                                        │  │
│  │  • ixheaace_ms_stereo.c - M/S stereo encoding            │  │
│  │  • ixheaace_calc_ms_band_energy.c - MS energy            │  │
│  │  • ixheaace_cplx_pred.c - Complex prediction             │  │
│  │  • iusace_ms.c - USAC M/S stereo                          │  │
│  │                                                            │  │
│  │  Operations:                                               │  │
│  │  ├─ M/S stereo: Convert L/R to M/S if beneficial         │  │
│  │  ├─ TNS analysis and application                          │  │
│  │  └─ Intensity stereo grouping                             │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ TNS (Temporal Noise Shaping)                               │  │
│  │  • ixheaace_tns.c (1,293 lines) - TNS encoder            │  │
│  │  • ixheaace_tns_init.c - TNS init                         │  │
│  │  • ixheaace_tns_params.c - TNS parameter selection       │  │
│  │  • ixheaace_tns_hp.c - High precision TNS                 │  │
│  │  • iusace_tns_usac.c - USAC TNS                           │  │
│  │                                                            │  │
│  │  TNS Process:                                              │  │
│  │  ├─ Autocorrelation analysis                              │  │
│  │  ├─ LPC coefficient calculation (Levinson-Durbin)        │  │
│  │  ├─ Parcor to LPC conversion                              │  │
│  │  ├─ Coefficient quantization                              │  │
│  │  ├─ Apply TNS filter to spectrum                          │  │
│  │  └─ Encode TNS parameters                                 │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Quantization & Encoding                                    │  │
│  │  • ixheaace_quant.c (1,632 lines) - Quantizer            │  │
│  │  • ixheaace_fd_quant.c - FD quantizer                     │  │
│  │  • iusace_fd_quant.h - USAC quantizer                     │  │
│  │                                                            │  │
│  │  Quantization:                                             │  │
│  │  ├─ Scale factor calculation per band                     │  │
│  │  ├─ Quantize: q = sign(x) * |x|^(3/4) / 2^(sf/4)        │  │
│  │  ├─ Rounding to integer                                   │  │
│  │  └─ Iterative loop to meet bit budget                     │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Huffman Encoding                                           │  │
│  │  • ixheaace_bits_count.c (1,337 lines) - Bit counting    │  │
│  │  • ixheaace_nf.c - Noise filling                          │  │
│  │                                                            │  │
│  │  Encoding:                                                 │  │
│  │  ├─ Section data (codebook selection)                     │  │
│  │  ├─ Scale factors (DPCM encoded)                          │  │
│  │  ├─ Spectral data (Huffman encoded)                       │  │
│  │  │  • Codebook selection (1-11)                           │  │
│  │  │  • ESC sequences for large values                      │  │
│  │  └─ TNS data                                               │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Scale Factor Estimation                                    │  │
│  │  • ixheaace_sf_estimation.c (1,008 lines)                │  │
│  │  • Calculate optimal scale factors per band               │  │
│  │  • Balance between quantization noise and bit usage       │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

**Core Encoding Iterations:**
1. Psychoacoustic analysis
2. Initial quantization
3. Huffman encode + count bits
4. Adjust scale factors
5. Re-quantize
6. Repeat until bit budget met (typically 2-5 iterations)

---


### Layer 5: USAC/xHE-AAC Encoder
**Purpose:** Extended encoder for speech/music hybrid coding  
**Dependencies:** Layers 0-4  
**Files:** ~20 files, ~35,000 lines

```
┌─────────────────────────────────────────────────────────────────┐
│                  USAC/xHE-AAC ENCODER                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  USAC supports dual coding modes per frame:                      │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ FD Mode (Frequency Domain - AAC-like)                     │  │
│  │  • iusace_fd_enc.h - FD encoder                           │  │
│  │  • Standard MDCT-based coding                             │  │
│  │  • Uses AAC core encoder pipeline                         │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ LPD Mode (Linear Predictive Domain)                       │  │
│  │  • iusace_lpd_enc.c (1,853 lines) - LPD encoder          │  │
│  │  • iusace_lpd.h - LPD structures                          │  │
│  │  • iusace_lpd_utils.c - LPD utilities                     │  │
│  │                                                            │  │
│  │  ┌────────────────────────────────────────────────────┐   │  │
│  │  │ ACELP Encoder (Speech Coding)                     │   │  │
│  │  │  • iusace_acelp_enc.c (2,679 lines)               │   │  │
│  │  │  • iusace_acelp_tools.c (1,625 lines)             │   │  │
│  │  │                                                    │   │  │
│  │  │  ACELP Process:                                    │   │  │
│  │  │  ├─ LPC analysis                                   │   │  │
│  │  │  │  • iusace_lpc.c (1,470 lines)                  │   │  │
│  │  │  │  • Autocorrelation                              │   │  │
│  │  │  │  • Levinson-Durbin algorithm                   │   │  │
│  │  │  ├─ LPC filtering (get residual)                  │   │  │
│  │  │  ├─ Open-loop pitch search                        │   │  │
│  │  │  ├─ Closed-loop pitch search                      │   │  │
│  │  │  ├─ Adaptive codebook contribution                │   │  │
│  │  │  ├─ Fixed codebook search (algebraic)             │   │  │
│  │  │  └─ Gain quantization                             │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  │                                                            │  │
│  │  ┌────────────────────────────────────────────────────┐   │  │
│  │  │ TCX Encoder (Transform Coded Excitation)          │   │  │
│  │  │  • iusace_tcx_enc.c (2,159 lines)                 │   │  │
│  │  │  • iusace_tcx_mdct.c - TCX MDCT                   │   │  │
│  │  │                                                    │   │  │
│  │  │  TCX Process:                                      │   │  │
│  │  │  ├─ LPC analysis                                   │   │  │
│  │  │  ├─ Compute LPC residual                          │   │  │
│  │  │  ├─ MDCT of residual                              │   │  │
│  │  │  ├─ AVQ quantization                              │   │  │
│  │  │  │  • iusace_avq_enc.c (1,914 lines)              │   │  │
│  │  │  │  • iusace_lpc_avq.c - LPC-AVQ                  │   │  │
│  │  │  │  • Algebraic Vector Quantization               │   │  │
│  │  │  └─ Arithmetic encoding                           │   │  │
│  │  │     • iusace_arith_enc.c (1,339 lines)            │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  │                                                            │  │
│  │  ┌────────────────────────────────────────────────────┐   │  │
│  │  │ FAC (Forward Aliasing Cancellation)                │   │  │
│  │  │  • iusace_enc_fac.c - FAC encoder                 │   │  │
│  │  │  • iusace_fd_fac.c - FD FAC                       │   │  │
│  │  │  • Handles transitions between FD and LPD         │   │  │
│  │  │  • Removes aliasing artifacts                     │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Mode Decision                                              │  │
│  │  • Signal classification (speech vs music)                │  │
│  │  • FD vs LPD decision per channel/frame                   │  │
│  │  • ACELP vs TCX decision (multiple sub-frame configs)     │  │
│  │  • Rate-distortion optimization                           │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

**Complexity:** USAC is the most complex encoder mode due to mode decisions and LPC analysis.

---

### Layer 6: SBR Encoder (Spectral Band Replication)
**Purpose:** High frequency reconstruction for HE-AAC  
**Dependencies:** Layers 0-4  
**Files:** ~25 files, ~15,000 lines

```
┌─────────────────────────────────────────────────────────────────┐
│                    SBR ENCODER                                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Input: High-frequency content of PCM audio                      │
│  Output: SBR side information (envelopes, noise, transients)    │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Main SBR Encoder                                           │  │
│  │  • ixheaace_sbr_main.c (2,002 lines) - Main controller    │  │
│  │  • ixheaace_sbr_main.h - SBR structures                   │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ QMF Analysis                                               │  │
│  │  • ixheaace_sbr_qmf_enc.c (1,637 lines) - QMF encoder    │  │
│  │  • ixheaace_sbr_qmf_enc_init.c - QMF init                │  │
│  │  • 64-band analysis filterbank                            │  │
│  │  • Complex QMF samples                                     │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Envelope Estimation                                        │  │
│  │  • ixheaace_sbr_env_est.c (2,187 lines)                  │  │
│  │  • ixheaace_sbr_env_est_init.c                           │  │
│  │                                                            │  │
│  │  Process:                                                  │  │
│  │  ├─ Calculate energy per QMF band                         │  │
│  │  ├─ Group into SBR frequency bands                        │  │
│  │  ├─ Time/frequency grid selection                         │  │
│  │  ├─ Envelope quantization (dB scale)                      │  │
│  │  └─ Delta coding of envelopes                             │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Noise Floor Estimation                                     │  │
│  │  • ixheaace_sbr_noise_floor_est.c (1,127 lines)          │  │
│  │  • Estimate noise energy per band                         │  │
│  │  • Quantize noise floor levels                            │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Transient Detection                                        │  │
│  │  • ixheaace_sbr_tran_det.c (1,108 lines)                 │  │
│  │  • ixheaace_sbr_tran_det_hp.c - High precision           │  │
│  │  • Detect attacks for time/freq grid                      │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Inverse Filtering Estimation                              │  │
│  │  • ixheaace_sbr_inv_filtering_estimation.c (813 lines)   │  │
│  │  • Determines inverse filtering levels                    │  │
│  │  • For better transposer quality                          │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Tonality & Missing Harmonics Detection                    │  │
│  │  • ixheaace_sbr_ton_corr.c (1,638 lines)                 │  │
│  │  • ixheaace_sbr_ton_corr_hp.c                            │  │
│  │  • ixheaace_sbr_missing_harmonics_det.c (730 lines)      │  │
│  │  • Detect tonal components                                │  │
│  │  • Signal missing harmonics for addition at decoder       │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Frequency Scaling & Frame Info                            │  │
│  │  • ixheaace_sbr_freq_scaling.c (1,057 lines)             │  │
│  │  • ixheaace_sbr_frame_info_gen.c (974 lines)             │  │
│  │  • Calculate frequency resolution                         │  │
│  │  • Generate frame info structure                          │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Enhanced SBR Extensions                                    │  │
│  │  • iusace_esbr_inter_tes.c (1,308 lines)                 │  │
│  │  • iusace_esbr_pvc.c (1,232 lines) - PVC encoding        │  │
│  │  • Inter-channel TES                                      │  │
│  │  • Predictive Vector Coding                               │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ HBE (Harmonic Bandwidth Extension)                        │  │
│  │  • ixheaace_sbr_hbe_trans.c - HBE transposer             │  │
│  │  • ixheaace_sbr_hbe_polyphase.c - Polyphase              │  │
│  │  • ixheaace_sbr_hbe_dft_trans.c - DFT transposer         │  │
│  │  • For ultra-high frequencies (>20 kHz)                   │  │
│  └───────────────────────────────────────────────────────────┘  │
│                          ↓                                        │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Bitstream Encoding                                         │  │
│  │  • ixheaace_sbr_code_envelope.c (1,346 lines)            │  │
│  │  • ixheaace_sbr_code_envelope_lp.c - Low power           │  │
│  │  • ixheaace_sbr_write_bitstream.c (1,452 lines)          │  │
│  │  • Encode SBR data into extension payload                 │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

**SBR Bitrate Savings:** Enables 50% bitrate reduction by not coding high frequencies in AAC core.

---


### Layer 7: PS & MPS Encoders (Stereo/Multichannel)
**Purpose:** Parametric stereo and spatial audio encoding  
**Dependencies:** Layers 0-6  
**Files:** ~35 files, ~18,000 lines

```
┌─────────────────────────────────────────────────────────────────┐
│              PS & MPS ENCODERS                                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ PS Encoder (Parametric Stereo - HE-AACv2)                 │  │
│  │  • ixheaace_ps_enc.c (1,850 lines) - PS encoder          │  │
│  │  • ixheaace_ps_enc_init.c - PS initialization             │  │
│  │  • ixheaace_ps_bitenc.c - PS bitstream encoding           │  │
│  │                                                            │  │
│  │  PS Process:                                               │  │
│  │  ├─ Input: Stereo QMF signals (from SBR encoder)         │  │
│  │  ├─ Hybrid analysis (low freq sub-bands)                  │  │
│  │  ├─ Parameter extraction:                                 │  │
│  │  │  • IID (Inter-channel Intensity Difference)           │  │
│  │  │  • ICC (Inter-channel Coherence)                      │  │
│  │  │  • IPD (Inter-channel Phase Difference)               │  │
│  │  │  • OPD (Overall Phase Difference)                     │  │
│  │  ├─ Parameter quantization (20/34 bands)                  │  │
│  │  ├─ Downmix to mono                                       │  │
│  │  └─ Encode PS parameters                                  │  │
│  │                                                            │  │
│  │  Output: Mono + PS parameters                             │  │
│  │  Bitrate Savings: ~50% compared to dual-channel SBR       │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ MPS Encoder (MPEG Surround - Multi-channel)               │  │
│  │  • ixheaace_mps_enc.c (2,963 lines) - Main MPS encoder   │  │
│  │                                                            │  │
│  │  ┌────────────────────────────────────────────────────┐   │  │
│  │  │ MPS Analysis Pipeline                              │   │  │
│  │  │  • ixheaace_mps_filter.c - Filterbanks             │   │  │
│  │  │  • ixheaace_mps_qmf.c - QMF analysis               │   │  │
│  │  │  • ixheaace_mps_hybrid_filter.c - Hybrid analysis  │   │  │
│  │  │  • ixheaace_mps_polyphase.c - Polyphase filters    │   │  │
│  │  │  • ixheaace_mps_dct.c - DCT transform              │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  │                                                            │  │
│  │  ┌────────────────────────────────────────────────────┐   │  │
│  │  │ Spatial Parameter Extraction                       │   │  │
│  │  │  • ixheaace_mps_param_extract.c (1,858 lines)     │   │  │
│  │  │                                                    │   │  │
│  │  │  Parameters:                                       │   │  │
│  │  │  ├─ CLD (Channel Level Difference)                │   │  │
│  │  │  ├─ ICC (Inter-Channel Coherence)                 │   │  │
│  │  │  ├─ CPC (Channel Prediction Coefficients)         │   │  │
│  │  │  ├─ ADG (Arbitrary Downmix Gains)                 │   │  │
│  │  │  └─ IOC (Inter-Object Coherence)                  │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  │                                                            │  │
│  │  ┌────────────────────────────────────────────────────┐   │  │
│  │  │ Tree Configuration                                 │   │  │
│  │  │  • ixheaace_mps_tree.c (1,139 lines)              │   │  │
│  │  │  • Supports 5.1, 7.1, 5.1.4, 7.1.4 configs        │   │  │
│  │  │  • Tree defines downmix hierarchy                 │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  │                                                            │  │
│  │  ┌────────────────────────────────────────────────────┐   │  │
│  │  │ Downmix Generation                                 │   │  │
│  │  │  • ixheaace_mps_dmx_tdom_enh.c (1,180 lines)      │   │  │
│  │  │  • Create stereo/mono downmix                     │   │  │
│  │  │  • Time-domain enhancement                        │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  │                                                            │  │
│  │  ┌────────────────────────────────────────────────────┐   │  │
│  │  │ Onset Detection                                    │   │  │
│  │  │  • ixheaace_mps_onset_detect.c (895 lines)        │   │  │
│  │  │  • Detect transients for time resolution          │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  │                                                            │  │
│  │  ┌────────────────────────────────────────────────────┐   │  │
│  │  │ Frame Windowing                                    │   │  │
│  │  │  • ixheaace_mps_frame_windowing.c (758 lines)     │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  │                                                            │  │
│  │  ┌────────────────────────────────────────────────────┐   │  │
│  │  │ Bitstream Encoding                                 │   │  │
│  │  │  • ixheaace_mps_bitstream.c (1,296 lines)         │   │  │
│  │  │  • ixheaace_mps_nlc_enc.c - NLC encoding          │   │  │
│  │  │  • ixheaace_mps_huff_tab.c - Huffman tables       │   │  │
│  │  └────────────────────────────────────────────────────┘   │  │
│  │                                                            │  │
│  │  Output: Downmix + Spatial metadata                       │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

**MPS Advantage:** 5.1 surround encoded at stereo bitrate + ~10% overhead.

---

### Layer 8: DRC Encoder & Loudness Measurement
**Purpose:** Dynamic range control metadata generation  
**Dependencies:** Layers 0-7  
**Files:** 8 files, ~8,500 lines

```
┌─────────────────────────────────────────────────────────────────┐
│              DRC ENCODER & LOUDNESS                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ DRC Encoder (MPEG-D DRC)                                   │  │
│  │  • encoder/drc_src/impd_drc_enc.c (1,544 lines)          │  │
│  │  • encoder/drc_src/impd_drc_api.c (934 lines)            │  │
│  │  • encoder/drc_src/impd_drc_gain_enc.c (1,191 lines)     │  │
│  │  • encoder/drc_src/impd_drc_gain_calculator.c            │  │
│  │                                                            │  │
│  │  DRC Encoding Process:                                     │  │
│  │  ├─ Analyze input audio dynamics                          │  │
│  │  ├─ Detect peaks and RMS levels                           │  │
│  │  ├─ Calculate gain curves                                 │  │
│  │  │  • Attack time                                         │  │
│  │  │  • Release time                                        │  │
│  │  │  • Compression ratio                                   │  │
│  │  │  • Threshold                                           │  │
│  │  ├─ Encode gain sequences                                 │  │
│  │  ├─ Multi-band DRC (optional)                             │  │
│  │  │  • encoder/drc_src/impd_drc_uni_drc_filter_bank.c    │  │
│  │  │  • Split into frequency bands                         │  │
│  │  │  • Independent DRC per band                           │  │
│  │  └─ Generate DRC metadata                                 │  │
│  │     • encoder/drc_src/impd_drc_mux.c (3,374 lines)       │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Loudness Measurement                                       │  │
│  │  • ixheaace_loudness_measurement.c (1,046 lines)         │  │
│  │                                                            │  │
│  │  ITU-R BS.1770 Algorithm:                                 │  │
│  │  ├─ K-weighted filtering                                  │  │
│  │  │  • High-pass filter (pre-filter)                      │  │
│  │  │  • High-shelf filter (RLB weighting)                  │  │
│  │  ├─ Gating (block-based)                                  │  │
│  │  │  • Absolute threshold: -70 LKFS                       │  │
│  │  │  • Relative threshold: -10 LU below mean              │  │
│  │  ├─ Calculate integrated loudness (LUFS/LKFS)            │  │
│  │  ├─ Peak detection (True Peak)                            │  │
│  │  └─ Output: Loudness normalization metadata               │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ DRC Configuration & Tables                                 │  │
│  │  • encoder/drc_src/impd_drc_tables.c (686 lines)         │  │
│  │  • encoder/drc_src/impd_drc_uni_drc_eq.c                 │  │
│  │  • Parametric EQ support                                   │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

### Layer 9: Bitstream Formatting & Output
**Purpose:** Final bitstream assembly and formatting  
**Dependencies:** All previous layers  
**Files:** ~12 files

```
┌─────────────────────────────────────────────────────────────────┐
│              BITSTREAM FORMATTING & OUTPUT                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Bitstream Writing                                          │  │
│  │  • ixheaace_write_bitstream.c (2,190 lines)              │  │
│  │  • iusace_write_bitstream.c (1,937 lines) - USAC         │  │
│  │                                                            │  │
│  │  Bitstream Assembly:                                       │  │
│  │  ├─ Write element headers (ID_SCE, ID_CPE, etc.)         │  │
│  │  ├─ Write ICS info                                        │  │
│  │  ├─ Write section data                                    │  │
│  │  ├─ Write scale factors                                   │  │
│  │  ├─ Write spectral data (Huffman encoded)                │  │
│  │  ├─ Write TNS data                                        │  │
│  │  ├─ Write fill elements (SBR/PS/DRC/MPS payloads)        │  │
│  │  └─ Byte alignment                                        │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Container Formatting                                       │  │
│  │  • ixheaace_write_adts_adif.c (838 lines)                │  │
│  │                                                            │  │
│  │  Output Formats:                                           │  │
│  │  ├─ Raw AAC (no container)                                │  │
│  │  ├─ ADTS (Audio Data Transport Stream)                    │  │
│  │  │  • Sync word 0xFFF                                     │  │
│  │  │  • Frame header (7 or 9 bytes)                         │  │
│  │  │  • CRC optional                                        │  │
│  │  ├─ ADIF (Audio Data Interchange Format)                 │  │
│  │  │  • Single header for entire file                      │  │
│  │  │  • Not commonly used                                   │  │
│  │  └─ LATM/LOAS (Low Overhead Audio Stream)                │  │
│  │     • For streaming/broadcast                             │  │
│  │     • More efficient than ADTS                            │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Audio Specific Config (ASC) Generation                    │  │
│  │  • ixheaace_asc_write.c (1,038 lines)                    │  │
│  │                                                            │  │
│  │  ASC Contents:                                             │  │
│  │  ├─ Audio object type                                     │  │
│  │  ├─ Sampling frequency                                    │  │
│  │  ├─ Channel configuration                                 │  │
│  │  ├─ Frame length flag                                     │  │
│  │  ├─ Extension flags (SBR, PS, MPS)                        │  │
│  │  └─ USAC config (if xHE-AAC)                              │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Bitstream Buffer Management                                │  │
│  │  • ixheaace_bitbuffer.c (746 lines)                      │  │
│  │  • ixheaace_bitbuffer_hp.c - High precision              │  │
│  │  • Bit-level writing                                      │  │
│  │  • Buffer management                                       │  │
│  │  • Byte alignment                                          │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ CRC Calculation                                            │  │
│  │  • ixheaace_sbr_crc.c - SBR CRC                           │  │
│  │  • Optional error detection                                │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---


### Layer 10: Top-Level API & Initialization
**Purpose:** Public API and encoder orchestration  
**Dependencies:** All layers  
**Files:** ~8 files

```
┌─────────────────────────────────────────────────────────────────┐
│                  TOP-LEVEL API & CONTROL                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Main API                                                   │  │
│  │  • ixheaace_api.c (3,934 lines) - API dispatcher         │  │
│  │                                                            │  │
│  │  API Commands:                                             │  │
│  │  ├─ IA_API_CMD_GET_API_SIZE                               │  │
│  │  ├─ IA_API_CMD_INIT                                       │  │
│  │  ├─ IA_API_CMD_SET_CONFIG_PARAM                           │  │
│  │  ├─ IA_API_CMD_GET_CONFIG_PARAM                           │  │
│  │  ├─ IA_API_CMD_EXECUTE                                    │  │
│  │  ├─ IA_API_CMD_GET_MEMTABS_SIZE                           │  │
│  │  ├─ IA_API_CMD_SET_MEMTABS_PTR                            │  │
│  │  ├─ IA_API_CMD_GET_N_MEMTABS                              │  │
│  │  └─ IA_API_CMD_SET_INPUT_BYTES / GET_OUTPUT_BYTES         │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Initialization & Configuration                             │  │
│  │  • ixheaace_enc_init.c (2,011 lines)                     │  │
│  │                                                            │  │
│  │  Initialization Steps:                                     │  │
│  │  ├─ Parse configuration parameters                        │  │
│  │  │  • Sample rate, channels, bitrate                     │  │
│  │  │  • Object type (LC, HE, HEv2, xHE)                    │  │
│  │  │  • Output format (RAW, ADTS, LATM)                    │  │
│  │  │  • SBR/PS/MPS enable flags                            │  │
│  │  ├─ Allocate memory (persistent + scratch)               │  │
│  │  ├─ Initialize sub-modules:                               │  │
│  │  │  • Psychoacoustic model                               │  │
│  │  │  • Block switcher                                     │  │
│  │  │  • AAC encoder                                        │  │
│  │  │  • SBR encoder (if enabled)                           │  │
│  │  │  • PS encoder (if enabled)                            │  │
│  │  │  • MPS encoder (if enabled)                           │  │
│  │  │  • DRC encoder (if enabled)                           │  │
│  │  ├─ Initialize lookup tables                              │  │
│  │  └─ Setup channel mapping                                 │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Main Encoding Entry Point                                  │  │
│  │  • ixheaace_enc_main.c (2,231 lines)                     │  │
│  │                                                            │  │
│  │  Per-Frame Processing:                                     │  │
│  │  ├─ Read PCM input                                        │  │
│  │  ├─ Resampling (if needed)                                │  │
│  │  ├─ Block switching decision                              │  │
│  │  ├─ Psychoacoustic analysis                               │  │
│  │  ├─ AAC core encoding                                     │  │
│  │  ├─ SBR encoding (if enabled)                             │  │
│  │  ├─ PS encoding (if enabled)                              │  │
│  │  ├─ MPS encoding (if enabled)                             │  │
│  │  ├─ DRC metadata generation                               │  │
│  │  ├─ Bitstream assembly                                    │  │
│  │  └─ Output encoded frame                                  │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Channel Mapping & Routing                                  │  │
│  │  • ixheaace_channel_map.c (792 lines)                    │  │
│  │  • Map input channels to AAC elements                     │  │
│  │  • Support 1 to 24 channels                               │  │
│  │  • Standard configurations: Mono, Stereo, 5.1, 7.1       │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ Interface & Utilities                                      │  │
│  │  • ixheaace_interface.c (887 lines)                      │  │
│  │  • Helper functions for API                                │  │
│  │  • Parameter validation                                    │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Encoder State Management

### Main State Structures

```
┌─────────────────────────────────────────────────────────────────┐
│                    ENCODER STATE HIERARCHY                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ixheaace_api_struct (Top-level API structure)                  │
│  │                                                               │
│  ├─ ixheaace_state_struct (Main encoder state)                 │
│  │  │                                                            │
│  │  ├─ ixheaace_config_struct (Configuration)                  │
│  │  │  • Sample rate, channels, bitrate                        │
│  │  │  • Object type, output format                            │
│  │  │  • Feature enables (SBR, PS, MPS, DRC)                   │
│  │  │                                                            │
│  │  ├─ ixheaace_temporal_data (Block switching)                │
│  │  │  • Attack detector state                                 │
│  │  │  • Energy history buffers                                │
│  │  │  • Window sequence decisions                             │
│  │  │                                                            │
│  │  ├─ ixheaace_psy_mod_struct (Psychoacoustic model)          │
│  │  │  • FFT state                                             │
│  │  │  • Critical band grouping                                │
│  │  │  • Masking thresholds                                    │
│  │  │  • Spreading function data                               │
│  │  │  • Previous frame data                                   │
│  │  │                                                            │
│  │  ├─ ixheaace_element_info_struct[] (Per-element encoders)   │
│  │  │  • AAC encoder instances                                 │
│  │  │  • Channel pair elements (CPE)                           │
│  │  │  • Single channel elements (SCE)                         │
│  │  │  │                                                         │
│  │  │  ├─ ixheaace_channel_info_struct[] (Per-channel data)   │
│  │  │  │  • Spectral coefficients                             │
│  │  │  │  • Quantized values                                  │
│  │  │  │  • Scale factors                                     │
│  │  │  │  • Section data                                      │
│  │  │  │  • TNS data                                          │
│  │  │  │                                                        │
│  │  │  └─ ixheaace_qc_out_struct (Quantizer/coder output)    │
│  │  │     • Bit consumption                                   │
│  │  │     • Quantization quality metrics                      │
│  │  │                                                           │
│  │  ├─ ixheaace_sbr_encoder[] (SBR encoders, per element)     │
│  │  │  • QMF analysis state                                    │
│  │  │  • Envelope estimator                                    │
│  │  │  • Transient detector                                    │
│  │  │  • Tonality analyzer                                     │
│  │  │  • Previous frame QMF data                               │
│  │  │                                                            │
│  │  ├─ ixheaace_ps_encoder (PS encoder, if stereo)             │
│  │  │  • Hybrid filterbank state                               │
│  │  │  • IID/ICC/IPD/OPD parameters                            │
│  │  │                                                            │
│  │  ├─ ixheaace_mps_encoder (MPS encoder, if multi-channel)    │
│  │  │  • QMF analysis state (all channels)                     │
│  │  │  • Spatial parameter extraction                          │
│  │  │  • Tree configuration                                    │
│  │  │  • Downmix generation                                    │
│  │  │                                                            │
│  │  ├─ ixheaace_drc_encoder (DRC encoder)                      │
│  │  │  • Gain calculation state                                │
│  │  │  • Peak/RMS history                                      │
│  │  │  • Loudness measurement                                  │
│  │  │                                                            │
│  │  ├─ ixheaace_bit_buf_struct (Output bitstream buffer)       │
│  │  │  • Bit buffer                                            │
│  │  │  • Write position                                        │
│  │  │                                                            │
│  │  └─ Scratch buffers                                         │
│  │     • MDCT working memory                                   │
│  │     • FFT scratch                                           │
│  │     • Temporary spectral data                               │
│  │                                                               │
│  └─ ROM tables (shared, read-only)                              │
│     • Huffman codebooks                                          │
│     • Quantization tables                                        │
│     • Window coefficients                                        │
│     • Psychoacoustic tables                                      │
└─────────────────────────────────────────────────────────────────┘
```

---


## Module Dependency Graph (Minimized Connections)

```
Layer 0: Foundation (No dependencies)
│
├─ Types, Constants, Basic Ops
│
Layer 1: Lookup Tables (Depends on Layer 0 only)
│
├─ ROM data: Huffman, Quant, Windows, PSY, FFT, QMF tables
│
Layer 2: DSP Primitives (Depends on Layers 0-1)
│
├─ MDCT/FFT, Windowing, Filterbanks, Resampling
│
Layer 3: Input Processing & Analysis (Depends on Layers 0-2)
│
├─ Block Switching ──┐
├─ PSY Model ────────┼─> Uses DSP primitives
└─ Stereo Preproc ───┘
│
Layer 4: AAC Core Encoder (Depends on Layers 0-3)
│
├─ Main Encoding Loop
├─ MDCT & Windowing (Layer 2)
├─ Rate Control ──────┐
├─ Spectral Processing│
├─ TNS Analysis ──────┼─> Core AAC encoding path
├─ Quantization ──────┤
└─ Huffman Encoding ──┘
│
Layer 5: USAC Encoder (Depends on Layers 0-4)
│
├─ FD Mode (uses Layer 4)
├─ LPD Mode ──┐
│  ├─ ACELP ─┼─> Speech/music hybrid
│  ├─ TCX ────┤
│  └─ FAC ────┘
│
Layer 6: SBR Encoder (Parallel, Depends on Layers 0-2)
│
├─ QMF Analysis (Layer 2)
├─ Envelope Estimation
├─ Transient Detection
├─ Tonality Analysis
└─ Bitstream Encoding
│
Layer 7: PS & MPS Encoders (Parallel, Depends on Layers 0-2, 6)
│
├─ PS Encoder ─> Depends on SBR QMF output
└─ MPS Encoder ─> Independent QMF analysis
│
Layer 8: DRC Encoder (Parallel, Depends on Layers 0-1)
│
├─ Gain Calculation
├─ Loudness Measurement
└─ Metadata Generation
│
Layer 9: Bitstream Formatting (Depends on all above)
│
├─ Bitstream Assembly
├─ Container Formatting (ADTS/LATM)
└─ ASC Generation
│
Layer 10: Top-Level API (Orchestrates all layers)
│
├─ API Dispatcher
├─ Initialization
└─ Main Encoding Loop
```

**Design Philosophy:** Enhancement modules (SBR, PS, MPS, DRC) are independent of AAC core, allowing:
- Optional features via compile-time flags
- Parallel development
- Independent testing
- Modular Rust crates

---

## Encoder Codebase Metrics for Rust Migration

### File Statistics

```
┌──────────────────────────────────────────────────────────────────┐
│                    ENCODER FILE METRICS                           │
├──────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Total C Source Files:        137 files                          │
│  Total C Header Files:        147 files                          │
│  Total Assembly Files:        0 (encoder is pure C)              │
│                                                                   │
│  Total Lines of C Code:       ~99,118 lines                      │
│  Total Lines of H Code:       ~11,582 lines                      │
│  Average C File Size:         ~723 lines                         │
│  Average H File Size:         ~79 lines                          │
│                                                                   │
│  Component Breakdown:                                            │
│  ├─ AAC Core Encoder:         ~35,000 lines (35.3%)             │
│  ├─ USAC Encoder:             ~25,000 lines (25.2%)             │
│  ├─ SBR Encoder:              ~15,000 lines (15.1%)             │
│  ├─ MPS Encoder:              ~12,000 lines (12.1%)             │
│  ├─ DRC Encoder:              ~8,500 lines  (8.6%)              │
│  ├─ PS Encoder:               ~2,000 lines  (2.0%)              │
│  └─ Infrastructure & API:     ~1,618 lines  (1.6%)              │
│                                                                   │
└──────────────────────────────────────────────────────────────────┘
```

### Top 20 Largest Encoder Files

```
Rank  Lines  File
────  ─────  ─────────────────────────────────────────────────────
1.   15,083  encoder/iusace_rom.c (USAC tables)
2.    3,964  encoder/ixheaace_rom.c (AAC tables)
3.    3,934  encoder/ixheaace_api.c (Main API)
4.    3,374  encoder/drc_src/impd_drc_mux.c (DRC muxing)
5.    2,963  encoder/ixheaace_mps_enc.c (MPS encoder)
6.    2,679  encoder/iusace_acelp_enc.c (ACELP)
7.    2,548  encoder/ixheaace_adjust_threshold.c (Rate control)
8.    2,485  encoder/ixheaace_fft.c (FFT)
9.    2,231  encoder/ixheaace_enc_main.c (Main encoder)
10.   2,190  encoder/ixheaace_write_bitstream.c (Bitstream)
11.   2,187  encoder/ixheaace_sbr_env_est.c (SBR envelope)
12.   2,175  encoder/iusace_psy_mod.c (USAC PSY model)
13.   2,159  encoder/iusace_tcx_enc.c (TCX encoder)
14.   2,011  encoder/ixheaace_enc_init.c (Initialization)
15.   2,002  encoder/ixheaace_sbr_main.c (SBR main)
16.   1,967  encoder/ixheaace_psy_mod.c (AAC PSY model)
17.   1,939  encoder/ixheaace_sbr_rom.c (SBR tables)
18.   1,937  encoder/iusace_write_bitstream.c (USAC bitstream)
19.   1,914  encoder/iusace_avq_enc.c (AVQ quantization)
20.   1,858  encoder/ixheaace_mps_param_extract.c (MPS params)
```

### Complexity Analysis by Component

```
┌──────────────────────────────────────────────────────────────────┐
│              COMPLEXITY ASSESSMENT                                │
├──────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Component               LOC      Files  Complexity  Risk         │
│  ───────────────────────────────────────────────────────────────│
│  Foundation & Tables     21,047    23    LOW         LOW         │
│  DSP Primitives          8,000     12    MEDIUM      LOW         │
│  Block Switching         2,500     3     MEDIUM      MEDIUM      │
│  Psychoacoustic Model    6,500     7     HIGH        HIGH        │
│  AAC Core Encoder        20,000    20    HIGH        MEDIUM      │
│  Rate Control            4,000     4     VERY HIGH   HIGH        │
│  USAC Encoder            25,000    20    VERY HIGH   VERY HIGH   │
│  SBR Encoder             15,000    25    HIGH        MEDIUM      │
│  PS Encoder              2,000     3     MEDIUM      LOW         │
│  MPS Encoder             12,000    35    HIGH        HIGH        │
│  DRC Encoder             8,500     8     MEDIUM      MEDIUM      │
│  Bitstream/API           5,000     12    MEDIUM      LOW         │
│  ───────────────────────────────────────────────────────────────│
│  TOTAL                   99,118    137                           │
└──────────────────────────────────────────────────────────────────┘

Complexity Factors:
• PSY Model: Complex perceptual algorithms, many tuning parameters
• Rate Control: Iterative optimization, convergence challenges
• USAC: Multiple coding modes, mode decisions, LPC analysis
• MPS: Spatial analysis, tree configurations, multi-channel coordination
```

### Estimated Rust Migration Metrics

```
┌──────────────────────────────────────────────────────────────────┐
│              RUST MIGRATION ESTIMATES                             │
├──────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Estimated Rust LOC:      ~75,000 - 80,000 lines                 │
│    (24-20% reduction from C due to:)                             │
│    • Iterator chains vs manual loops                             │
│    • Match expressions vs if-else                                │
│    • Type safety reduces defensive code                          │
│    • Macro usage for repetitive patterns                         │
│    • Result<T, E> eliminates error checking boilerplate          │
│                                                                   │
│  Estimated Rust Modules:  ~60-70 modules                         │
│  Estimated Rust Crates:   ~12 crates                             │
│                                                                   │
│  Module Breakdown:                                               │
│  ├─ xheaace_types          ~500 lines                            │
│  ├─ xheaace_tables         ~18,000 lines (mostly const data)     │
│  ├─ xheaace_dsp            ~6,000 lines                          │
│  ├─ xheaace_bitstream      ~1,500 lines                          │
│  ├─ xheaace_psy            ~5,000 lines                          │
│  ├─ xheaace_core           ~15,000 lines                         │
│  ├─ xheaace_usac           ~19,000 lines                         │
│  ├─ xheaace_sbr            ~11,000 lines                         │
│  ├─ xheaace_ps             ~1,500 lines                          │
│  ├─ xheaace_mps            ~9,000 lines                          │
│  ├─ xheaace_drc            ~6,500 lines                          │
│  └─ xheaace (top-level)    ~2,000 lines                          │
│                                                                   │
└──────────────────────────────────────────────────────────────────┘
```

---

## Rust Migration Strategy for Encoder

### Phase 1: Foundation (3-4 weeks)
**Priority:** START HERE  
**Complexity:** LOW  
**Risk:** LOW  

**Modules:**
- Basic types and constants
- Error handling infrastructure
- Bitstream writer
- Memory management abstractions

**Deliverables:**
- `xheaace_types` crate
- `xheaace_bitstream` crate
- Basic test infrastructure

**Success Criteria:**
- Type-safe bitstream writing
- Zero-copy where possible
- Const-correct error types

---

### Phase 2: Lookup Tables (2-3 weeks)
**Priority:** SECOND  
**Complexity:** LOW  
**Risk:** LOW  

**Modules:**
- Convert all ROM data to const arrays
- Generate large tables at build time
- Huffman codebook structures

**Deliverables:**
- `xheaace_tables` crate
- Build script for table generation
- Verification tests against C tables

**Rust Benefits:**
- Const generics for table sizes
- Compile-time table validation
- Zero runtime overhead

---

### Phase 3: DSP Primitives (5-6 weeks)
**Priority:** THIRD  
**Complexity:** MEDIUM  
**Risk:** LOW  

**Modules:**
- FFT/MDCT implementations
- Windowing functions
- QMF filterbank
- Resampling

**Deliverables:**
- `xheaace_dsp` crate
- Generic over sample types
- SIMD optimizations
- Comprehensive benchmarks

**Rust Benefits:**
- Generic transforms
- Safe SIMD via portable_simd
- Zero-cost abstractions

---

### Phase 4: Block Switching (3-4 weeks)
**Priority:** FOURTH  
**Complexity:** MEDIUM  
**Risk:** MEDIUM  

**Modules:**
- Attack detection
- Energy calculation
- Window sequence decision

**Deliverables:**
- Block switching as part of `xheaace_core`
- Attack detector state machine
- Reference comparison tests

**Rust Benefits:**
- Enum-based window types
- State machine via match
- Iterator-based energy calculation

---

### Phase 5: Psychoacoustic Model (8-10 weeks)
**Priority:** FIFTH  
**Complexity:** VERY HIGH  
**Risk:** HIGH  

**Modules:**
- Critical band analysis
- Spreading function
- Masking threshold calculation
- SMR computation

**Deliverables:**
- `xheaace_psy` crate
- Perceptual model implementations
- Extensive validation tests

**Challenges:**
- Complex floating-point calculations
- Many tuning parameters
- Perceptually sensitive

**Rust Benefits:**
- Type-safe band representations
- Functional-style computations
- Easier to verify correctness

---

### Phase 6: AAC Core Encoder (10-12 weeks)
**Priority:** SIXTH  
**Complexity:** VERY HIGH  
**Risk:** HIGH  

**Modules:**
- Main encoding loop
- Rate control
- Quantization
- Huffman encoding
- TNS analysis
- M/S stereo

**Deliverables:**
- `xheaace_core` crate
- Complete AAC-LC encoder
- Bitstream comparison with C

**Challenges:**
- Rate control convergence
- Quantization iterations
- Bit exact matching (difficult)

**Rust Benefits:**
- Iterator-based quantization
- Type-safe scale factors
- Clear ownership of spectral data

---

### Phase 7: USAC Encoder (12-16 weeks)
**Priority:** SEVENTH (optional)  
**Complexity:** VERY HIGH  
**Risk:** VERY HIGH  

**Modules:**
- LPC analysis
- ACELP encoder
- TCX encoder
- AVQ quantization
- Arithmetic encoder
- FAC
- Mode decision

**Deliverables:**
- `xheaace_usac` crate
- xHE-AAC support
- Complex mode switching

**Challenges:**
- Most complex encoder mode
- Multiple sub-modes
- Speech/music classification

**Rust Benefits:**
- Enum-based mode representation
- Type-safe LPC coefficients
- Clear mode state machines

---

### Phase 8: Enhancement Modules (Parallel, 16-20 weeks)
**Priority:** EIGHTH (parallel)  
**Complexity:** HIGH  
**Risk:** MEDIUM-HIGH  

**Modules (can be done in parallel):**

**SBR Encoder (6-8 weeks):**
- QMF analysis
- Envelope estimation
- Transient detection
- Tonality analysis
- Bitstream encoding

**PS Encoder (3-4 weeks):**
- Hybrid filterbank
- IID/ICC/IPD extraction
- Parameter encoding

**MPS Encoder (6-8 weeks):**
- Spatial analysis
- Tree configurations
- Downmix generation
- Parameter encoding

**DRC Encoder (4-6 weeks):**
- Gain calculation
- Loudness measurement (ITU-R BS.1770)
- Metadata generation

**Deliverables:**
- `xheaace_sbr` crate
- `xheaace_ps` crate
- `xheaace_mps` crate
- `xheaace_drc` crate
- Each as optional cargo feature

**Rust Benefits:**
- Trait-based interfaces
- Optional features
- Independent testing

---

### Phase 9: Bitstream Formatting & API (6-8 weeks)
**Priority:** NINTH  
**Complexity:** MEDIUM  
**Risk:** LOW  

**Modules:**
- Bitstream assembly
- Container formatting (ADTS/LATM)
- ASC generation
- Top-level API
- C FFI (if needed)

**Deliverables:**
- `xheaace` top-level crate
- Complete public API
- Container format writers
- C compatibility layer

**Rust Benefits:**
- Type-safe API
- Builder pattern for config
- Zero-cost FFI

---

### Phase 10: Integration & Optimization (8-10 weeks)
**Priority:** FINAL  
**Complexity:** MEDIUM  
**Risk:** LOW  

**Tasks:**
- End-to-end testing
- Performance optimization
- SIMD tuning
- Memory optimization
- Documentation
- Examples

**Deliverables:**
- Production-ready encoder
- Comprehensive test suite
- Performance benchmarks
- API documentation
- Example applications

---


## Timeline Estimates

### Sequential Timeline (Single Developer)

```
Phase 1:  Foundation                 3-4 weeks      ████
Phase 2:  Lookup Tables              2-3 weeks      ██
Phase 3:  DSP Primitives             5-6 weeks      ██████
Phase 4:  Block Switching            3-4 weeks      ████
Phase 5:  Psychoacoustic Model       8-10 weeks     ██████████
Phase 6:  AAC Core Encoder          10-12 weeks     ████████████
Phase 7:  USAC Encoder              12-16 weeks     ████████████████
Phase 8:  Enhancement Modules       16-20 weeks     ████████████████████
Phase 9:  Bitstream & API            6-8 weeks      ████████
Phase 10: Integration & Testing      8-10 weeks     ██████████
──────────────────────────────────────────────────────────────
Total:                              73-93 weeks     (~17-21 months)
```

### Parallelized Timeline (3-4 Developers)

```
Team Member 1: Foundation → DSP → AAC Core → Integration
Team Member 2: Tables → PSY Model → Rate Control → Testing
Team Member 3: Block Switch → USAC → Optimization
Team Member 4: SBR → PS → MPS → DRC (parallel enhancements)

Estimated Total: 12-15 months with proper coordination
```

---

## Encoder vs Decoder Migration Comparison

| Aspect | Decoder | Encoder |
|--------|---------|---------|
| **Complexity** | Lower (parsing + decoding) | Higher (analysis + optimization) |
| **LOC (C)** | 246,189 lines | 99,118 lines |
| **LOC (Rust)** | ~180-200k lines | ~75-80k lines |
| **Files** | 301 C files | 137 C files |
| **Timeline** | 11-16 months | 17-21 months (sequential) |
| **Critical Paths** | Huffman decode, IMDCT | PSY model, Rate control |
| **Parallelization** | High (modules independent) | Medium (some dependencies) |
| **Testing Complexity** | Medium (compare outputs) | High (perceptual quality) |
| **Most Complex Module** | MPS Decoder | USAC Encoder |

**Key Insight:** Encoder is ~2.5x less code but ~30% more complex due to:
- Psychoacoustic analysis (perceptually driven)
- Rate control iterations (optimization)
- Mode decisions (USAC FD vs LPD)
- Quality metrics (subjective testing needed)

---

## Critical Performance Paths

### Encoding Time Distribution (AAC-LC Profile)

```
┌─────────────────────────────────────────────────────────────────┐
│              ENCODER CPU TIME BREAKDOWN                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  MDCT/FFT:                 25-30% ████████████████              │
│  Psychoacoustic Analysis:  15-20% ████████████                  │
│  Quantization Loop:        15-20% ████████████                  │
│  Huffman Encoding:         10-15% ████████                      │
│  Rate Control:             8-12%  ██████                        │
│  Block Switching:          5-8%   ████                          │
│  TNS Analysis:             3-5%   ███                           │
│  Bitstream Writing:        2-3%   ██                            │
│  Other:                    5-10%  ████                          │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### HE-AAC Additional Overhead

```
SBR Encoding adds:         +40-50% encoding time
PS Encoding adds:          +10-15% (on top of SBR)
MPS Encoding adds:         +60-80% (multi-channel)
```

### Optimization Priorities for Rust

1. **MDCT/FFT (Highest Priority)**
   - Use SIMD aggressively
   - Consider rustfft or custom implementation
   - Benchmark against C version

2. **Psychoacoustic Model**
   - Vectorize band energy calculations
   - Optimize spreading function
   - Cache-friendly data layout

3. **Quantization Loop**
   - Minimize allocations
   - In-place operations
   - Fast convergence heuristics

4. **Huffman Encoding**
   - Table-based fast paths
   - Branch prediction friendly
   - Batch processing

---

## Rust-Specific Design Patterns for Encoder

### 1. Builder Pattern for Configuration

```rust
let encoder = EncoderBuilder::new()
    .sample_rate(48000)
    .channels(2)
    .bitrate(128_000)
    .profile(Profile::AacLc)
    .enable_sbr(true)
    .enable_ps(false)
    .output_format(OutputFormat::Adts)
    .build()?;
```

### 2. Trait-Based Quantizers

```rust
pub trait Quantizer {
    fn quantize(&mut self, 
                spectrum: &[f32], 
                available_bits: usize) 
                -> Result<QuantizedData, EncodeError>;
}

impl Quantizer for AacQuantizer { /* ... */ }
impl Quantizer for UsacQuantizer { /* ... */ }
```

### 3. Iterator-Based Spectral Processing

```rust
// Calculate energies per band
let energies: Vec<f32> = spectrum
    .chunks(band_width)
    .map(|band| band.iter().map(|x| x * x).sum())
    .collect();

// Apply spreading function
let masked_thresholds = energies
    .iter()
    .enumerate()
    .map(|(i, &e)| apply_spreading(i, e, &spreading_fn))
    .collect();
```

### 4. Type-Safe Window Types

```rust
pub enum WindowSequence {
    OnlyLong,
    LongStart,
    EightShort,
    LongStop,
}

pub struct WindowDecision {
    sequence: WindowSequence,
    attack_position: Option<usize>,
}
```

### 5. State Machine for USAC Mode

```rust
pub enum UsacMode {
    Fd { /* FD-specific state */ },
    Lpd { 
        submode: LpdSubmode,
        lpc_coeffs: [f32; 16],
    },
}

pub enum LpdSubmode {
    Acelp { pitch_lag: u16, gains: [f32; 4] },
    Tcx { length: TcxLength },
}
```

### 6. Error Handling with Context

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EncodeError {
    #[error("Rate control failed to converge after {0} iterations")]
    RateControlFailure(usize),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    
    #[error("Quantization overflow in band {band}, value {value}")]
    QuantizationOverflow { band: usize, value: f32 },
    
    #[error("Bitstream buffer full")]
    BufferFull,
}
```

---

## Testing Strategy for Encoder

### Test Pyramid

```
                    /\
                   /  \
                  / E2E \           End-to-End Tests (5%)
                 / Tests \          • Full encode → decode
                /          \        • Subjective quality
               /____________\       • Conformance tests
              /              \
             /  Integration   \     Integration Tests (15%)
            /     Tests        \    • Module interactions
           /                    \   • Bitrate accuracy
          /______________________\  • Format compliance
         /                        \
        /      Unit Tests          \ Unit Tests (80%)
       /                            \• Algorithm correctness
      /______________________________\• Perceptual model
                                      • Quantization
```

### Test Categories

**1. Unit Tests**
```rust
#[test]
fn test_psychoacoustic_spreading() {
    let energies = vec![1.0, 2.0, 1.5, 0.8];
    let spread = apply_spreading(&energies, &SPREADING_FUNC);
    assert_approx_eq!(spread[1], expected_value, epsilon = 0.01);
}

#[test]
fn test_huffman_encode_escape() {
    let values = [150, -200]; // Requires ESC
    let bits = huffman_encode(&values, Codebook::CB11);
    assert_eq!(bits.len(), expected_bit_length);
}
```

**2. Integration Tests**
```rust
#[test]
fn test_aac_lc_encode_stereo_48k() {
    let pcm = load_test_audio("test_48k_stereo.wav");
    let encoder = create_test_encoder(48000, 2, 128_000);
    
    let encoded = encoder.encode(&pcm)?;
    
    assert!(encoded.len() > 0);
    assert_bitrate_within_tolerance(encoded, 128_000, 0.05);
}
```

**3. Conformance Tests**
- ISO/IEC 14496-4 reference encoder tests
- Compare with reference implementation
- Bitstream syntax validation

**4. Quality Tests**
- PEAQ (Perceptual Evaluation of Audio Quality)
- PESQ (for speech, if USAC)
- ABX listening tests
- ODG (Objective Difference Grade) > -1.0 target

**5. Performance Tests**
```rust
#[bench]
fn bench_mdct_1024(b: &mut Bencher) {
    let input = vec![0.0f32; 1024];
    let mut output = vec![0.0f32; 1024];
    
    b.iter(|| {
        mdct(&input, &mut output);
    });
}
```

---

## Memory Architecture for Encoder

```
┌─────────────────────────────────────────────────────────────────┐
│                    ENCODER MEMORY LAYOUT                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  1. PERSISTENT MEMORY (~200-500 KB)                              │
│     • Encoder state structures                                   │
│     • Psychoacoustic model state                                 │
│     • Filter states (QMF, hybrid)                                │
│     • Previous frame data                                        │
│     • SBR/PS/MPS encoder states                                  │
│     • Block switching history                                    │
│                                                                   │
│  2. SCRATCH MEMORY (~128-256 KB)                                 │
│     • MDCT working buffers                                       │
│     • FFT scratch                                                │
│     • Quantization temporaries                                   │
│     • Rate control iterations                                    │
│                                                                   │
│  3. INPUT BUFFERS (~10-50 KB)                                    │
│     • PCM input ring buffer                                      │
│     • Lookahead for block switching                              │
│                                                                   │
│  4. OUTPUT BUFFERS (~10-20 KB)                                   │
│     • Bitstream output buffer                                    │
│     • Container formatting buffer                                │
│                                                                   │
│  5. ROM TABLES (~200-400 KB, read-only, shared)                 │
│     • Huffman codebooks                                          │
│     • Quantization tables                                        │
│     • Window coefficients                                        │
│     • Psychoacoustic tables                                      │
│     • FFT twiddle factors                                        │
│                                                                   │
│  Total: ~550-1200 KB per encoder instance                        │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

---

## Recommended Rust Crate Structure

```
workspace/
├── Cargo.toml (workspace root)
│
├── xheaace_types/         # Phase 1
│   ├── src/
│   │   ├── lib.rs
│   │   ├── constants.rs
│   │   ├── error.rs
│   │   └── config.rs
│   └── Cargo.toml
│
├── xheaace_tables/        # Phase 2
│   ├── src/
│   │   ├── lib.rs
│   │   ├── huffman.rs
│   │   ├── quantization.rs
│   │   ├── windows.rs
│   │   └── psychoacoustic.rs
│   ├── build.rs
│   └── Cargo.toml
│
├── xheaace_bitstream/     # Phase 1
│   ├── src/
│   │   ├── lib.rs
│   │   ├── writer.rs
│   │   └── adts.rs
│   └── Cargo.toml
│
├── xheaace_dsp/           # Phase 3
│   ├── src/
│   │   ├── lib.rs
│   │   ├── fft.rs
│   │   ├── mdct.rs
│   │   ├── qmf.rs
│   │   ├── hybrid.rs
│   │   └── resampler.rs
│   └── Cargo.toml
│
├── xheaace_psy/           # Phase 5
│   ├── src/
│   │   ├── lib.rs
│   │   ├── model.rs
│   │   ├── spreading.rs
│   │   ├── masking.rs
│   │   └── block_switch.rs
│   └── Cargo.toml
│
├── xheaace_core/          # Phase 6
│   ├── src/
│   │   ├── lib.rs
│   │   ├── encoder.rs
│   │   ├── quantizer.rs
│   │   ├── rate_control.rs
│   │   ├── huffman.rs
│   │   ├── tns.rs
│   │   └── stereo.rs
│   └── Cargo.toml
│
├── xheaace_usac/          # Phase 7
│   ├── src/
│   │   ├── lib.rs
│   │   ├── lpd/
│   │   │   ├── mod.rs
│   │   │   ├── acelp.rs
│   │   │   ├── tcx.rs
│   │   │   └── lpc.rs
│   │   ├── avq.rs
│   │   ├── arith.rs
│   │   └── fac.rs
│   └── Cargo.toml
│
├── xheaace_sbr/           # Phase 8
│   ├── src/
│   │   ├── lib.rs
│   │   ├── encoder.rs
│   │   ├── envelope.rs
│   │   ├── transient.rs
│   │   └── tonality.rs
│   └── Cargo.toml
│
├── xheaace_ps/            # Phase 8
│   └── ...
│
├── xheaace_mps/           # Phase 8
│   └── ...
│
├── xheaace_drc/           # Phase 8
│   └── ...
│
└── xheaace/               # Phase 9 (Top-level)
    ├── src/
    │   ├── lib.rs
    │   ├── api.rs
    │   ├── encoder.rs
    │   └── ffi.rs (optional C FFI)
    ├── examples/
    │   ├── encode_file.rs
    │   └── streaming_encode.rs
    ├── benches/
    │   └── encode_bench.rs
    └── Cargo.toml
```

---


## Key Advantages of Rust Migration (Encoder)

### 1. **Type Safety for Perceptual Parameters**
```rust
// Type-safe masking thresholds
pub struct MaskingThreshold {
    band: BandIndex,
    threshold_db: f32,
    signal_db: f32,
}

// Impossible to mix up band indices with sample values
pub struct BandIndex(usize);
pub struct SampleIndex(usize);
```

### 2. **Safe Rate Control Iterations**
```rust
// Guaranteed termination with iterator
pub fn rate_control_loop(
    spectrum: &[f32],
    target_bits: usize
) -> Result<QuantizedData, RateControlError> {
    (0..MAX_ITERATIONS)
        .find_map(|iter| {
            let quantized = quantize(spectrum, current_scale_factors);
            if quantized.bits <= target_bits {
                Some(Ok(quantized))
            } else {
                adjust_scale_factors();
                None
            }
        })
        .unwrap_or(Err(RateControlError::ConvergenceFailed))
}
```

### 3. **Functional-Style Psychoacoustic Analysis**
```rust
// Clean, composable perceptual calculations
let masked_thresholds = critical_bands
    .iter()
    .zip(energies.iter())
    .map(|(band, &energy)| {
        let tonal_masking = calculate_tonal_masking(band, energy);
        let noise_masking = calculate_noise_masking(band, energy);
        tonal_masking.min(noise_masking)
    })
    .collect();
```

### 4. **Memory Safety in Multi-threaded Encoding**
```rust
// Safe parallel frame encoding
use rayon::prelude::*;

let encoded_frames: Vec<_> = frames
    .par_iter()
    .map(|frame| encoder.encode_frame(frame))
    .collect();
```

### 5. **Zero-Cost Abstractions**
```rust
// No runtime overhead for type safety
#[repr(transparent)]
pub struct ScaleFactor(i16);

impl ScaleFactor {
    pub fn to_linear(&self) -> f32 {
        2.0f32.powf(self.0 as f32 / 4.0)
    }
}
```

---

## Summary Statistics

### Encoder Codebase Size Comparison

```
┌──────────────────────────────────────────────────────────────────┐
│                    SIZE COMPARISON                                │
├──────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Metric                        C Code        Estimated Rust      │
│  ─────────────────────────────────────────────────────────────── │
│  Source Files                  137           60-70 modules        │
│  Lines of Code                 99,118        75,000-80,000        │
│  Average File Size             723 lines     ~1,000-1,200 lines  │
│  Largest File                  15,083 lines  ~12,000 lines        │
│  Header Files                  147           Minimal (pub use)    │
│  Assembly Files                0             0 (use SIMD)         │
│  Lookup Table Size             ~21,000 lines ~18,000 lines        │
│  Main API File                 3,934 lines   ~2,000 lines         │
│                                                                   │
│  Reduction Factor:             1.0x          0.76-0.81x           │
│                                                                   │
└──────────────────────────────────────────────────────────────────┘
```

### Development Timeline Summary

```
┌──────────────────────────────────────────────────────────────────┐
│                    TIMELINE SUMMARY                               │
├──────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Sequential (1 developer):     73-93 weeks (17-21 months)        │
│  Parallelized (3-4 devs):      52-65 weeks (12-15 months)        │
│                                                                   │
│  Phase Breakdown:                                                │
│  ├─ Foundation + Tables:       5-7 weeks    (7.5%)               │
│  ├─ DSP Primitives:            5-6 weeks    (7.5%)               │
│  ├─ Block Switch + PSY:        11-14 weeks  (16%)                │
│  ├─ AAC Core:                  10-12 weeks  (14%)                │
│  ├─ USAC:                      12-16 weeks  (18%)                │
│  ├─ Enhancements:              16-20 weeks  (24%)                │
│  └─ Integration:               14-18 weeks  (20%)                │
│                                                                   │
│  Critical Path: PSY Model → AAC Core → Rate Control              │
│                                                                   │
└──────────────────────────────────────────────────────────────────┘
```

### Complexity Distribution

```
Component              Complexity    Lines    Effort     Risk
─────────────────────────────────────────────────────────────
Foundation & Tables    ★☆☆☆☆        21,047   Low        Low
DSP Primitives         ★★☆☆☆        8,000    Medium     Low
Block Switching        ★★★☆☆        2,500    Medium     Medium
Psychoacoustic Model   ★★★★★        6,500    Very High  High
AAC Core Encoder       ★★★★☆        20,000   High       Medium
Rate Control           ★★★★★        4,000    Very High  High
USAC Encoder           ★★★★★        25,000   Very High  Very High
SBR Encoder            ★★★★☆        15,000   High       Medium
PS Encoder             ★★☆☆☆        2,000    Medium     Low
MPS Encoder            ★★★★☆        12,000   High       High
DRC Encoder            ★★★☆☆        8,500    Medium     Medium
Bitstream/API          ★★☆☆☆        5,000    Medium     Low
```

---

## Critical Success Factors

### 1. **Perceptual Quality Validation**
- Implement PEAQ or similar objective quality metrics
- Setup listening test infrastructure
- Compare against reference implementation
- Target: ODG (Objective Difference Grade) ≥ -1.0

### 2. **Rate Control Stability**
- Ensure convergence in all scenarios
- Bitrate accuracy within ±5%
- Handle edge cases (silence, white noise, transients)
- Prevent bit reservoir overflow

### 3. **Performance Parity**
- Match or exceed C implementation speed
- Encode time ≤ 0.1x real-time on modern CPU
- Memory usage comparable to C version
- Profile and optimize hot paths

### 4. **Standards Compliance**
- Pass ISO/IEC 14496-4 conformance tests
- Valid bitstream syntax
- Interoperability with all decoders
- Correct ASC generation

### 5. **API Usability**
- Ergonomic Rust API
- Clear error messages
- Good documentation
- Example code

---

## Recommendations

### For Complete Migration (Decoder + Encoder)

**Suggested Approach: Decoder First**

1. **Start with Decoder** (11-16 months)
   - Less complex
   - Easier to validate (bitexact comparison)
   - Builds foundation for shared modules
   - Provides confidence for encoder

2. **Then Encoder** (12-15 months, leveraging decoder work)
   - Reuse DSP primitives from decoder
   - Shared tables and constants
   - Similar bitstream handling
   - Can validate encoder by decoding output

**Total Timeline: 23-31 months sequential**  
**With Overlap: 18-24 months (start encoder while finalizing decoder)**

### Team Composition Recommendation

**Minimum Viable Team (4 people):**
1. **Rust Expert** - Architecture, DSP, optimization
2. **Audio DSP Specialist** - PSY model, rate control, quality
3. **Standards Expert** - USAC, SBR, MPS, compliance
4. **QA Engineer** - Testing, validation, listening tests

**Optimal Team (6 people):**
- Add: Performance Engineer (SIMD, profiling)
- Add: DevOps Engineer (CI/CD, infrastructure)

---

## Conclusion

The xHE-AAC encoder migration to Rust is a substantial but achievable project:

**Pros:**
✅ 20-25% code reduction due to Rust expressiveness  
✅ Memory safety eliminates entire class of bugs  
✅ Type safety prevents perceptual parameter confusion  
✅ Modern tooling (cargo, docs, testing)  
✅ Better parallelization safety  
✅ Easier to maintain long-term  

**Cons:**
❌ 17-21 months development time (sequential)  
❌ High complexity in PSY model and rate control  
❌ Perceptual quality validation challenging  
❌ USAC encoder is very complex  
❌ Learning curve for team  

**Key Risks:**
1. Rate control convergence issues
2. Perceptual quality degradation
3. Performance not matching C
4. Timeline underestimation

**Mitigation Strategies:**
1. Extensive validation against reference
2. Comprehensive listening tests
3. Profile-guided optimization
4. Incremental delivery with milestones
5. Early proof-of-concept for critical modules

**Overall Assessment:** The encoder migration is feasible but requires:
- Experienced team with audio DSP knowledge
- 12-15 months with 3-4 developers
- Strong focus on quality validation
- Willingness to invest in thorough testing

---

## Next Steps

1. ✅ **Review architecture document** - Understand layers and dependencies
2. ⬜ **Proof of Concept** - Implement Phase 1 (Foundation) to validate approach
3. ⬜ **Resource Planning** - Assemble team, allocate time
4. ⬜ **Tooling Setup** - CI/CD, testing infrastructure, benchmarks
5. ⬜ **Begin Phase 1** - Foundation and lookup tables (5-7 weeks)
6. ⬜ **Milestone 1** - Working AAC-LC encoder (6-7 months)
7. ⬜ **Milestone 2** - Add SBR (HE-AAC) support (9-10 months)
8. ⬜ **Milestone 3** - Full xHE-AAC with USAC (12-15 months)

---

**Document Generated:** For Rust Migration Planning  
**Focus:** xHE-AAC Encoder Architecture  
**Detail Level:** Medium  
**Connection Minimization:** Optimized via 10-layer architecture  
**Migration Estimate:** 75,000-80,000 lines Rust, 12-15 months with team

---

**END OF ENCODER ARCHITECTURE DOCUMENT**

