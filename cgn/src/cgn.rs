use crate::pgn::PgnData;

pub trait CgnCompressionStrategy {
    fn compress(pgn_data: &PgnData) -> Vec<u8>;
    fn decompress(compressed_data: &[u8]) -> PgnData;
}

pub struct SerdeStrat;
impl CgnCompressionStrategy for SerdeStrat {
    fn compress(pgn_data: &PgnData) -> Vec<u8> {
        bincode::serialize(pgn_data).unwrap()
    }

    fn decompress(compressed_data: &[u8]) -> PgnData {
        bincode::deserialize(compressed_data).unwrap()
    }
}

pub struct SerdeCompressStrat;
impl CgnCompressionStrategy for SerdeCompressStrat {
    fn compress(pgn_data: &PgnData) -> Vec<u8> {
        let mut compressed_data = Vec::new();
        let mut encoder =
            flate2::write::ZlibEncoder::new(&mut compressed_data, flate2::Compression::best());
        bincode::serialize_into(&mut encoder, pgn_data).unwrap();
        encoder.finish().unwrap();
        compressed_data
    }

    fn decompress(compressed_data: &[u8]) -> PgnData {
        let mut decoder = flate2::read::ZlibDecoder::new(compressed_data);
        bincode::deserialize_from(&mut decoder).unwrap()
    }
}

pub fn compress<S: CgnCompressionStrategy>(_strategy: &S, pgn_data: &PgnData) -> Vec<u8> {
    S::compress(pgn_data)
}

pub fn decompress<S: CgnCompressionStrategy>(_strategy: &S, compressed_data: &[u8]) -> PgnData {
    S::decompress(compressed_data)
}
