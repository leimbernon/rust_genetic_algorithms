use crate::{population::{Population}, traits::{GenotypeT, GeneT}, operations::{Crossover, Selection, Mutation, Survivor, selection}};

#[derive(Copy, Clone)]
pub enum ProblemSolving {
    Minimization,
    Maximization,
}

#[derive(Copy, Clone)]
pub struct GaConfiguration {
    problem_solving: ProblemSolving,
    max_generations: i32,
    selection: Selection,
    crossover: Crossover,
    mutation: Mutation,
    survivor: Survivor,
}

/**
 * Function to start the genetic algorithms cycle
 */
pub fn start<T:GeneT, U:GenotypeT<T>>(population: Population<T,U>, configuration: GaConfiguration)->Vec<U>{

    let initial_population_size = population.size();

    //We start the cycles
    for i in 0..configuration.max_generations {

        //1- Parent selection
        selection::factory(configuration.selection, population.individuals());

    }

    return Vec::new();
}