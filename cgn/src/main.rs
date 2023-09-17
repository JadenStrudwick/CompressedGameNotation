use cgn::pgn_parser::PgnData;
use pgn_reader::BufferedReader;

fn main() {
    let str = include_str!("pgn.txt");
    let mut reader = BufferedReader::new_cursor(&str[..]);
    let mut vis = PgnData::new();

    let result = reader.read_game(&mut vis);
    for mov in result.unwrap().unwrap().moves {
        println!("{:?}", mov.to_string());
    }
}
