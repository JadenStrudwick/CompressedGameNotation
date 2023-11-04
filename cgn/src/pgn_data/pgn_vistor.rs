use super::san_plus_wrapper::SanPlusWrapper;
use super::PgnData;

/// A visitor that collects the data from a PGN file into a PgnData struct.
pub struct PgnVisitor {
    data: PgnData,
}

impl PgnVisitor {
    /// Creates a new PgnVisitor.
    pub fn new() -> PgnVisitor {
        PgnVisitor {
            data: PgnData::new(),
        }
    }
}

impl pgn_reader::Visitor for PgnVisitor {
    /// The result type of the visitor.
    type Result = PgnData;

    /// Called when a header is found in the PGN file.
    fn header(&mut self, _key: &[u8], _value: pgn_reader::RawHeader<'_>) {
        // convert the key and value to strings and add them to the headers vector
        if let (Ok(key), Ok(value)) = (String::from_utf8(_key.to_vec()), _value.decode_utf8()) {
            // match the key and set the corresponding field in the PgnData struct
            match key.as_str() {
                "Event" => self.data.event = value.to_string(),
                "Site" => self.data.site = value.to_string(),
                "Date" => self.data.date = value.to_string(),
                "Round" => self.data.round = value.to_string(),
                "White" => self.data.white = value.to_string(),
                "Black" => self.data.black = value.to_string(),
                "Result" => self.data.result = value.to_string(),
                _ => (),
            }
        }
    }

    /// Called when a move is found in the PGN file.
    fn san(&mut self, _san_plus: pgn_reader::SanPlus) {
        // add the move to the moves vector
        self.data.moves.push(SanPlusWrapper(_san_plus));
    }

    /// Called when the game ends.
    fn end_game(&mut self) -> Self::Result {
        // return the PgnData struct with the collected data
        self.data.to_owned()
    }
}
