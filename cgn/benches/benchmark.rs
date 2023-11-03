mod utils;
use criterion::{criterion_group, criterion_main, Criterion};
use cgn::bincode_zlib;

// Number of games to collect metrics for
const NUM_TO_COLLECT: usize = 10000;

/// Collects and prints metrics for the bincode_zlib compression strategy.
fn bench_bincode_zlib(_c: &mut Criterion) {
    println!("[BENCHMARK] Collecting metrics for bincode_zlib...");
    utils::collect_metrics(
        NUM_TO_COLLECT,
        bincode_zlib::compress_pgn_data,
        bincode_zlib::decompress_pgn_data,
    );
}

criterion_group!(benches, bench_bincode_zlib);
criterion_main!(benches);
