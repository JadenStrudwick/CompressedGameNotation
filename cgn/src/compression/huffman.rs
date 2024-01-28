//! This strategy uses the huffman_compress crate to compress the moves of a PGN file
//! using Huffman encoding. The headers are compressed using the same method as the
//! bincode strategy.

use super::utils::huffman_codes::{convert_hashmap_to_weights, get_lichess_hashmap};
use super::utils::score_move::{generate_moves, get_move_index};
use super::utils::{compress_headers, decompress_headers, get_bitvec_slice, i8_to_bit_vec};

use crate::export_to_wasm;
use crate::pgn_data::{PgnData, SanPlusWrapper};

use anyhow::{anyhow, Result};
use bit_vec::BitVec;
use pgn_reader::SanPlus;
use shakmaty::{Chess, Position};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

/// Encode the moves of a PGN file using Huffman encoding
fn compress_moves(pgn: &PgnData) -> Result<BitVec> {
    let book = convert_hashmap_to_weights(&get_lichess_hashmap()).0;
    let mut pos = Chess::default();
    let mut move_bits = BitVec::new();

    // for each move, encode the move and play it on the position
    for san_plus in pgn.moves.iter() {
        let san_move = san_plus.0.san.to_move(&pos)?;

        // get the index of the move and encode it into the bit vector
        match get_move_index(&pos, &san_move) {
            Some(i) => {
                let index: u8 = i.try_into()?;
                book.encode(&mut move_bits, &(index))?;
                pos.play_unchecked(&san_move);
            }
            None => {
                return Err(anyhow!(
                    "compress_moves() - Invalid move {} for position {}",
                    san_move,
                    pos.board().to_string()
                ))
            }
        }
    }

    Ok(move_bits)
}

/// Compress a PGN file
pub fn compress_pgn_data(pgn: &PgnData) -> Result<BitVec> {
    let mut headers = compress_headers(pgn)?;
    let mut moves = compress_moves(pgn)?;

    // if headers are empty, set bitvec to [1], otherwise set to signed i8 (1 byte)
    let mut encoded_pgn;
    if headers.is_empty() {
        encoded_pgn = BitVec::from_elem(1, true);
    } else {
        encoded_pgn = i8_to_bit_vec(i8::try_from(headers.to_bytes().len())?);
    }

    // add the headers and moves to the encoded pgn
    encoded_pgn.append(&mut headers);
    encoded_pgn.append(&mut moves);
    Ok(encoded_pgn)
}

/// Decode the moves of a PGN file using Huffman encoding
fn decompress_moves(move_bits: &BitVec) -> Result<Vec<SanPlusWrapper>> {
    let tree = convert_hashmap_to_weights(&get_lichess_hashmap()).1;
    let mut pos = Chess::default();
    let mut moves = Vec::new();

    // decode the moves from the bit vector
    for i in tree.decoder(move_bits, 256) {
        let legal_moves = generate_moves(&pos);
        let index: usize = i.try_into()?;

        // get the move from the index
        // TODO replace with san_move
        let san_move = legal_moves.get(index).ok_or(anyhow!(
            "decompress_moves() - Failed to decode index {} into a move",
            index
        ))?;

        // play the move on the position and add it to the moves vector
        let san_plus = SanPlus::from_move_and_play_unchecked(&mut pos, san_move);
        moves.push(SanPlusWrapper(san_plus));
    }

    Ok(moves)
}

/// Decompress a PGN file
pub fn decompress_pgn_data(bit_vec: &BitVec) -> Result<PgnData> {
    let (headers, header_bytes_len) = decompress_headers(bit_vec)?;
    if header_bytes_len == 0 {
        let move_bits = get_bitvec_slice(bit_vec, 1, bit_vec.len())?;
        Ok(PgnData {
            headers,
            moves: decompress_moves(&move_bits)?,
        })
    } else {
        let move_bits = get_bitvec_slice(bit_vec, header_bytes_len, bit_vec.len())?;
        Ok(PgnData {
            headers,
            moves: decompress_moves(&move_bits)?,
        })
    }
}

export_to_wasm!("huffman", compress_pgn_data, decompress_pgn_data);

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
    /// Test if the compression is correct for PGN structs.
    fn test_compress_pgn_data() {
        let pgn_str = PGN_STR_EXAMPLE;
        let pgn_data = PgnData::from_str(pgn_str).unwrap();
        let compressed_data = compress_pgn_data(&pgn_data).unwrap();
        let decompressed_data = decompress_pgn_data(&compressed_data).unwrap();
        let decompressed_pgn_str = decompressed_data.to_string();
        println!("{}", decompressed_pgn_str);
        println!("{}", pgn_str);
        assert_eq!(pgn_str, decompressed_pgn_str);
    }

    #[test]
    fn test_compress_pgn_str() {
        let pgn_str = PGN_STR_EXAMPLE;
        let compressed_data = huffman_compress_pgn_str(pgn_str);
        let decompressed_pgn_str = huffman_decompress_pgn_str(&compressed_data);
        assert_eq!(pgn_str, decompressed_pgn_str);
    }

    #[test]
    /// Tests if the compression is correct for a PGN string with no headers.
    fn test_compress_pgn_str_no_headers() {
        let mut pgn_data = PgnData::from_str(PGN_STR_EXAMPLE).unwrap();
        pgn_data.clear_headers();
        let compressed_data = compress_pgn_data(&pgn_data).unwrap();
        let decompressed_pgn_str = decompress_pgn_data(&compressed_data).unwrap();
        assert_eq!(pgn_data.to_string(), decompressed_pgn_str.to_string());
    }

    #[test]
    /// Tests that if the headers are empty, the first bit is set to 1
    fn first_bit_zero_when_empty_headers() {
        let mut pgn = PgnData::from_str(PGN_STR_EXAMPLE).unwrap();
        pgn.clear_headers();
        let compressed_pgn = compress_pgn_data(&pgn).unwrap();
        assert_eq!(compressed_pgn[0], true);
    }

    #[test]
    /// Tests that if the headers are not empty, the first bit is set to 0
    fn first_bit_one_when_full_headers() {
        let pgn = PgnData::from_str(PGN_STR_EXAMPLE).unwrap();
        let compressed_pgn = compress_pgn_data(&pgn).unwrap();
        assert_eq!(compressed_pgn[0], false);
    }

    #[test]
    /// Test that an invalid string cannot be compressed
    fn invalid_pgn_str_compress() {
        let pgn_str = "foo bar";
        let compressed_data = huffman_compress_pgn_str(pgn_str);
        assert_eq!(compressed_data.len(), 0);
    }

    #[test]
    /// Test that an invalid string cannot be decompressed
    fn invalid_pgn_str_decompress() {
        let compressed_data = vec![0, 1, 2, 3];
        let decompressed_pgn_str = huffman_decompress_pgn_str(&compressed_data);
        assert_eq!(decompressed_pgn_str.len(), 0);
    }
}
