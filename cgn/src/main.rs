use cgn::pgn::string_to_pgn;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::prelude::*;

fn main() {
    let str = include_str!("pgn.txt");
    let pgn = string_to_pgn(str);

    let pgn_bytes = bincode::serialize(&pgn).unwrap();

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(&pgn_bytes).unwrap();

    let pgn_compressed = encoder.finish().unwrap();

    println!("Original size: {}", str.as_bytes().len());
    println!("Serialized size: {}", pgn_bytes.len());
    println!("Compressed size: {}", pgn_compressed.len());
}
