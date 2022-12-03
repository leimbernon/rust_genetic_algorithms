use crate::{population::{Population}, traits::{GenotypeT, GeneT}, operations::{Crossover, Selection, Mutation, Survivor, selection, crossover, mutation, survivor}};

#[derive(Copy, Clone)]
pub enum ProblemSolving {
    Minimization,
    Maximization,
}

#[derive(Copy, Clone)]
pub struct GaConfiguration {
    pub problem_solving: ProblemSolving,
    pub max_generations: i32,
    pub selection: Selection,
    pub crossover: Crossover,
    pub mutation: Mutation,
    pub survivor: Survivor,
}

/**
 * Function to start the genetic algorithms cycle
 */
pub fn start<T:GeneT, U:GenotypeT<T>>(mut population: Population<T,U>, configuration: GaConfiguration)->Vec<U>
{
    let initial_population_size = population.size();

    //We start the cycles
    for _i in 0..configuration.max_generations {

        //1- Parent selection for reproduction
        let parents = selection::factory(configuration.selection, &population.individuals);

        //2- Reproduce the parents
        for j in parents.keys() {
            
            let index_parent_1 = parents.get_key_value(&j).unwrap().0;
            let index_parent_2 = parents.get_key_value(&j).unwrap().1;
            let parent_1 = population.individuals.get(*index_parent_1).unwrap().clone();
            let parent_2 = population.individuals.get(*index_parent_2).unwrap().clone();

            let mut offspring = crossover::factory(configuration.crossover, parent_1, parent_2).unwrap();
            let mut child_1 = offspring.pop().unwrap();
            let mut child_2 = offspring.pop().unwrap();

            //3- Do the mutation of the children           
            mutation::factory(configuration.mutation, &mut child_1);
            mutation::factory(configuration.mutation, &mut child_2);

            //Insert the children in the population
            population.individuals.push(child_1);
            population.individuals.push(child_2);
        }

        //4- Survivor selection
        survivor::factory(configuration.survivor, &mut population.individuals, initial_population_size, configuration.problem_solving);
    }

    return Vec::new();
}