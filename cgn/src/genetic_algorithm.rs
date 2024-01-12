use cgn::benchmark_utils::{collect_metrics_custom, metrics_to_summary, ToTake};
use cgn::compression::dynamic_huffman::compress_pgn_data_custom;
use cgn::compression::dynamic_huffman::decompress_pgn_data_custom;
use rand::{seq::SliceRandom, thread_rng, Rng};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::fs::File;
use std::io::Write;

/// Configuration for the genetic algorithm used to find the optimal height and dev values for the dynamic Huffman compression algorithm
pub struct GeneticAlgorithmConfig {
    pub init_population: usize,
    pub number_of_games: ToTake,
    pub generations: u32,
    pub mutation_rate: f64,
    pub tournament_size: usize,
    pub height_min: f64,
    pub height_max: f64,
    pub dev_min: f64,
    pub dev_max: f64,
    pub input_db_path: String,
    pub output_path: String,
}

/// An individual in the genetic algorithm
#[derive(Debug, Clone)]
struct Individual {
    height: f64,
    dev: f64,
}

/// Runs a genetic algorithm to find the optimal height and dev values for the dynamic Huffman compression algorithm
pub fn genetic_algorithm(config: GeneticAlgorithmConfig) {
    // create the initial population and create the output file
    let mut population = init_population(&config);
    let mut file = File::create(&config.output_path).unwrap();

    // run the genetic algorithm for the specified number of generations
    for gen_num in 0..config.generations {
        population = create_new_generation(&config, population);
        population.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());

        // write the individuals to the output file
        population
            .iter()
            .enumerate()
            .for_each(|(rank, individual)| {
                file.write_all(
                    format!(
                        "Generation: {}, Rank: {}, Height: {}, Dev: {}, Fitness: {}\n",
                        gen_num, rank, individual.0.height, individual.0.dev, individual.1
                    )
                    .as_bytes(),
                )
                .unwrap()
            });
    }
}

/// Create an inital population of random individuals and evaluate their fitness
fn init_population(config: &GeneticAlgorithmConfig) -> Vec<(Individual, f64)> {
    // create a population of random individuals
    let mut population = Vec::with_capacity(config.init_population);
    let mut rng = thread_rng();
    for _ in 0..config.init_population {
        let height = rng.gen_range(config.height_min..=config.height_max);
        let dev = rng.gen_range(config.dev_min..=config.dev_max);
        population.push(Individual { height, dev });
    }

    // evaluate the fitness of each individual in the initial population
    population
        .into_iter()
        .par_bridge()
        .map(|individual| {
            let fitness = fitness_function(config, &individual);
            (individual, fitness)
        })
        .collect()
}

/// Create a new generation of individuals using crossover and mutation of randomly selected parents
fn create_new_generation(
    config: &GeneticAlgorithmConfig,
    population: Vec<(Individual, f64)>,
) -> Vec<(Individual, f64)> {
    let mut rng = rand::thread_rng();
    let parents = select_parents(config, &population);
    let mut children = Vec::with_capacity(population.len() / 2);

    // create children by crossover of randomly selected parents
    for _ in 0..population.len() {
        let parent1 = parents.choose(&mut rng).unwrap();
        let parent2 = parents.choose(&mut rng).unwrap();
        let child = crossover(config, &parent1.0, &parent2.0);
        children.push(child);
    }

    // evaluate the fitness of each child
    children
        .into_iter()
        .par_bridge()
        .map(|x| {
            let fitness = fitness_function(config, &x);
            (x, fitness)
        })
        .collect()
}

/// Select parents for crossover using tournament selection
fn select_parents<'a>(
    config: &GeneticAlgorithmConfig,
    population: &'a Vec<(Individual, f64)>,
) -> Vec<&'a (Individual, f64)> {
    let mut rng = rand::thread_rng();
    let mut parents = Vec::with_capacity(population.len() / 2);

    // take 50% of the population to be parents
    for _ in 0..population.len() / 2 {
        // randomly select individuals from the population to compete in the tournament
        let mut tournament = Vec::with_capacity(config.tournament_size);
        for _ in 0..config.tournament_size {
            tournament.push(population.choose(&mut rng).unwrap());
        }

        // sort the tournament by fitness (ascending) and select the individual with the lowest fitness
        tournament.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
        parents.push(tournament.remove(0));
    }

    parents
}

/// Create a new individual using crossover of two parents and mutation
fn crossover(
    config: &GeneticAlgorithmConfig,
    parent1: &Individual,
    parent2: &Individual,
) -> Individual {
    // create a child by averaging the height and dev values of the parents
    let mut child = Individual {
        height: (parent1.height + parent2.height) / 2.0,
        dev: (parent1.dev + parent2.dev) / 2.0,
    };

    // randomly mutate the child by changing its height and dev values
    let mut rng = rand::thread_rng();
    if rng.gen_range(0.0..=1.0) < config.mutation_rate {
        child.height = rng.gen_range(config.height_min..=config.height_max);
    }
    if rng.gen_range(0.0..=1.0) < config.mutation_rate {
        child.dev = rng.gen_range(config.dev_min..=config.dev_max);
    }

    child
}

/// Calculate the fitness of an individual
fn fitness_function(config: &GeneticAlgorithmConfig, individual: &Individual) -> f64 {
    metrics_to_summary(collect_metrics_custom(
        compress_pgn_data_custom,
        decompress_pgn_data_custom,
        &config.input_db_path,
        &config.number_of_games,
        individual.height,
        individual.dev,
    ))
    .avg_bits_per_move_excluding_headers
}
