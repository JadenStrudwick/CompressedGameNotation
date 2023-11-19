// Put all compression modules here for export to root lib.rs.
pub mod bincode_zlib;
pub mod huffman;

/// Accepts a module that contains the following and exports them to WASM string versions.
/// * compress_pgn_data(&PgnData) -> Result<Vec<u8>>.
/// * decompress_pgn_data(&[u8]) -> Result<PgnData>.
#[macro_export]
macro_rules! export_to_wasm {
    ($module_name:ident) => {
        /// Compresses a PGN string into a vector of bytes.
        #[wasm_bindgen]
        pub fn compress_pgn_str(pgn_str: &str) -> Vec<u8> {
            // if pgn_data is invalid, return an empty vector
            let pgn_data = match PgnData::from_str(pgn_str) {
                Ok(pgn_data) => pgn_data,
                Err(_) => return Vec::new(),
            };

            // compress the data and return the result
            match $module_name::compress_pgn_data(&pgn_data) {
                Ok(compressed_data) => compressed_data,
                Err(_) => Vec::new(),
            }
        }
        /// Decompresses a vector of bytes into a PGN string.
        #[wasm_bindgen]
        pub fn decompress_pgn_str(compressed_data: &[u8]) -> String {
            match $module_name::decompress_pgn_data(compressed_data) {
                Ok(pgn_data) => pgn_data.to_string(),
                Err(_) => String::new(),
            }
        }
    };
}
