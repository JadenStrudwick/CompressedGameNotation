use crate::pgn_data::PgnData;
use anyhow::{anyhow, Result};
use bit_vec::BitVec;
use rayon::prelude::*;
use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    time::Instant,
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
fn pgn_db_into_iter(path: &str) -> Result<PgnDBIter<BufReader<File>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(PgnDBIter::new(reader))
}

///  Metrics for a compression strategy.
/// * Time to compress game (seconds)
/// * Time to decompress game (seconds)
/// * Size of uncompressed game (total bits including headers)
/// * Size of compressed game (total bits including headers)
/// * Bits per move (total bits / number of moves)
/// * Bits per move excluding headers (total move bits / number of moves)
pub struct Metrics {
    time_to_compress: f64,
    time_to_decompress: f64,
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
        return Err(anyhow!("Game is empty"));
    }

    // time to compress
    let start = Instant::now();
    let compressed_data = compress_fn(&pgn_data)?;
    let end = Instant::now();
    let time_to_compress = end.duration_since(start).as_secs_f64();

    // compressed size
    let compressed_size = compressed_data.len();

    // time to decompress
    let start = Instant::now();
    let decompressed_data = decompress_fn(&compressed_data)?;
    let end = Instant::now();
    let time_to_decompress = end.duration_since(start).as_secs_f64();

    // decompressed size
    let decompressed_size = decompressed_data.to_string().len() * 8;

    // bits per move
    let bits_per_move = compressed_size as f64 / pgn_data.moves.len() as f64;

    // bits per move excluding headers
    pgn_data.clear_headers();
    let compressed_data_no_headers = compress_fn(&pgn_data)?;
    let bits_per_move_excluding_headers =
        (compressed_data_no_headers.len()) as f64 / pgn_data.moves.len() as f64;

    Ok(Metrics {
        time_to_compress,
        time_to_decompress,
        compressed_size,
        decompressed_size,
        bits_per_move,
        bits_per_move_excluding_headers,
    })
}

/// Collect a single metric for a compression strategy.
fn collect_single_metric_custom(
    pgn_str: &str,
    compress_fn: fn(&PgnData, f64, f64) -> Result<BitVec>,
    decompress_fn: fn(&BitVec, f64, f64) -> Result<PgnData>,
    height: f64,
    dev: f64,
) -> Result<Metrics> {
    let mut pgn_data = PgnData::from_str(pgn_str)?;

    // if the game is empty, skip it
    if pgn_data.moves.is_empty() {
        return Err(anyhow!("Game is empty"));
    }

    // time to compress
    let start = Instant::now();
    let compressed_data = compress_fn(&pgn_data, height, dev)?;
    let end = Instant::now();
    let time_to_compress = end.duration_since(start).as_secs_f64();

    // compressed size
    let compressed_size = compressed_data.len();

    // time to decompress
    let start = Instant::now();
    let decompressed_data = decompress_fn(&compressed_data, height, dev)?;
    let end = Instant::now();
    let time_to_decompress = end.duration_since(start).as_secs_f64();

    // decompressed size
    let decompressed_size = decompressed_data.to_string().len() * 8;

    // bits per move
    let bits_per_move = compressed_size as f64 / pgn_data.moves.len() as f64;

    // bits per move excluding headers
    pgn_data.clear_headers();
    let compressed_data_no_headers = compress_fn(&pgn_data, height, dev)?;
    let bits_per_move_excluding_headers =
        (compressed_data_no_headers.len()) as f64 / pgn_data.moves.len() as f64;

    Ok(Metrics {
        time_to_compress,
        time_to_decompress,
        compressed_size,
        decompressed_size,
        bits_per_move,
        bits_per_move_excluding_headers,
    })
}

#[derive(Clone)]
/// A number of games to take from a PGN database. If `all`, take all games.
pub enum ToTake {
    All,
    N(usize),
}

impl FromStr for ToTake {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "all" {
            Ok(ToTake::All)
        } else {
            Ok(ToTake::N(s.parse()?))
        }
    }
}

impl Display for ToTake {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ToTake::All => write!(f, "all"),
            ToTake::N(n) => write!(f, "{}", n),
        }
    }
}

/// Collect the metrics for a compression strategy. Only guaranteed to work with Lichess PGN databases.
pub fn collect_metrics(
    compress_fn: fn(&PgnData) -> Result<BitVec>,
    decompress_fn: fn(&BitVec) -> Result<PgnData>,
    db_path: &str,
    n: &ToTake,
) -> Vec<Metrics> {
    if let ToTake::N(n) = n {
        pgn_db_into_iter(db_path)
            .expect("Failed to open PGN database file")
            .par_bridge()
            .take_any(*n)
            .map(|pgn_str| collect_single_metric(&pgn_str, compress_fn, decompress_fn))
            .filter_map(|x| x.ok())
            .collect::<Vec<_>>()
    } else {
        pgn_db_into_iter(db_path)
            .expect("Failed to open PGN database file")
            .par_bridge()
            .map(|pgn_str| collect_single_metric(&pgn_str, compress_fn, decompress_fn))
            .filter_map(|x| x.ok())
            .collect::<Vec<_>>()
    }
}

/// Collect the metrics for a compression strategy. Only guaranteed to work with Lichess PGN databases.
pub fn collect_metrics_custom(
    compress_fn: fn(&PgnData, f64, f64) -> Result<BitVec>,
    decompress_fn: fn(&BitVec, f64, f64) -> Result<PgnData>,
    db_path: &str,
    n: &ToTake,
    height: f64,
    dev: f64,
) -> Vec<Metrics> {
    if let ToTake::N(n) = n {
        pgn_db_into_iter(db_path)
            .expect("Failed to open PGN database file")
            .par_bridge()
            .take_any(*n)
            .map(|pgn_str| {
                collect_single_metric_custom(&pgn_str, compress_fn, decompress_fn, height, dev)
            })
            .filter_map(|x| x.ok())
            .collect::<Vec<_>>()
    } else {
        pgn_db_into_iter(db_path)
            .expect("Failed to open PGN database file")
            .par_bridge()
            .map(|pgn_str| {
                collect_single_metric_custom(&pgn_str, compress_fn, decompress_fn, height, dev)
            })
            .filter_map(|x| x.ok())
            .collect::<Vec<_>>()
    }
}

/// Summarize the metrics for a compression strategy.
pub fn metrics_to_summary(metrics: Vec<Metrics>) -> Summary {
    if metrics.is_empty() {
        return Summary {
            avg_time_to_compress: 0.0,
            avg_time_to_decompress: 0.0,
            avg_compressed_size: 0,
            avg_decompressed_size: 0,
            avg_bits_per_move: 0.0,
            avg_bits_per_move_excluding_headers: 0.0,
            compression_ratio: 0.0,
        };
    }

    // compute averages
    let avg_time_to_compress =
        metrics.iter().map(|x| x.time_to_compress).sum::<f64>() / metrics.len() as f64;
    let avg_time_to_decompress =
        metrics.iter().map(|x| x.time_to_decompress).sum::<f64>() / metrics.len() as f64;
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

    Summary {
        avg_time_to_compress,
        avg_time_to_decompress,
        avg_compressed_size,
        avg_decompressed_size,
        avg_bits_per_move,
        avg_bits_per_move_excluding_headers,
        compression_ratio,
    }
}

/// A summary of the metrics for a compression strategy
pub struct Summary {
    pub avg_time_to_compress: f64,
    pub avg_time_to_decompress: f64,
    pub avg_compressed_size: usize,
    pub avg_decompressed_size: usize,
    pub avg_bits_per_move: f64,
    pub avg_bits_per_move_excluding_headers: f64,
    pub compression_ratio: f64,
}

impl Display for Summary {
    /// Display the summary
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Average time to compress: {} seconds",
            self.avg_time_to_compress
        )?;
        writeln!(
            f,
            "Average time to decompress: {} seconds",
            self.avg_time_to_decompress
        )?;
        writeln!(
            f,
            "Average compressed size: {} bits",
            self.avg_compressed_size
        )?;
        writeln!(
            f,
            "Average decompressed size: {} bits",
            self.avg_decompressed_size
        )?;
        writeln!(f, "Average bits per move: {}", self.avg_bits_per_move)?;
        writeln!(
            f,
            "Average bits per move excluding headers: {}",
            self.avg_bits_per_move_excluding_headers
        )?;
        writeln!(f, "Average compression ratio: {}", self.compression_ratio)
    }
}
