mod genetic_algorithm;
use cgn::benchmark_utils::ToTake;
use cgn::compression::bincode::{bincode_compress_pgn_str, bincode_decompress_pgn_str};
use cgn::compression::dynamic_huffman::{
    dynamic_huffman_compress_pgn_str, dynamic_huffman_decompress_pgn_str,
};
use cgn::compression::huffman::{huffman_compress_pgn_str, huffman_decompress_pgn_str};
use clap::{Parser, Subcommand};
use genetic_algorithm::{genetic_algorithm, GeneticAlgorithmConfig};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Parser)]
#[clap(name = "cgn", version = "0.1.0", author = "Jaden M. Strudwick")]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compress a single PGN file
    Compress {
        /// Optimization level (0-2)
        #[clap(short, default_value = "2", value_parser = |s: &str| match s.parse::<u8>() {
            Ok(n) if n <= 2 => Ok(n),
            _ => Err(String::from("Optimization level must be between 0 and 2")),
        })]
        optimization_level: u8,

        /// Input file path
        #[clap(value_parser)]
        input_path: String,

        /// Output file path
        #[clap(value_parser)]
        output_path: String,
    },
    /// Decompress a single PGN file
    Decompress {
        /// Optimization level (0-2)
        #[clap(short, default_value = "2", value_parser = |s: &str| match s.parse::<u8>() {
            Ok(n) if n <= 2 => Ok(n),
            _ => Err(String::from("Optimization level must be between 0 and 2")),
        })]
        optimization_level: u8,

        /// Input file path
        #[clap(value_parser)]
        input_path: String,

        /// Output file path
        #[clap(value_parser)]
        output_path: String,
    },
    /// Run a genetic algorithm to find the optimal height and dev values for the dynamic Huffman compression algorithm. Used during development.
    GenAlgo {
        /// Initial population size
        #[clap(value_parser)]
        init_population: usize,

        /// Number of games to benchmark each individual on
        #[clap(value_parser)]
        number_of_games: ToTake,

        /// Number of generations to run the genetic algorithm for
        #[clap(value_parser)]
        generations: u32,

        /// Mutation rate (0.0-1.0)
        #[clap(value_parser = |s: &str| s.parse::<f64>().map(|n| n.clamp(0.0, 1.0)))]
        mutation_rate: f64,

        /// Tournament size
        #[clap(value_parser)]
        tournament_size: usize,

        /// Minimum height value
        #[clap(value_parser)]
        height_min: f64,

        /// Maximum height value
        #[clap(value_parser)]
        height_max: f64,

        /// Minimum dev value
        #[clap(value_parser)]
        dev_min: f64,

        /// Maximum dev value
        #[clap(value_parser)]
        dev_max: f64,

        /// Input database path (Lichess PGN database format required)
        #[clap(value_parser)]
        input_db_path: String,

        /// Output file path for the genetic algorithm results
        #[clap(value_parser)]
        output_path: String,
    },
}

fn main() {
    let cli = Args::parse();

    match cli.command {
        Commands::Compress {
            optimization_level,
            input_path,
            output_path,
        } => {
            // open and read the file into a string
            let mut input_file = File::open(input_path).unwrap();
            let mut pgn_str = String::new();
            input_file.read_to_string(&mut pgn_str).unwrap();

            // compress the PGN data using the specified optimization level
            let compressed_pgn_data = match optimization_level {
                0 => bincode_compress_pgn_str(&pgn_str),
                1 => huffman_compress_pgn_str(&pgn_str),
                2 => dynamic_huffman_compress_pgn_str(&pgn_str),
                _ => unreachable!(),
            };

            // if the vector is empty, then the compression failed
            if compressed_pgn_data.is_empty() {
                println!("Compression failed");
                return;
            }

            // write the compressed PGN data to the output file
            let mut output_file = File::create(output_path).unwrap();
            output_file.write_all(&compressed_pgn_data).unwrap();
        }
        Commands::Decompress {
            optimization_level,
            input_path,
            output_path,
        } => {
            // open and read the file into a string
            let mut input_file = File::open(input_path).unwrap();
            let mut compressed_pgn_data = Vec::new();
            input_file.read_to_end(&mut compressed_pgn_data).unwrap();

            // decompress the PGN data using the specified optimization level
            let pgn_data = match optimization_level {
                0 => bincode_decompress_pgn_str(&compressed_pgn_data),
                1 => huffman_decompress_pgn_str(&compressed_pgn_data),
                2 => dynamic_huffman_decompress_pgn_str(&compressed_pgn_data),
                _ => unreachable!(),
            };

            // if the string is empty, then the decompression failed
            if pgn_data.is_empty() {
                println!("Decompression failed");
                return;
            }

            // write the decompressed PGN data to the output file
            let mut output_file = File::create(output_path).unwrap();
            output_file.write_all(pgn_data.as_bytes()).unwrap();
        }
        Commands::GenAlgo {
            input_db_path,
            output_path,
            number_of_games,
            init_population,
            generations,
            height_min,
            height_max,
            dev_min,
            dev_max,
            mutation_rate,
            tournament_size,
        } => {
            let config = GeneticAlgorithmConfig {
                init_population,
                number_of_games,
                generations,
                mutation_rate,
                tournament_size,
                height_min,
                height_max,
                dev_min,
                dev_max,
                input_db_path,
                output_path,
            };
            genetic_algorithm(config);
        }
    }
}
