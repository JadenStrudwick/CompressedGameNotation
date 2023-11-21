use cgn::compression::bincode_zlib;
use criterion::{criterion_group, criterion_main, Criterion};

/// Collects and prints metrics for the bincode_zlib compression strategy.
fn bench_bincode_zlib(_c: &mut Criterion) {
    println!("[BENCHMARK] Collecting metrics for bincode_zlib...");
    utils::collect_metrics(
        bincode_zlib::compress_pgn_data,
        bincode_zlib::decompress_pgn_data,
    );
}

/// Collects and prints metrics for the huffman compression strategy.
fn bench_huffman(_c: &mut Criterion) {
    println!("[BENCHMARK] Collecting metrics for huffman...");
    utils::collect_metrics(
        cgn::compression::huffman::compress_pgn_data,
        cgn::compression::huffman::decompress_pgn_data,
    );
}

// criterion_group!(benches, bench_bincode_zlib);
// criterion_group!(benches, bench_huffman);
criterion_group!(benches, bench_bincode_zlib, bench_huffman);
criterion_main!(benches);

mod utils {
    use anyhow::Result;
    use bit_vec::BitVec;
    use cgn::pgn_data::PgnData;
    use rayon::prelude::*;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        str::FromStr,
    };

    /// An iterator over the games in a PGN database file.
    struct PgnDBIter<R: BufRead> {
        reader: R,
        buffer: String,
    }

    impl<R: BufRead> PgnDBIter<R> {
        fn new(reader: R) -> Self {
            Self {
                reader,
                buffer: String::new(),
            }
        }
    }

    impl<R: BufRead> Iterator for PgnDBIter<R> {
        // The type of the elements being iterated over.
        type Item = String;

        /// Get the next game in the database.
        fn next(&mut self) -> Option<Self::Item> {
            let mut game = String::new();

            // read until the next game
            loop {
                self.buffer.clear();
                match self.reader.read_line(&mut self.buffer) {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        // skip empty lines at the start of a game
                        if game.is_empty() && self.buffer.trim().is_empty() {
                            continue;
                        // stop reading if we reach the start of the next game
                        } else if self.buffer.starts_with("[Event") && !game.is_empty() {
                            break;
                        }
                        // otherwise, add the line to the game
                        game.push_str(&self.buffer);
                    }
                    Err(_) => return None,
                }
            }

            // return the game if it's not empty
            if game.trim().is_empty() {
                None
            } else {
                Some(game)
            }
        }
    }

    /// Opens a PGN database file and returns an iterator over the games in the database.
    fn pgn_db_into_iter(path: &str) -> Result<PgnDBIter<BufReader<File>>, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(PgnDBIter::new(reader))
    }

    ///  Metrics for a compression strategy.
    /// * Time to compress game (nanoseconds)
    /// * Time to decompress game (nanoseconds)
    /// * Size of uncompressed game (total bits including headers)
    /// * Size of compressed game (total bits including headers)
    /// * Bits per move (total bits / number of moves)
    /// * Bits per move excluding headers (total move bits / number of moves)
    pub struct Metrics {
        time_to_compress: u128,
        time_to_decompress: u128,
        compressed_size: usize,
        decompressed_size: usize,
        bits_per_move: f64,
        bits_per_move_excluding_headers: f64,
    }

    /// Collect a single metric for a compression strategy.
    fn collect_single_metric(
        pgn_str: &str,
        compress_fn: fn(&PgnData) -> Result<BitVec>,
        decompress_fn: fn(&BitVec) -> Result<PgnData>,
    ) -> Result<Metrics> {
        let mut pgn_data = PgnData::from_str(pgn_str)?;

        // if the game is empty, skip it
        if pgn_data.moves.is_empty() {
            return Err(anyhow::anyhow!("Game is empty"));
        }

        // time to compress
        let start = std::time::Instant::now();
        let compressed_data = compress_fn(&pgn_data)?;
        let end = std::time::Instant::now();
        let time_to_compress = end.duration_since(start).as_nanos();

        // compressed size
        let compressed_size = compressed_data.len();

        // time to decompress
        let start = std::time::Instant::now();
        let decompressed_data = decompress_fn(&compressed_data)?;
        let end = std::time::Instant::now();
        let time_to_decompress = end.duration_since(start).as_nanos();

        // decompressed size
        let decompressed_size = decompressed_data.to_string().len() * 8;

        // bits per move
        let bits_per_move = compressed_size as f64 / pgn_data.moves.len() as f64;

        // bits per move excluding headers
        pgn_data.clear_headers();
        let compressed_data_no_headers = compress_fn(&pgn_data)?;
        let bits_per_move_excluding_headers =
            compressed_data_no_headers.len() as f64 / pgn_data.moves.len() as f64;

        Ok(Metrics {
            time_to_compress,
            time_to_decompress,
            compressed_size,
            decompressed_size,
            bits_per_move,
            bits_per_move_excluding_headers,
        })
    }

    /// Collect the metrics for a compression strategy.
    pub fn collect_metrics(
        compress_fn: fn(&PgnData) -> Result<BitVec>,
        decompress_fn: fn(&BitVec) -> Result<PgnData>,
    ) {
        let start = std::time::Instant::now();
        let metrics = pgn_db_into_iter("./benches/lichessDB.pgn")
            .expect("Failed to open PGN database file")
            .par_bridge()
            .map(|pgn_str| collect_single_metric(&pgn_str, compress_fn, decompress_fn))
            .filter_map(|x| x.ok())
            .collect::<Vec<_>>();
        let end = std::time::Instant::now();

        if metrics.is_empty() {
            println!("\tNo metrics collected");
            return;
        }

        // compute averages
        let avg_time_to_compress =
            metrics.iter().map(|x| x.time_to_compress).sum::<u128>() / metrics.len() as u128;
        let avg_time_to_decompress =
            metrics.iter().map(|x| x.time_to_decompress).sum::<u128>() / metrics.len() as u128;
        let avg_compressed_size =
            metrics.iter().map(|x| x.compressed_size).sum::<usize>() / metrics.len();
        let avg_decompressed_size =
            metrics.iter().map(|x| x.decompressed_size).sum::<usize>() / metrics.len();
        let avg_bits_per_move =
            metrics.iter().map(|x| x.bits_per_move).sum::<f64>() / metrics.len() as f64;
        let avg_bits_per_move_excluding_headers = metrics
            .iter()
            .map(|x| x.bits_per_move_excluding_headers)
            .sum::<f64>()
            / metrics.len() as f64;
        let compression_ratio = avg_compressed_size as f64 / avg_decompressed_size as f64;

        println!("\tNumber of games benchmarked against: {}", metrics.len());
        println!(
            "\tTotal time to benchmark: {} seconds",
            end.duration_since(start).as_nanos() as f64 / 1_000_000_000.0
        );
        println!(
            "\tAverage time to compress: {} seconds",
            avg_time_to_compress as f64 / 1_000_000_000.0
        );
        println!(
            "\tAverage time to decompress: {} seconds",
            avg_time_to_decompress as f64 / 1_000_000_000.0
        );
        println!("\tAverage compressed size: {} bits", avg_compressed_size);
        println!(
            "\tAverage decompressed size: {} bits",
            avg_decompressed_size
        );
        println!("\tAverage bits per move: {}", avg_bits_per_move);
        println!(
            "\tAverage bits per move excluding headers: {}",
            avg_bits_per_move_excluding_headers
        );
        println!("\tAverage compression ratio: {}", compression_ratio);
    }
}
