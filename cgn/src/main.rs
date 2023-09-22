mod pgn;

fn main() {
    let pgn_str = include_str!("pgn.txt");

    let mut visitor = pgn::PgnVisitor::new();
    let pgn_data = pgn_reader::BufferedReader::new_cursor(&pgn_str)
        .read_game(&mut visitor)
        .unwrap()
        .unwrap();

    if pgn_str == pgn_data.to_string() {
        println!("Parsed equals original file");
    } else {
        println!("Parsed does not equal original file");
    }
}
