use crate::{population::{Population}, traits::{GenotypeT, GeneT}, operations::{Crossover, Selection, Mutation, Survivor, selection, crossover}};

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
pub fn start<T:GeneT, U:GenotypeT<T> + Copy>(population: Population<T,U>, configuration: GaConfiguration)->Vec<U>
{
    let initial_population_size = population.size();

    //We start the cycles
    for i in 0..configuration.max_generations {

        //1- Parent selection for reproduction
        let parents = selection::factory(configuration.selection, &population.individuals());

        //2- Reproduce the parents
        for j in 0..parents.len() {
            
            let mut parent_1 = population.individuals().get(0).copied().unwrap();
            let mut parent_2 = population.individuals().get(1).copied().unwrap();

            let offspring = crossover::factory(configuration.crossover, &mut parent_1, &mut parent_2);
        }

    }

    return Vec::new();
}