//! Compression algorithms for PGN data.
//! 
//! Order of compression algorithms from most efficient to least efficient (and slowest to fastest)
//! 1. Dynamic Huffman coding
//! 2. Huffman coding
//! 3. Bincode

pub mod bincode;
pub mod dynamic_huffman;
pub mod huffman;
mod utils;
