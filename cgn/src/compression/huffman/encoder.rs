use super::huffman_codes::lichess_huffman_weights;
use super::score_move::get_move_index;
use crate::pgn_data::PgnData;
use anyhow::{anyhow, Result};
use bit_vec::BitVec;
use huffman_compress::Book;
use shakmaty::{Chess, Move, Position};
use bincode::serialize_into;
use flate2::{write::ZlibEncoder, Compression};

/// Converts an i8 to a bit vector of length 8
fn i8_to_bit_vec(i: i8) -> BitVec {
    let mut bit_vec = BitVec::new();
    for j in (0..8).rev() {
        bit_vec.push((i >> j) & 1 == 1);
    }
    bit_vec
}

/// Game encoder that encodes moves into a bit vector using Huffman encoding
struct GameEncoder {
    book: Book<u8>, // The Huffman book
    pub pos: Chess, // The current position 
    pub bit_moves: BitVec, // The encoded moves
}

impl GameEncoder {
    /// Creates a new GameEncoder with the huffman book and a default position
    pub fn new() -> GameEncoder {
        let (book, _) = lichess_huffman_weights();
        GameEncoder {
            book,
            pos: Chess::default(),
            bit_moves: BitVec::new(),
        }
    }

    /// Encodes a move into the bit vector
    pub fn encode(&mut self, m: &Move) -> Result<()> {
        match get_move_index(&self.pos, m) {
            Some(i) => {
                if i > 255 {
                    return Err(anyhow!("Move index exceeds maximum value"));
                }
                self.book.encode(&mut self.bit_moves, &(i as u8))?;
                self.pos.play_unchecked(m);
                Ok(())
            }
            None => Err(anyhow!("Move not found")),
        }
    }
}

/// Compress the headers of a PGN file using ZLib maximum compression
fn compress_headers(pgn: &PgnData) -> Result<BitVec> {
    // if the headers are empty, return an empty bit vector
    if pgn.headers.is_empty() {
        return Ok(BitVec::new());
    }

    // otherwise compress the headers
    let mut compressed_headers = Vec::new();
    let mut encoder = ZlibEncoder::new(&mut compressed_headers, Compression::best());
    serialize_into(&mut encoder, &pgn.headers)?;
    encoder.finish()?;
    Ok(BitVec::from_bytes(&compressed_headers))
}

/// Encode the moves of a PGN file using Huffman encoding
fn compress_moves(pgn: &PgnData) -> Result<BitVec> {
    let mut encoder = GameEncoder::new();
    for san_plus in pgn.moves.iter() {
        let m = san_plus.0.san.to_move(&encoder.pos)?;
        encoder.encode(&m)?
    }
    Ok(encoder.bit_moves)
}

/// Compress a PGN file
pub fn compress_pgn_data(pgn: &PgnData) -> Result<BitVec> {
    let headers = compress_headers(pgn)?;
    let moves = compress_moves(pgn)?;

    // if headers are empty, set bitvec to [1], otherwise set to signed i8 (1 byte)
    let mut encoded_pgn;
    if headers.is_empty() {
        encoded_pgn = BitVec::from_elem(1, true);
    } else {
        encoded_pgn = i8_to_bit_vec(i8::try_from(headers.to_bytes().len())?);
    }

    // add the headers and moves to the encoded pgn 
    encoded_pgn.append(&mut headers.clone());
    encoded_pgn.append(&mut moves.clone());
    Ok(encoded_pgn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    const PGN_STR_EXAMPLE: &str = r#"[Event "Titled Tuesday Blitz January 03 Early 2023"]
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
    /// Tests that we can convert a 0 i8 to a bit vector
    fn test_i8_to_bit_vec_0() {
        let x = 0 as i8;
        let mut expected = BitVec::new();
        for _ in 0..8 {
            expected.push(false);
        }
        assert_eq!(i8_to_bit_vec(x), expected);
    }

    #[test]
    /// Tests that we can convert a 1 i8 to a bit vector
    fn test_i8_to_bit_vec_1() {
        let x = 1 as i8;
        let mut expected = BitVec::new();
        for _ in 0..7 {
            expected.push(false);
        }
        expected.push(true);
        assert_eq!(i8_to_bit_vec(x), expected);
    }

    #[test]
    /// Tests that we can convert a 10 i8 to a bit vector
    fn test_i8_to_bit_vec_10() {
        let x = 10 as i8;
        let mut expected = BitVec::new();
        expected.push(false); // 0
        expected.push(false); // 0
        expected.push(false); // 0
        expected.push(false); // 0
        expected.push(true); // 1
        expected.push(false); // 0
        expected.push(true); // 1
        expected.push(false); // 0
        assert_eq!(i8_to_bit_vec(x), expected);
    }

    #[test]
    /// Tests that we can compress the headers of a game
    fn test_compress_headers() {
        let pgn = PgnData::from_str(PGN_STR_EXAMPLE).unwrap(); 
        let headers = compress_headers(&pgn).unwrap();
        assert_eq!(headers.len(), 960);
    }

    #[test]
    /// Tests that we can compress the moves of a game
    fn test_compress_moves() {
        let pgn = PgnData::from_str(PGN_STR_EXAMPLE).unwrap(); 
        let bit_moves = compress_moves(&pgn).unwrap();
        assert_eq!(bit_moves.len(), 463);
    }

    #[test]
    /// Tests that we can compress a game
    fn test_compress_pgn() {
        let pgn = PgnData::from_str(PGN_STR_EXAMPLE).unwrap(); 
        let compressed_pgn = compress_pgn_data(&pgn).unwrap();
        assert_eq!(compressed_pgn.len(), 1431);
    }

    #[test]
    /// Tests that if the headers are empty, the first bit is set to 1
    fn test_compress_pgn_empty_headers() {
        let mut pgn = PgnData::from_str(PGN_STR_EXAMPLE).unwrap();
        pgn.clear_headers();
        let compressed_pgn = compress_pgn_data(&pgn).unwrap();
        assert_eq!(compressed_pgn[0], true);
    }

    #[test]
    /// Tests that if the headers are not empty, the first bit is set to 0
    fn test_compress_pgn_non_empty_headers() {
        let pgn = PgnData::from_str(PGN_STR_EXAMPLE).unwrap();
        let compressed_pgn = compress_pgn_data(&pgn).unwrap();
        assert_eq!(compressed_pgn[0], false);
    }
}