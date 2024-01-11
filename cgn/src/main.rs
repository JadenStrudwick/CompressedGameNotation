use rand::seq::SliceRandom;

use cgn::benchmark_utils::collect_metrics_custom;
use cgn::benchmark_utils::metrics_to_summary;
use cgn::compression::dynamic_huffman::compress_pgn_data_custom;
use cgn::compression::dynamic_huffman::decompress_pgn_data_custom;

use rand::{thread_rng, Rng};
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;

const N: usize = 100;
const HEIGHT_MIN: f64 = 1.0;
const HEIGHT_MAX: f64 = 225_883_932.0;
const DEV_MIN: f64 = 1.0;
const DEV_MAX: f64 = 10.0;
const MUTATION_RATE: f64 = 0.2;
const TOURNAMENT_SIZE: usize = 2;

#[derive(Debug, Clone)]
struct Individual {
    height: f64,
    dev: f64,
}

fn fitness_function(indiv: &Individual) -> f64 {
    let height = indiv.height;
    let dev = indiv.dev;
    let metrics = collect_metrics_custom(
        compress_pgn_data_custom,
        decompress_pgn_data_custom,
        N,
        height,
        dev,
    );
    let summary = metrics_to_summary(metrics);

    summary.avg_bits_per_move_excluding_headers
}

fn init_population(n: usize) -> Vec<(Individual, f64)> {
    let mut population = Vec::with_capacity(n);
    let mut rng = thread_rng();

    for _ in 0..n {
        let height = rng.gen_range(HEIGHT_MIN..=HEIGHT_MAX);
        let dev = rng.gen_range(DEV_MIN..=DEV_MAX);
        population.push(Individual { height, dev });
    }

    population
        .into_iter()
        .par_bridge()
        .map(|x| {
            let fitness = fitness_function(&x);
            (x, fitness)
        })
        .collect()
}

/// Select parents using tournament selection
fn select_parents(population: &Vec<(Individual, f64)>) -> Vec<&(Individual, f64)> {
    let mut rng = rand::thread_rng();
    let mut parents = Vec::with_capacity(population.len() / 2);

    for _ in 0..population.len() / 2 {
        let mut tournament = Vec::with_capacity(TOURNAMENT_SIZE);
        for _ in 0..TOURNAMENT_SIZE {
            let indiv = population.choose(&mut rng).unwrap();
            tournament.push(indiv);
        }

        // sort the tournament by fitness (ascending)
        tournament.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
        parents.push(tournament.remove(0));
    }

    parents
}

fn crossover(parent1: &Individual, parent2: &Individual) -> Individual {
    let height = (parent1.height + parent2.height) / 2.0;
    let dev = (parent1.dev + parent2.dev) / 2.0;
    let mut child = Individual { height, dev };
    mutate(&mut child);
    child
}

// Possibly change
fn mutate(indiv: &mut Individual) {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0.0..=1.0) < MUTATION_RATE {
        indiv.height = rng.gen_range(HEIGHT_MIN..=HEIGHT_MAX);
    }
    if rng.gen_range(0.0..=1.0) < MUTATION_RATE {
        indiv.dev = rng.gen_range(DEV_MIN..=DEV_MAX);
    }
}

/// Create a new generation of individuals using crossover and mutation of randomly selected parents
fn new_generation(population: Vec<(Individual, f64)>) -> Vec<(Individual, f64)> {
    let mut rng = rand::thread_rng();
    let parents = select_parents(&population);
    let mut children = Vec::with_capacity(population.len() / 2);

    for _ in 0..population.len() {
        let parent1 = parents.choose(&mut rng).unwrap();
        let parent2 = parents.choose(&mut rng).unwrap();
        let child = crossover(&parent1.0, &parent2.0);
        children.push(child);
    }

    children
        .into_iter()
        .par_bridge()
        .map(|x| {
            let fitness = fitness_function(&x);
            (x, fitness)
        })
        .collect()
}

fn main() {
    let mut population = init_population(100);
    let mut best = (
        Individual {
            height: 1.0,
            dev: 1.0,
        },
        10.0,
    );

    for i in 0..100 {
        population = new_generation(population);
        population.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());

        for (j, indiv) in population.iter().enumerate() {
            println!("{} {}: {:?}", i, j, indiv);
        }

        if population[0].1 < best.1 {
            println!("New best: {:?}", population[0]);
            best = population.remove(0)
        }
    }
}
