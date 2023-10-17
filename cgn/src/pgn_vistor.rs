use crate::pgn_data::PgnData;
use crate::san_plus_wrapper::SanPlusWrapper;
use pgn_reader::{RawHeader, SanPlus, Visitor};

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

impl Visitor for PgnVisitor {
    /// The result type of the visitor.
    type Result = PgnData;

    /// Called when a header is found in the PGN file.
    fn header(&mut self, _key: &[u8], _value: RawHeader<'_>) {
        // convert the key and value to strings and add them to the headers vector
        if let (Ok(key), Ok(value)) = (String::from_utf8(_key.to_vec()), _value.decode_utf8()) {
            self.data.headers.push((key, value.to_string()));
        }
    }

    /// Called when a move is found in the PGN file.
    fn san(&mut self, _san_plus: SanPlus) {
        // add the move to the moves vector
        self.data.moves.push(SanPlusWrapper(_san_plus));
    }

    /// Called when the game ends.
    fn end_game(&mut self) -> Self::Result {
        // return the PgnData struct with the collected data
        self.data.to_owned()
    }
}
