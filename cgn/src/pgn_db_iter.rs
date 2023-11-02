use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// An iterator over the games in a PGN database file.
struct PgnDBIter<R: BufRead> {
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
                Err(e) => panic!("Error reading line: {}", e),
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
fn pgn_db_into_iter(path: &str) -> PgnDBIter<BufReader<File>> {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    PgnDBIter::new(reader)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pgn_db_iter() {
        // if the file is not found, pass the test (this is for CI)
        let file = std::fs::File::open("./lichessDB.pgn");
        match file {
            Ok(file) => {
                let reader = std::io::BufReader::new(file);
                let iter = super::PgnDBIter::new(reader);

                // convert the first 100 games to PgnData
                iter.take(100).for_each(|game| {
                    crate::pgn_data::PgnData::from_str(&game);
                });
            }
            Err(_) => (),
        }
    }
}
