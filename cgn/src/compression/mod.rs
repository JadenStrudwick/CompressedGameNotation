//! Compression algorithms for PGN data.
//! 
//! Order of compression algorithms from most efficient to least efficient 
//! 1. Opening Huffman coding
//! 2. Dynamic Huffman coding
//! 3. Huffman coding
//! 4. Bincode

pub mod bincode;
pub mod dynamic_huffman;
pub mod huffman;
pub mod opening_huffman;
mod utils;
