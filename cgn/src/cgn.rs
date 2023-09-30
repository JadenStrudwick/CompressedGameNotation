use crate::pgn::PgnData;

type CompressFn = fn(&PgnData) -> Vec<u8>;
type DecompressFn = fn(&[u8]) -> PgnData;

fn compress(_strategy: CompressFn, pgn_data: &PgnData) -> Vec<u8> {
    _strategy(pgn_data)
}

fn decompress(_strategy: DecompressFn, compressed_data: &[u8]) -> PgnData {
    _strategy(compressed_data)
}

mod SerdeStrategy {
    use super::*;

    fn compress(pgn_data: &PgnData) -> Vec<u8> {
        bincode::serialize(pgn_data).unwrap()
    }

    fn decompress(compressed_data: &[u8]) -> PgnData {
        bincode::deserialize(compressed_data).unwrap()
    }
}

mod SerdeCompressStrategy {
    use super::*;

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
