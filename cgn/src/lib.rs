mod compression;
mod pgn_data;

pub use pgn_data::PgnData;
pub use compression::bincode_zlib;