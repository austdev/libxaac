use divan::Bencher;

// Import the functions we want to benchmark
use decoder::ixheaacd_basic_ops::{scale_down, scale_down_adj};

fn main() {
    divan::main();
}

// ============================================================================
// Benchmark Groups
// ============================================================================

#[divan::bench_group]
mod scale_down_bench {
    use super::*;

    // ----------------------------------------------------------------------------
    // scale_down() benchmarks - Right shift (scale down)
    // ----------------------------------------------------------------------------

    #[divan::bench]
    fn scale_down_right_shift_len23(bencher: Bencher) {
        let src = vec![
            8000, 9009, 9090, 9099, 9900, 9990,9999, 16000, 24670, 32023, 40001, 48002, 56003, 64000,
            72077, 80070, 88009, 96000, 104780, 1120067, 120345, 128000, 345958
        ];
        let mut dest = vec![0; 23];
        
        bencher.bench_local(|| {
            scale_down(&mut dest, &src, 5, 2);
            divan::black_box(&dest);
        });
    }

    #[divan::bench(args = [1024, 512, 128, 64, 16])]
    fn scale_down_right_shift_len(bencher: Bencher, len: usize) {
        let src = vec![120345; len];
        let mut dest = vec![0; len];
        
        bencher.bench_local(|| {
            scale_down(&mut dest, &src, 5, 2);
            divan::black_box(&dest);
        });
    }

    // ----------------------------------------------------------------------------
    // scale_down() benchmarks - Left shift (scale up)
    // ----------------------------------------------------------------------------

    #[divan::bench]
    fn scale_down_left_shift_len23(bencher: Bencher) {
        let src = vec![
            8000, 9009, 9090, 9099, 9900, 9990,9999, 16000, 24670, 32023, 40001, 48002, 56003, 64000,
            72077, 80070, 88009, 96000, 104780, 1120067, 120345, 128000, 345958
        ];
        let mut dest = vec![0; 23];
        
        bencher.bench_local(|| {
            scale_down(&mut dest, &src, 2, 5); // Left shift by 3
            divan::black_box(&dest);
        });
    }

    #[divan::bench(args = [1024, 512, 128, 64, 16])]
    fn scale_down_left_shift_len(bencher: Bencher, len: usize) {
        let src = vec![120345; len];
        let mut dest = vec![0; len];
        
        bencher.bench_local(|| {
            scale_down(&mut dest, &src, 2, 5);
            divan::black_box(&dest);
        });
    }

    // ----------------------------------------------------------------------------
    // scale_down() benchmarks - With saturation
    // ----------------------------------------------------------------------------

    #[divan::bench]
    fn scale_down_saturation_len16(bencher: Bencher) {
        let src = vec![
            i32::MAX / 2, 1000, i32::MIN / 2, -1000,
            i32::MAX / 4, 2000, i32::MIN / 4, -2000,
            i32::MAX / 2, 1000, i32::MIN / 2, -1000,
            i32::MAX / 4, 2000, i32::MIN / 4, -2000
        ];
        let mut dest = vec![0; 16];
        
        bencher.bench_local(|| {
            scale_down(&mut dest, &src, 0, 2);
            divan::black_box(&dest);
        });
    }

    #[divan::bench]
    fn scale_down_saturation_len128(bencher: Bencher) {
        let src = vec![i32::MAX / 2; 128];
        let mut dest = vec![0; 128];
        
        bencher.bench_local(|| {
            scale_down(&mut dest, &src, 0, 2);
            divan::black_box(&dest);
        });
    }

    // ----------------------------------------------------------------------------
    // scale_down_adj() benchmarks - Right shift with adjustment
    // ----------------------------------------------------------------------------

    #[divan::bench]
    fn scale_down_adj_right_shift_len23(bencher: Bencher) {
        let src = vec![
            8000, 9009, 9090, 9099, 9900, 9990,9999, 16000, 24670, 32023, 40001, 48002, 56003, 64000,
            72077, 80070, 88009, 96000, 104780, 1120067, 120345, 128000, 345958
        ];
        let mut dest = vec![0; 23];
        
        bencher.bench_local(|| {
            scale_down_adj(&mut dest, &src, 5, 2);
            divan::black_box(&dest);
        });
    }

    #[divan::bench(args = [1024, 512, 128, 16])]
    fn scale_down_adj_right_shift_len(bencher: Bencher, len: usize) {
        let src = vec![120345; len];
        let mut dest = vec![0; len];
        
        bencher.bench_local(|| {
            scale_down_adj(&mut dest, &src, 5, 2);
            divan::black_box(&dest);
        });
    }


    // ----------------------------------------------------------------------------
    // scale_down_adj() benchmarks - Left shift with adjustment
    // ----------------------------------------------------------------------------

    #[divan::bench]
    fn scale_down_adj_left_shift_len23(bencher: Bencher) {
        let src = vec![
            8000, 9009, 9090, 9099, 9900, 9990,9999, 16000, 24670, 32023, 40001, 48002, 56003, 64000,
            72077, 80070, 88009, 96000, 104780, 1120067, 120345, 128000, 345958
        ];
        let mut dest = vec![0; 23];
        
        bencher.bench_local(|| {
            scale_down_adj(&mut dest, &src, 2, 5);
            divan::black_box(&dest);
        });
    }

    #[divan::bench(args = [1024, 512, 128, 16])]
    fn scale_down_adj_left_shift_len(bencher: Bencher, len: usize) {
        let src = vec![1000; len];
        let mut dest = vec![0; len];
        
        bencher.bench_local(|| {
            scale_down_adj(&mut dest, &src, 2, 5);
            divan::black_box(&dest);
        });
    }

    // ============================================================================
    // Real-world scenario benchmarks
    // ============================================================================

    #[divan::bench]
    fn scale_down_scenario_imdct_overlap(bencher: Bencher) {
        // Simulate typical IMDCT overlap buffer scaling
        // Long block: 1024 samples, using half (512) for overlap
        let src = vec![8000; 512];
        let mut dest = vec![0; 512];
        
        bencher.bench_local(|| {
            scale_down(&mut dest, &src, 4, 15); // Typical shiftp=4, output_q=15
            divan::black_box(&dest);
        });
    }

    #[divan::bench]
    fn scale_down_scenario_short_block(bencher: Bencher) {
        // Simulate short block processing (128 samples)
        let src = vec![16000; 128];
        let mut dest = vec![0; 128];
        
        bencher.bench_local(|| {
            scale_down(&mut dest, &src, 5, 15);
            divan::black_box(&dest);
        });
    }

    #[divan::bench]
    fn scale_down_scenario_adj_imdct(bencher: Bencher) {
        // Simulate IMDCT long output with adjustment
        let src = vec![12000; 1024];
        let mut dest = vec![0; 1024];
        
        bencher.bench_local(|| {
            scale_down_adj(&mut dest, &src, 3, 15);
            divan::black_box(&dest);
        });
    }

    // ============================================================================
    // Memory access patterns
    // ============================================================================

    #[divan::bench]
    fn scale_down_mem_sequential_access(bencher: Bencher) {
        let src = vec![8000; 1024];
        let mut dest = vec![0; 1024];
        
        bencher.bench_local(|| {
            scale_down(&mut dest, &src, 5, 2);
            divan::black_box(&dest);
        });
    }

    #[divan::bench]
    fn scale_down_mem_stride_pattern(bencher: Bencher) {
        // Benchmark with non-contiguous pattern (every other element)
        // This simulates potential cache miss scenarios
        let src: Vec<i32> = (0..2048).map(|i| i * 100).collect();
        let mut dest = vec![0; 1024];
        let src_slice: Vec<i32> = src.iter().step_by(2).copied().collect();
        
        bencher.bench_local(|| {
            scale_down(&mut dest, &src_slice, 5, 2);
            divan::black_box(&dest);
        });
    }
}
