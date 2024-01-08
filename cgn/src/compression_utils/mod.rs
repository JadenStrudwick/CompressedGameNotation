pub mod huffman_codes;
pub mod score_move;
use crate::pgn_data::{PgnData, PgnHeaders};
use anyhow::{anyhow, Result};
use bincode::serialize_into;
use bit_vec::BitVec;
use flate2::{write::ZlibEncoder, Compression, read::ZlibDecoder};

/// Converts an i8 to a bit vector of length 8
pub fn i8_to_bit_vec(i: i8) -> BitVec {
    let mut bit_vec = BitVec::new();
    for j in (0..8).rev() {
        bit_vec.push((i >> j) & 1 == 1);
    }
    bit_vec
}

/// Gets the bit vector slice from start (inclusive) to end (exclusive)
pub fn get_bitvec_slice(bit_vec: &BitVec, start: usize, end: usize) -> Result<BitVec> {
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

/// Compress the headers of a PGN file using ZLib maximum compression
pub fn compress_headers(pgn: &PgnData) -> Result<BitVec> {
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

/// Decompress the headers of a PGN file using ZLib maximum compression
pub fn decompress_headers(bit_vec: &BitVec) -> Result<(PgnHeaders, usize)> {
    // if the first bit is 1, then there are no headers 
    if bit_vec[0] {
        return Ok((PgnHeaders::new(), 0));
    }

    // get the header length in bytes from the first byte of the data
    let header_bytes_len = bit_vec
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
    let headers_bytes = get_bitvec_slice(bit_vec, 8, (header_bytes_len + 1) * 8)?.to_bytes();
    let headers_slice = headers_bytes.as_slice();

    // decompress the headers
    let mut decoder = ZlibDecoder::new(headers_slice);
    let headers: PgnHeaders = bincode::deserialize_from(&mut decoder)?;
    Ok((headers, (header_bytes_len + 1) * 8))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests that we can convert a 0 i8 to a bit vector
    fn test_i8_to_bit_vec_0() {
        let x = 0;
        let mut expected = BitVec::new();
        for _ in 0..8 {
            expected.push(false);
        }
        assert_eq!(i8_to_bit_vec(x), expected);
    }

    #[test]
    /// Tests that we can convert a 1 i8 to a bit vector
    fn test_i8_to_bit_vec_1() {
        let x = 1;
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
        let x = 10;
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