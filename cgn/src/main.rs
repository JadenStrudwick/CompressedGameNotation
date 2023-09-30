mod cgn;
mod pgn;

fn main() {
    let pgn_str = include_str!("pgn.txt");

    let mut visitor = pgn::PgnVisitor::new();
    let pgn_data = pgn_reader::BufferedReader::new_cursor(&pgn_str)
        .read_game(&mut visitor)
        .unwrap()
        .unwrap();

    let serde_data = cgn::apply_compression(cgn::serde_strategy::compress, &pgn_data);
    println!("Compressed data size: {}", serde_data.len());
    let serde_decompress_data = cgn::apply_decompression(cgn::serde_strategy::decompress, &serde_data);
    assert_eq!(serde_decompress_data.to_string(), pgn_str);

    let serde_compress_data = cgn::apply_compression(cgn::serde_compress_strategy::compress, &pgn_data);
    println!("Compressed data size: {}", serde_compress_data.len());
    let serde_compress_decompress_data = cgn::apply_decompression(
        cgn::serde_compress_strategy::decompress,
        &serde_compress_data,
    );
    assert_eq!(serde_compress_decompress_data.to_string(), pgn_str);
}
