use cgn::pgn::{pgn_to_string, string_to_pgn};
fn main() {
    let pgn = include_str!("pgn.txt");

    let pgn_s = string_to_pgn(pgn);
    println!("{:?}", pgn_s.tags);
    println!("{:?}", pgn_s.moves);

    let str = pgn_to_string(&pgn_s);
    println!("{}", str);
}
