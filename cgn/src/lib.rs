pub mod pgn {
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
                let mut tag = line
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .splitn(2, ' ');
                let key = tag.next();
                let value = tag.next();
                match (key, value) {
                    (Some(key), Some(value)) => {
                        let key = key.trim_end_matches('"').to_string();
                        let value = value.trim_matches('"').to_string();
                        pgn.tags.push((key, value));
                    }
                    _ => continue,
                }
            } else {
                let mut moves = line.split_whitespace();
                while let Some(m) = moves.next() {
                    if m.ends_with('.') {
                        continue;
                    }
                    pgn.moves.push(m.to_string());
                }
            }
        }
        pgn
    }

    pub fn pgn_to_string(pgn: &Pgn) -> String {
        let mut s = String::new();
        for tag in &pgn.tags {
            s.push_str(&format!("[{} \"{}\"]\n", tag.0, tag.1));
        }
        s.push_str("\n");
        for (i, m) in pgn.moves.iter().enumerate() {
            if i % 2 == 0 {
                s.push_str(&format!("{}. ", i / 2 + 1));
            }
            s.push_str(&format!("{} ", m));
        }
        s.pop();
        textwrap::fill(&s, 80)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn string_to_pgn_are_tags_correct() {
            let str = "[Event \"F/S Return Match\"]\n\
                   [Site \"Belgrade, Serbia JUG\"]\n\
                   [Date \"1992.11.04\"]\n\
                   [Round \"29\"]\n\
                   [White \"Fischer, Robert J.\"]\n\
                   [Black \"Spassky, Boris V.\"]\n\
                   [Result \"1/2-1/2\"]\n\
                   \n\
                   1. e4 e5 2. Nf3 Nc6 3. Bb5 a6";
            let pgn = string_to_pgn(str);
            assert_eq!(
                pgn.tags[0],
                ("Event".to_string(), "F/S Return Match".to_string())
            );
            assert_eq!(
                pgn.tags[1],
                ("Site".to_string(), "Belgrade, Serbia JUG".to_string())
            );
            assert_eq!(pgn.tags[2], ("Date".to_string(), "1992.11.04".to_string()));
            assert_eq!(pgn.tags[3], ("Round".to_string(), "29".to_string()));
            assert_eq!(
                pgn.tags[4],
                ("White".to_string(), "Fischer, Robert J.".to_string())
            );
            assert_eq!(
                pgn.tags[5],
                ("Black".to_string(), "Spassky, Boris V.".to_string())
            );
            assert_eq!(pgn.tags[6], ("Result".to_string(), "1/2-1/2".to_string()));
        }

        #[test]
        fn string_to_pgn_are_moves_correct() {
            let str = "[Event \"F/S Return Match\"]\n\
                   [Site \"Belgrade, Serbia JUG\"]\n\
                   [Date \"1992.11.04\"]\n\
                   [Round \"29\"]\n\
                   [White \"Fischer, Robert J.\"]\n\
                   [Black \"Spassky, Boris V.\"]\n\
                   [Result \"1/2-1/2\"]\n\
                   \n\
                   1. e4 e5 2. Nf3 Nc6 3. Bb5 a6";
            let pgn = string_to_pgn(str);
            assert_eq!(pgn.moves[0], "e4");
            assert_eq!(pgn.moves[1], "e5");
            assert_eq!(pgn.moves[2], "Nf3");
            assert_eq!(pgn.moves[3], "Nc6");
            assert_eq!(pgn.moves[4], "Bb5");
            assert_eq!(pgn.moves[5], "a6");
        }

        #[test]
        fn pgn_to_string_is_equal() {
            let str = "[Event \"F/S Return Match\"]\n\
                   [Site \"Belgrade, Serbia JUG\"]\n\
                   [Date \"1992.11.04\"]\n\
                   [Round \"29\"]\n\
                   [White \"Fischer, Robert J.\"]\n\
                   [Black \"Spassky, Boris V.\"]\n\
                   [Result \"1/2-1/2\"]\n\
                   \n\
                   1. e4 e5 2. Nf3 Nc6 3. Bb5 a6";
            let pgn = string_to_pgn(str);
            let s = pgn_to_string(&pgn);
            assert_eq!(s, str);
        }
    }
}
