# FFT Functions Analysis

This document analyzes the FFT (Fast Fourier Transform) functions in `decoder/ixheaacd_fft.c` for the Rust migration effort.

## Table of Contents

1. [Overview](#overview)
2. [Function Hierarchy](#function-hierarchy)
3. [Type Definitions](#type-definitions)
4. [External Dependencies](#external-dependencies)
5. [Function Descriptions](#function-descriptions)
6. [Rust Migration Recommendations](#rust-migration-recommendations)

---

## Overview

The FFT module implements complex FFT operations for both floating-point and fixed-point arithmetic. These functions are used primarily in the IMDCT (Inverse Modified Discrete Cosine Transform) computations for audio decoding.

The implementation uses radix-4 and radix-2 butterfly operations with mixed-radix support for non-power-of-2 lengths (specifically lengths containing factors of 3).

---

## Function Hierarchy

```
ixheaacd_complex_fft()                    # Main dispatcher
    |
    +-- ixheaacd_complex_fft_p2()         # Power-of-2 FFT (via function pointer)
    |       |
    |       +-- ixheaacd_complex_fft_p2_dec()  # Decoder variant
    |
    +-- ixheaacd_complex_fft_p3()         # Mixed-radix FFT (power-of-2 * power-of-3)
            |
            +-- ixheaacd_complex_fft_p2()
            +-- ixheaacd_complex_3point_fft()

ixheaacd_mps_complex_fft()                # MPS (MPEG Surround) FFT (floating-point)

ixheaacd_mps_synth_calc_fft()             # MPS synthesis 32-point FFT (specialized)
```

---

## Type Definitions

| C Type    | Size    | Rust Equivalent | Description                    |
|-----------|---------|-----------------|--------------------------------|
| `WORD32`  | 32-bit  | `i32`           | Signed 32-bit integer          |
| `WORD8`   | 8-bit   | `i8`            | Signed 8-bit integer           |
| `FLOAT32` | 32-bit  | `f32`           | 32-bit floating point          |
| `VOID`    | -       | `()`            | Void return type               |

---

## External Dependencies

### Twiddle Factor Tables

| Table Name                              | Size  | Type      | Purpose                                |
|-----------------------------------------|-------|-----------|----------------------------------------|
| `ixheaacd_twiddle_table_fft_32x32`      | 514   | `WORD32`  | Fixed-point twiddle factors            |
| `ixheaacd_twiddle_table_fft`            | 514   | `FLOAT32` | Floating-point twiddle factors         |
| `ixheaacd_twiddle_table_fft_flt`        | 16    | `FLOAT32` | Specialized 16-point twiddle factors   |
| `ixheaacd_twiddle_table_3pr`            | 1155  | `WORD32`  | Radix-3 twiddle factors (real part)    |
| `ixheaacd_twiddle_table_3pi`            | 1155  | `WORD32`  | Radix-3 twiddle factors (imaginary)    |
| `ixheaacd_mps_dig_rev`                  | 8     | `WORD8`   | Digit reversal table for MPS           |

### Helper Functions

| Function                    | Purpose                                           |
|-----------------------------|---------------------------------------------------|
| `ixheaac_norm32()`          | Compute normalization shift for 32-bit value      |
| `ixheaac_add32_sat()`       | Saturating 32-bit addition                        |
| `ixheaac_sub32_sat()`       | Saturating 32-bit subtraction                     |
| `ixheaac_shl32_sat()`       | Saturating 32-bit left shift                      |
| `ixheaac_mult32_shl()`      | 32-bit multiply with left shift                   |
| `ixheaac_sat64_32()`        | Saturate 64-bit to 32-bit                         |

---

## Function Descriptions

### 1. `ixheaacd_mps_synth_calc_fft`

```c
VOID ixheaacd_mps_synth_calc_fft(FLOAT32 *ptr_xr, FLOAT32 *ptr_xi, WORD32 npoints);
```

#### Purpose
Specialized 32-point FFT for MPEG Surround (MPS) synthesis filter bank. Operates on interleaved real/imaginary pairs and uses hardcoded twiddle factors for efficiency.

#### Parameters

| Parameter  | Type      | Direction | Description                                      |
|------------|-----------|-----------|--------------------------------------------------|
| `ptr_xr`   | `FLOAT32*`| In/Out    | Real part array (interleaved format: r0,i0,r1,i1,...) |
| `ptr_xi`   | `FLOAT32*`| In/Out    | Imaginary part array (interleaved format)        |
| `npoints`  | `WORD32`  | In        | FFT length (expected: 32)                        |

#### Buffer Lengths
- `ptr_xr`: `npoints * 2` elements (64 for 32-point FFT)
- `ptr_xi`: `npoints * 2` elements (64 for 32-point FFT)
- Local `y[64]`: Temporary real buffer
- Local `z[64]`: Temporary imaginary buffer

#### Algorithm
- Uses hardcoded 3-stage radix-4 butterfly
- Fixed twiddle factors: 0.707107, 0.923880, 0.382683
- Input is expected in bit-reversed order via `ixheaacd_mps_dig_rev` lookup

---

### 2. `ixheaacd_mps_complex_fft`

```c
VOID ixheaacd_mps_complex_fft(FLOAT32 *xr, FLOAT32 *xi, WORD32 nlength);
```

#### Purpose
General-purpose floating-point complex FFT for MPS processing. Supports power-of-2 lengths up to 128 points.

#### Parameters

| Parameter  | Type      | Direction | Description                              |
|------------|-----------|-----------|------------------------------------------|
| `xr`       | `FLOAT32*`| In/Out    | Real part array (separate storage)       |
| `xi`       | `FLOAT32*`| In/Out    | Imaginary part array (separate storage)  |
| `nlength`  | `WORD32`  | In        | FFT length (power of 2, up to 128)       |

#### Buffer Lengths
- `xr`: `nlength` elements
- `xi`: `nlength` elements
- Local `ptr_x[256]`: Interleaved working buffer (`2 * nlength`)
- Local `y[256]`: Output staging buffer (`2 * nlength`)

#### Algorithm
- Radix-4 Cooley-Tukey with decimation-in-time
- Supports non-power-of-4 lengths via final radix-2 stage
- In-place after initial interleaving

---

### 3. `ixheaacd_complex_fft_p2_dec`

```c
VOID ixheaacd_complex_fft_p2_dec(WORD32 *xr, WORD32 *xi, WORD32 nlength,
                                  WORD32 fft_mode, WORD32 *preshift);
```

#### Purpose
Fixed-point power-of-2 complex FFT with automatic scaling. Used as the primary FFT engine for the decoder.

#### Parameters

| Parameter  | Type      | Direction | Description                                          |
|------------|-----------|-----------|------------------------------------------------------|
| `xr`       | `WORD32*` | In/Out    | Real part array (Q-format fixed-point)               |
| `xi`       | `WORD32*` | In/Out    | Imaginary part array (Q-format fixed-point)          |
| `nlength`  | `WORD32`  | In        | FFT length (power of 2, up to 512)                   |
| `fft_mode` | `WORD32`  | In        | Direction: -1 = forward FFT, +1 = inverse FFT        |
| `preshift` | `WORD32*` | In/Out    | Scaling factor (updated with total shift applied)    |

#### Buffer Lengths
- `xr`: `nlength` elements
- `xi`: `nlength` elements
- Local `ptr_x[1024]`: Interleaved working buffer (`2 * nlength`)
- Local `y[1024]`: Output staging buffer (`2 * nlength`)

#### Algorithm
- Radix-4 with saturating arithmetic
- Automatic scaling based on input magnitude
- Supports both forward and inverse transform via sign change in twiddle application

#### Scaling Behavior
The function computes:
```
shift = (n % 2 == 0) ? (n + 4) / 2 : (n + 3) / 2
```
where `n = log2(nlength)`. This ensures numerical stability.

---

### 4. `ixheaacd_complex_3point_fft`

```c
static PLATFORM_INLINE void ixheaacd_complex_3point_fft(WORD32 *inp, WORD32 *op,
                                                         WORD32 sign_dir);
```

#### Purpose
Radix-3 DFT butterfly for mixed-radix FFT. Implements the 3-point DFT kernel.

#### Parameters

| Parameter  | Type      | Direction | Description                                      |
|------------|-----------|-----------|--------------------------------------------------|
| `inp`      | `WORD32*` | In        | Input buffer (3 complex values = 6 elements)     |
| `op`       | `WORD32*` | Out       | Output buffer (3 complex values = 6 elements)    |
| `sign_dir` | `WORD32`  | In        | Direction: -1 = forward, +1 = inverse            |

#### Buffer Lengths
- `inp`: 6 elements (3 complex pairs: [r0, i0, r1, i1, r2, i2])
- `op`: 6 elements (3 complex pairs)

#### Algorithm
Uses the constant `sinmu = -1859775393 * sign_dir` (Q31 representation of `-sqrt(3)/2`).

The 3-point DFT formula:
```
X[0] = x[0] + x[1] + x[2]
X[1] = x[0] + x[1]*W3^1 + x[2]*W3^2
X[2] = x[0] + x[1]*W3^2 + x[2]*W3^4
```
where `W3 = exp(-2*pi*j/3)`.

---

### 5. `ixheaacd_complex_fft_p3`

```c
VOID ixheaacd_complex_fft_p3(WORD32 *xr, WORD32 *xi, WORD32 nlength,
                              WORD32 fft_mode, WORD32 *preshift);
```

#### Purpose
Mixed-radix FFT for lengths that are products of powers of 2 and 3 (e.g., 48, 96, 192, 384, 768).

#### Parameters

| Parameter  | Type      | Direction | Description                                          |
|------------|-----------|-----------|------------------------------------------------------|
| `xr`       | `WORD32*` | In/Out    | Real part array                                      |
| `xi`       | `WORD32*` | In/Out    | Imaginary part array                                 |
| `nlength`  | `WORD32`  | In        | FFT length (must be 3^k * 2^m)                       |
| `fft_mode` | `WORD32`  | In        | Direction: -1 = forward, +1 = inverse                |
| `preshift` | `WORD32*` | In/Out    | Scaling factor                                       |

#### Buffer Lengths
- `xr`: `nlength` elements
- `xi`: `nlength` elements
- Local `xr_3[384]`: Temporary real buffer for sub-FFTs
- Local `xi_3[384]`: Temporary imaginary buffer for sub-FFTs
- Local `x[1024]`: Interleaved working buffer
- Local `y[1024]`: Output staging buffer

#### Algorithm
1. Factor out all 3s: compute `cnfac` (count of factor 3) and `mpass` (remaining power of 2)
2. Perform `3 * cnfac` sub-FFTs of length `mpass` using `ixheaacd_complex_fft_p2`
3. Apply twiddle factors between stages
4. Perform `mpass` 3-point FFTs using `ixheaacd_complex_3point_fft`
5. Transpose output to natural order

---

### 6. `ixheaacd_complex_fft`

```c
VOID ixheaacd_complex_fft(WORD32 *data_r, WORD32 *data_i, WORD32 nlength,
                           WORD32 fft_mode, WORD32 *preshift);
```

#### Purpose
Main FFT dispatcher that routes to the appropriate implementation based on FFT length.

#### Parameters

| Parameter  | Type      | Direction | Description                                          |
|------------|-----------|-----------|------------------------------------------------------|
| `data_r`   | `WORD32*` | In/Out    | Real part array                                      |
| `data_i`   | `WORD32*` | In/Out    | Imaginary part array                                 |
| `nlength`  | `WORD32`  | In        | FFT length                                           |
| `fft_mode` | `WORD32`  | In        | Direction: -1 = forward, +1 = inverse                |
| `preshift` | `WORD32*` | In/Out    | Scaling factor                                       |

#### Dispatch Logic
```c
if (nlength & (nlength - 1)) {
    // Not power of 2 -> use mixed-radix (p3)
    ixheaacd_complex_fft_p3(data_r, data_i, nlength, fft_mode, preshift);
} else {
    // Power of 2 -> use radix-4/2 (p2)
    ixheaacd_complex_fft_p2(data_r, data_i, nlength, fft_mode, preshift);
}
```

---

## Rust Migration Recommendations

### 1. Unified Complex Number Type

Replace separate real/imaginary arrays with a complex number type:

```rust
use num_complex::Complex;

// Instead of:
// fn complex_fft(xr: &mut [i32], xi: &mut [i32], ...)

// Use:
fn complex_fft(data: &mut [Complex<i32>], mode: FftMode, preshift: &mut i32)
```

### 2. FFT Direction Enum

Replace the `fft_mode` integer with a type-safe enum:

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FftDirection {
    Forward,  // replaces fft_mode = -1
    Inverse,  // replaces fft_mode = +1
}

impl FftDirection {
    /// Returns the sign multiplier for twiddle factor application
    pub fn sign(&self) -> i32 {
        match self {
            FftDirection::Forward => -1,
            FftDirection::Inverse => 1,
        }
    }
}
```

### 3. Simplified Function Signatures

#### Original C (ixheaacd_complex_fft_p2_dec)
```c
VOID ixheaacd_complex_fft_p2_dec(
    WORD32 *xr,        // real input/output
    WORD32 *xi,        // imaginary input/output
    WORD32 nlength,    // FFT length
    WORD32 fft_mode,   // direction (-1/+1)
    WORD32 *preshift   // scaling factor
);
```

#### Proposed Rust
```rust
pub fn complex_fft_p2(
    data: &mut [Complex<i32>],  // nlength complex values
    direction: FftDirection,
) -> i32  // returns shift amount instead of mutating preshift
```

Or with explicit output buffer:

```rust
pub fn complex_fft_p2(
    input: &[Complex<i32>],      // nlength complex values
    output: &mut [Complex<i32>], // nlength complex values
    direction: FftDirection,
) -> i32
```

### 4. FFT Length Validation with Enums

Define supported FFT lengths explicitly:

```rust
#[derive(Clone, Copy)]
pub enum FftLength {
    // Power of 2 lengths
    N16 = 16,
    N32 = 32,
    N64 = 64,
    N128 = 128,
    N256 = 256,
    N512 = 512,

    // Mixed-radix lengths (3^k * 2^m)
    N48 = 48,
    N96 = 96,
    N192 = 192,
    N384 = 384,
    N768 = 768,
}

impl FftLength {
    pub fn is_power_of_2(&self) -> bool {
        let n = *self as usize;
        n & (n - 1) == 0
    }

    pub fn value(&self) -> usize {
        *self as usize
    }
}
```

### 5. Unified FFT Trait

```rust
pub trait Fft {
    type Sample;

    fn transform(
        &self,
        data: &mut [Complex<Self::Sample>],
        direction: FftDirection,
    ) -> i32;
}

pub struct FixedPointFft {
    length: FftLength,
    twiddles: &'static [i32],
}

pub struct FloatingPointFft {
    length: FftLength,
    twiddles: &'static [f32],
}
```

### 6. Buffer Abstraction

Use Rust's type system to enforce buffer requirements:

```rust
/// Wrapper ensuring buffer has exactly the required FFT length
pub struct FftBuffer<T, const N: usize> {
    data: [Complex<T>; N],
}

impl<T, const N: usize> FftBuffer<T, N> {
    pub fn new(data: [Complex<T>; N]) -> Self {
        Self { data }
    }

    pub fn as_slice(&self) -> &[Complex<T>] {
        &self.data
    }

    pub fn as_mut_slice(&mut self) -> &mut [Complex<T>] {
        &mut self.data
    }
}
```

### 7. Complete Proposed API

```rust
mod fft {
    use num_complex::Complex;

    /// FFT transform direction
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum FftDirection {
        Forward,
        Inverse,
    }

    /// Result containing the computed shift for fixed-point FFT
    pub struct FftResult {
        pub shift: i32,
    }

    /// Fixed-point complex FFT
    pub fn complex_fft_i32(
        data: &mut [Complex<i32>],  // length must be power of 2 or mixed-radix
        direction: FftDirection,
    ) -> FftResult;

    /// Floating-point complex FFT (for MPS)
    pub fn complex_fft_f32(
        data: &mut [Complex<f32>],  // length must be power of 2
        direction: FftDirection,
    );

    /// Specialized 32-point MPS synthesis FFT
    pub fn mps_synth_fft_32(
        data_r: &mut [Complex<f32>; 32],
        data_i: &mut [Complex<f32>; 32],
    );

    /// 3-point DFT kernel (internal)
    pub(crate) fn dft_3point(
        input: &[Complex<i32>; 3],
        direction: FftDirection,
    ) -> [Complex<i32>; 3];
}
```

### 8. Twiddle Factor Storage

Use const generics for compile-time known tables:

```rust
pub struct TwiddleTable<const N: usize> {
    cos: [i32; N],
    sin: [i32; N],
}

impl<const N: usize> TwiddleTable<N> {
    pub const fn new(cos: [i32; N], sin: [i32; N]) -> Self {
        Self { cos, sin }
    }

    pub fn get(&self, index: usize) -> Complex<i32> {
        Complex::new(self.cos[index], self.sin[index])
    }
}

// Static twiddle tables
pub static TWIDDLES_512: TwiddleTable<514> = TwiddleTable::new(
    ixheaacd_twiddle_table_fft_32x32_cos,
    ixheaacd_twiddle_table_fft_32x32_sin,
);
```

---

## Summary of Key Simplifications

| C Pattern                                | Rust Replacement                                  |
|------------------------------------------|---------------------------------------------------|
| Separate `xr[]` and `xi[]` arrays        | Single `&mut [Complex<T>]` slice                  |
| `fft_mode` as `WORD32` (-1/+1)           | `FftDirection` enum                               |
| `WORD32 nlength` parameter               | Slice length from `data.len()` or `FftLength`     |
| Output via pointer mutation (`*preshift`)| Return value `-> i32` or `FftResult` struct       |
| Manual buffer size tracking              | Compile-time const generics or runtime checks    |
| Hardcoded twiddle constants              | `TwiddleTable` with typed accessors               |

These changes would significantly reduce the parameter count, improve type safety, and make the code more idiomatic for Rust while maintaining the same algorithmic behavior.
