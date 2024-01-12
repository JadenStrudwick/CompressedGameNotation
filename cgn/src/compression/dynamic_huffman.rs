use crate::compression_utils::huffman_codes::{convert_hashmap_to_weights, get_lichess_hashmap};
use crate::compression_utils::score_move::{generate_moves, get_move_index};
use crate::compression_utils::{
    compress_headers, decompress_headers, get_bitvec_slice, i8_to_bit_vec,
};
use crate::export_to_wasm;
use crate::pgn_data::{PgnData, SanPlusWrapper};
use anyhow::{anyhow, Result};
use bit_vec::BitVec;
use pgn_reader::SanPlus;
use shakmaty::{Chess, Move, Position};
use std::collections::HashMap;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

const GAUSSIAN_HEIGHT: f64 = 742325.3537353727;
const GAUSSIAN_DEV: f64 = 2.5635425103971308;

fn gaussian(height: f64, dev: f64, mean: f64, x: f64) -> f64 {
    let b = -((x - mean).powi(2) / (2.0 * dev.powi(2)));
    (height - 1.0) * b.exp() + 1.0
}

fn adjust_haspmap(
    hashmap: &HashMap<u8, u64>,
    gaussian: impl Fn(f64, f64) -> f64,
    mean: f64,
) -> HashMap<u8, u64> {
    hashmap
        .iter()
        .map(|(key, value)| {
            let scale_factor = gaussian(mean, *key as f64);
            (*key, (*value as f64 + scale_factor) as u64)
        })
        .collect()
}

struct GameEncoder {
    white_hashmap: HashMap<u8, u64>,
    black_hashmap: HashMap<u8, u64>,
    is_white: bool,
    height: f64,
    dev: f64,
    pub pos: Chess,
    pub bit_moves: BitVec,
}

impl GameEncoder {
    pub fn new(height: f64, dev: f64) -> GameEncoder {
        GameEncoder {
            white_hashmap: get_lichess_hashmap(),
            black_hashmap: get_lichess_hashmap(),
            is_white: true,
            height,
            dev,
            pos: Chess::default(),
            bit_moves: BitVec::new(),
        }
    }

    pub fn encode(&mut self, m: &Move) -> Result<()> {
        match get_move_index(&self.pos, m) {
            Some(i) => {
                if i > 255 {
                    return Err(anyhow!("Move index exceeds maximum value"));
                }
                let index: u8 = i.try_into()?;

                let gaussian = |mean: f64, x: f64| gaussian(self.height, self.dev, mean, x);
                if self.is_white {
                    let (book, _) = convert_hashmap_to_weights(&self.white_hashmap);
                    book.encode(&mut self.bit_moves, &(index))?;
                    self.pos.play_unchecked(m);
                    self.white_hashmap =
                        adjust_haspmap(&self.white_hashmap, gaussian, index as f64);
                } else {
                    let (book, _) = convert_hashmap_to_weights(&self.black_hashmap);
                    book.encode(&mut self.bit_moves, &(index))?;
                    self.pos.play_unchecked(m);
                    self.black_hashmap =
                        adjust_haspmap(&self.black_hashmap, gaussian, index as f64);
                }
                self.is_white = !self.is_white;
                Ok(())
            }
            None => Err(anyhow!("Move not found")),
        }
    }
}

fn compress_moves(pgn: &PgnData) -> Result<BitVec> {
    let mut encoder = GameEncoder::new(GAUSSIAN_HEIGHT, GAUSSIAN_DEV);
    for san_plus in pgn.moves.iter() {
        let m = san_plus.0.san.to_move(&encoder.pos)?;
        encoder.encode(&m)?
    }
    Ok(encoder.bit_moves)
}

fn compress_moves_custom(pgn: &PgnData, height: f64, dev: f64) -> Result<BitVec> {
    let mut encoder = GameEncoder::new(height, dev);
    for san_plus in pgn.moves.iter() {
        let m = san_plus.0.san.to_move(&encoder.pos)?;
        encoder.encode(&m)?
    }
    Ok(encoder.bit_moves)
}

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

pub fn compress_pgn_data_custom(pgn: &PgnData, height: f64, dev: f64) -> Result<BitVec> {
    let mut headers = compress_headers(pgn)?;
    let mut moves = compress_moves_custom(pgn, height, dev)?;

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

struct GameDecoder {
    white_hashmap: HashMap<u8, u64>,
    black_hashmap: HashMap<u8, u64>,
    is_white: bool,
    height: f64,
    dev: f64,
    pub pos: Chess,
    pub moves: Vec<SanPlusWrapper>,
}

impl GameDecoder {
    pub fn new(height: f64, dev: f64) -> GameDecoder {
        GameDecoder {
            white_hashmap: get_lichess_hashmap(),
            black_hashmap: get_lichess_hashmap(),
            is_white: true,
            height,
            dev,
            pos: Chess::default(),
            moves: Vec::new(),
        }
    }

    fn decode_all(&mut self, move_bits: &BitVec) -> Result<()> {
        let mut move_bits_copy = move_bits.clone();

        let gaussian = |mean: f64, x: f64| gaussian(self.height, self.dev, mean, x);

        // while we still have bits to decode
        loop {
            // start book and tree
            let (book, tree) = if self.is_white {
                convert_hashmap_to_weights(&self.white_hashmap)
            } else {
                convert_hashmap_to_weights(&self.black_hashmap)
            };

            // decode one move
            let i = tree
                .decoder(&move_bits_copy, 256)
                .next()
                .ok_or(anyhow!("Failed to decode move"))?;
            let moves = generate_moves(&self.pos);
            let index: usize = i.try_into()?;
            let m = moves.get(index).ok_or(anyhow!("Move not found"))?;
            let san_plus = SanPlus::from_move_and_play_unchecked(&mut self.pos, m);
            let san_plus_wrapper = SanPlusWrapper(san_plus);
            self.moves.push(san_plus_wrapper);

            if self.is_white {
                self.white_hashmap = adjust_haspmap(&self.white_hashmap, gaussian, index as f64);
            } else {
                self.black_hashmap = adjust_haspmap(&self.black_hashmap, gaussian, index as f64);
            }
            self.is_white = !self.is_white;

            // encode the move to learn the bitstring
            let mut bitstring = BitVec::new();
            book.encode(&mut bitstring, &i)?;
            if bitstring == move_bits_copy {
                break;
            } else {
                move_bits_copy =
                    get_bitvec_slice(&move_bits_copy, bitstring.len(), move_bits_copy.len())?;
            }
        }
        Ok(())
    }
}

fn decompress_moves(move_bits: &BitVec) -> Result<Vec<SanPlusWrapper>> {
    let mut decoder = GameDecoder::new(GAUSSIAN_HEIGHT, GAUSSIAN_DEV);
    decoder.decode_all(move_bits)?;
    Ok(decoder.moves)
}

fn decompress_moves_custom(
    move_bits: &BitVec,
    height: f64,
    dev: f64,
) -> Result<Vec<SanPlusWrapper>> {
    let mut decoder = GameDecoder::new(height, dev);
    decoder.decode_all(move_bits)?;
    Ok(decoder.moves)
}

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

pub fn decompress_pgn_data_custom(bit_vec: &BitVec, height: f64, dev: f64) -> Result<PgnData> {
    let (headers, header_bytes_len) = decompress_headers(bit_vec)?;
    if header_bytes_len == 0 {
        let move_bits = get_bitvec_slice(bit_vec, 1, bit_vec.len())?;
        Ok(PgnData {
            headers,
            moves: decompress_moves_custom(&move_bits, height, dev)?,
        })
    } else {
        let move_bits = get_bitvec_slice(bit_vec, header_bytes_len, bit_vec.len())?;
        Ok(PgnData {
            headers,
            moves: decompress_moves_custom(&move_bits, height, dev)?,
        })
    }
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
    fn test_compress_pgn_str_no_headers() {
        let mut pgn_data = PgnData::from_str(PGN_STR_EXAMPLE).unwrap();
        pgn_data.clear_headers();
        let compressed_data = compress_pgn_data(&pgn_data).unwrap();
        let decompressed_pgn_str = decompress_pgn_data(&compressed_data).unwrap();
        assert_eq!(pgn_data.to_string(), decompressed_pgn_str.to_string());
    }
}
