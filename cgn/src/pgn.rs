use pgn_reader::{RawHeader, SanPlus, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct SanPlusWrapper(SanPlus);

impl Serialize for SanPlusWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for SanPlusWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(SanPlusWrapper(pgn_reader::SanPlus::from_str(&s).unwrap()))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PgnData {
    pub headers: Vec<(String, String)>,
    pub moves: Vec<SanPlusWrapper>,
}

impl PgnData {
    pub fn new() -> PgnData {
        PgnData {
            headers: vec![],
            moves: vec![],
        }
    }
}

impl std::fmt::Display for PgnData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for (key, value) in &self.headers {
            s.push_str(&format!("[{} \"{}\"]\n", key, value));
        }
        s.push('\n');
        for (i, san_plus) in self.moves.iter().enumerate() {
            if i % 2 == 0 {
                s.push_str(&format!("{}. ", i / 2 + 1));
            }
            s.push_str(&san_plus.0.to_string());
            s.push(' ');
        }
        if let Some((_, result)) = self.headers.iter().find(|(key, _)| key == "Result") {
            s.push_str(result);
        }
        s = textwrap::fill(&s, 80);
        write!(f, "{}", s)
    }
}

pub struct PgnVisitor {
    data: PgnData,
}

impl PgnVisitor {
    pub fn new() -> PgnVisitor {
        PgnVisitor {
            data: PgnData::new(),
        }
    }
}

impl Visitor for PgnVisitor {
    type Result = PgnData;

    fn header(&mut self, _key: &[u8], _value: RawHeader<'_>) {
        if let (Ok(key), Ok(value)) = (String::from_utf8(_key.to_vec()), _value.decode_utf8()) {
            self.data.headers.push((key, value.to_string()));
        }
    }

    fn san(&mut self, _san_plus: SanPlus) {
        self.data.moves.push(SanPlusWrapper(_san_plus));
    }

    fn end_game(&mut self) -> Self::Result {
        self.data.to_owned()
    }
}

mod tests {

    #[test]
    fn headers_are_parsed() {
        let pgn_str = r#"[Event "F/S Return Match"]"#;

        let mut visitor = super::PgnVisitor::new();
        let pgn_data = pgn_reader::BufferedReader::new_cursor(&pgn_str)
            .read_game(&mut visitor)
            .unwrap()
            .unwrap();

        assert_eq!(pgn_data.headers.get(0).unwrap().0, "Event");
        assert_eq!(pgn_data.headers.get(0).unwrap().1, "F/S Return Match");
    }

    #[test]
    fn moves_are_parsed() {
        let pgn_str = r#"
        1. e4 e5+
        "#;

        let mut visitor = super::PgnVisitor::new();
        let pgn_data = pgn_reader::BufferedReader::new_cursor(&pgn_str)
            .read_game(&mut visitor)
            .unwrap()
            .unwrap();

        assert_eq!(pgn_data.moves.get(0).unwrap().0.to_string(), "e4");
        assert_eq!(pgn_data.moves.get(1).unwrap().0.to_string(), "e5+");
        assert_eq!(pgn_data.moves.len(), 2);
    }

    #[test]
    fn parsed_equals_original_file() {
        let pgn_str = include_str!("pgn.txt").replace("\r\n", "\n");

        let mut visitor = super::PgnVisitor::new();
        let pgn_data = pgn_reader::BufferedReader::new_cursor(&pgn_str)
            .read_game(&mut visitor)
            .unwrap()
            .unwrap();

        assert_eq!(pgn_data.to_string(), pgn_str);
    }
}
