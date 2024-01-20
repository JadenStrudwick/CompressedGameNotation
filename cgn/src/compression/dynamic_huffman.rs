//! This strategy extends the Huffman encoding strategy by using a dynamic Huffman tree.
//! The tree is updated after each move is encoded. The height and deviation of a Gaussian 
//! function used to update the weights of the Huffman tree.

use super::utils::huffman_codes::{convert_hashmap_to_weights, get_lichess_hashmap};
use super::utils::score_move::{generate_moves, get_move_index};
use super::utils::{compress_headers, decompress_headers, get_bitvec_slice, i8_to_bit_vec};

use crate::export_to_wasm;
use crate::pgn_data::{PgnData, SanPlusWrapper};

use anyhow::{anyhow, Result};
use bit_vec::BitVec;
use pgn_reader::SanPlus;
use shakmaty::{Chess, Position};
use std::collections::HashMap;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

// Constants for the Gaussian function
const GAUSSIAN_HEIGHT: f64 = 742325.3537353727;
const GAUSSIAN_DEV: f64 = 2.5635425103971308;

/// Gaussian function to adjust the weights of the Huffman tree
fn gaussian(height: f64, dev: f64, mean: f64, x: f64) -> f64 {
    let b = -((x - mean).powi(2) / (2.0 * dev.powi(2)));
    height * b.exp()
}

/// Update the weights of the Huffman tree using a Gaussian function
fn adjust_haspmap(
    hashmap: &mut HashMap<u8, u64>,
    gaussian: impl Fn(f64, f64) -> f64,
    mean: f64,
) {
    for (key, value) in hashmap.iter_mut() {
        let scale_factor = gaussian(mean, *key as f64);
        *value = (*value as f64 + scale_factor) as u64;
    }
}

/// Compress the moves of a PGN file with a custom height and dev for the Gaussian function
fn compress_moves_custom(pgn: &PgnData, height: f64, dev: f64) -> Result<BitVec> {
    let mut white_hashmap = get_lichess_hashmap();
    let mut black_hashmap = get_lichess_hashmap();
    let mut pos = Chess::default();
    let mut bit_moves = BitVec::new();
    let mut is_white = true;

    // for each move, encode the move and play it on the position
    for san_plus in pgn.moves.iter() {
        let san_move = san_plus.0.san.to_move(&pos)?;

        // get the index of the move and encode it into the bit vector
        match get_move_index(&pos, &san_move) {
            Some(i) => {
                let index: u8 = i.try_into()?;

                // gaussian function to adjust the weights of the Huffman tree
                let gaussian = |mean: f64, x: f64| gaussian(height, dev, mean, x);

                // get the hashmap for the current player 
                let hash_map= if is_white {
                    &mut white_hashmap
                } else {
                    &mut black_hashmap
                };

                // adjust encode the move and adjust the weights of the Huffman tree
                let book = convert_hashmap_to_weights(hash_map).0;
                book.encode(&mut bit_moves, &(index))?;
                pos.play_unchecked(&san_move);
                adjust_haspmap(hash_map, gaussian, index as f64);

                // flip the player
                is_white = !is_white;
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

    Ok(bit_moves)
}

/// Compress a PGN file with a custom height and dev for the Gaussian function
pub fn compress_pgn_data_custom(pgn: &PgnData, height: f64, dev: f64) -> Result<BitVec> {
    let mut headers = compress_headers(pgn)?;
    let mut moves = compress_moves_custom(pgn, height, dev)?;

    // if headers are empty, set bitvec to [1], otherwise set to signed i8 (1 byte) representing the length of the headers in bytes
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

/// Compress a PGN file
pub fn compress_pgn_data(pgn: &PgnData) -> Result<BitVec> {
    compress_pgn_data_custom(pgn, GAUSSIAN_HEIGHT, GAUSSIAN_DEV)
}

fn decompress_moves_custom(
    mut move_bits: BitVec,
    height: f64,
    dev: f64,
) -> Result<Vec<SanPlusWrapper>> {
    let mut white_hashmap = get_lichess_hashmap();
    let mut black_hashmap = get_lichess_hashmap();
    let mut pos = Chess::default();
    let mut moves = Vec::new();
    let mut is_white = true;

    // gaussian function to adjust the weights of the Huffman tree
    let gaussian = |mean: f64, x: f64| gaussian(height, dev, mean, x);

    // while we still have bits to decode
    loop {
        // get the hashmap for the current player 
        let current_hasp_map = if is_white {
            &mut white_hashmap
        } else {
            &mut black_hashmap
        };

        // get the book and tree for the current player
        let (book, tree) = convert_hashmap_to_weights(&current_hasp_map);

        // decode one index
        let i = tree.decoder(move_bits.clone(), 256).next().ok_or(anyhow!(
            "decompress_moves_custom() - Failed to decode next move from tree"
        ))?;

        // get the legal moves for the current position and decode the index into a move
        let legal_moves = generate_moves(&pos);
        let index: usize = i.try_into()?;
        let m = legal_moves.get(index).ok_or(anyhow!(
            "decompress_moves_custom() - Failed to decode index {} into a move",
            index
        ))?;

        // play the move on the position and add it to the moves vector
        let san_plus = SanPlus::from_move_and_play_unchecked(&mut pos, m);
        moves.push(SanPlusWrapper(san_plus));

        // adjust the weights of the Huffman coding 
        adjust_haspmap(current_hasp_map, gaussian, index as f64);

        // flip the player
        is_white = !is_white;

        // encode the move to learn the bitstring
        let mut bitstring = BitVec::new();
        book.encode(&mut bitstring, &i)?;

        // if the bistring and remaining bits are equal, OR the game is over, we are done decoding
        if bitstring == move_bits || pos.is_checkmate() || pos.is_stalemate() {
            break;
        }

        // otherwise, remove the bitstring from the remaining bits
        move_bits = get_bitvec_slice(&move_bits, bitstring.len(), move_bits.len())?;
    }

    Ok(moves)
}

/// Compress a PGN string with custom height and dev
pub fn decompress_pgn_data_custom(bit_vec: &BitVec, height: f64, dev: f64) -> Result<PgnData> {
    let (headers, header_bytes_len) = decompress_headers(bit_vec)?;
    if header_bytes_len == 0 {
        let move_bits = get_bitvec_slice(bit_vec, 1, bit_vec.len())?;
        Ok(PgnData {
            headers,
            moves: decompress_moves_custom(move_bits, height, dev)?,
        })
    } else {
        let move_bits = get_bitvec_slice(bit_vec, header_bytes_len, bit_vec.len())?;
        Ok(PgnData {
            headers,
            moves: decompress_moves_custom(move_bits, height, dev)?,
        })
    }
}

/// Decompress a PGN file
pub fn decompress_pgn_data(bit_vec: &BitVec) -> Result<PgnData> {
    decompress_pgn_data_custom(bit_vec, GAUSSIAN_HEIGHT, GAUSSIAN_DEV)
}

export_to_wasm!("dynamic_huffman", compress_pgn_data, decompress_pgn_data);

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
        assert_eq!(pgn_str, decompressed_pgn_str);
    }

    #[test]
    /// Tests if the compression is correct for a PGN string with no headers.
    fn test_compress_pgn_data_no_headers() {
        let mut pgn_data = PgnData::from_str(PGN_STR_EXAMPLE).unwrap();
        pgn_data.clear_headers();
        let compressed_data = compress_pgn_data(&pgn_data).unwrap();
        let decompressed_pgn_str = decompress_pgn_data(&compressed_data).unwrap();
        assert_eq!(pgn_data.to_string(), decompressed_pgn_str.to_string());
    }

    #[test]
    fn test_compress_pgn_str() {
        let pgn_str = PGN_STR_EXAMPLE;
        let compressed_data = dynamic_huffman_compress_pgn_str(pgn_str);
        let decompressed_pgn_str = dynamic_huffman_decompress_pgn_str(&compressed_data);
        assert_eq!(pgn_str, decompressed_pgn_str);
    }

    #[test]
    fn test_compress_pgn_str_no_headers() {
        let mut pgn_data = PgnData::from_str(PGN_STR_EXAMPLE).unwrap();
        pgn_data.clear_headers();
        let compressed_data = dynamic_huffman_compress_pgn_str(&pgn_data.to_string());
        let decompressed_pgn_str = dynamic_huffman_decompress_pgn_str(&compressed_data);
        assert_eq!(pgn_data.to_string(), decompressed_pgn_str);
    }

    #[test]
    /// Tests if the compression is correct for PGN structs with custom height and dev.
    fn test_compress_pgn_data_custom() {
        let pgn_str = PGN_STR_EXAMPLE;
        let pgn_data = PgnData::from_str(pgn_str).unwrap();
        let compressed_data = compress_pgn_data_custom(&pgn_data, 1000000.0, 1000000.0).unwrap();
        let decompressed_data =
            decompress_pgn_data_custom(&compressed_data, 1000000.0, 1000000.0).unwrap();
        let decompressed_pgn_str = decompressed_data.to_string();
        assert_eq!(pgn_str, decompressed_pgn_str);
    }

    #[test]
    /// Tests if the compression is correct for a PGN string with no headers and custom height and dev.
    fn test_compress_pgn_data_no_headers_custom() {
        let mut pgn_data = PgnData::from_str(PGN_STR_EXAMPLE).unwrap();
        pgn_data.clear_headers();
        let compressed_data = compress_pgn_data_custom(&pgn_data, 1000000.0, 1000000.0).unwrap();
        let decompressed_pgn_str =
            decompress_pgn_data_custom(&compressed_data, 1000000.0, 1000000.0).unwrap();
        assert_eq!(pgn_data.to_string(), decompressed_pgn_str.to_string());
    }

    #[test]
    /// Test that an invalid string cannot be compressed
    fn invalid_pgn_str_compress() {
        let pgn_str = "foo bar";
        let compressed_data = dynamic_huffman_compress_pgn_str(pgn_str);
        assert_eq!(compressed_data.len(), 0);
    }

    #[test]
    /// Test that an invalid string cannot be decompressed
    fn invalid_pgn_str_decompress() {
        let compressed_data = vec![0, 1, 2, 3];
        let decompressed_pgn_str = dynamic_huffman_decompress_pgn_str(&compressed_data);
        assert_eq!(decompressed_pgn_str.len(), 0);
    }
}
