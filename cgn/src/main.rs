mod cgn;
mod pgn;

fn main() {
    let pgn_str = include_str!("pgn.txt");

    let mut visitor = pgn::PgnVisitor::new();
    let pgn_data = pgn_reader::BufferedReader::new_cursor(&pgn_str)
        .read_game(&mut visitor)
        .unwrap()
        .unwrap();

    let serde_data = cgn::compress(&cgn::SerdeStrat, &pgn_data);
    println!("Compressed data size: {}", serde_data.len());
    let serde_decompress_data = cgn::decompress(&cgn::SerdeStrat, &serde_data);
    assert_eq!(serde_decompress_data.to_string(), pgn_str);

    let serde_compress_data = cgn::compress(&cgn::SerdeCompressStrat, &pgn_data);
    println!("Compressed data size: {}", serde_compress_data.len());
    let serde_compress_decompress_data =
        cgn::decompress(&cgn::SerdeCompressStrat, &serde_compress_data);
    assert_eq!(serde_compress_decompress_data.to_string(), pgn_str);
}
