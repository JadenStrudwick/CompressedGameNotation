use cgn::{
    benchmark_utils::{collect_metrics, metrics_to_summary, ToTake},
    compression::{bincode, dynamic_huffman, huffman},
};
use criterion::{criterion_group, criterion_main, Criterion};

const DB_PATH: &str = "./benches/lichessDB.pgn";
const N: ToTake = ToTake::N(100_000);

/// Collects and prints metrics for the bincode_zlib compression strategy.
fn bench_bincode(_c: &mut Criterion) {
    println!("[BENCHMARK] Collecting metrics for bincode...");
    let metrics = collect_metrics(bincode::compress_pgn_data, bincode::decompress_pgn_data, DB_PATH, N);
    println!("{}", metrics_to_summary(metrics));
}

/// Collects and prints metrics for the huffman compression strategy.
fn bench_huffman(_c: &mut Criterion) {
    println!("[BENCHMARK] Collecting metrics for huffman...");
    let metrics = collect_metrics(huffman::compress_pgn_data, huffman::decompress_pgn_data, DB_PATH, N);
    println!("{}", metrics_to_summary(metrics));
}

/// Collects and prints metrics for the dynamic huffman compression strategy.
fn bench_dynamic_huffman(_c: &mut Criterion) {
    println!("[BENCHMARK] Collecting metrics for dynamic huffman...");
    let metrics = collect_metrics(
        dynamic_huffman::compress_pgn_data,
        dynamic_huffman::decompress_pgn_data,
        DB_PATH,
        N,
    );
    println!("{}", metrics_to_summary(metrics));
}

criterion_group!(benches, bench_bincode, bench_huffman, bench_dynamic_huffman);
criterion_main!(benches);
