# Benchmark Suite

## Overview

This benchmark suite uses [Criterion](https://github.com/bheisler/criterion.rs) to measure the performance of migrated Rust functions against the original C implementation.

## Running Benchmarks

**Important:** The laptop must be connected to an external power source to eliminate CPU throttling.

### Basic Usage

```bash
cd decoder

# Run benchmarks matching a pattern
cargo bench -- "windowing_long1"

# Run specific benchmark
cargo bench -- "windowing_long1/1024_shift1_gt"

# List all available benchmarks (test mode)
cargo bench -- --test
```

---

## Comparing Rust vs C Implementation

### Prerequisites

**The C library must be built in release configuration before running legacy-build benchmarks.**

```bash
# Build C library in release mode (from project root)
cmake -B build -DCMAKE_BUILD_TYPE=Release -G Ninja -DLEGACY_BUILD
cmake --build build --config Release
```

### Step 1: Save C Implementation Baseline

Run benchmarks with the `legacy-build` feature enabled to use the original C implementation via FFI:

```bash
# Save baseline for specific benchmark group
cargo bench --features legacy-build -- --save-baseline c_impl "windowing_long1"

# Save baseline for single benchmark
cargo bench --features legacy-build -- --save-baseline c_impl "windowing_long1/1024_shift1_gt"
```

### Step 2: Compare Rust Implementation

Run benchmarks without the `legacy-build` feature to use the pure Rust implementation and compare against the saved baseline:

```bash
# Compare specific benchmark group
cargo bench -- --baseline c_impl "windowing_long1"

# Compare single benchmark
cargo bench -- --baseline c_impl "windowing_long1/1024_shift1_gt"
```

### Example Output

```text
windowing_long1/1024_shift1_gt
                        time:   [1.0409 µs 1.0451 µs 1.0498 µs]
                        change: [+3.4014% +5.5436% +7.0222%] (p = 0.00 < 0.05)
                        Performance has regressed.
```

**Interpreting results:**
- **Negative change** = Rust is faster than C
- **Positive change** = Rust is slower than C
- **p-value < 0.05** = statistically significant difference

---

## Benchmark Groups

| Group | Description | Functions |
|-------|-------------|-----------|
| `combine_fac` | FAC buffer combination | `combine_fac` |
| `windowing_long1` | Long block overlap-add | `windowing_long1` |
| `windowing_long2` | Long block with FAC | `windowing_long2` |
| `windowing_long3` | Long block transition | `windowing_long3` |
| `windowing_short1` | Short block initialization | `windowing_short1` |
| `windowing_short2` | Short block overlap-add | `windowing_short2` |
| `windowing_short3` | Short block final stage | `windowing_short3` |
| `windowing_short4` | Short block with windowing flag | `windowing_short4` |
| `scenario` | Real-world IMDCT scenarios | Mixed |
| `scale_down` | Buffer scaling operations | `scale_down` |
| `scale_down_adj` | Adjusted scaling | `scale_down_adj` |

---

## Buffer Sizes

| Size | Significance | Use Case |
|------|--------------|----------|
| 48 | Smallest FAC length | 768-sample TD short |
| 64 | Short FAC length | 1024-sample TD short |
| 96 | Short block (768) | `n_short` for 768-sample frames |
| 128 | Short block (1024) | `n_short` for 1024-sample frames |
| 768 | Long frame | USAC 768-sample mode |
| 1024 | Long frame | Standard AAC frame |

---

## Advanced Usage

### Adjusting Sample Size

```bash
# Quick run with fewer samples
cargo bench -- --sample-size 50

# Thorough run with more samples
cargo bench -- --sample-size 500
```

### Warm-up and Measurement Time

```bash
# Longer warm-up for stable results
cargo bench -- --warm-up-time 5

# Extended measurement time
cargo bench -- --measurement-time 10
```

### Output Formats

```bash
# Verbose output
cargo bench -- --verbose

# No plots (faster)
cargo bench -- --noplot
```

---

## HTML Reports

Criterion generates HTML reports in `target/criterion/`. Open `target/criterion/report/index.html` in a browser to view:

- Performance history over time
- Violin plots showing distribution
- Comparison charts between baselines
- Statistical analysis details

---

## Expected Performance

### Windowing Functions

| Function | Expected Time (1024 samples) | Notes |
|----------|------------------------------|-------|
| `windowing_long1` | 1-2 µs | Most common path |
| `windowing_long2` | 2-3 µs | FAC processing overhead |
| `windowing_long3` | 1-2 µs | Transition regions |
| `windowing_short*` | 100-300 ns | Smaller buffers |

### Scale Down Functions

| Operation | Expected Time | Notes |
|-----------|---------------|-------|
| Right shift (1024) | 200-400 ns | Simple bit shift |
| Left shift (1024) | 400-800 ns | Saturation checks |
| With adjustment | +20-30% | Additional add operation |

---

## Troubleshooting

### High Variance in Results

**Symptoms:** Large confidence intervals, inconsistent results

**Solutions:**
```bash
# Windows: Set high priority
start /high cargo bench

# Increase sample size
cargo bench -- --sample-size 200

# Disable CPU frequency scaling (Linux)
sudo cpupower frequency-set --governor performance
```

### Baseline Not Found

**Error:** `Baseline 'c_impl' not found`

**Solution:** Run the baseline save command first:
```bash
cargo bench --features legacy-build -- --save-baseline c_impl
```

### Unexpectedly Slow Performance

**Checks:**
1. Verify release build: `cargo bench` automatically uses release
2. Check C library is release: rebuild with `CMAKE_BUILD_TYPE=Release`
3. Use native CPU features: `RUSTFLAGS="-C target-cpu=native" cargo bench`

---

## CI Integration

### GitHub Actions Example

```yaml
name: Benchmark
on: [pull_request]
jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      # Build C library for baseline
      - name: Build C library
        run: |
          cmake -B build -DCMAKE_BUILD_TYPE=Release
          cmake --build build --config Release

      # Save C baseline
      - name: Save C baseline
        run: cargo bench --features legacy-build -- --save-baseline c_impl

      # Compare Rust implementation
      - name: Compare Rust vs C
        run: cargo bench -- --baseline c_impl

      # Upload results
      - uses: actions/upload-artifact@v4
        with:
          name: benchmark-results
          path: target/criterion/
```

---

## Adding New Benchmarks

```rust
fn my_new_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("my_group");

    let src = audio_samples(128);
    let mut dest = vec![0i32; 128];

    group.bench_function("my_function", |b| {
        b.iter(|| {
            my_function(&src, &mut dest);
            black_box(&dest);
        });
    });

    group.finish();
}

// Add to criterion_group! macro
criterion_group!(benches, ..., my_new_benchmark);
```

**Guidelines:**
1. Use `black_box()` to prevent optimization
2. Initialize data outside the benchmark closure
3. Use realistic data from AAC/USAC specifications
4. Name benchmarks with format: `{size}_{variant}`
5. Group related benchmarks together

---

## References

- [Criterion Documentation](https://bheisler.github.io/criterion.rs/book/)
- [Criterion GitHub](https://github.com/bheisler/criterion.rs)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [AAC/USAC Specification](docs/LIBXAAC-Enc-API.pdf)
