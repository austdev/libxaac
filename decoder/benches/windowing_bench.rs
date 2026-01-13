use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use decoder::ixheaacd::OffsetLengths;

// ============================================================================
// Test Fixtures - Realistic AAC/USAC buffer configurations
// ============================================================================

// Standard AAC configurations from the specification
const CFG_1024_FD: OffsetLengths = OffsetLengths {
    n_long: 1024, n_short: 128, lfac: 128, n_flat_ls: 448, n_trans_ls: 128,
};

const CFG_1024_TD_SHORT: OffsetLengths = OffsetLengths {
    n_long: 1024, n_short: 128, lfac: 64, n_flat_ls: 448, n_trans_ls: 128,
};

const CFG_1024_TD_LONG: OffsetLengths = OffsetLengths {
    n_long: 1024, n_short: 128, lfac: 128, n_flat_ls: 384, n_trans_ls: 256,
};

const CFG_768_FD: OffsetLengths = OffsetLengths {
    n_long: 768, n_short: 96, lfac: 128, n_flat_ls: 336, n_trans_ls: 96,
};

const CFG_768_TD_SHORT: OffsetLengths = OffsetLengths {
    n_long: 768, n_short: 96, lfac: 48, n_flat_ls: 336, n_trans_ls: 96,
};

const CFG_768_TD_LONG: OffsetLengths = OffsetLengths {
    n_long: 768, n_short: 96, lfac: 96, n_flat_ls: 288, n_trans_ls: 192,
};

/// Generate realistic audio sample data (simulated Q15 fixed-point)
fn audio_samples(len: usize) -> Vec<i32> {
    (0..len).map(|i| ((i as i32 * 1234567) % 32768) - 16384).collect()
}

/// Generate window coefficients (simulated Q31 fixed-point, 0..1 range)
fn window_coeffs(len: usize) -> Vec<i32> {
    (0..len).map(|i| {
        let t = i as f64 / len as f64;
        let w = (std::f64::consts::PI * (t + 0.5) / 2.0).sin();
        (w * w * (i32::MAX as f64)) as i32
    }).collect()
}

// ============================================================================
// combine_fac benchmarks
// ============================================================================

fn combine_fac_bench(c: &mut Criterion) {
    use decoder::ixheaacd::combine_fac;

    let mut group = c.benchmark_group("combine_fac");

    // Parameterized right shift benchmarks
    for lfac in [128, 96, 64, 48] {
        let src1 = audio_samples(lfac);
        let src2 = audio_samples(lfac);
        let mut dest = vec![0i32; lfac];

        group.bench_with_input(BenchmarkId::new("right_shift", lfac), &lfac, |b, _| {
            b.iter(|| {
                combine_fac(&src1, &src2, &mut dest, 5, 8);
                black_box(&dest);
            });
        });
    }

    // Parameterized left shift benchmarks
    for lfac in [128, 96, 64, 48] {
        let src1 = audio_samples(lfac);
        let src2 = audio_samples(lfac);
        let mut dest = vec![0i32; lfac];

        group.bench_with_input(BenchmarkId::new("left_shift", lfac), &lfac, |b, _| {
            b.iter(|| {
                combine_fac(&src1, &src2, &mut dest, 8, 5);
                black_box(&dest);
            });
        });
    }

    // Config-specific benchmarks
    {
        let cfg = CFG_1024_FD;
        let src1 = audio_samples(cfg.lfac);
        let src2 = audio_samples(cfg.lfac);
        let mut dest = vec![0i32; cfg.lfac];

        group.bench_function("1024_fd", |b| {
            b.iter(|| {
                combine_fac(&src1, &src2, &mut dest, 15, 12);
                black_box(&dest);
            });
        });
    }

    {
        let cfg = CFG_768_TD_LONG;
        let src1 = audio_samples(cfg.lfac);
        let src2 = audio_samples(cfg.lfac);
        let mut dest = vec![0i32; cfg.lfac];

        group.bench_function("768_td_long", |b| {
            b.iter(|| {
                combine_fac(&src1, &src2, &mut dest, 15, 12);
                black_box(&dest);
            });
        });
    }

    group.finish();
}

// ============================================================================
// windowing_long1 benchmarks
// ============================================================================

fn windowing_long1_bench(c: &mut Criterion) {
    use decoder::ixheaacd::windowing_long1;

    let mut group = c.benchmark_group("windowing_long1");

    // 1024 samples, shift1 > shift2
    {
        let cfg = CFG_1024_FD;
        let src1 = audio_samples(cfg.n_long / 2);
        let src2 = audio_samples(cfg.n_long);
        let win_fwd = window_coeffs(cfg.n_long);
        let win_rev = window_coeffs(cfg.n_long);
        let mut dest = vec![0i32; cfg.n_long];

        group.bench_function("1024_shift1_gt", |b| {
            b.iter(|| {
                let _ = windowing_long1(&src1, &src2, &win_fwd, &win_rev, &mut dest, 15, 12);
                black_box(&dest);
            });
        });
    }

    // 1024 samples, shift2 > shift1
    {
        let cfg = CFG_1024_FD;
        let src1 = audio_samples(cfg.n_long / 2);
        let src2 = audio_samples(cfg.n_long);
        let win_fwd = window_coeffs(cfg.n_long);
        let win_rev = window_coeffs(cfg.n_long);
        let mut dest = vec![0i32; cfg.n_long];

        group.bench_function("1024_shift2_gt", |b| {
            b.iter(|| {
                let _ = windowing_long1(&src1, &src2, &win_fwd, &win_rev, &mut dest, 12, 15);
                black_box(&dest);
            });
        });
    }

    // 768 samples, shift1 > shift2
    {
        let cfg = CFG_768_FD;
        let src1 = audio_samples(cfg.n_long / 2);
        let src2 = audio_samples(cfg.n_long);
        let win_fwd = window_coeffs(cfg.n_long);
        let win_rev = window_coeffs(cfg.n_long);
        let mut dest = vec![0i32; cfg.n_long];

        group.bench_function("768_shift1_gt", |b| {
            b.iter(|| {
                let _ = windowing_long1(&src1, &src2, &win_fwd, &win_rev, &mut dest, 15, 12);
                black_box(&dest);
            });
        });
    }

    // 768 samples, shift2 > shift1
    {
        let cfg = CFG_768_FD;
        let src1 = audio_samples(cfg.n_long / 2);
        let src2 = audio_samples(cfg.n_long);
        let win_fwd = window_coeffs(cfg.n_long);
        let win_rev = window_coeffs(cfg.n_long);
        let mut dest = vec![0i32; cfg.n_long];

        group.bench_function("768_shift2_gt", |b| {
            b.iter(|| {
                let _ = windowing_long1(&src1, &src2, &win_fwd, &win_rev, &mut dest, 12, 15);
                black_box(&dest);
            });
        });
    }

    group.finish();
}

// ============================================================================
// windowing_long2 benchmarks
// ============================================================================

fn windowing_long2_bench(c: &mut Criterion) {
    use decoder::ixheaacd::windowing_long2;

    let mut group = c.benchmark_group("windowing_long2");

    // Note: windowing_long2 is specifically for LONG_START with FAC processing
    // which only occurs when td_frame_prev=true (transition from ACELP to FD)

    // Branch 1: shiftp > fac_q && shift_olap > fac_q
    {
        let cfg = CFG_1024_TD_LONG;
        let src1 = audio_samples(cfg.n_long);
        let win_fwd = window_coeffs(cfg.n_trans_ls + cfg.lfac);
        let fac_data_out = audio_samples(cfg.lfac * 2);
        let over_lap = audio_samples(cfg.n_flat_ls + cfg.lfac);
        let mut dest = vec![0i32; cfg.n_long];

        group.bench_function("1024_td_long_branch1", |b| {
            b.iter(|| {
                let _ = windowing_long2(&src1, &win_fwd, &fac_data_out, &over_lap, &mut dest, &cfg, 15, 14, 12);
                black_box(&dest);
            });
        });
    }

    // Branch 2: shiftp > fac_q && shift_olap <= fac_q
    {
        let cfg = CFG_1024_TD_LONG;
        let src1 = audio_samples(cfg.n_long);
        let win_fwd = window_coeffs(cfg.n_trans_ls + cfg.lfac);
        let fac_data_out = audio_samples(cfg.lfac * 2);
        let over_lap = audio_samples(cfg.n_flat_ls + cfg.lfac);
        let mut dest = vec![0i32; cfg.n_long];

        group.bench_function("1024_td_long_branch2", |b| {
            b.iter(|| {
                let _ = windowing_long2(&src1, &win_fwd, &fac_data_out, &over_lap, &mut dest, &cfg, 15, 10, 12);
                black_box(&dest);
            });
        });
    }

    // Branch 3: shiftp <= fac_q && shift_olap > shiftp
    {
        let cfg = CFG_1024_TD_LONG;
        let src1 = audio_samples(cfg.n_long);
        let win_fwd = window_coeffs(cfg.n_trans_ls + cfg.lfac);
        let fac_data_out = audio_samples(cfg.lfac * 2);
        let over_lap = audio_samples(cfg.n_flat_ls + cfg.lfac);
        let mut dest = vec![0i32; cfg.n_long];

        group.bench_function("1024_td_long_branch3", |b| {
            b.iter(|| {
                let _ = windowing_long2(&src1, &win_fwd, &fac_data_out, &over_lap, &mut dest, &cfg, 10, 14, 12);
                black_box(&dest);
            });
        });
    }

    // Branch 4: shiftp <= fac_q && shift_olap <= shiftp
    {
        let cfg = CFG_1024_TD_LONG;
        let src1 = audio_samples(cfg.n_long);
        let win_fwd = window_coeffs(cfg.n_trans_ls + cfg.lfac);
        let fac_data_out = audio_samples(cfg.lfac * 2);
        let over_lap = audio_samples(cfg.n_flat_ls + cfg.lfac);
        let mut dest = vec![0i32; cfg.n_long];

        group.bench_function("1024_td_long_branch4", |b| {
            b.iter(|| {
                let _ = windowing_long2(&src1, &win_fwd, &fac_data_out, &over_lap, &mut dest, &cfg, 10, 8, 12);
                black_box(&dest);
            });
        });
    }

    // EIGHT_SHORT_SEQUENCE with FAC (lfac=64)
    {
        let cfg = CFG_1024_TD_SHORT;
        let src1 = audio_samples(cfg.n_long);
        let win_fwd = window_coeffs(cfg.n_trans_ls + cfg.lfac);
        let fac_data_out = audio_samples(cfg.lfac * 2);
        let over_lap = audio_samples(cfg.n_flat_ls + cfg.lfac);
        let mut dest = vec![0i32; cfg.n_long];

        group.bench_function("1024_td_short", |b| {
            b.iter(|| {
                let _ = windowing_long2(&src1, &win_fwd, &fac_data_out, &over_lap, &mut dest, &cfg, 15, 14, 12);
                black_box(&dest);
            });
        });
    }

    // 768 TD Long
    {
        let cfg = CFG_768_TD_LONG;
        let src1 = audio_samples(cfg.n_long);
        let win_fwd = window_coeffs(cfg.n_trans_ls + cfg.lfac);
        let fac_data_out = audio_samples(cfg.lfac * 2);
        let over_lap = audio_samples(cfg.n_flat_ls + cfg.lfac);
        let mut dest = vec![0i32; cfg.n_long];

        group.bench_function("768_td_long", |b| {
            b.iter(|| {
                let _ = windowing_long2(&src1, &win_fwd, &fac_data_out, &over_lap, &mut dest, &cfg, 15, 14, 12);
                black_box(&dest);
            });
        });
    }

    // 768 TD Short
    {
        let cfg = CFG_768_TD_SHORT;
        let src1 = audio_samples(cfg.n_long);
        let win_fwd = window_coeffs(cfg.n_trans_ls + cfg.lfac);
        let fac_data_out = audio_samples(cfg.lfac * 2);
        let over_lap = audio_samples(cfg.n_flat_ls + cfg.lfac);
        let mut dest = vec![0i32; cfg.n_long];

        group.bench_function("768_td_short", |b| {
            b.iter(|| {
                let _ = windowing_long2(&src1, &win_fwd, &fac_data_out, &over_lap, &mut dest, &cfg, 15, 14, 12);
                black_box(&dest);
            });
        });
    }

    group.finish();
}

// ============================================================================
// windowing_long3 benchmarks
// ============================================================================

fn windowing_long3_bench(c: &mut Criterion) {
    use decoder::ixheaacd::windowing_long3;

    let mut group = c.benchmark_group("windowing_long3");

    // 1024 FD, shiftp > shift_olap
    {
        let cfg = CFG_1024_FD;
        let src1 = audio_samples(cfg.n_long);
        let win_fwd = window_coeffs(cfg.n_trans_ls);
        let over_lap = audio_samples(cfg.n_flat_ls + cfg.n_trans_ls);
        let mut dest = vec![0i32; cfg.n_long];
        let win_rev = window_coeffs(cfg.n_trans_ls);

        group.bench_function("1024_fd_shiftp_gt", |b| {
            b.iter(|| {
                let _ = windowing_long3(&src1, &win_fwd, &over_lap, &mut dest, &win_rev, &cfg, 15, 12);
                black_box(&dest);
            });
        });
    }

    // 1024 FD, shiftp <= shift_olap
    {
        let cfg = CFG_1024_FD;
        let src1 = audio_samples(cfg.n_long);
        let win_fwd = window_coeffs(cfg.n_trans_ls);
        let over_lap = audio_samples(cfg.n_flat_ls + cfg.n_trans_ls);
        let mut dest = vec![0i32; cfg.n_long];
        let win_rev = window_coeffs(cfg.n_trans_ls);

        group.bench_function("1024_fd_shiftp_le", |b| {
            b.iter(|| {
                let _ = windowing_long3(&src1, &win_fwd, &over_lap, &mut dest, &win_rev, &cfg, 12, 15);
                black_box(&dest);
            });
        });
    }

    // 1024 TD Long
    {
        let cfg = CFG_1024_TD_LONG;
        let src1 = audio_samples(cfg.n_long);
        let win_fwd = window_coeffs(cfg.n_trans_ls);
        let over_lap = audio_samples(cfg.n_flat_ls + cfg.n_trans_ls);
        let mut dest = vec![0i32; cfg.n_long];
        let win_rev = window_coeffs(cfg.n_trans_ls);

        group.bench_function("1024_td_long", |b| {
            b.iter(|| {
                let _ = windowing_long3(&src1, &win_fwd, &over_lap, &mut dest, &win_rev, &cfg, 15, 12);
                black_box(&dest);
            });
        });
    }

    // 768 FD
    {
        let cfg = CFG_768_FD;
        let src1 = audio_samples(cfg.n_long);
        let win_fwd = window_coeffs(cfg.n_trans_ls);
        let over_lap = audio_samples(cfg.n_flat_ls + cfg.n_trans_ls);
        let mut dest = vec![0i32; cfg.n_long];
        let win_rev = window_coeffs(cfg.n_trans_ls);

        group.bench_function("768_fd", |b| {
            b.iter(|| {
                let _ = windowing_long3(&src1, &win_fwd, &over_lap, &mut dest, &win_rev, &cfg, 15, 12);
                black_box(&dest);
            });
        });
    }

    // 768 TD Long
    {
        let cfg = CFG_768_TD_LONG;
        let src1 = audio_samples(cfg.n_long);
        let win_fwd = window_coeffs(cfg.n_trans_ls);
        let over_lap = audio_samples(cfg.n_flat_ls + cfg.n_trans_ls);
        let mut dest = vec![0i32; cfg.n_long];
        let win_rev = window_coeffs(cfg.n_trans_ls);

        group.bench_function("768_td_long", |b| {
            b.iter(|| {
                let _ = windowing_long3(&src1, &win_fwd, &over_lap, &mut dest, &win_rev, &cfg, 15, 12);
                black_box(&dest);
            });
        });
    }

    group.finish();
}

// ============================================================================
// windowing_short1 benchmarks
// ============================================================================

fn windowing_short1_bench(c: &mut Criterion) {
    use decoder::ixheaacd::windowing_short1;

    let mut group = c.benchmark_group("windowing_short1");

    // 1024 FD, shift_olap > shiftp
    {
        let cfg = CFG_1024_FD;
        let src1 = audio_samples(cfg.n_short);
        let src2 = window_coeffs(cfg.n_short);
        let mut fp = audio_samples(cfg.n_flat_ls + cfg.lfac);

        group.bench_function("1024_fd_shift_olap_gt", |b| {
            b.iter(|| {
                windowing_short1(&src1, &src2, &mut fp, &cfg, 12, 15);
                black_box(&fp);
            });
        });
    }

    // 1024 FD, shiftp > shift_olap
    {
        let cfg = CFG_1024_FD;
        let src1 = audio_samples(cfg.n_short);
        let src2 = window_coeffs(cfg.n_short);
        let mut fp = audio_samples(cfg.n_flat_ls + cfg.lfac);

        group.bench_function("1024_fd_shiftp_gt", |b| {
            b.iter(|| {
                windowing_short1(&src1, &src2, &mut fp, &cfg, 15, 12);
                black_box(&fp);
            });
        });
    }

    // 1024 TD Short (lfac=64, n_short=128 -> n_short > lfac branch)
    {
        let cfg = CFG_1024_TD_SHORT;
        let src1 = audio_samples(cfg.n_short);
        let src2 = window_coeffs(cfg.n_short);
        let mut fp = audio_samples(cfg.n_flat_ls + cfg.lfac);

        group.bench_function("1024_td_short", |b| {
            b.iter(|| {
                windowing_short1(&src1, &src2, &mut fp, &cfg, 15, 12);
                black_box(&fp);
            });
        });
    }

    // 768 FD
    {
        let cfg = CFG_768_FD;
        let src1 = audio_samples(cfg.n_short);
        let src2 = window_coeffs(cfg.n_short);
        let mut fp = audio_samples(cfg.n_flat_ls + cfg.lfac);

        group.bench_function("768_fd", |b| {
            b.iter(|| {
                windowing_short1(&src1, &src2, &mut fp, &cfg, 15, 12);
                black_box(&fp);
            });
        });
    }

    // 768 TD Short (lfac=48, n_short=96 -> n_short > lfac branch)
    {
        let cfg = CFG_768_TD_SHORT;
        let src1 = audio_samples(cfg.n_short);
        let src2 = window_coeffs(cfg.n_short);
        let mut fp = audio_samples(cfg.n_flat_ls + cfg.lfac);

        group.bench_function("768_td_short", |b| {
            b.iter(|| {
                windowing_short1(&src1, &src2, &mut fp, &cfg, 15, 12);
                black_box(&fp);
            });
        });
    }

    group.finish();
}

// ============================================================================
// windowing_short2 benchmarks
// ============================================================================

fn windowing_short2_bench(c: &mut Criterion) {
    use decoder::ixheaacd::windowing_short2;

    let mut group = c.benchmark_group("windowing_short2");

    // 1024 FD, shift_olap > shiftp
    {
        let cfg = CFG_1024_FD;
        let src1 = audio_samples(cfg.n_short / 2);
        let win_fwd = window_coeffs(cfg.n_short);
        let mut fp = audio_samples(cfg.n_flat_ls + cfg.n_short);

        group.bench_function("1024_fd_shift_olap_gt", |b| {
            b.iter(|| {
                windowing_short2(&src1, &win_fwd, &mut fp, &cfg, 12, 15);
                black_box(&fp);
            });
        });
    }

    // 1024 FD, shiftp > shift_olap
    {
        let cfg = CFG_1024_FD;
        let src1 = audio_samples(cfg.n_short / 2);
        let win_fwd = window_coeffs(cfg.n_short);
        let mut fp = audio_samples(cfg.n_flat_ls + cfg.n_short);

        group.bench_function("1024_fd_shiftp_gt", |b| {
            b.iter(|| {
                windowing_short2(&src1, &win_fwd, &mut fp, &cfg, 15, 12);
                black_box(&fp);
            });
        });
    }

    // 768 FD, shift_olap > shiftp
    {
        let cfg = CFG_768_FD;
        let src1 = audio_samples(cfg.n_short / 2);
        let win_fwd = window_coeffs(cfg.n_short);
        let mut fp = audio_samples(cfg.n_flat_ls + cfg.n_short);

        group.bench_function("768_fd_shift_olap_gt", |b| {
            b.iter(|| {
                windowing_short2(&src1, &win_fwd, &mut fp, &cfg, 12, 15);
                black_box(&fp);
            });
        });
    }

    // 768 FD, shiftp > shift_olap
    {
        let cfg = CFG_768_FD;
        let src1 = audio_samples(cfg.n_short / 2);
        let win_fwd = window_coeffs(cfg.n_short);
        let mut fp = audio_samples(cfg.n_flat_ls + cfg.n_short);

        group.bench_function("768_fd_shiftp_gt", |b| {
            b.iter(|| {
                windowing_short2(&src1, &win_fwd, &mut fp, &cfg, 15, 12);
                black_box(&fp);
            });
        });
    }

    group.finish();
}

// ============================================================================
// windowing_short3 benchmarks
// ============================================================================

fn windowing_short3_bench(c: &mut Criterion) {
    use decoder::ixheaacd::windowing_short3;

    let mut group = c.benchmark_group("windowing_short3");

    for (n_short, name) in [(128, "128"), (96, "96")] {
        // shift_olap > shiftp
        {
            let src1 = audio_samples(n_short);
            let win_fwd = window_coeffs(n_short);
            let mut fp = audio_samples(n_short);

            group.bench_function(format!("{}_shift_olap_gt", name), |b| {
                b.iter(|| {
                    let _ = windowing_short3(&src1, &win_fwd, &mut fp, 12, 15);
                    black_box(&fp);
                });
            });
        }

        // shiftp >= shift_olap
        {
            let src1 = audio_samples(n_short);
            let win_fwd = window_coeffs(n_short);
            let mut fp = audio_samples(n_short);

            group.bench_function(format!("{}_shiftp_ge", name), |b| {
                b.iter(|| {
                    let _ = windowing_short3(&src1, &win_fwd, &mut fp, 15, 12);
                    black_box(&fp);
                });
            });
        }
    }

    group.finish();
}

// ============================================================================
// windowing_short4 benchmarks
// ============================================================================

fn windowing_short4_bench(c: &mut Criterion) {
    use decoder::ixheaacd::windowing_short4;

    let mut group = c.benchmark_group("windowing_short4");

    // 128 samples, windowed, shift_olap > output_q
    {
        let src1 = audio_samples(128);
        let win_fwd = window_coeffs(128);
        let mut fp = audio_samples(256);
        let win_rev1 = window_coeffs(128);

        group.bench_function("128_windowed_shift_olap_gt", |b| {
            b.iter(|| {
                let _ = windowing_short4(&src1, &win_fwd, &mut fp, &win_rev1, true, 15, 14, 12);
                black_box(&fp);
            });
        });
    }

    // 128 samples, windowed, shift_olap <= output_q
    {
        let src1 = audio_samples(128);
        let win_fwd = window_coeffs(128);
        let mut fp = audio_samples(256);
        let win_rev1 = window_coeffs(128);

        group.bench_function("128_windowed_shift_olap_le", |b| {
            b.iter(|| {
                let _ = windowing_short4(&src1, &win_fwd, &mut fp, &win_rev1, true, 15, 10, 12);
                black_box(&fp);
            });
        });
    }

    // 128 samples, unwindowed, shift_olap > output_q
    {
        let src1 = audio_samples(128);
        let win_fwd = window_coeffs(128);
        let mut fp = audio_samples(256);
        let win_rev1 = window_coeffs(128);

        group.bench_function("128_unwindowed_shift_olap_gt", |b| {
            b.iter(|| {
                let _ = windowing_short4(&src1, &win_fwd, &mut fp, &win_rev1, false, 15, 14, 12);
                black_box(&fp);
            });
        });
    }

    // 128 samples, unwindowed, shift_olap <= output_q
    {
        let src1 = audio_samples(128);
        let win_fwd = window_coeffs(128);
        let mut fp = audio_samples(256);
        let win_rev1 = window_coeffs(128);

        group.bench_function("128_unwindowed_shift_olap_le", |b| {
            b.iter(|| {
                let _ = windowing_short4(&src1, &win_fwd, &mut fp, &win_rev1, false, 15, 10, 12);
                black_box(&fp);
            });
        });
    }

    // 96 samples, windowed, shift_olap > output_q
    {
        let src1 = audio_samples(96);
        let win_fwd = window_coeffs(96);
        let mut fp = audio_samples(192);
        let win_rev1 = window_coeffs(96);

        group.bench_function("96_windowed_shift_olap_gt", |b| {
            b.iter(|| {
                let _ = windowing_short4(&src1, &win_fwd, &mut fp, &win_rev1, true, 15, 14, 12);
                black_box(&fp);
            });
        });
    }

    // 96 samples, windowed, shift_olap <= output_q
    {
        let src1 = audio_samples(96);
        let win_fwd = window_coeffs(96);
        let mut fp = audio_samples(192);
        let win_rev1 = window_coeffs(96);

        group.bench_function("96_windowed_shift_olap_le", |b| {
            b.iter(|| {
                let _ = windowing_short4(&src1, &win_fwd, &mut fp, &win_rev1, true, 15, 10, 12);
                black_box(&fp);
            });
        });
    }

    // 96 samples, unwindowed
    {
        let src1 = audio_samples(96);
        let win_fwd = window_coeffs(96);
        let mut fp = audio_samples(192);
        let win_rev1 = window_coeffs(96);

        group.bench_function("96_unwindowed", |b| {
            b.iter(|| {
                let _ = windowing_short4(&src1, &win_fwd, &mut fp, &win_rev1, false, 15, 14, 12);
                black_box(&fp);
            });
        });
    }

    group.finish();
}

// ============================================================================
// Real-world scenario benchmarks
// ============================================================================

fn scenario_bench(c: &mut Criterion) {
    use decoder::ixheaacd::{windowing_long1, windowing_long3, windowing_short2, windowing_short3};

    let mut group = c.benchmark_group("scenario");

    // LONG_SEQUENCE window processing (most common case)
    {
        let cfg = CFG_1024_FD;
        let src1 = audio_samples(cfg.n_long / 2);
        let src2 = audio_samples(cfg.n_long);
        let win_fwd = window_coeffs(cfg.n_long);
        let win_rev = window_coeffs(cfg.n_long);
        let mut dest = vec![0i32; cfg.n_long];

        group.bench_function("long_sequence_1024", |b| {
            b.iter(|| {
                let _ = windowing_long1(&src1, &src2, &win_fwd, &win_rev, &mut dest, 15, 15);
                black_box(&dest);
            });
        });
    }

    // LONG_STOP window processing
    {
        let cfg = CFG_1024_FD;
        let src1 = audio_samples(cfg.n_long);
        let win_fwd = window_coeffs(cfg.n_trans_ls);
        let over_lap = audio_samples(cfg.n_flat_ls + cfg.n_trans_ls);
        let mut dest = vec![0i32; cfg.n_long];
        let win_rev = window_coeffs(cfg.n_trans_ls);

        group.bench_function("long_stop_1024", |b| {
            b.iter(|| {
                let _ = windowing_long3(&src1, &win_fwd, &over_lap, &mut dest, &win_rev, &cfg, 15, 15);
                black_box(&dest);
            });
        });
    }

    // One short block in EIGHT_SHORT_SEQUENCE
    {
        let cfg = CFG_1024_FD;
        let src1 = audio_samples(cfg.n_short / 2);
        let win_fwd = window_coeffs(cfg.n_short);
        let mut fp = audio_samples(cfg.n_flat_ls + cfg.n_short);

        group.bench_function("short_block_128", |b| {
            b.iter(|| {
                windowing_short2(&src1, &win_fwd, &mut fp, &cfg, 15, 15);
                black_box(&fp);
            });
        });
    }

    // Final short block overlap-add
    {
        let src1 = audio_samples(128);
        let win_fwd = window_coeffs(128);
        let mut fp = audio_samples(128);

        group.bench_function("short_final_overlap", |b| {
            b.iter(|| {
                let _ = windowing_short3(&src1, &win_fwd, &mut fp, 15, 15);
                black_box(&fp);
            });
        });
    }

    // 768-sample frame processing (lower sample rate)
    {
        let cfg = CFG_768_FD;
        let src1 = audio_samples(cfg.n_long / 2);
        let src2 = audio_samples(cfg.n_long);
        let win_fwd = window_coeffs(cfg.n_long);
        let win_rev = window_coeffs(cfg.n_long);
        let mut dest = vec![0i32; cfg.n_long];

        group.bench_function("long_sequence_768", |b| {
            b.iter(|| {
                let _ = windowing_long1(&src1, &src2, &win_fwd, &win_rev, &mut dest, 15, 15);
                black_box(&dest);
            });
        });
    }

    group.finish();
}

// ============================================================================
// scale_down benchmarks
// ============================================================================

fn scale_down_bench(c: &mut Criterion) {
    use decoder::ixheaacd::{scale_down, scale_down_adj};

    let mut group = c.benchmark_group("scale_down");

    // Right shift benchmarks
    {
        let src = vec![
            8000, 9009, 9090, 9099, 9900, 9990, 9999, 16000, 24670, 32023, 40001, 48002, 56003, 64000,
            72077, 80070, 88009, 96000, 104780, 1120067, 120345, 128000, 345958
        ];
        let mut dest = vec![0; 23];

        group.bench_function("right_shift_len23", |b| {
            b.iter(|| {
                scale_down(&mut dest, &src, 5, 2);
                black_box(&dest);
            });
        });
    }

    for len in [1024, 512, 128, 64, 16] {
        let src = vec![120345; len];
        let mut dest = vec![0; len];

        group.bench_with_input(BenchmarkId::new("right_shift", len), &len, |b, _| {
            b.iter(|| {
                scale_down(&mut dest, &src, 5, 2);
                black_box(&dest);
            });
        });
    }

    // Left shift benchmarks
    {
        let src = vec![
            8000, 9009, 9090, 9099, 9900, 9990, 9999, 16000, 24670, 32023, 40001, 48002, 56003, 64000,
            72077, 80070, 88009, 96000, 104780, 1120067, 120345, 128000, 345958
        ];
        let mut dest = vec![0; 23];

        group.bench_function("left_shift_len23", |b| {
            b.iter(|| {
                scale_down(&mut dest, &src, 2, 5);
                black_box(&dest);
            });
        });
    }

    for len in [1024, 512, 128, 64, 16] {
        let src = vec![120345; len];
        let mut dest = vec![0; len];

        group.bench_with_input(BenchmarkId::new("left_shift", len), &len, |b, _| {
            b.iter(|| {
                scale_down(&mut dest, &src, 2, 5);
                black_box(&dest);
            });
        });
    }

    // Saturation benchmarks
    {
        let src = vec![
            i32::MAX / 2, 1000, i32::MIN / 2, -1000,
            i32::MAX / 4, 2000, i32::MIN / 4, -2000,
            i32::MAX / 2, 1000, i32::MIN / 2, -1000,
            i32::MAX / 4, 2000, i32::MIN / 4, -2000
        ];
        let mut dest = vec![0; 16];

        group.bench_function("saturation_len16", |b| {
            b.iter(|| {
                scale_down(&mut dest, &src, 0, 2);
                black_box(&dest);
            });
        });
    }

    {
        let src = vec![i32::MAX / 2; 128];
        let mut dest = vec![0; 128];

        group.bench_function("saturation_len128", |b| {
            b.iter(|| {
                scale_down(&mut dest, &src, 0, 2);
                black_box(&dest);
            });
        });
    }

    group.finish();

    // scale_down_adj benchmarks
    let mut group = c.benchmark_group("scale_down_adj");

    // Right shift with adjustment
    {
        let src = vec![
            8000, 9009, 9090, 9099, 9900, 9990, 9999, 16000, 24670, 32023, 40001, 48002, 56003, 64000,
            72077, 80070, 88009, 96000, 104780, 1120067, 120345, 128000, 345958
        ];
        let mut dest = vec![0; 23];

        group.bench_function("right_shift_len23", |b| {
            b.iter(|| {
                scale_down_adj(&mut dest, &src, 5, 2);
                black_box(&dest);
            });
        });
    }

    for len in [1024, 512, 128, 16] {
        let src = vec![120345; len];
        let mut dest = vec![0; len];

        group.bench_with_input(BenchmarkId::new("right_shift", len), &len, |b, _| {
            b.iter(|| {
                scale_down_adj(&mut dest, &src, 5, 2);
                black_box(&dest);
            });
        });
    }

    // Left shift with adjustment
    {
        let src = vec![
            8000, 9009, 9090, 9099, 9900, 9990, 9999, 16000, 24670, 32023, 40001, 48002, 56003, 64000,
            72077, 80070, 88009, 96000, 104780, 1120067, 120345, 128000, 345958
        ];
        let mut dest = vec![0; 23];

        group.bench_function("left_shift_len23", |b| {
            b.iter(|| {
                scale_down_adj(&mut dest, &src, 2, 5);
                black_box(&dest);
            });
        });
    }

    for len in [1024, 512, 128, 16] {
        let src = vec![1000; len];
        let mut dest = vec![0; len];

        group.bench_with_input(BenchmarkId::new("left_shift", len), &len, |b, _| {
            b.iter(|| {
                scale_down_adj(&mut dest, &src, 2, 5);
                black_box(&dest);
            });
        });
    }

    group.finish();

    // Real-world scenarios
    let mut group = c.benchmark_group("scale_down_scenario");

    // IMDCT overlap buffer scaling
    {
        let src = vec![8000; 512];
        let mut dest = vec![0; 512];

        group.bench_function("imdct_overlap", |b| {
            b.iter(|| {
                scale_down(&mut dest, &src, 4, 15);
                black_box(&dest);
            });
        });
    }

    // Short block processing
    {
        let src = vec![16000; 128];
        let mut dest = vec![0; 128];

        group.bench_function("short_block", |b| {
            b.iter(|| {
                scale_down(&mut dest, &src, 5, 15);
                black_box(&dest);
            });
        });
    }

    // IMDCT long output with adjustment
    {
        let src = vec![12000; 1024];
        let mut dest = vec![0; 1024];

        group.bench_function("adj_imdct", |b| {
            b.iter(|| {
                scale_down_adj(&mut dest, &src, 3, 15);
                black_box(&dest);
            });
        });
    }

    // Memory access patterns
    {
        let src = vec![8000; 1024];
        let mut dest = vec![0; 1024];

        group.bench_function("sequential_access", |b| {
            b.iter(|| {
                scale_down(&mut dest, &src, 5, 2);
                black_box(&dest);
            });
        });
    }

    {
        let src: Vec<i32> = (0..2048).map(|i| i * 100).collect();
        let mut dest = vec![0; 1024];
        let src_slice: Vec<i32> = src.iter().step_by(2).copied().collect();

        group.bench_function("stride_pattern", |b| {
            b.iter(|| {
                scale_down(&mut dest, &src_slice, 5, 2);
                black_box(&dest);
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    combine_fac_bench,
    windowing_long1_bench,
    windowing_long2_bench,
    windowing_long3_bench,
    windowing_short1_bench,
    windowing_short2_bench,
    windowing_short3_bench,
    windowing_short4_bench,
    scenario_bench,
    scale_down_bench,
);

criterion_main!(benches);
