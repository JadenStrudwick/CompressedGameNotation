use crate::{export_to_wasm, pgn_data::PgnData};
use anyhow::Result;
use bit_vec::BitVec;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

/// This strategy uses the bincode crate to serialize the data and
/// then compresses it using the flate2 crate's ZlibEncoder at the
/// best compression level.

/// Compresses the PGN data using bincode and ZlibEncoder at the maximum compression level.
pub fn compress_pgn_data(pgn_data: &PgnData) -> Result<BitVec> {
    // create a buffer to store the compressed data and a ZlibEncoder
    let mut compressed_data = Vec::new();
    let mut encoder =
        flate2::write::ZlibEncoder::new(&mut compressed_data, flate2::Compression::best());

    // serialize the data into the encoder and finish the compression
    bincode::serialize_into(&mut encoder, pgn_data)?;
    encoder.finish()?;
    Ok(BitVec::from_bytes(&compressed_data))
}

/// Decompresses the PGN data using bincode and ZlibDecoder.
pub fn decompress_pgn_data(compressed_data: &BitVec) -> Result<PgnData> {
    let compressed_data_bytes = compressed_data.to_bytes();
    let mut decoder = flate2::read::ZlibDecoder::new(compressed_data_bytes.as_slice());
    Ok(bincode::deserialize_from(&mut decoder)?)
}

// Wrap the functions in a macro for export to WASM.
export_to_wasm!(self);

#[cfg(test)]
mod tests {
    use super::*;

    /// Example PGN string.
    pub const PGN_STR_EXAMPLE: &str = r#"[Event "Titled Tuesday Blitz January 03 Early 2023"]
[Site ""]
[Date "2023.01.03"]
[Round "?"]
[White "Magnus Carlsen"]
[Black "Samvel Ter-Sahakyan"]
[Result "1-0"]

1. a4 Nf6 2. d4 d5 3. Nf3 Bf5 4. Nh4 Be4 5. f3 Bg6 6. Nc3 c5 7. e4 cxd4 8. Nxg6
hxg6 9. Qxd4 Nc6 10. Qf2 d4 11. Nd1 e5 12. Bc4 Rc8 13. Qe2 Bb4+ 14. Kf1 Na5 15.
Bd3 O-O 16. Nf2 Qb6 17. h4 Nh5 18. Rh3 Qf6 19. g4 Nf4 20. Bxf4 Qxf4 21. h5 g5
22. Rd1 a6 23. Kg2 Rc7 24. Rhh1 Rfc8 25. Nh3 Qf6 26. Ra1 Nc6 27. Rhc1 Bd6 28.
Qd2 Bb4 29. c3 Be7 30. Nf2 dxc3 31. bxc3 Nd8 32. Bb1 Ne6 33. Nh3 Bc5 34. Ba2 Rd8
35. Qe2 Nf4+ 36. Nxf4 gxf4 37. Kh3 g6 38. Rd1 Rcd7 39. Rxd7 Rxd7 40. Rd1 Bf2 41.
Bxf7+ Kf8 42. Qxf2 Rxd1 43. Bxg6 Qd6 44. g5 Qd3 45. Qc5+ Qd6 46. Qc8+ Kg7 47.
Qxb7+ Kf8 48. Qf7# 1-0"#;

    #[test]
    /// Test if the bincode Zlib compression is correct for PGN structs.
    fn bincode_zlib_pgn_data() {
        let pgn_str = PGN_STR_EXAMPLE;
        let pgn_data = PgnData::from_str(pgn_str).unwrap();
        let compressed_data = compress_pgn_data(&pgn_data).unwrap();
        let decompressed_data = decompress_pgn_data(&compressed_data).unwrap();
        let decompressed_pgn_str = decompressed_data.to_string();
        assert_eq!(pgn_str, decompressed_pgn_str);
    }

    #[test]
    /// Test if the bincode Zlib compression is correct for PGN strings.
    fn bincode_zlib_pgn_str() {
        let pgn_str = PGN_STR_EXAMPLE;
        let compressed_data = compress_pgn_str(pgn_str);
        let decompressed_pgn_str = decompress_pgn_str(&compressed_data);
        assert_eq!(pgn_str, decompressed_pgn_str);
    }
}
