/// This strategy uses the bincode crate to serialize the data and
/// then compresses it using the flate2 crate's ZlibEncoder at the
/// best compression level.
pub mod bincode_zlib {
    use crate::pgn_data::PgnData;

    /// Compresses the PGN data using bincode and ZlibEncoder at the maximum compression level.
    pub fn compress(pgn_data: &PgnData) -> Vec<u8> {
        // create a buffer to store the compressed data and a ZlibEncoder
        let mut compressed_data = Vec::new();
        let mut encoder =
            flate2::write::ZlibEncoder::new(&mut compressed_data, flate2::Compression::best());

        // serialize the data into the encoder and finish the compression
        bincode::serialize_into(&mut encoder, pgn_data).expect("Failed to serialize PGN data");
        encoder.finish().expect("Failed to compress PGN data");
        compressed_data
    }

    /// Decompresses the PGN data using bincode and ZlibDecoder.
    pub fn decompress(compressed_data: &[u8]) -> PgnData {
        let mut decoder = flate2::read::ZlibDecoder::new(compressed_data);
        bincode::deserialize_from(&mut decoder).expect("Failed to deserialize PGN data")
    }

    #[cfg(test)]
    mod tests {
        #[test]
        /// Tests if the bincode Zlib compression is correct.
        fn is_bincode_zlib_compression_correct() {
            let pgn_str = crate::pgn_examples::PGN_STR_EXAMPLE;
            let pgn_data = super::PgnData::from_str(pgn_str);
            let compressed_data = super::compress(&pgn_data);
            let decompressed_data = super::decompress(&compressed_data);
            let decompressed_pgn_str = decompressed_data.to_string();
            assert_eq!(pgn_str, decompressed_pgn_str);
        }
    }
}
