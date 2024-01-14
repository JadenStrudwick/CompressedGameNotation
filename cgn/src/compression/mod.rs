// Put all compression modules here for export to root lib.rs.
pub mod bincode;
pub mod dynamic_huffman;
pub mod huffman;
mod utils;

/// Accepts a module that contains the following and exports them to WASM string versions.
/// * compress_pgn_data(&PgnData) -> Result<BitVec>.
/// * decompress_pgn_data(&BitVec) -> Result<PgnData>.
#[macro_export]
macro_rules! export_to_wasm {
    ($module_name:literal, $compress_pgn_data:ident, $decompress_pgn_data:ident) => {
        ::paste::paste! {
            /// Compresses a PGN string into a vector of bytes.
            #[wasm_bindgen]
            pub fn [<$module_name _compress_pgn_str>](pgn_str: &str) -> Vec<u8> {
                // if pgn_data is invalid, return an empty vector
                let pgn_data = match PgnData::from_str(pgn_str) {
                    Ok(pgn_data) => pgn_data,
                    Err(_) => return Vec::new(),
                };

                // compress the data and return the result
                match $compress_pgn_data(&pgn_data) {
                    Ok(compressed_data) => compressed_data.to_bytes(),
                    Err(_) => Vec::new(),
                }
            }
            /// Decompresses a vector of bytes into a PGN string.
            #[wasm_bindgen]
            pub fn [<$module_name _decompress_pgn_str>](compressed_data: &[u8]) -> String {
                match $decompress_pgn_data(&BitVec::from_bytes(compressed_data)) {
                    Ok(pgn_data) => pgn_data.to_string(),
                    Err(_) => String::new(),
                }
            }
        }
    };
}
