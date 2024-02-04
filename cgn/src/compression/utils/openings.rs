use std::collections::HashMap;

use anyhow::{anyhow, Result};
use bit_vec::BitVec;
use trie_rs::{Trie, TrieBuilder};

/// Converts a usize to a bit vector of specified length. Used for compressing the opening moves
fn usize_to_bitvec(i: usize, bitvec_len: usize) -> Result<BitVec> {
    // check that the usize is within the range of n bits
    if i > (1 << bitvec_len) - 1 {
        return Err(anyhow!(
            "usize_to_n_vec() - usize is too large to fit into {} bits, usize: {}",
            bitvec_len,
            i
        ));
    }
    let mut bit_vec = BitVec::new();
    for j in (0..bitvec_len).rev() {
        bit_vec.push((i >> j) & 1 == 1);
    }

    Ok(bit_vec)
}

/// Extracts the PGN openings from a txt file and returns them as a vector of strings
/// The txt file should contain one opening per line, with the moves separated by spaces
/// The minimum number of moves for an opening to be included can be specified
fn extract_openings(txt_contents: &str, min_opening_moves: usize) -> Vec<String> {
    let mut openings = Vec::new();
    for line in txt_contents.lines() {
        if line.split_ascii_whitespace().count() >= min_opening_moves {
            openings.push(line.to_string());
        }
    }
    openings
}

/// Constructs the trie and hashmap for the openings and their compressed versions
pub fn construct_trie_and_hashmap(
    min_opening_moves: usize,
    bitvec_len: usize,
) -> (Trie<u8>, HashMap<String, BitVec>) {
    let openings = extract_openings(include_str!("sorted_opening_moves.txt"), min_opening_moves);

    // construct the trie (for prefix matching the openings) and hashmap (for mapping the opening to a compressed version)
    let mut trie_builder = TrieBuilder::new();
    let mut hashmap = HashMap::new();

    // iterate through the openings and add them to the trie and hashmap
    openings.into_iter().enumerate().for_each(|(i, opening)| {
        // if the usize is too large to fit into 12 bits, skip it
        if let Ok(bitvec) = usize_to_bitvec(i, bitvec_len) {
            trie_builder.push(&opening);
            hashmap.insert(opening, bitvec);
        }
    });

    (trie_builder.build(), hashmap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usize_to_i12_vec() {
        let x = 1;
        let mut expected = BitVec::new();
        expected.push(false); // 0
        expected.push(false); // 0
        expected.push(false); // 0
        expected.push(false); // 0
        expected.push(false); // 0
        expected.push(false); // 0
        expected.push(false); // 0
        expected.push(false); // 0
        expected.push(false); // 0
        expected.push(false); // 0
        expected.push(false); // 0
        expected.push(true); // 1
        assert_eq!(usize_to_bitvec(x, 12).unwrap(), expected);
    }

    #[test]
    fn test_usize_to_i13_vec_2() {
        let x = 1;
        assert_eq!(usize_to_bitvec(x, 12).unwrap().len(), 12);
    }
}
