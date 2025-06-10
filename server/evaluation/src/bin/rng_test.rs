use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_pcg::Pcg64;
use rand_xoshiro::Xoshiro128PlusPlus;
use std::ops::Add;
use std::time::{Duration, Instant};

const NUM_SAMPLES: usize = 100_000_000;

fn benchmark_rng<R: Rng>(mut rng: R, name: &str) -> Duration {
    let start = Instant::now();

    let mut sum = 0u64;
    for _ in 0..NUM_SAMPLES {
        sum = sum.wrapping_add(rng.random::<u64>());
    }

    if sum == 1 {
        println!(
            "This will never be printed, but prevents optimization: {}",
            sum
        );
    }

    let duration = start.elapsed();
    println!(
        "{}: Generated {} random numbers in {:.6} seconds ({:.2} million numbers/second)",
        name,
        NUM_SAMPLES,
        duration.as_secs_f64(),
        NUM_SAMPLES as f64 / duration.as_secs_f64() / 1_000_000.0
    );

    duration
}

fn benchmark_number_types<R: Rng + Clone>(rng: R, name: &str) {
    println!("\n{}:", name);

    // Benchmark u32
    let mut rng_clone = rng.clone();
    let start = Instant::now();
    let mut sum_u32 = 0u32;
    for _ in 0..NUM_SAMPLES {
        sum_u32 = sum_u32.wrapping_add(rng_clone.random::<u32>());
    }
    let duration = start.elapsed();

    println!(
        "{} (u32): {:.6} seconds ({:.2} million numbers/second)",
        name,
        duration.as_secs_f64(),
        NUM_SAMPLES as f64 / duration.as_secs_f64() / 1_000_000.0
    );

    // Benchmark u64
    let mut rng_clone = rng.clone();
    let start = Instant::now();
    let mut sum_u64 = 0u64;
    for _ in 0..NUM_SAMPLES {
        sum_u64 = sum_u64.wrapping_add(rng_clone.random::<u64>());
    }
    let duration = start.elapsed();
    println!(
        "{} (u64): {:.6} seconds ({:.2} million numbers/second)",
        name,
        duration.as_secs_f64(),
        NUM_SAMPLES as f64 / duration.as_secs_f64() / 1_000_000.0
    );

    // Benchmark f64
    let mut rng_clone = rng.clone();
    let start = Instant::now();
    let mut sum_f64 = 0.0f64;
    for _ in 0..NUM_SAMPLES {
        sum_f64 = sum_f64.add(rng_clone.random::<f64>());
    }
    let duration = start.elapsed();
    println!(
        "{} (f64): {:.6} seconds ({:.2} million numbers/second)",
        name,
        duration.as_secs_f64(),
        NUM_SAMPLES as f64 / duration.as_secs_f64() / 1_000_000.0
    );

    if sum_u32 == 1 && sum_u64 == 1 && sum_f64 == 1.0 {
        println!("Prevent optimization!");
    }
}

fn compare_rngs() {
    let std_rng = StdRng::from_os_rng();
    let std_rng_duration = benchmark_rng(std_rng, "StdRng");

    let xoshiro_rng = Xoshiro128PlusPlus::from_os_rng();
    let xoshiro_duration = benchmark_rng(xoshiro_rng, "Xoshiro128PlusPlus");

    let chacha8_rng = ChaCha8Rng::from_os_rng();
    let chacha8_duration = benchmark_rng(chacha8_rng, "Chacha8Rng");

    let pcg64_rng = Pcg64::from_os_rng();
    let pcg64_duration = benchmark_rng(pcg64_rng, "Pcg64");

    let baseline = std_rng_duration;
    println!("StdRng: {:.6} seconds (baseline)", baseline.as_secs_f64());
    print_result("Xoshiro128PlusPlus", baseline, xoshiro_duration);
    print_result("ChaCha8Rng", baseline, chacha8_duration);
    print_result("Pcg64", baseline, pcg64_duration);

    // Benchmark different number types
    benchmark_number_types(StdRng::from_os_rng(), "StdRng");
    benchmark_number_types(Xoshiro128PlusPlus::from_os_rng(), "Xoshiro128PlusPlus");
    benchmark_number_types(ChaCha8Rng::from_os_rng(), "ChaCha8Rng");
    benchmark_number_types(Pcg64::from_os_rng(), "Pcg64");
}

fn print_result(name: &str, baseline: Duration, duration: Duration) {
    println!(
        "{}: {:.6} seconds ({:.2}x {})",
        name,
        duration.as_secs_f64(),
        if duration < baseline {
            baseline.as_secs_f64() / duration.as_secs_f64()
        } else {
            duration.as_secs_f64() / baseline.as_secs_f64()
        },
        if duration < baseline {
            "faster"
        } else {
            "slower"
        }
    );
}

fn main() {
    compare_rngs();
}
