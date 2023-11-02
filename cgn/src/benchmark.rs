use crate::pgn_data::PgnData;

#[derive(Debug)]
///  Metrics for a compression strategy.
/// * Time to compress game (nanoseconds)
/// * Time to decompress game (nanoseconds)
/// * Size of uncompressed game (total bytes including headers)
/// * Size of compressed game (total bytes including headers)
/// * Bits per move (total bits / number of moves)
/// * Bits per move excluding headers (total move bits / number of moves)
struct Metrics {
    time_to_compress: u128,
    time_to_decompress: u128,
    compressed_size: usize,
    decompressed_size: usize,
    bits_per_move: f64,
    bits_per_move_excluding_headers: f64,
}

/// Collect the metrics for a compression strategy.
/// # Arguments
/// * `pgn_str` - The PGN data.
/// * `compress_fn` - The compression function.
/// * `decompress_fn` - The decompression function.
/// # Returns
/// The metrics for the compression strategy.
fn collect_metrics(
    pgn_str: &str,
    compress_fn: fn(&PgnData) -> Vec<u8>,
    decompress_fn: fn(&[u8]) -> PgnData,
) -> Metrics {
    let mut pgn_data = PgnData::from_str(pgn_str);

    // time to compress
    let start = std::time::Instant::now();
    let compressed_data = compress_fn(&pgn_data);
    let end = std::time::Instant::now();
    let time_to_compress = end.duration_since(start).as_nanos();

    // compressed size
    let compressed_size = compressed_data.len();

    // time to decompress
    let start = std::time::Instant::now();
    let decompressed_data = decompress_fn(&compressed_data);
    let end = std::time::Instant::now();
    let time_to_decompress = end.duration_since(start).as_nanos();

    // decompressed size
    let decompressed_size = decompressed_data.to_string().len();

    // bits per move
    let bits_per_move = (compressed_size * 8) as f64 / pgn_data.moves.len() as f64;

    // bits per move excluding headers
    pgn_data.clear_headers();
    let compressed_data_no_headers = compress_fn(&pgn_data);
    let bits_per_move_excluding_headers =
        (compressed_data_no_headers.len() * 8) as f64 / pgn_data.moves.len() as f64;

    Metrics {
        time_to_compress,
        time_to_decompress,
        compressed_size,
        decompressed_size,
        bits_per_move,
        bits_per_move_excluding_headers,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    /// Test that metrics can be generated from a single PGN.
    fn can_generate_metrics() {
        let pgn_str = crate::pgn_examples::PGN_STR_EXAMPLE;
        let metrics = super::collect_metrics(
            pgn_str,
            crate::compression::bincode_zlib::compress,
            crate::compression::bincode_zlib::decompress,
        );
        assert_eq!(metrics.compressed_size, 403);
        assert_eq!(metrics.decompressed_size, 744);
    }
}
