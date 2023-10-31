mod compression;
mod pgn_data;
mod pgn_examples;
mod pgn_vistor;
mod san_plus_wrapper;
mod benchmark;
mod pgn_db_iter;

use wasm_bindgen::prelude::*;

use pgn_data::PgnData;

/// Compresses the PGN data using bincode and ZlibEncoder at the maximum compression level.
/// # Arguments
/// * `pgn` - The PGN data.
/// # Returns
/// The compressed PGN data.
#[wasm_bindgen]
pub fn bincode_zlib_compress(pgn: &str) -> Vec<u8> {
    let pgn = PgnData::from_str(pgn);
    compression::bincode_zlib::compress(&pgn)
}

/// Decompresses the PGN data using bincode and ZlibDecoder.
/// # Arguments
/// * `compressed_pgn` - The compressed PGN data.
/// # Returns
/// The decompressed PGN data.
#[wasm_bindgen]
pub fn bincode_zlib_decompress(compressed_pgn: &[u8]) -> String {
    let pgn = compression::bincode_zlib::decompress(compressed_pgn);
    pgn.to_string()
}
