use crate::pgn_vistor::PgnVisitor;
use crate::san_plus_wrapper::SanPlusWrapper;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
/// PGN data struct that holds the headers and moves of a PGN game.
pub struct PgnData {
    pub headers: Vec<(String, String)>,
    pub moves: Vec<SanPlusWrapper>,
}

impl PgnData {
    /// Creates a new empty PgnData struct.
    pub fn new() -> PgnData {
        PgnData {
            headers: vec![],
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
}

impl std::fmt::Display for PgnData {
    /// Formats the PgnData struct into a PGN string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Create a string buffer and write the headers to it
        let mut s = String::new();
        for (key, value) in &self.headers {
            s.push_str(&format!("[{} \"{}\"]\n", key, value));
        }

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
        if let Some((_, result)) = self.headers.iter().find(|(key, _)| key == "Result") {
            s.push_str(result);
        }

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
