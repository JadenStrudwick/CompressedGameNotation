use std::env;
use anyhow::Result;

use bit_vec::BitVec;
use cgn::benchmark_utils::metrics_to_summary;
use cgn::compression::dynamic_huffman::compress_pgn_data_custom;
use cgn::compression::dynamic_huffman::decompress_pgn_data_custom;

use cgn::benchmark_utils::collect_metrics;
use cgn::pgn_data::PgnData;

const N: usize = 100;

fn compress_pgn_data(pgn_data: &PgnData) -> Result<BitVec> {
  let height = env::args().nth(1).unwrap_or("0.0".to_string()).parse::<f64>().unwrap();
  let dev = env::args().nth(2).unwrap_or("0.0".to_string()).parse::<f64>().unwrap();
  compress_pgn_data_custom(pgn_data, height, dev)
}

fn decompress_pgn_data(compressed_data: &BitVec) -> Result<PgnData> {
  let height = env::args().nth(1).unwrap_or("0.0".to_string()).parse::<f64>().unwrap();
  let dev = env::args().nth(2).unwrap_or("0.0".to_string()).parse::<f64>().unwrap();
  decompress_pgn_data_custom(compressed_data, height, dev)
}

fn main() {
  let metrics = collect_metrics(
    compress_pgn_data,
    decompress_pgn_data,
    N
  );
  let KPI = metrics_to_summary(metrics).avg_bits_per_move_excluding_headers;
  println!("{}", KPI)
}