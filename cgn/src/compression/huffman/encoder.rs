use crate::compression_utils::{compress_headers, i8_to_bit_vec};
use crate::compression_utils::huffman_codes::{convert_hashmap_to_weights, get_lichess_hashmap};
use crate::compression_utils::score_move::get_move_index;
use crate::pgn_data::PgnData;
use anyhow::{anyhow, Result};
use bit_vec::BitVec;
use huffman_compress::Book;
use shakmaty::{Chess, Move, Position};

/// Game encoder that encodes moves into a bit vector using Huffman encoding
struct GameEncoder {
    book: Book<u8>,        
    pub pos: Chess,        
    pub bit_moves: BitVec, 
}

impl GameEncoder {
    /// Creates a new GameEncoder with the huffman book and a default position
    pub fn new() -> GameEncoder {
        GameEncoder {
            book: convert_hashmap_to_weights(&get_lichess_hashmap()).0,
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
                let index: u8 = i.try_into()?;
                self.book.encode(&mut self.bit_moves, &(index))?;
                self.pos.play_unchecked(m);
                Ok(())
            }
            None => Err(anyhow!("Move not found")),
        }
    }
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
}
