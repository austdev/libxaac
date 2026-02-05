# Basic Operations Analysis - Parameter Documentation

## Overview

This document analyzes `ixheaacd_basic_ops.c` and documents the purpose of each parameter in the Rust implementation (`ixheaacd_basic_ops.rs`). These functions implement DSP primitives for AAC decoding, focusing on windowing operations and Q-format conversions.

---

## 1. `combine_fac()` - Combine FAC Data

**Purpose:** Combines Forward Aliasing Cancellation (FAC) data by adding two sources with Q-format alignment.

**C Signature:**
```c
VOID ixheaacd_combine_fac(WORD32 *src1, WORD32 *src2, WORD32 *dest, WORD32 len,
                          WORD8 output_q, WORD8 fac_q);
```

**Rust Signature:**
```rust
pub fn combine_fac(src1: &[i32], src2: &[i32], dest: &mut [i32],
                   shift1: i8, shift2: i8)
```

### Parameters:

| Parameter | C Type | Rust Type | Purpose |
|-----------|--------|-----------|---------|
| `src1` | `WORD32*` | `&[i32]` | First input buffer (overlap data) |
| `src2` | `WORD32*` | `&[i32]` | Second input buffer (FAC data) |
| `dest` | `WORD32*` | `&mut [i32]` | Output buffer for combined result |
| `output_q` | `WORD8` | `shift1: i8` | Q-format of output/src1 |
| `fac_q` | `WORD8` | `shift2: i8` | Q-format of FAC data (src2) |

### Q-Format Handling:
- If `fac_q > output_q`: Right-shift src2 before adding
- If `fac_q < output_q`: Left-shift src2 with saturation before adding
- Result has `output_q` precision

**Rust Changes:**
- Length is implicit from slice bounds checking
- Parameter names changed to generic `shift1/shift2` (could be improved for clarity)

---

## 2. `windowing_long1()` - Long Block Windowing (Overlap-Add)

**Purpose:** Performs overlap-add synthesis for long IMDCT blocks with different Q-formats for current and overlap data.

**C Signature:**
```c
WORD8 ixheaacd_windowing_long1(WORD32 *src1, WORD32 *src2,
                               const WORD32 *win_fwd, const WORD32 *win_rev,
                               WORD32 *dest, WORD32 vlen, WORD8 shift1,
                               WORD8 shift2);
```

**Rust Signature:**
```rust
pub fn windowing_long1(src1: &[i32], src2: &[i32],
                       win_fwd: &[i32], win_rev: &[i32],
                       dest: &mut [i32], vlen: i32,
                       shift1: i8, shift2: i8) -> i8
```

### Parameters:

| Parameter | C Type | Rust Type | Purpose |
|-----------|--------|-----------|---------|
| `src1` | `WORD32*` | `&[i32]` | Current IMDCT output (first half) |
| `src2` | `WORD32*` | `&[i32]` | Current IMDCT output (second half) |
| `win_fwd` | `const WORD32*` | `&[i32]` | Forward window coefficients (vlen/2 samples) |
| `win_rev` | `const WORD32*` | `&[i32]` | Reverse window coefficients (vlen/2 samples, descending) |
| `dest` | `WORD32*` | `&mut [i32]` | Output buffer (vlen samples) |
| `vlen` | `WORD32` | `i32` | Vector length (total samples to process) |
| `shift1` | `WORD8` | `i8` | Q-format of src1 (current IMDCT) |
| `shift2` | `WORD8` | `i8` | Q-format of src2 (overlap buffer) |
| **Return** | `WORD8` | `i8` | **Output Q-format** (min of shift1, shift2) |

### Algorithm:
Processes `vlen/2` samples:
- `dest[i] = src1[i] * win_fwd[i] + src2[i] * win_rev[i]` (Q-aligned)
- `dest[vlen-i-1] = -src1[i] * win_rev[i] + src2[vlen-i-1] * win_fwd[i]` (mirrored)

---

## 3. `windowing_long2()` - Long Block with FAC Transition

**Purpose:** Handles long block windowing when transitioning from short to long blocks, incorporating FAC data.

**C Signature:**
```c
WORD8 ixheaacd_windowing_long2(WORD32 *src1, const WORD32 *win_fwd,
                               WORD32 *fac_data_out, WORD32 *over_lap,
                               WORD32 *p_out_buffer,
                               offset_lengths *ixheaacd_drc_offset,
                               WORD8 shiftp, WORD8 shift_olap, WORD8 fac_q);
```

**Rust Signature:**
```rust
pub fn windowing_long2(src1: &[i32], win_fwd: &[i32],
                       fac_data_out: &mut [i32], over_lap: &mut [i32],
                       p_out_buffer: &mut [i32],
                       ixheaacd_drc_offset: &OffsetLengths,
                       shift1: i8, shift2: i8, shift3: i8) -> i8
```

### Parameters:

| Parameter | C Type | Rust Type | Purpose |
|-----------|--------|-----------|---------|
| `src1` | `WORD32*` | `&[i32]` | Current IMDCT output (long block) |
| `win_fwd` | `const WORD32*` | `&[i32]` | Forward window coefficients |
| `fac_data_out` | `WORD32*` | `&mut [i32]` | FAC transition data |
| `over_lap` | `WORD32*` | `&mut [i32]` | Overlap buffer from previous frame |
| `p_out_buffer` | `WORD32*` | `&mut [i32]` | Output buffer (n_long samples) |
| `ixheaacd_drc_offset` | `offset_lengths*` | `&OffsetLengths` | Frame geometry (lfac, n_flat_ls, n_trans_ls, n_long) |
| `shiftp` | `WORD8` | `shift1: i8` | Q-format of current IMDCT (src1) |
| `shift_olap` | `WORD8` | `shift2: i8` | Q-format of overlap buffer |
| `fac_q` | `WORD8` | `shift3: i8` | Q-format of FAC data |
| **Return** | `WORD8` | `i8` | **Output Q-format** |

### Frame Regions (controlled by `ixheaacd_drc_offset`):
1. **Flat left** `[0 .. n_flat_ls + lfac)`: Copy overlap buffer
2. **Transition** `[n_flat_ls + lfac .. n_flat_ls + n_trans_ls)`: Windowed IMDCT + FAC
3. **Flat middle** `[n_flat_ls + n_trans_ls .. n_flat_ls + 3*lfac)`: Direct IMDCT + FAC
4. **Right half** `[n_flat_ls + 3*lfac .. n_long)`: Negated IMDCT only

---

## 4. `windowing_long3()` - Long Block Standard Overlap-Add

**Purpose:** Standard overlap-add for long blocks with flat and transition regions.

**C Signature:**
```c
WORD8 ixheaacd_windowing_long3(WORD32 *src1, const WORD32 *win_fwd,
                               WORD32 *over_lap, WORD32 *p_out_buffer,
                               const WORD32 *win_rev,
                               offset_lengths *ixheaacd_drc_offset,
                               WORD8 shiftp, WORD8 shift_olap);
```

**Rust Signature:**
```rust
pub fn windowing_long3(src1: &[i32], win_fwd: &[i32],
                       over_lap: &mut [i32], p_out_buffer: &mut [i32],
                       win_rev: &[i32],
                       ixheaacd_drc_offset: &OffsetLengths,
                       shift1: i8, shift2: i8) -> i8
```

### Parameters:

| Parameter | C Type | Rust Type | Purpose |
|-----------|--------|-----------|---------|
| `src1` | `WORD32*` | `&[i32]` | Current IMDCT output |
| `win_fwd` | `const WORD32*` | `&[i32]` | Forward window coefficients |
| `over_lap` | `WORD32*` | `&mut [i32]` | Overlap buffer from previous frame |
| `p_out_buffer` | `WORD32*` | `&mut [i32]` | Output buffer |
| `win_rev` | `const WORD32*` | `&[i32]` | Reverse window coefficients |
| `ixheaacd_drc_offset` | `offset_lengths*` | `&OffsetLengths` | Frame geometry |
| `shiftp` | `WORD8` | `shift1: i8` | Q-format of current IMDCT |
| `shift_olap` | `WORD8` | `shift2: i8` | Q-format of overlap buffer |
| **Return** | `WORD8` | `i8` | **Output Q-format** |

### Frame Regions:
1. **Flat left** `[0 .. n_flat_ls)`: Copy overlap buffer
2. **First transition** `[n_flat_ls .. n_long/2)`: `src1[i] * win_fwd + overlap[i] * win_rev`
3. **Second transition** `[n_long/2 .. n_flat_ls + n_trans_ls)`: `-src1[n_long-i-1] * win_fwd + overlap[i] * win_rev`
4. **Right** `[n_flat_ls + n_trans_ls .. n_long)`: Negated IMDCT only

---

## 5. `windowing_short1()` - Short Block Initial Processing

**Purpose:** Initializes overlap buffer for short block processing, handling FAC region.

**C Signature:**
```c
VOID ixheaacd_windowing_short1(WORD32 *src1, WORD32 *src2, WORD32 *fp,
                               offset_lengths *ixheaacd_drc_offset,
                               WORD8 shiftp, WORD8 shift_olap);
```

**Rust Signature:**
```rust
pub fn windowing_short1(src1: &[i32], src2: &[i32], fp: &mut [i32],
                        ixheaacd_drc_offset: &OffsetLengths,
                        shiftp: i8, shift_olap: i8)
```

### Parameters:

| Parameter | C Type | Rust Type | Purpose |
|-----------|--------|-----------|---------|
| `src1` | `WORD32*` | `&[i32]` | Current IMDCT output (short block) |
| `src2` | `WORD32*` | `&[i32]` | Window coefficients |
| `fp` | `WORD32*` | `&mut [i32]` | **In/Out** overlap buffer (modified in-place) |
| `ixheaacd_drc_offset` | `offset_lengths*` | `&OffsetLengths` | Frame geometry (lfac, n_flat_ls, n_short) |
| `shiftp` | `WORD8` | `i8` | Q-format of current IMDCT |
| `shift_olap` | `WORD8` | `i8` | Q-format of overlap buffer (fp) |

### Processing:
- Adjusts first `lfac` samples of `fp` for Q-format alignment
- If `n_short > lfac`: Fills `[lfac .. n_short)` with windowed IMDCT
- Zeros out `[n_short .. n_flat_ls + lfac)`

---

## 6. `windowing_short2()` - Short Block Overlap-Add

**Purpose:** Performs overlap-add for short blocks with window application.

**C Signature:**
```c
VOID ixheaacd_windowing_short2(WORD32 *src1, WORD32 *win_fwd, WORD32 *fp,
                               offset_lengths *ixheaacd_drc_offset,
                               WORD8 shiftp, WORD8 shift_olap);
```

**Rust Signature:**
```rust
pub fn windowing_short2(src1: &[i32], win_fwd: &[i32], fp: &mut [i32],
                        ixheaacd_drc_offset: &OffsetLengths,
                        shiftp: i8, shift_olap: i8)
```

### Parameters:

| Parameter | C Type | Rust Type | Purpose |
|-----------|--------|-----------|---------|
| `src1` | `WORD32*` | `&[i32]` | Current IMDCT output (short block) |
| `win_fwd` | `WORD32*` | `&[i32]` | Window coefficients (n_short samples) |
| `fp` | `WORD32*` | `&mut [i32]` | **In/Out** overlap buffer |
| `ixheaacd_drc_offset` | `offset_lengths*` | `&OffsetLengths` | Frame geometry |
| `shiftp` | `WORD8` | `i8` | Q-format of current IMDCT |
| `shift_olap` | `WORD8` | `i8` | Q-format of overlap buffer |

### Processing:
- `fp[i] = src1[i] * win_fwd[i] + fp[i] * win_rev[i]` (Q-aligned)
- `fp[n_short-i-1] = -src1[i] * win_rev[i] + fp[n_short-i-1] * win_fwd[i]`
- Zeros out `[n_short .. n_flat_ls + n_short)`

---

## 7. `windowing_short3()` - Short Block Finalization

**Purpose:** Final windowing stage for short blocks in EIGHT_SHORT_SEQUENCE mode.

**C Signature:**
```c
WORD8 ixheaacd_windowing_short3(WORD32 *src1, WORD32 *win_rev, WORD32 *fp,
                                WORD32 nshort, WORD8 shiftp, WORD8 shift_olap);
```

**Rust Signature:**
```rust
pub fn windowing_short3(src1: &[i32], win_rev: &[i32], fp: &mut [i32],
                        shiftp: i8, shift_olap: i8) -> i8
```

### Parameters:

| Parameter | C Type | Rust Type | Purpose |
|-----------|--------|-----------|---------|
| `src1` | `WORD32*` | `&[i32]` | Current IMDCT output (second half used: `[n_short/2 .. n_short)`) |
| `win_rev` | `WORD32*` | `&[i32]` | Reverse window coefficients |
| `fp` | `WORD32*` | `&mut [i32]` | **In/Out** overlap buffer |
| `shiftp` | `WORD8` | `i8` | Q-format of current IMDCT |
| `shift_olap` | `WORD8` | `i8` | Q-format of overlap buffer |
| **Return** | `WORD8` | `i8` | **Output Q-format** (min of shiftp, shift_olap) |

### Processing:
- Processes `n_short/2` samples using second half of src1
- `fp[i] = -src1[n_short/2-i-1] * win_rev + fp[i]` (Q-aligned)
- `fp[n_short-i-1] = -src1[n_short/2-i-1] * win_fwd + fp[n_short-i-1]`

---

## 8. `windowing_short4()` - Eight Short Sequence Processing

**Purpose:** Performs overlap-add synthesis with windowing for short IMDCT blocks in AAC's EIGHT_SHORT_SEQUENCE mode, ensuring smooth transitions between consecutive audio frames.

**C Signature:**
```c
WORD8 ixheaacd_windowing_short4(WORD32 *src1, WORD32 *win_fwd, WORD32 *fp,
                                WORD32 *win_fwd1, WORD32 nshort, WORD32 flag,
                                WORD8 shiftp, WORD8 shift_olap, WORD8 output_q);
```

**Rust Signature:**
```rust
pub fn windowing_short4(
    src1: &[i32],          // Read-only in Rust (was mutable in C)
    win_fwd: &[i32],       // Read-only in Rust (was mutable in C)
    fp: &mut [i32],        // In/Out overlap buffer
    win_rev1: &[i32],      // Read-only in Rust (was POINTER ARITHMETIC in C!)
    windowed_flag: bool,   // Type-safe boolean (was WORD32 in C)
    shiftp: i8,
    shift_olap: i8,
    output_q: i8
) -> i8
```

### Parameters - DETAILED ANALYSIS:

| Parameter | C Type | Rust Type | **Purpose & Notes** |
|-----------|--------|-----------|---------------------|
| `src1` | `WORD32*` | `&[i32]` | **Current IMDCT output** (n_short samples). Contains time-domain audio for current short block. C version mutates via pointer arithmetic; Rust uses immutable slice. |
| `win_fwd` | `WORD32*` | `&[i32]` | **Forward window coefficients** (n_short samples). Typically sine/Kaiser-Bessel window for smooth transitions. C uses mutable pointer; Rust uses immutable slice. |
| `fp` | `WORD32*` | `&mut [i32]` | **Overlap buffer (In/Out)** - MUST be 2*n_short samples! Contains overlap from previous frame, updated with current frame's overlap for next iteration. This is the only truly mutable parameter. |
| `win_fwd1` | `WORD32*` | `win_rev1: &[i32]` | **⚠️ TRICKY PARAMETER!** In C, this is a **pointer to END of window array** (points to `win_array + n_short - 1`). C code accesses it with decrement (`win_fwd1--`). In Rust, we pass the **base array** and compute offset in unsafe block: `win_fwd1 = win_rev1.as_ptr().add(n_short - 1)`. Represents backward/reverse window coefficients. |
| `nshort` | `WORD32` | *implicit* | **Short block length** (typically 128 samples for AAC). In Rust, inferred from `src1.len()` via slice bounds. |
| `flag` | `WORD32` | `windowed_flag: bool` | **Windowing mode**: `1/true` = apply second window stage, `0/false` = direct copy without windowing. C uses integer (0/1); Rust uses type-safe boolean. |
| `shiftp` | `WORD8` | `i8` | **Q-format of current IMDCT** (src1). Represents fixed-point precision (Q15 = 15 fractional bits). |
| `shift_olap` | `WORD8` | `i8` | **Q-format of overlap buffer** (fp). May differ from current frame due to adaptive scaling. |
| `output_q` | `WORD8` | `i8` | **Target Q-format for output**. Function aligns all data to this precision. |
| **Return** | `WORD8` | `i8` | **Actual output Q-format** achieved (returns `output_q` or `shift_olap` depending on branch). |

### Q-Format Alignment Logic:

The function handles **three different Q-formats** simultaneously:
1. `shiftp` - precision of current IMDCT
2. `shift_olap` - precision of overlap buffer
3. `output_q` - target precision

**Branch Selection:**
```
if (shift_olap > output_q):
    // Scale DOWN overlap buffer to match output_q
    // Return output_q
else:
    // Scale DOWN output to match shift_olap
    // Return shift_olap
```

### Processing Stages:

**Stage 1: First Half Windowing** `[0 .. n_short/2)`
- Reads second half of src1: `src1[n_short/2 + i]`
- Applies forward/reverse windows
- Updates first half and mirror positions of `fp`
- Formula: `fp[i] = src1[n_short/2 + i] * win_fwd[i] + fp[i]` (Q-aligned)

**Stage 2: Second Half Processing** `[n_short/2 .. n_short)` - Controlled by `windowed_flag`

**If `windowed_flag == true`:**
- Applies backward window using `win_fwd1` (reverse coefficients)
- Formula: `fp[i + n_short/2] = -src1[n_short - i - 1] * win_fwd1 + fp[i + n_short/2]`

**If `windowed_flag == false`:**
- Direct negated copy without windowing
- Formula: `fp[i + n_short/2] = -src1[n_short - i - 1] + fp[i + n_short/2]`

### Rust-Specific Changes:

1. **Pointer Arithmetic Eliminated:**
   - C: `win_fwd1--` (decrements pointer)
   - Rust: Pass base array, compute offset in unsafe block

2. **Type Safety:**
   - C: `WORD32 flag` (0 or 1)
   - Rust: `bool windowed_flag`

3. **Length Inference:**
   - C: Explicit `WORD32 nshort` parameter
   - Rust: Inferred from slice length with assertions

4. **Const Correctness:**
   - C: All pointers mutable (`WORD32*`)
   - Rust: Only `fp` is mutable; inputs are immutable references

### Call Site Example (C to Rust):

**C Call:**
```c
// C code passes pointer to END of window array!
WORD32 *win_end = &window_array[n_short - 1];
WORD8 result_q = ixheaacd_windowing_short4(
    imdct_out, window_fwd, overlap_buf,
    win_end,  // ← Pointer to last element!
    n_short, 1,
    15, 14, 12
);
```

**Rust Call:**
```rust
// Rust code passes base array, offset computed internally
let result_q = windowing_short4(
    &imdct_out, &window_fwd, &mut overlap_buf,
    &window_array,  // ← Pass base array, not pointer to end!
    true,  // Type-safe boolean
    15, 14, 12
);
```

---

## 9. `scale_down()` - Q-Format Conversion

**Purpose:** Converts audio samples between Q-formats (fixed-point precision scaling) with saturation.

**C Signature:**
```c
VOID ixheaacd_scale_down(WORD32 *dest, WORD32 *src, WORD32 len,
                         WORD8 shift1, WORD8 shift2);
```

**Rust Signature:**
```rust
pub fn scale_down(dest: &mut [i32], src: &[i32], shift1: i8, shift2: i8)
```

### Parameters:

| Parameter | C Type | Rust Type | Purpose |
|-----------|--------|-----------|---------|
| `dest` | `WORD32*` | `&mut [i32]` | Output buffer |
| `src` | `WORD32*` | `&[i32]` | Input buffer |
| `len` | `WORD32` | *implicit* | Number of samples (inferred from slice length) |
| `shift1` | `WORD8` | `i8` | Q-format of source |
| `shift2` | `WORD8` | `i8` | Q-format of destination (target precision) |

### Behavior:
- If `shift1 > shift2`: Right-shift (reduce precision)
- If `shift1 < shift2`: Left-shift with saturation (increase precision)
- If `shift1 == shift2`: Copy (no-op in Rust implementation)

**Rust Implementation:**
- `#[cfg(feature = "legacy_build")]`: Calls C via FFI
- `#[cfg(not(feature = "legacy_build"))]`: Pure Rust with `shl32_sat()` helper

---

## 10. `scale_down_adj()` - Q-Format Conversion with Adjustment

**Purpose:** Same as `scale_down()` but adds `ADJ_SCALE` constant (11) after scaling for bias correction.

**C Signature:**
```c
VOID ixheaacd_scale_down_adj(WORD32 *dest, WORD32 *src, WORD32 len,
                             WORD8 shift1, WORD8 shift2);
```

**Rust Signature:**
```rust
pub fn scale_down_adj(dest: &mut [i32], src: &[i32], shift1: i8, shift2: i8)
```

### Parameters: (Same as `scale_down()`)

### Behavior:
- Performs same scaling as `scale_down()`
- Adds `ADJ_SCALE` (11) to result with saturation
- Formula: `dest[i] = saturate(scale(src[i]) + 11)`

---

## Common Patterns Across Functions

### 1. Q-Format Parameters
All shift parameters (`shiftp`, `shift_olap`, `output_q`, etc.) represent fixed-point precision:
- Higher value = more fractional bits (e.g., Q15 = 15 bits after decimal)
- Functions align data to common Q-format before arithmetic operations

### 2. Saturation Arithmetic
All operations use saturating addition/multiplication to prevent overflow:
- C: `ixheaac_add32_sat()`, `ixheaac_shl32_sat()`
- Rust: `saturating_add()`, `shl32_sat()` helper

### 3. In-Place Modification
Parameters like `fp` are modified in-place (overlap buffers updated for next frame).

### 4. Window Coefficient Formats
Window arrays are typically Q31 format (0x7FFFFFFF = 1.0).

### 5. Rust Signature Changes
- **Length parameters removed** (inferred from slices)
- **Const correctness** (immutable references for read-only data)
- **Pointer arithmetic eliminated** (e.g., `win_fwd1` in `windowing_short4`)
- **Type safety** (`bool` instead of `WORD32` for flags)

---

## Key Insights for Rust Migration

1. **`win_fwd1` Parameter Gotcha:**
   - C passes pointer to END of array, decrements in loop
   - Rust must pass BASE array, compute offset in unsafe block
   - This affects `windowing_short4()` most notably

2. **Overlap Buffers (fp):**
   - Always mutable, carry state between frames
   - Size constraints critical (e.g., 2*n_short for `windowing_short4`)

3. **Q-Format Return Values:**
   - Functions return ACHIEVED Q-format, not requested
   - Caller must track Q-format through processing chain

4. **Frame Geometry (`offset_lengths`):**
   - Controls regional processing in long block functions
   - Critical for AAC frame structure compliance

5. **Legacy Build Feature:**
   - `#[cfg(feature = "legacy_build")]` enables C FFI for validation
   - Pure Rust implementation must match C behavior exactly
