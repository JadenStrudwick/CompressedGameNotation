use crate::pgn_vistor::PgnVisitor;
use crate::san_plus_wrapper::SanPlusWrapper;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
/// PGN data struct that holds the headers and moves of a PGN game.
/// Only stores the data required for PGN 'reduced export format'.
/// A PGN game is in 'reduced export format' if abide by the following rules:
/// 1) There are no comments.
/// 2) Only the 7 mandatory tags are used (Event, Site, Date, Round, White, Black, Result).
/// 3) There are no recursive annotations.
/// 4) There are no numeric annotation glyphs.
pub struct PgnData {
    pub event: String,
    pub site: String,
    pub date: String,
    pub round: String,
    pub white: String,
    pub black: String,
    pub result: String,
    pub moves: Vec<SanPlusWrapper>,
}

impl PgnData {
    /// Creates a new empty PgnData struct.
    pub fn new() -> PgnData {
        PgnData {
            event: String::new(),
            site: String::new(),
            date: String::new(),
            round: String::new(),
            white: String::new(),
            black: String::new(),
            result: String::new(),
            moves: vec![],
        }
    }

    /// Creates a new PgnData struct from a string.
    pub fn from_str(s: &str) -> PgnData {
        let mut visitor = PgnVisitor::new();
        pgn_reader::BufferedReader::new_cursor(&s)
            .read_game(&mut visitor)
            .expect("Failed to read PGN game")
            .expect("Failed to read PGN game")
    }

    /// Clear headers from the PgnData struct.
    pub fn clear_headers(&mut self) {
        self.event.clear();
        self.site.clear();
        self.date.clear();
        self.round.clear();
        self.white.clear();
        self.black.clear();
        self.result.clear();
    }
}

impl std::fmt::Display for PgnData {
    /// Formats the PgnData struct into a PGN string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Create a string buffer and write the headers to it
        let mut s = String::new();
        s.push_str(&format!("[Event \"{}\"]\n", self.event));
        s.push_str(&format!("[Site \"{}\"]\n", self.site));
        s.push_str(&format!("[Date \"{}\"]\n", self.date));
        s.push_str(&format!("[Round \"{}\"]\n", self.round));
        s.push_str(&format!("[White \"{}\"]\n", self.white));
        s.push_str(&format!("[Black \"{}\"]\n", self.black));
        s.push_str(&format!("[Result \"{}\"]\n", self.result));

        // Write the moves to the string buffer
        s.push('\n');
        for (i, san_plus) in self.moves.iter().enumerate() {
            if i % 2 == 0 {
                s.push_str(&format!("{}. ", i / 2 + 1));
            }
            s.push_str(&san_plus.0.to_string());
            s.push(' ');
        }

        // Write the result to the string buffer
        s.push_str(self.result.as_str());

        //  Wrap the string buffer to 80 characters and write it to the formatter
        write!(f, "{}", textwrap::fill(&s, 80))
    }
}

mod tests {
    #[test]
    /// Tests if the PgnData struct can be parsed and then converted back to a string.
    fn parsed_eq_original() {
        let pgn_str = crate::pgn_examples::PGN_STR_EXAMPLE;
        let pgn_data = super::PgnData::from_str(pgn_str);
        assert_eq!(pgn_str, pgn_data.to_string());
    }
}
