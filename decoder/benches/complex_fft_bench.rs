use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use decoder::ixheaacd::fft::{complex_fft_p2, complex_fft_p3, FftMode};
use std::hint::black_box;

// ============================================================================
// Test Fixtures - Realistic FFT input data
// ============================================================================

/// Generate realistic audio sample data (simulated Q15 fixed-point)
fn fft_samples(len: usize) -> Vec<i32> {
    (0..len).map(|i| ((i as i32 * 1234567) % 32768) - 16384).collect()
}

// ============================================================================
// complex_fft_p2 benchmarks - Power-of-2 FFT (64, 128, 256, 512)
// ============================================================================

fn complex_fft_p2_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_fft_p2");

    for size in [64, 128, 256, 512] {
        // Forward FFT
        {
            let xr_orig = fft_samples(size);
            let xi_orig = fft_samples(size);

            group.bench_with_input(BenchmarkId::new("forward", size), &size, |b, _| {
                b.iter(|| {
                    let mut xr = xr_orig.clone();
                    let mut xi = xi_orig.clone();
                    let result = complex_fft_p2(&mut xr, &mut xi, FftMode::Forward, 0);
                    black_box((&xr, &xi, result));
                });
            });
        }

        // Inverse FFT
        {
            let xr_orig = fft_samples(size);
            let xi_orig = fft_samples(size);

            group.bench_with_input(BenchmarkId::new("inverse", size), &size, |b, _| {
                b.iter(|| {
                    let mut xr = xr_orig.clone();
                    let mut xi = xi_orig.clone();
                    let result = complex_fft_p2(&mut xr, &mut xi, FftMode::Inverse, 0);
                    black_box((&xr, &xi, result));
                });
            });
        }
    }

    group.finish();
}

// ============================================================================
// complex_fft_p3 benchmarks - Mixed-radix FFT (48, 96, 192, 384)
// ============================================================================

fn complex_fft_p3_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_fft_p3");

    for size in [48, 96, 192, 384] {
        // Forward FFT
        {
            let xr_orig = fft_samples(size);
            let xi_orig = fft_samples(size);

            group.bench_with_input(BenchmarkId::new("forward", size), &size, |b, _| {
                b.iter(|| {
                    let mut xr = xr_orig.clone();
                    let mut xi = xi_orig.clone();
                    let result = complex_fft_p3(&mut xr, &mut xi, FftMode::Forward, 0);
                    black_box((&xr, &xi, result));
                });
            });
        }

        // Inverse FFT
        {
            let xr_orig = fft_samples(size);
            let xi_orig = fft_samples(size);

            group.bench_with_input(BenchmarkId::new("inverse", size), &size, |b, _| {
                b.iter(|| {
                    let mut xr = xr_orig.clone();
                    let mut xi = xi_orig.clone();
                    let result = complex_fft_p3(&mut xr, &mut xi, FftMode::Inverse, 0);
                    black_box((&xr, &xi, result));
                });
            });
        }
    }

    group.finish();
}

criterion_group!(benches, complex_fft_p2_bench, complex_fft_p3_bench);
criterion_main!(benches);
