use crate::pgn_data::PgnData;

#[derive(Debug)]
///  Metrics for a compression strategy.
/// * Time to compress game (nanoseconds)
/// * Time to decompress game (nanoseconds)
/// * Size of uncompressed game (total bytes including headers)
/// * Size of compressed game (total bytes including headers)
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

/// Collect the metrics for a compression strategy.
/// # Arguments
/// * `pgn_str` - The PGN data.
/// * `compress_fn` - The compression function.
/// * `decompress_fn` - The decompression function.
/// # Returns
/// The metrics for the compression strategy.
pub fn collect_metrics(
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

    #[test]
    /// Test that metrics can be generated from multiple PGNs.
    fn can_generate_metrics_multiple() {
        const NUM_TO_COLLECT : usize = 1000;
        let iter = crate::pgn_db_iter::pgn_db_into_iter("./lichessDB.pgn");

        let mut avg_time_to_compress = 0;
        let mut avg_time_to_decompress = 0;
        let mut avg_compressed_size = 0;
        let mut avg_decompressed_size = 0;
        let mut avg_bits_per_move = 0.0;
        let mut avg_bits_per_move_excluding_headers = 0.0;

        for pgn_str in iter.take(NUM_TO_COLLECT) {
            let met = super::collect_metrics(
                &pgn_str,
                crate::compression::bincode_zlib::compress,
                crate::compression::bincode_zlib::decompress,
            );
            avg_time_to_compress += met.time_to_compress;
            avg_time_to_decompress += met.time_to_decompress;
            avg_compressed_size += met.compressed_size;
            avg_decompressed_size += met.decompressed_size;
            avg_bits_per_move += met.bits_per_move;
            avg_bits_per_move_excluding_headers += met.bits_per_move_excluding_headers;
        }

        avg_time_to_compress /= NUM_TO_COLLECT as u128;
        avg_time_to_decompress /= NUM_TO_COLLECT as u128;
        avg_compressed_size /= NUM_TO_COLLECT;
        avg_decompressed_size /= NUM_TO_COLLECT;
        avg_bits_per_move /= NUM_TO_COLLECT as f64;
        avg_bits_per_move_excluding_headers /= NUM_TO_COLLECT as f64;

        println!("avg_time_to_compress: {:.3}", avg_time_to_compress);
        println!("avg_time_to_decompress: {:.3}", avg_time_to_decompress);
        println!("avg_compressed_size: {:.3}", avg_compressed_size);
        println!("avg_decompressed_size: {:.3}", avg_decompressed_size);
        println!("avg_bits_per_move: {:.3}", avg_bits_per_move);
        println!(
            "avg_bits_per_move_excluding_headers: {:.3}",
            avg_bits_per_move_excluding_headers
        );

        // compression ratio
        let compression_ratio = avg_compressed_size as f64 / avg_decompressed_size as f64; 
        println!("compression_ratio: {:.3}", compression_ratio);
    }
}
