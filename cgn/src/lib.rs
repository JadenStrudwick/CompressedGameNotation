pub mod pgn {
    /**
     * Module that contains the code that loads the PGN file into a PGN struct.
     * This struct contains all the raw data of the PGN file.
     *
     * It stores the tags as a vector of 2-pair tuples, and moves as a vector of strings
     * (all additional information about moves such as captures as kept).
     *
     * Because it retains information about captures, it is not a 'minimal' representation of the game since,
     * those captures could be infered. However, the upside of this is that it is very ease to decode a PGN,
     * struct back into a pgn file as no inference step is required, just straight reading.
     *
     * The only small optimation is that in a normal PGN file, the result is printed at the end of the moveset.
     * This is duplicated and unrequired data, as we already have this in the tags, hence we remove it from the
     * moveset of the PGN struct, and simply paste from the struct tags when decoding a PGN struct.
     *
     */
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Pgn {
        pub tags: Vec<(String, String)>,
        pub moves: Vec<String>,
    }

    pub fn string_to_pgn(s: &str) -> Pgn {
        let mut pgn = Pgn {
            tags: Vec::new(),
            moves: Vec::new(),
        };

        let mut lines = s.lines();
        while let Some(line) = lines.next() {
            if line.starts_with('[') {
                // Extract tag key and value
                let mut tag = line
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .splitn(2, ' ');
                let key = tag.next();
                let value = tag.next();

                // If both key and value are Some, push to tags vector
                match (key, value) {
                    (Some(key), Some(value)) => {
                        let key = key.trim_end_matches('"').to_string();
                        let value = value.trim_matches('"').to_string();
                        pgn.tags.push((key, value));
                    }
                    _ => continue,
                }
            } else {
                // Extract moves
                let mut moves = line.split_whitespace();
                while let Some(m) = moves.next() {
                    // If string ends with a dot, skip it since it's not a move
                    if m.ends_with('.') {
                        continue;
                    }

                    pgn.moves.push(m.to_string());
                }
            }
        }

        // Remove last move since it's the result of the game
        pgn.moves.pop();
        pgn
    }

    pub fn pgn_to_string(pgn: &Pgn) -> String {
        let mut s = String::new();

        // Add tags
        for tag in &pgn.tags {
            s.push_str(&format!("[{} \"{}\"]\n", tag.0, tag.1));
        }
        s.push_str("\n");

        // Add moves
        for (i, m) in pgn.moves.iter().enumerate() {
            // Add move number
            if i % 2 == 0 {
                s.push_str(&format!("{}. ", i / 2 + 1));
            }
            s.push_str(&format!("{} ", m));
        }

        // Add result of game after moves
        let result = pgn.tags.iter().find(|(k, _)| k == "Result");
        match result {
            Some((_, r)) => s.push_str(&format!("{}", r)),
            None => s.push_str("*"),
        }

        // Wrap text to 80 characters
        textwrap::fill(&s, 80)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn string_to_pgn_are_tags_correct() {
            let str = include_str!("pgn.txt");
            let pgn = string_to_pgn(str);
            assert_eq!(
                pgn.tags[0],
                (
                    "Event".to_string(),
                    "Titled Tuesday Blitz January 03 Early 2023".to_string()
                )
            );
            assert_eq!(pgn.tags[1], ("Site".to_string(), "".to_string()));
            assert_eq!(pgn.tags[2], ("Date".to_string(), "2023.01.03".to_string()));
            assert_eq!(pgn.tags[3], ("Round".to_string(), "?".to_string()));
            assert_eq!(
                pgn.tags[4],
                ("White".to_string(), "Magnus Carlsen".to_string())
            );
            assert_eq!(
                pgn.tags[5],
                ("Black".to_string(), "Samvel Ter-Sahakyan".to_string())
            );
            assert_eq!(pgn.tags[6], ("Result".to_string(), "1-0".to_string()));
        }

        #[test]
        fn string_to_pgn_are_moves_correct() {
            let str = include_str!("pgn.txt");
            let pgn = string_to_pgn(str);
            assert_eq!(pgn.moves[0], "a4");
            assert_eq!(pgn.moves[1], "Nf6");
            assert_eq!(pgn.moves[2], "d4");
            assert_eq!(pgn.moves[3], "d5");
        }

        #[test]
        fn pgn_to_string_is_equal() {
            let str = include_str!("pgn.txt");
            let pgn = string_to_pgn(str);
            let s = pgn_to_string(&pgn);
            assert_eq!(s, str);
        }
    }
}

pub mod pgn_parser {
    use pgn_reader::{RawHeader, SanPlus, Visitor};

    #[derive(Clone, Debug)]
    pub struct PgnData {
        pub headers: Vec<(String, String)>,
        pub moves: Vec<SanPlus>,
    }

    impl PgnData {
        pub fn new() -> PgnData {
            PgnData {
                headers: vec![],
                moves: vec![],
            }
        }
    }

    impl Visitor for PgnData {
        type Result = PgnData;

        fn header(&mut self, _key: &[u8], _value: RawHeader<'_>) {
            let key = String::from_utf8(_key.to_vec());
            let value = _value.decode_utf8();
            match (key, value) {
                (Ok(key), Ok(value)) => self.headers.push((key, value.to_string())),
                _ => (),
            }
        }

        fn san(&mut self, _san_plus: SanPlus) {
            self.moves.push(_san_plus)
        }

        fn end_game(&mut self) -> Self::Result {
            self.clone()
        }
    }

    // TODO: Write tests if this ends up being the way I do things
}
