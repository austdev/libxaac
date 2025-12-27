# Benchmark Suite

## Overview

This benchmark suite uses [Divan](https://github.com/nvzqz/divan) to measure the performance of old and new functions across various scenarios.

## Running Benchmarks

Please note that the laptop must be connected to an external power source to eliminate the influence of CPU throttling.

### Basic Usage

```bash
# Run all benchmarks
cd decoder
cargo bench

# Run benchmarks matching a pattern
cargo bench -- scale_down_right
```

## Buffer Sizes Explained

### Why These Sizes?

| Size | Significance | Use Case |
| ---- | ------------ | -------- |
| 8 | AVX2 SIMD boundary (8×i32) | Minimum efficient SIMD size |
| 16 | 2× AVX2 vectors | Common loop unrolling target |
| 64 | Cache line aligned | L1 cache efficiency test |
| 128 | AAC short block size | Real decoder scenario |
| 512 | Half of long block | IMDCT overlap buffer |
| 1024 | AAC long block size | Full frame processing |

---

## Expected Performance Characteristics

### Right Shift (Fast Path)

- **Best case**: ~0.5-1 ns/element on modern CPUs
- **SIMD potential**: 8× speedup with AVX2
- **Bottleneck**: Memory bandwidth for large buffers

### Left Shift with Saturation (Slow Path)

- **Best case**: ~1-2 ns/element (saturation checks add overhead)
- **SIMD challenges**: Saturation logic is complex
- **Bottleneck**: Branch misprediction on mixed saturation

### scale_down_adj() (Additional Overhead)

- **Overhead**: +0.2-0.5 ns/element for saturating add
- **Expected**: 20-30% slower than base scale_down()

---

## Interpreting Results

### Example Output

```text
Timer precision: 41 ns
scale_down_bench           fastest       │ slowest       │ median        │ mean
├─ scale_down_right_shift
│  ├─ len8                 45.83 ns      │ 52.08 ns      │ 46.25 ns      │ 46.83 ns
│  ├─ len16                87.08 ns      │ 95.42 ns      │ 88.33 ns      │ 89.17 ns
│  ├─ len64                325.0 ns      │ 358.3 ns      │ 331.7 ns      │ 335.8 ns
│  ├─ len128               641.7 ns      │ 708.3 ns      │ 658.3 ns      │ 665.0 ns
│  ├─ len512               2.525 µs      │ 2.791 µs      │ 2.583 µs      │ 2.625 µs
│  ╰─ len1024              5.041 µs      │ 5.583 µs      │ 5.166 µs      │ 5.250 µs
```

---

## Optimization Targets

### Current Implementation (Scalar)

Expected performance:
    - Right shift: ~0.5-1.0 ns/element
    - Left shift: ~1.0-2.0 ns/element
    - With adjustment: +0.2-0.5 ns/element

### SIMD Optimized (AVX2)

Expected improvement:
    - Right shift: 4-8× faster for buffers ≥ 16
    - Left shift: 2-4× faster (saturation overhead)
    - Best for: len ≥ 64

### Memory Bound Threshold

For large buffers (≥ 1024), expect:
    - Memory bandwidth: ~10-20 GB/s
    - Cache effects: L1 < 32KB, L2 < 256KB
    - DRAM latency: ~50-100 ns

---

## Comparison with C Implementation

### To benchmark C version

1. Build C library in release mode
2. Link benchmarks against C functions
3. Run identical workloads
4. Compare median times

### Expected Results

| Metric | C (gcc -O3) | Rust (release) | Notes |
| ------ | ----------- | -------------- | ----- |
| Right shift | Baseline | 0.95-1.05× | Similar performance |
| Left shift | Baseline | 0.90-1.10× | Rust may be faster due to LLVM |
| With saturation | Baseline | 0.95-1.05× | Similar branch prediction |
| SIMD optimized | Baseline | 1.0-1.5× | Rust SIMD can be faster |

---

## Profiling Integration

### CPU Profiling with perf

```bash
# Run benchmarks with perf
cargo bench -- scale_down_right_shift_len1024 --profile-time 10

# Analyze hotspots
perf record -g cargo bench -- scale_down_right_shift_len1024
perf report
```

### Cachegrind Analysis

```bash
# Run under Valgrind
valgrind --tool=cachegrind --cache-sim=yes \
  cargo bench -- scale_down_right_shift_len1024 --bench

# Analyze cache misses
cg_annotate cachegrind.out.*
```

---

### CI Integration

Add to `.github/workflows/benchmark.yml`:

```yaml
name: Benchmark
on: [pull_request]
jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: cargo bench
      - uses: benchmark-action/github-action-benchmark@v1
        with:
          tool: 'cargo'
          output-file-path: target/criterion/output.txt
```

---

## Future Optimizations

### 1. SIMD Implementation

**Target:** 4-8× speedup for large buffers

**Implementation:**
    - AVX2 for x86_64 (8×i32 per instruction)
    - NEON for ARM (4×i32 per instruction)
    - Portable SIMD with `std::simd`

### 2. Inline Optimization

**Target:** Eliminate function call overhead

**Implementation:**
    - Mark hot paths with `#[inline(always)]`
    - Use const generics for small buffers

### 3. Branch Prediction

**Target:** Reduce misprediction on saturation

**Implementation:**
    - Likely/unlikely hints
    - Branch-free saturation using bit tricks

### 4. Memory Prefetching

**Target:** Hide DRAM latency

**Implementation:**
    - Software prefetch hints
    - Streaming stores for large buffers

---

## Troubleshooting

### High Variance in Results

**Symptoms:** Large difference between fastest and slowest times

**Causes:**
    - CPU frequency scaling
    - Background processes
    - Thermal throttling

**Solutions:**

```bash
# Disable CPU frequency scaling
sudo cpupower frequency-set --governor performance

# Isolate CPUs
taskset -c 0 cargo bench

# Increase sample size
cargo bench -- --sample-size 1000
```

### Unexpectedly Slow Performance

**Symptoms:** Much slower than expected

**Checks:**

1. Verify release build: `cargo bench` (not `cargo test`)
2. Check compiler flags: `RUSTFLAGS="-C target-cpu=native"`
3. Disable debug assertions
4. Profile with `perf` to find bottlenecks

---

## References

- [Divan Documentation](https://docs.rs/divan/)
- [Divan GitHub](https://github.com/nvzqz/divan)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Intel Intrinsics Guide](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/)

---

## Benchmark Maintenance

### Adding New Benchmarks

```rust
#[divan::bench]
fn my_new_benchmark(bencher: Bencher) {
    let src = vec![1000; 128];
    let mut dest = vec![0; 128];
    
    bencher.bench_local(|| {
        scale_down(&mut dest, &src, 5, 2);
        divan::black_box(&dest);
    });
}
```

**Guidelines:**

1. ✅ Use `divan::black_box()` to prevent optimization
2. ✅ Initialize data outside `bench_local()`
3. ✅ Use realistic data patterns
4. ✅ Name benchmarks descriptively
5. ✅ Group related benchmarks
6. ✅ Document expected performance

### Place Benchmarks Into Separated Directory

Rust's Cargo build system has special handling for the benches/ directory:

```text
 project/
 ├── src/           # Library code (compiled once)
 ├── tests/         # Integration tests (each file = separate binary)
 ├── benches/       # Benchmarks (each file = separate binary)
 └── examples/      # Example programs
```

**Key differences:**

 | Location | Compilation | Purpose | Run Command |
 | --------- | ----------- | -------------- | ------------------ |
 | src/ | Library crate | Production code | cargo build |
 | tests/ | Test binaries | Integration tests | cargo test | 
 | benches/ | Benchmark binaries | Performance tests | cargo bench |

**Benefits:**

 1. ✅ Benchmarks don't increase library compilation time
 2. ✅ Benchmark dependencies (Divan) don't pollute library deps
 3. ✅ Can have multiple benchmark files
 4. ✅ Each benchmark is independently compiled and run
