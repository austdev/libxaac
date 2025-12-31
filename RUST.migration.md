# Rust Migration Guide for libxaac

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Migration Cycle](#migration-cycle)
3. [Detailed Process](#detailed-process)
4. [Best Practices](#best-practices)
5. [Appendix](#appendix)

---

## Prerequisites

### Required Tools and Environment

#### 1. Rust Toolchain
- **Rust 2021+ edition** (currently using edition 2024)
- **Cargo** build system
- Install via: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

#### 2. C Build Tools

- **CMake** 3.10 or higher
- **C compiler**: Clang or MSVC
- **Cross-compilation toolchains** (for ARM targets, optional)

#### 3. Bindgen Dependencies

- **LLVM/Clang** development libraries
- **libclang** (for bindgen to parse C headers)
- Install on Ubuntu/Debian: `apt-get install llvm-dev libclang-dev clang`
- Install on macOS: `brew install llvm`

### Required Knowledge

- **Rust fundamentals**: Ownership, borrowing, lifetimes, unsafe code
- **C/FFI interop**: Foreign Function Interface, ABI compatibility
- **Audio codecs**: AAC/HE-AAC basics

### Project Structure Understanding

```text
libxaac/
├── decoder/
│   ├── Cargo.toml                 # Rust library config
│   ├── build.rs                   # Bindgen script for initial wrappers
│   ├── src/
│   │   ├── ixheaacd.rs            # Main module for publishing all functions
│   │   ├── ixheaacd_basic_ops.rs  # Migration unit 
│   │   ├── ...
│   │   ├── lib.rs                 # Rust library root with FFI exports
│   │   └── gen_ixheaacd_ref.rs    # Auto-generated FFI bindings
│   ├── benches/                   # Performance benchmarks
│   │   └── scale_down_bench.rs
│   └── *.c, *.h                   # Original C implementation
├── common/                        # Shared types and constants
├── encoder/                       # Encoder (future migration)
└── test/                          # Integration tests
```

---

## Migration Cycle

The migration follows a **5-phase iterative cycle** for each C function or module:

### Phase Overview

```text
┌─────────────────────────────────────────────────────────────
│                    MIGRATION CYCLE
└─────────────────────────────────────────────────────────────
1. ISOLATE
        →  Identify isolated set of C functions suitable for next migration cycle
    ↓
2. STUB
        →  Create Rust wrappers that calling original C code via FFI
    ↓
3. TESTS
        →  Write comprehensive unit tests on the Rust
    ↓
4. IMPLEMENT
        →  Replace stubs with pure Rust implementation, proved by unit tests 
    ↓
5. INTEGRATE
        →  Export Rust impl back to C lib and validate with integration testing
    ↓
   [REPEAT for next set of functions]
```

### Quick Reference

| Phase | Input | Output | Validation |
| ----- | ----- | ------ | ---------- |
| **1. Isolate** | C codebase | Function candidates | Dependency analysis |
| **2. Stub** | C signatures | Rust signatures/wrappers | Compilation `cargo build` |
| **3. Unit tests** | Function behavior | Unit test suite | `cargo test -F fallback` |
| **4. Implement** | Test suite | Pure Rust implementation | Tests pass `cargo test` |
| **5. Integrate** | Rust functions | C-compatible API | Integration tests `cmake build` |

---

## Detailed Process

### Phase 1: Isolate C Functions

**Goal**: Identify self-contained C functions without dependencies on unmigrated ones. 

**Selection Criteria:**

- ✅ Low external dependencies
- ✅ Clear input/output contract
- ✅ No global state mutations (or minimal)
- ✅ Single header

**Tools:**

- AI CLI
- Sourcetrail
- Debugger

**Document:**

- Purpose and algorithm
- Parameter semantics
- Side effects
- Performance characteristics
- Usage context in decoder pipeline

---

### Phase 2: Stub Functions with FFI

**Goal**: Develop signatures of Rust functions that satisfy both sides. Such functions should be ready for wrapping (redirecting) into original C functions.

#### Step 2.1: Add the header to Bindgen (build.rs)

```rust
// decoder/build.rs

    let ixheaacd_headers = vec![
        "ixheaacd_vec_baisc_ops.h",
    ];
 ```
 
Note, that ixheaac headers are not self-sufficient, so you may need to fix this by adding others includes to them.
An output of Bindgen tool is saved into the next file:
`decoder/src/gen_ixheaacd_ref.rs`
You could find declarations of the FFI functions for the next step in this file.

**Build:**

```bash
cd libxaac # root of repo
cmake -DRC_FALLBACK -DCMAKE_BUILD_TYPE=Release -B build -G Ninja
cmake --build # Runs cmake build once to create reference C lib
cd decoder
cargo build --features fallback
```

#### Step 2.2: Create Rust wrapper

Avoid using C types or C defines in the signature of Rust function - remember, in the future, this function will only be used in the Rust code. Also remove the `ixheaacd_`  prefix from the function name - it will be part of the Rust module name.

```rust
// decoder/src/ixheaacd_basic_ops.rs
use crate::gen_ixheaacd_ref::*;

pub fn scale_down(dest: &mut [i32], src: &[i32], shift1: i8, shift2: i8) {
    assert_eq!(dest.len(), src.len());
    
    #[cfg(feature = "fallback")]
    unsafe {
        // Call original C implementation via FFI
        ixheaacd_scale_down(
            dest.as_mut_ptr(),
            src.as_ptr() as *mut i32,
            dest.len() as i32,
            shift1,
            shift2,
        );
    }
}
```

**Check Build:**

```bash
cargo build --features fallback
```

---

### Phase 3: Write Unit Tests

**Goal**: Create a comprehensive test suite that validates both C and Rust implementations.

```rust
// decoder/src/ixheaacd_basic_ops.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_down_right_shift_len8() {
        let src = [1000, -1000, 2000, -2000, 4000, -4000, 8000, -8000];
        let mut dest = [0i32; 8];
        
        scale_down(&mut dest, &src, 10, 8);  // Right shift by 2
        
        let expected = [250, -250, 500, -500, 1000, -1000, 2000, -2000];
        assert_eq!(dest, expected);
    }
}
```

**Build and test:**

```bash
# Run tests with original C implementation
cargo test --features fallback
```

---

### Phase 4: Implement Pure Rust

**Goal**: Replace C implementation with idiomatic, safe Rust code while passing all tests.

#### Step 4.1: Implement Rust Function

```rust
pub fn scale_down(dest: &mut [i32], src: &[i32], shift1: i8, shift2: i8)
{
    assert_eq!(dest.len(), src.len(), "dest and src must have same length");

    #[cfg(feature = "fallback")]
    unsafe {
        crate::gen_ixheaacd_ref::ixheaacd_scale_down(
            dest.as_mut_ptr(), 
            src.as_ptr() as *mut i32, 
            src.len() as i32, 
            shift1, shift2);
    }

    #[cfg(not(feature = "fallback"))]
    if shift1 > shift2 {
        let shift = (shift1 - shift2) as u32;
        for (d, s) in dest.iter_mut().zip(src.iter()) {
            *d = s >> shift;
        }
    } else {
        let shift = (shift2 - shift1) as u32;
        for (d, s) in dest.iter_mut().zip(src.iter()) {
            *d = shl32_sat(*s, shift);
        }
    }
}

```

#### Step 4.2: Verify Implementation

```bash
# Run tests without fallback (pure Rust)
cargo test

# Compare with C implementation
cargo test --features fallback
```

**Both must pass identically!**

#### Step 4.3: Publish Rust functions

Invoke new functions into `ixheaacd` module, so they could be called as `ixheaacd::scale_down` from the Rust code.

```rust
// decoder/src/ixheaacd.rs

#[path = "ixheaacd_basic_ops.rs"]
mod basic_ops;
pub use basic_ops::*;
```

---

### Phase 5: Integration Testing

**Goal**: Export Rust functions as C-compatible API and validate with integration test.

#### Step 5.1: Create FFI Functions

```rust
// decoder/src/lib.rs

#[cfg(not(feature = "fallback"))]
mod integration_test {
    use crate::gen_ixheaacd_ref::*;
    use std::slice;

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_scale_down(
        dest: *mut WORD32,
        src: *mut WORD32,
        len: WORD32,
        shift1: WORD8,
        shift2: WORD8,
    ) {
        // Validate pointers and length
        if dest.is_null() || src.is_null() || len < 0 {
            return;
        }
        
        let len = len as usize;
        unsafe {
            let dest_slice = slice::from_raw_parts_mut(dest, len);
            let src_slice = slice::from_raw_parts(src, len);
            super::ixheaacd::scale_down(dest_slice, src_slice, shift1, shift2);
        }
    }
}
```

#### Step 5.2: Build Integration Test

```bash
cd libxaac # root of repo
cmake -B build -G Ninja  -DCMAKE_BUILD_TYPE=Debug
cmake --build 

# Output: build/xaacdec
```

#### Step 5.2: Run Integration Test

It needs to turn on according encoder features/formats to make sure that decoder will use our migrated functions. You could set breakpoints in the implemented functions and run decoder under debugger to check.

```bash
# An example of encoding stream for testing ixheaacd_scale_down()
./build/xaacenc -ifile:test/input.wav -ofile:test/aot42.aac -aot:42 -usac:1 -br:128000 -ccfl_idx:3 -esbr:1 -harmonic_sbr:1

# Run decoder test
./build/xaacdec -ifile:test/aot42.aac -imeta:test/aot42.txt -ofile:test/output.pcm -mp4:1

# Verify output matches reference
diff test/output.pcm /test/reference_output.pcm
```

#### Step 5.5: Validation Checklist

- [ ] Rust library builds successfully
- [ ] C code links with Rust static library
- [ ] Integration tests pass
- [ ] No performance regression (benchmark)
- [ ] No memory leaks (valgrind)
- [ ] Cross-platform builds work (ARM, x86, x64)

---

## Best Practices

### Code Quality

1. **Safety First**
   - Minimize `unsafe` blocks
   - Document all unsafe code with safety invariants
   - Use Rust's type system for compile-time guarantees

2. **Testing**
   - Write tests before implementation (TDD approach)
   - Aim for >90% code coverage
   - Include property-based tests for complex logic

3. **Documentation**
   - Document all public APIs
   - Include examples in doc comments
   - Explain algorithm choices and trade-offs

4. **Performance**
   - Profile before optimizing
   - Use benchmarks to validate improvements
   - Consider SIMD for hot paths

### Migration Strategy

1. **Incremental Migration**
   - Start with leaf functions (no dependencies)
   - Move up the call tree gradually
   - Keep C and Rust versions side-by-side during transition

2. **Risk Mitigation**
   - Always have fallback to C implementation
   - Run extensive regression tests
   - Deploy gradually (feature flags)

3. **Team Collaboration**
   - Document decisions and trade-offs
   - Code reviews for all migrations
   - Share knowledge through pair programming

---

## Appendix

### A. Cargo Features Reference

```toml
[features]
default = []
fallback = []           # Use C implementation via FFI
simd = []              # Enable SIMD optimizations (future)
```

**Usage:**
```bash
cargo build --features fallback
cargo test --features "fallback,simd"
```

### B. Common Type Mappings

| C Type | Rust Type | Notes |
|--------|-----------|-------|
| `WORD8` | `i8` | 8-bit signed |
| `UWORD8` | `u8` | 8-bit unsigned |
| `WORD16` | `i16` | 16-bit signed |
| `WORD32` | `i32` | 32-bit signed |
| `FLOAT32` | `f32` | 32-bit float |
| `VOID` | `()` | Unit type |
| `WORD32*` | `*mut i32` | Mutable pointer |

### C. Bindgen Configuration Examples

**Whitelist specific functions:**
```rust
let bindings = bindgen::Builder::default()
    .header("ixheaacd.h")
    .allowlist_function("ixheaacd_.*")
    .allowlist_type("ia_.*")
    .generate()
    .expect("Unable to generate bindings");
```

**Blacklist problematic headers:**
```rust
let bindings = bindgen::Builder::default()
    .header("ixheaacd.h")
    .blocklist_file(".*stdint.h")
    .generate()
    .expect("Unable to generate bindings");
```

### D. Performance Comparison Template

```rust
// benches/comparison_bench.rs
use divan::Bencher;

#[divan::bench]
fn rust_implementation(bencher: Bencher) {
    bencher.bench(|| {
        // Rust code
    });
}

#[cfg(feature = "fallback")]
#[divan::bench]
fn c_implementation(bencher: Bencher) {
    bencher.bench(|| {
        // C code via FFI
    });
}
```

### E. Migration Progress Tracking

**Current Status (as of 2025-12-30):**

| Phase | Functions | Status |
|-------|-----------|--------|
| **Phase 1: Initialization** | Core setup | ✅ Complete |
| **Phase 2: DSP Primitives** | | 🔄 In Progress |
| ├─ `scale_down` | Basic ops | ✅ Complete |
| ├─ `scale_down_adj` | Basic ops | ✅ Complete |
| └─ Others | ~50 functions | ⏳ Pending |
| **Phase 3: Bitstream** | Parsing/unpacking | 📋 Planned |
| **Phase 4: Core Decode** | IMDCT, TNS, etc. | 📋 Planned |
| **Phase 5: Full Pipeline** | End-to-end | 📋 Planned |

### F. Troubleshooting

**Issue: Bindgen fails to find headers**
```bash
# Solution: Set LIBCLANG_PATH
export LIBCLANG_PATH=/usr/lib/llvm-14/lib
cargo build
```

**Issue: Tests fail with segmentation fault**
```rust
// Solution: Check slice bounds
assert_eq!(dest.len(), src.len());  // Always validate!
```

**Issue: Performance regression in Rust version**
```bash
# Solution: Profile and optimize
cargo build --release
perf record ./target/release/benchmark
perf report
```

### G. Related Documentation

- **[decoder/benches/README.md](decoder/benches/README.md)** - Benchmarking guide
- **[README.md](README.md)** - Main project documentation

### H. External Resources

- **Rust FFI Guide:** https://doc.rust-lang.org/nomicon/ffi.html
- **Bindgen User Guide:** https://rust-lang.github.io/rust-bindgen/
- **Cargo Book:** https://doc.rust-lang.org/cargo/
- **Rust Performance Book:** https://nnethercote.github.io/perf-book/

---

## Summary

This migration process provides a **systematic, low-risk approach** to transitioning libxaac from C to Rust:

1. ✅ **Isolate** - Identify migration candidates
2. ✅ **Stub** - Create FFI wrappers with bindgen
3. ✅ **Test** - Comprehensive unit test coverage
4. ✅ **Implement** - Pure Rust with test validation
5. ✅ **Export** - C-compatible API with integration tests

**Key Success Factors:**

- Incremental migration with fallback support
- Test-driven development
- Performance validation at each step
- Comprehensive documentation

---

**Document Version:** 1.0  
**Last Updated:** 2025-12-31  
**Maintained By:** Rust Migration Team
