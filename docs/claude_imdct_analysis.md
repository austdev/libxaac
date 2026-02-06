# IMDCT Functions Analysis

Analysis of `ixheaacd_fd_imdct_long()`, `ixheaacd_fd_imdct_short()`, and `ixheaacd_acelp_imdct()` functions from `decoder/ixheaacd_imdct.c`.

## Overview

These functions implement the Inverse Modified Discrete Cosine Transform (IMDCT) for USAC (Unified Speech and Audio Coding) decoder. The IMDCT is a critical component in transform-based audio codecs, converting frequency-domain coefficients back to time-domain samples with overlap-add reconstruction.

---

## 1. `ixheaacd_acelp_imdct()`

**Location:** `decoder/ixheaacd_imdct.c:186-208`

### Purpose

Low-level IMDCT computation wrapper that handles pre-processing and delegates to FFT-based IMDCT. Used for both ACELP (Algebraic Code-Excited Linear Prediction) transition frames and as a building block for long/short window IMDCT.

### Function Signature

```c
VOID ixheaacd_acelp_imdct(
    WORD32 *imdct_in,    // Input/output spectral coefficients (in-place transform)
    WORD32 npoints,      // Total number of points (2x the IMDCT length)
    WORD8 *qshift,       // Quantization shift factor (modified on output)
    WORD32 *tmp_data     // Scratch buffer for intermediate calculations
);
```

### Parameters

| Parameter | Type | Direction | Description |
|-----------|------|-----------|-------------|
| `imdct_in` | `WORD32*` | In/Out | Spectral coefficient buffer, transformed in-place to time-domain samples |
| `npoints` | `WORD32` | In | Number of points = 2 * IMDCT_length (e.g., 2048 for 1024-point IMDCT) |
| `qshift` | `WORD8*` | In/Out | Quantization shift; adjusted by preshift value on output |
| `tmp_data` | `WORD32*` | In | Scratch memory for FFT computation |

### Buffer Lengths

| Buffer | Size (WORD32) | Notes |
|--------|---------------|-------|
| `imdct_in` | `npoints / 2` | Contains `npoints/2` samples (actual IMDCT length) |
| `tmp_data` | 1024 | Scratch buffer from `usac_data->scratch_buffer` |

### Algorithm

1. **Calculate preshift**: Determines scaling based on the FFT radix factorization
   - Counts powers of 2 in `(npoints/2) / 2`
   - For non-power-of-2 lengths (e.g., 384, 768), applies `/3` scaling with extra shift

2. **FFT-based IMDCT**: Calls `ixheaacd_fft_based_imdct()` with half the points

3. **Shift adjustment**: Adds 2 to preshift and subtracts from output `qshift`

### Supported Lengths

| npoints | IMDCT Length | Use Case |
|---------|--------------|----------|
| 2048 | 1024 | Long window |
| 1536 | 768 | Long window (alternative) |
| 256 | 128 | Short window |
| 192 | 96 | Short window (alternative) |

---

## 2. `ixheaacd_fd_imdct_long()`

**Location:** `decoder/ixheaacd_imdct.c:477-594`

### Purpose

Performs frequency-domain IMDCT for **long window sequences**. Handles overlap-add processing, windowing, and optional bass post-filter (BPF) application for frames following LPD (Linear Prediction Domain) coding.

### Function Signature

```c
static IA_ERRORCODE ixheaacd_fd_imdct_long(
    ia_usac_data_struct *usac_data,         // Main decoder state structure
    WORD32 i_ch,                            // Channel index
    WORD32 *fac_idata,                      // FAC (Forward Aliasing Cancellation) data
    offset_lengths *ixheaacd_drc_offset,    // Window offset/length parameters
    WORD8 fac_q                             // FAC quantization factor
);
```

### Parameters

| Parameter | Type | Direction | Description |
|-----------|------|-----------|-------------|
| `usac_data` | `ia_usac_data_struct*` | In/Out | Main USAC decoder state containing all channel buffers and flags |
| `i_ch` | `WORD32` | In | Channel index (0 to MAX_NUM_CHANNELS-1) |
| `fac_idata` | `WORD32*` | In | FAC data for aliasing cancellation at ACELP/FD transitions |
| `ixheaacd_drc_offset` | `offset_lengths*` | In | Structure with window parameters |
| `fac_q` | `WORD8` | In | Quantization factor for FAC data |

### Buffer Lengths

| Buffer | Size (WORD32) | Source |
|--------|---------------|--------|
| `p_in_ibuffer` (coef_fix) | 4096 | `usac_data->coef_fix[i_ch]` |
| `p_overlap_ibuffer` | 4096 | `usac_data->overlap_data_ptr[i_ch]` |
| `p_out_ibuffer` | 4096 | `usac_data->output_data_ptr[i_ch]` |
| `p_out_buffer` | 4096 | `usac_data->time_sample_vector[i_ch]` (FLOAT32) |
| `scratch_mem` | 1024 | `usac_data->scratch_buffer` |
| `fac_idata` | 2*FAC_LENGTH+16 = 272 | Caller-allocated |

### Window Sequences Handled

| Sequence | Value | Description |
|----------|-------|-------------|
| `ONLY_LONG_SEQUENCE` | 0 | Standard long window |
| `LONG_START_SEQUENCE` | 1 | Long-to-short transition |
| `STOP_START_SEQUENCE` | 4 | Short-to-long-to-short |
| `LONG_STOP_SEQUENCE` | 3 | Short-to-long transition |

### Processing Steps

1. **Input normalization**: Find max spectral line, normalize buffer
2. **IMDCT computation**: Call `ixheaacd_acelp_imdct()` with `2 * n_long` points
3. **Post-normalization**: Re-normalize transformed data
4. **Windowing**: Apply window function based on sequence type:
   - `ONLY_LONG/LONG_START`: Use `ixheaacd_windowing_long1()`
   - `STOP_START/LONG_STOP` with FAC: Use `ixheaacd_windowing_long2()`
   - `STOP_START/LONG_STOP` without FAC: Use `ixheaacd_windowing_long3()`
5. **Overlap buffer update**: Store second half for next frame
6. **Scale adjustment**: Apply `ixheaacd_scale_down_adj()` to output
7. **BPF (optional)**: If previous frame was time-domain, apply bass post-filter via `ixheaacd_lpd_bpf_fix()`

---

## 3. `ixheaacd_fd_imdct_short()`

**Location:** `decoder/ixheaacd_imdct.c:336-475`

### Purpose

Performs frequency-domain IMDCT for **short window sequences** (8 consecutive short windows). Handles the more complex overlap-add of 8 short transforms and their combination with the previous frame's overlap buffer.

### Function Signature

```c
static IA_ERRORCODE ixheaacd_fd_imdct_short(
    ia_usac_data_struct *usac_data,         // Main decoder state structure
    WORD32 i_ch,                            // Channel index
    WORD32 *fac_data_out,                   // FAC data output
    offset_lengths *ixheaacd_drc_offset,    // Window offset/length parameters
    WORD8 fac_q                             // FAC quantization factor
);
```

### Parameters

Same as `ixheaacd_fd_imdct_long()`.

### Buffer Lengths

| Buffer | Size (WORD32) | Source |
|--------|---------------|--------|
| `overlap_data_buf` | 2 * N_LONG_LEN_MAX = 2048 | Local stack allocation |
| `p_in_ibuffer` (coef_fix) | 4096 | `usac_data->coef_fix[i_ch]` |
| `p_overlap_ibuffer` | 4096 | `usac_data->overlap_data_ptr[i_ch]` |
| `p_out_ibuffer` | 4096 | `usac_data->output_data_ptr[i_ch]` |
| `p_out_buffer` | 4096 | `usac_data->time_sample_vector[i_ch]` (FLOAT32) |
| `scratch_mem` | 1024 | `usac_data->scratch_buffer` |

### Processing Steps

1. **Copy overlap data**: Load previous frame's overlap into local buffer
2. **IMDCT for 8 short blocks**: Loop calling `ixheaacd_acelp_imdct()` with `2 * n_short` points each
3. **Normalize**: Find max and normalize transformed data
4. **Window calculation**: Get window tables for current and previous shapes
5. **First window processing**:
   - With FAC: `ixheaacd_windowing_short1()`
   - Without FAC: `ixheaacd_windowing_short2()`
6. **Second window**: `ixheaacd_windowing_short3()`
7. **Windows 2-7**: Loop with `ixheaacd_windowing_short4()`
8. **Last window**: `ixheaacd_windowing_short4()` with flag=0
9. **FAC combination**: If FAC present, call `ixheaacd_combine_fac()`
10. **Scale and store**: Scale overlap data, store to output buffers
11. **BPF (optional)**: Apply bass post-filter if previous frame was time-domain

---

## Key Structures

### `offset_lengths`

**Location:** `decoder/ixheaacd_windows.h:54-61`

```c
typedef struct {
    WORD32 lfac;         // FAC length (n_long/8 or n_long/16 for short)
    WORD32 n_flat_ls;    // Flat region length for long-start/stop windows
    WORD32 n_trans_ls;   // Transition region length (= 2 * lfac)
    WORD32 n_long;       // Long window length (typically 1024 or 768)
    WORD32 n_short;      // Short window length (n_long / 8)
} offset_lengths;
```

**Typical Values:**

| Field | Long Window (1024) | Short Window (128) |
|-------|-------------------|-------------------|
| `n_long` | 1024 | 1024 |
| `n_short` | 128 | 128 |
| `lfac` (TD prev) | 128 (n_long/8) | 64 (n_long/16) |
| `lfac` (FD prev) | 128 (FAC_LENGTH) | 128 (FAC_LENGTH) |
| `n_flat_ls` | (n_long - 2*lfac)/2 | (n_long - 2*lfac)/2 |
| `n_trans_ls` | 2*lfac | 2*lfac |

### `ia_usac_data_struct` (Relevant Fields)

**Location:** `decoder/ixheaacd_main.h:75-232`

| Field | Type | Description |
|-------|------|-------------|
| `coef_fix[i_ch]` | `WORD32*` | Fixed-point spectral coefficients (input) |
| `overlap_data_ptr[i_ch]` | `WORD32[4096]` | Overlap-add buffer from previous frame |
| `output_data_ptr[i_ch]` | `WORD32[4096]` | Fixed-point output samples |
| `time_sample_vector[i_ch]` | `FLOAT32[4096]` | Floating-point output samples |
| `scratch_buffer` | `WORD32[1024]` | Scratch memory for IMDCT |
| `window_shape[i_ch]` | `WORD32` | Current window shape (0=sine, 1=KBD) |
| `window_shape_prev[i_ch]` | `WORD32` | Previous window shape |
| `window_sequence[i_ch]` | `WORD32` | Window sequence type (0-4) |
| `td_frame_prev[i_ch]` | `WORD32` | Flag: previous frame was time-domain |
| `fac_data_present[i_ch]` | `WORD32` | Flag: FAC data available |
| `fac_data[i_ch]` | `WORD32[129]` | FAC coefficients |
| `ccfl` | `WORD32` | Core Coder Frame Length (1024 or 768) |
| `str_tddec[i_ch]` | `ia_usac_lpd_decoder_handle` | LPD decoder state for BPF |
| `ec_flag` | `WORD32` | Error concealment flag |

---

## Constants

| Constant | Value | Description |
|----------|-------|-------------|
| `N_LONG_LEN_MAX` | 1024 | Maximum long window length |
| `FAC_LENGTH` | 128 | Forward Aliasing Cancellation length |
| `ORDER` | 16 | LPC filter order |
| `LEN_FRAME` | 256 | Base frame length |
| `LEN_SUBFR` | 64 | Subframe length |
| `FILTER_DELAY` | 12 | BPF filter delay |

---

## Call Graph

```
ixheaacd_fd_frm_dec()
    |
    +-- ixheaacd_cal_fac_data()  [if FAC present]
    |
    +-- ixheaacd_fd_imdct_long()  [if not EIGHT_SHORT_SEQUENCE]
    |       |
    |       +-- ixheaacd_calc_max_spectralline()
    |       +-- ixheaacd_normalize()
    |       +-- ixheaacd_acelp_imdct()
    |       |       |
    |       |       +-- ixheaacd_fft_based_imdct()
    |       |               |
    |       |               +-- ixheaacd_calc_pre_twid()
    |       |               +-- ixheaacd_complex_fft()
    |       |               +-- ixheaacd_calc_post_twid()
    |       |
    |       +-- ixheaacd_calc_window()
    |       +-- ixheaacd_windowing_long1/2/3()
    |       +-- ixheaacd_scale_down_adj()
    |       +-- ixheaacd_lpd_bpf_fix()  [if td_frame_prev]
    |
    +-- ixheaacd_fd_imdct_short()  [if EIGHT_SHORT_SEQUENCE]
            |
            +-- ixheaacd_calc_max_spectralline()
            +-- ixheaacd_normalize()
            +-- ixheaacd_acelp_imdct() x 8
            +-- ixheaacd_calc_window()
            +-- ixheaacd_windowing_short1/2/3/4()
            +-- ixheaacd_combine_fac()  [if FAC present]
            +-- ixheaacd_scale_down()
            +-- ixheaacd_lpd_bpf_fix()  [if td_frame_prev]
```

---

## Notes for Rust Migration

1. **Fixed-point arithmetic**: All operations use WORD32 (32-bit signed) with explicit shift tracking via `qshift`/`shiftp`/`output_q` variables

2. **In-place transforms**: `ixheaacd_acelp_imdct()` transforms data in-place

3. **Scratch buffer sharing**: The 1024-element `scratch_buffer` is shared across operations

4. **Window tables**: Pre-computed sine and KBD window tables are selected based on `window_shape` and length

5. **Error handling**: Functions return `IA_ERRORCODE` (-1 for error, 0 for success) with early validation of lengths

6. **EC (Error Concealment) mode**: When `ec_flag` is set, functions use alternative parameters and skip some validations
