use shakmaty::{Chess, Position};

mod cgn;
mod pgn;
mod eval;

fn main() {
    // let pgn_str = include_str!("pgn.txt");

    // let mut visitor = pgn::PgnVisitor::new();
    // let pgn_data = pgn_reader::BufferedReader::new_cursor(&pgn_str)
    //     .read_game(&mut visitor)
    //     .unwrap()
    //     .unwrap();

    // let serde_data = cgn::apply_compression(cgn::serde_strategy::compress, &pgn_data);
    // println!("Compressed data size: {}", serde_data.len());
    // let serde_decompress_data =
    //     cgn::apply_decompression(cgn::serde_strategy::decompress, &serde_data);
    // assert_eq!(serde_decompress_data.to_string(), pgn_str);

    // let serde_compress_data =
    //     cgn::apply_compression(cgn::serde_compress_strategy::compress, &pgn_data);
    // println!("Compressed data size: {}", serde_compress_data.len());
    // let serde_compress_decompress_data = cgn::apply_decompression(
    //     cgn::serde_compress_strategy::decompress,
    //     &serde_compress_data,
    // );
    // assert_eq!(serde_compress_decompress_data.to_string(), pgn_str);

    // test SanPlusWrapper to BitMove
    // let mut board = Chess::default();
    // for i in 0..pgn_data.moves.len() {
    //     let san_plus = pgn_data.moves.get(i).unwrap();
    //     let bit_move = eval::san_plus_wrapper_to_bit_move(&board, san_plus);
    //     println!("SanPlusWrapper: {}", san_plus.0.san);
    //     println!("BitMove: {}", bit_move);
        
    //     let mov = &san_plus.0.san;
    //     let mov2 = mov.to_move(&board).unwrap();
    //     board = board.play(&mov2).unwrap();
    // }

    let mut board = tanton::board::Board::default();
    
    for i in 0..100 {
        let movs = eval::eval_legal_moves(&mut board);
        println!("Moves: {:?}", movs);

        let first_move = movs.get(0).unwrap();
        board.apply_move(first_move.0);

        println!("Moves played: {}", board.moves_played());

    }
    
}
