
use super::huffman_codes::lichess_huffman_weights;
use super::score_move::generate_moves;
use crate::pgn_data::{PgnData, PgnHeaders, SanPlusWrapper};
use anyhow::{anyhow, Result};
use bit_vec::BitVec;
use flate2::read::ZlibDecoder;
use shakmaty::{san::SanPlus, Chess};

/// Gets the bit vector slice from start (inclusive) to end (exclusive)
fn get_bitvec_slice(bit_vec: &BitVec, start: usize, end: usize) -> Result<BitVec> {
    let len = bit_vec.len();
    
    // check for invalid indices
    if (start > end) || (start >= len) || (end > len) {
        return Err(anyhow!("Invalid indices"));
    }

    // push the bits into the result
    let mut result = BitVec::with_capacity(end - start);
    for i in start..end {
        result.push(bit_vec[i]);
    }

    Ok(result)
}

pub fn decompress_pgn_data(bit_vec: &BitVec) -> Result<PgnData> {
    // if the first bit is 1, then there are no headers, so just read the moves
    if bit_vec[0] {
        let move_bits = get_bitvec_slice(&bit_vec, 1, bit_vec.len())?;
        return Ok(PgnData {
            headers: PgnHeaders::new(),
            moves: decompress_moves(&move_bits)?,
        });
    }
    // if the first bit is 0, then there are headers, so read them
    else {
        // get the header length in bytes from the first byte of the data
        let header_bytes = bit_vec
            .iter()
            .take(8)
            .enumerate()
            .fold(
                0,
                |byte, (i, bit)| {
                    if bit {
                        byte | 1 << (7 - i)
                    } else {
                        byte
                    }
                },
            );

        // read the headers
        let headers_bytes = get_bitvec_slice(&bit_vec, 8, (header_bytes + 1) * 8)?.to_bytes();
        let headers_slice = headers_bytes.as_slice();

        // decompress the headers
        let mut decoder = ZlibDecoder::new(headers_slice);
        let headers: PgnHeaders = bincode::deserialize_from(&mut decoder)?;

        let move_bits = get_bitvec_slice(&bit_vec, (header_bytes + 1) * 8, bit_vec.len())?;
        Ok(PgnData {
            headers,
            moves: decompress_moves(&move_bits)?,
        })
    }
}

fn decompress_moves(move_bits: &BitVec) -> Result<Vec<SanPlusWrapper>> {
    let mut pos = Chess::default();
    let mut san_plus_moves = Vec::new();
    for i in lichess_huffman_weights().1.decoder(move_bits, 256) {
        let moves = generate_moves(&pos);
        let m = moves
            .get(i as usize)
            .ok_or(anyhow!("Failed to decode move"))?;
        let san_plus = SanPlus::from_move_and_play_unchecked(&mut pos, &m);
        let san_plus_wrapper = SanPlusWrapper(san_plus);
        san_plus_moves.push(san_plus_wrapper);
    }
    Ok(san_plus_moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests that we can slice a bit vector
    fn test_get_bitvec_slice() {
        let mut bit_vec = BitVec::new();
        bit_vec.push(true);
        bit_vec.push(false);
        bit_vec.push(true);
        bit_vec.push(false);
        assert_eq!(get_bitvec_slice(&bit_vec, 0, 4).unwrap(), bit_vec);
    }

    #[test]
    /// Tests we can take a subslice of a bit vector
    fn test_get_bitvec_slice_subslice() {
        let mut bit_vec = BitVec::new();
        bit_vec.push(true);
        bit_vec.push(false);
        bit_vec.push(true);
        bit_vec.push(false);
        let mut expected = BitVec::new();
        expected.push(false);
        expected.push(true);
        assert_eq!(get_bitvec_slice(&bit_vec, 1, 3).unwrap(), expected);
    }
}