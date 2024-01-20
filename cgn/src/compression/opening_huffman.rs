//! This strategy extends the Huffman encoding strategy by adding a trie to check for common openings.
//! The opening moves are prefix matched to the trie, represented by a 12 bit vector. The rest of the
//! moves are encoded using Huffman encoding.

use super::utils::huffman_codes::{convert_hashmap_to_weights, get_lichess_hashmap};
use super::utils::score_move::{generate_moves, get_move_index};
use super::utils::openings::construct_trie_and_hashmap;
use super::utils::{compress_headers, decompress_headers, get_bitvec_slice, i8_to_bit_vec};

use crate::export_to_wasm;
use crate::pgn_data::{PgnData, SanPlusWrapper};

use anyhow::{anyhow, Result};
use bit_vec::BitVec;
use pgn_reader::{SanPlus, San};
use shakmaty::{Chess, Position};
use std::str::FromStr;
use std::str;
use wasm_bindgen::prelude::*;

/// Encode the moves of a PGN file using Huffman encoding and a trie for the opening moves
fn compress_moves(pgn: &PgnData) -> Result<BitVec> {
    let book = convert_hashmap_to_weights(&get_lichess_hashmap()).0;
    let mut pos = Chess::default();
    let mut bit_moves = BitVec::new();
    let mut opening_move_count = 0;

    // get the pgn_moves and opening trie
    let pgn_str = pgn.to_string();
    let pgn_moves = pgn_str.split("]\n\n").nth(1).unwrap();
    let trie = construct_trie_and_hashmap();

    // check for a prefix match with the opening trie
    let matches = trie.0.common_prefix_search(pgn_moves);
    let matches_strings = matches
        .iter()
        .map(|x| str::from_utf8(x).unwrap())
        .collect::<Vec<&str>>(); 

    // if there are no matches, then return true (1 bit) and then the rest of the compressed moves
    if matches.is_empty() {
        bit_moves.push(true);
    } else {
        // get the longest match
        let longest_match = matches_strings.into_iter().max_by(|x, y| x.len().cmp(&y.len())).unwrap();
        let mut longest_match_bits = trie.1.get(longest_match).unwrap().clone();

        // add false (1 bit) to the bit vector and then the compressed opening (12 bits)
        bit_moves.push(false);
        bit_moves.append(&mut longest_match_bits);

        // play the opening moves so that we can encode the rest of the moves after the opening
        for san_str in longest_match.split(" ") {
            let san = San::from_str(san_str).unwrap();
            let m = san.to_move(&pos)?;
            pos.play_unchecked(&m);
            opening_move_count += 1;
        }
    }

    // encode the rest of the moves after the opening
    for san_plus in pgn.moves.iter().skip(opening_move_count) {
        let m = san_plus.0.san.to_move(&pos)?;

        // match the move to the index
        match get_move_index(&pos, &m) {
            Some(i) => {
                let index: u8 = i.try_into()?;
                book.encode(&mut bit_moves, &(index))?;
                pos.play_unchecked(&m);
            },
            None => return Err(anyhow!(
                "GameEncoder::encode() - Move {} is invalid for position {}",
                m,
                pos.board().to_string()
            )),
        }
    }

    Ok(bit_moves)
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

/// Decode the moves of a PGN file using Huffman encoding and a trie for the opening moves
fn decompress_moves(move_bits: &BitVec) -> Result<Vec<SanPlusWrapper>> {
    let new_move_bits;
    let tree = convert_hashmap_to_weights(&get_lichess_hashmap()).1;
    let trie = construct_trie_and_hashmap();
    let mut pos = Chess::default();
    let mut moves = Vec::new();

    // if the first bit is 1, then we skip decoding the opening and just decode the moves like normal
    if move_bits[0] {
        new_move_bits = get_bitvec_slice(move_bits, 1, move_bits.len())?;
    } else {
        // otherwise decode the opening
        let opening_bits = get_bitvec_slice(move_bits, 1, 13)?;
        let opening_string = trie.1.iter().find(|(_, v)| **v == opening_bits).unwrap().0;

        // play the opening moves so that we can decode the rest of the moves after the opening
        for san_str in opening_string.split(" ") {
            let san = San::from_str(san_str).unwrap();
            let m = san.to_move(&pos)?;
            let san_plus = SanPlus::from_move_and_play_unchecked(&mut pos, &m);
            moves.push(SanPlusWrapper(san_plus));
        }

        new_move_bits = get_bitvec_slice(move_bits, 13, move_bits.len())?;
    }

    // decode the rest of the moves after the opening
    for i in tree.decoder(new_move_bits, 256) {
        let legal_moves = generate_moves(&pos);
        let index: usize = i.try_into()?;
        let m = legal_moves.get(index).ok_or(anyhow!(
            "GameDecoder::decode_all() - Failed to decode index {} into a move",
            index
        ))?;
        let san_plus = SanPlus::from_move_and_play_unchecked(&mut pos, m);
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
