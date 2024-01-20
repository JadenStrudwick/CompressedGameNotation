use std::collections::HashMap;

use anyhow::{Result, anyhow};
use bit_vec::BitVec;
use trie_rs::{TrieBuilder, Trie};

/// Minimum length of an opening to be considered
const MIN_OPENING_LENGTH: usize = 4;

/// Converts a usize to a bit vector of length 12. Used for compressing the opening moves
fn usize_to_u12_vec(i: usize) -> Result<BitVec> {
  // check that the usize is within the range of 12 bits
  if i > 4095 {
    return Err(anyhow!(
      "usize_to_u12_vec() - usize is too large to fit into 12 bits, usize: {}",
      i
    ));
  } 

  let mut bit_vec = BitVec::new();
  for j in (0..12).rev() {
    bit_vec.push((i >> j) & 1 == 1);
  }

  Ok(bit_vec)
}

/// Extracts the PGN openings from a tsv file
fn extract_openings(tsv_contents: &str) -> Vec<String> {
  let mut openings = Vec::new();
  for line in tsv_contents.lines() {
    let mut line = line.split("\t");

    // skip the first two columns
    match line.nth(2) {
      Some(pgn_str) => {
        // only add the opening if it is long enough
        if pgn_str.split_ascii_whitespace().count() >= MIN_OPENING_LENGTH {
          openings.push(pgn_str.to_string());
        }
      },
      None => continue,
    }
  }
  openings
}

/// Constructs the trie and hashmap for the openings and their compressed versions
pub fn construct_trie_and_hashmap() -> (Trie<u8>, HashMap<String, BitVec>)  {
  // extract openings from tsv files
  let a_tsv = extract_openings(include_str!("./a.tsv"));
  let b_tsv = extract_openings(include_str!("./b.tsv"));
  let c_tsv = extract_openings(include_str!("./c.tsv"));
  let d_tsv = extract_openings(include_str!("./d.tsv"));
  let e_tsv = extract_openings(include_str!("./e.tsv"));

  // concat all openings into one vector
  let mut openings = Vec::new();
  openings.extend(a_tsv);
  openings.extend(b_tsv);
  openings.extend(c_tsv);
  openings.extend(d_tsv);
  openings.extend(e_tsv);

  // construct the trie (for prefix matching the openings) and hashmap (for mapping the opening to a compressed version)
  let mut trie_builder = TrieBuilder::new();
  let mut hashmap = HashMap::new();

  // iterate through the openings and add them to the trie and hashmap
  openings.into_iter().enumerate().for_each(|(i, opening)| {
    // if the usize is too large to fit into 12 bits, skip it
    match usize_to_u12_vec(i) {
      Ok(bitvec) => {
        trie_builder.push(&opening);
        hashmap.insert(opening, bitvec);
      }
      Err(_) => return,
    }
  });

  (trie_builder.build(), hashmap)
} 

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_extract_openings() {
    let a_tsv = include_str!("./a.tsv");
    let openings = extract_openings(a_tsv);
    println!("{:?}", openings);
  }

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
    assert_eq!(usize_to_u12_vec(x).unwrap(), expected);
  }

  #[test]
  fn test_usize_to_i13_vec_2() {
    let x = 1;
    assert_eq!(usize_to_u12_vec(x).unwrap().len(), 12);
  }

}