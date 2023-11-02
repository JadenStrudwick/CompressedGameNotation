use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// An iterator over the games in a PGN database file.
pub struct PgnDBIter<R: BufRead> {
    reader: R,
    buffer: String,
}

impl<R: BufRead> PgnDBIter<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: String::new(),
        }
    }
}

impl<R: BufRead> Iterator for PgnDBIter<R> {
    // The type of the elements being iterated over.
    type Item = String;

    /// Get the next game in the database.
    fn next(&mut self) -> Option<Self::Item> {
        let mut game = String::new();

        // read until the next game
        loop {
            self.buffer.clear();
            match self.reader.read_line(&mut self.buffer) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    // skip empty lines at the start of a game
                    if game.is_empty() && self.buffer.trim().is_empty() {
                        continue;
                    // stop reading if we reach the start of the next game
                    } else if self.buffer.starts_with("[Event") && !game.is_empty() {
                        break;
                    }
                    // otherwise, add the line to the game
                    game.push_str(&self.buffer);
                }
                Err(_) => return None,
            }
        }

        // return the game if it's not empty
        if game.trim().is_empty() {
            None
        } else {
            Some(game)
        }
    }
}

/// Opens a PGN database file and returns an iterator over the games in the database.
pub fn pgn_db_into_iter(path: &str) -> PgnDBIter<BufReader<File>> {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    PgnDBIter::new(reader)
}

#[cfg(test)]
mod tests {
    const TEST_DBS_DIR: &str = "./testDBs/";

    #[test]
    /// Test that the iterator can be created.
    fn pgn_db_into_iter() {
        let mut iter = super::pgn_db_into_iter(&format!("{}{}", TEST_DBS_DIR, "exampleDB.pgn"));
        assert!(iter.next().is_some());
        assert!(iter.next().is_some());
    }

    #[test]
    /// Test that the iterator is empty when the file is empty.
    fn none_on_empty_file() {
        let mut iter = super::pgn_db_into_iter(&format!("{}{}", TEST_DBS_DIR, "emptyDB.pgn"));
        assert!(iter.next().is_none());
    }

    #[test]
    /// Test that the iterator is empty when the file is not UTF-8.
    fn none_on_non_utf8_file() {
        let mut iter = super::pgn_db_into_iter(&format!("{}{}", TEST_DBS_DIR, "nonUtf8DB.pgn"));
        assert!(iter.next().is_none());
    }

    #[test]
    /// Test that the iterator skips empty lines at the start of a game.
    fn skip_empty_lines() {
        let mut iter = super::pgn_db_into_iter(&format!("{}{}", TEST_DBS_DIR, "emptyLinesDB.pgn"));
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
    }
}
