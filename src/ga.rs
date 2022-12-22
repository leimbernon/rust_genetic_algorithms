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
    pub number_of_couples: i32,
    pub crossover_number_of_points: i32,
    pub selection: Selection,
    pub crossover: Crossover,
    pub mutation: Mutation,
    pub survivor: Survivor,
}

/**
 * Function to run the genetic algorithms cycle
 */
pub fn run<T:GeneT, U:GenotypeT<T>>(mut population: Population<T,U>, configuration: GaConfiguration)->Population<T,U>
{
    let initial_population_size = population.size();
    let mut age = 0;

    //We first calculate the phenotype of the population and set the age of each parent
    for individual in &mut population.individuals{
        individual.calculate_phenotype();
        *individual.get_age_mut() = age;
    }

    //We start the cycles
    for i in 0..configuration.max_generations {

        println!("Generation number: {}", i+1);
        age += 1;

        //1- Parent selection for reproduction
        let parents = selection::factory(configuration.selection, &population.individuals, configuration.number_of_couples);

        //2- Reproduce the parents
        for j in parents.keys() {
            
            let index_parent_1 = parents.get_key_value(&j).unwrap().0;
            let index_parent_2 = parents.get_key_value(&j).unwrap().1;
            let parent_1 = population.individuals.get(*index_parent_1).unwrap().clone();
            let parent_2 = population.individuals.get(*index_parent_2).unwrap().clone();

            let mut offspring = crossover::factory(configuration.crossover, parent_1, parent_2, &configuration).unwrap();
            let mut child_1 = offspring.pop().unwrap();
            let mut child_2 = offspring.pop().unwrap();

            //3- Do the mutation of the children           
            mutation::factory(configuration.mutation, &mut child_1);
            mutation::factory(configuration.mutation, &mut child_2);

            //4- Calculate the phenotype of both children and set their age
            child_1.calculate_phenotype();
            child_2.calculate_phenotype();

            *child_1.get_age_mut() = age;
            *child_2.get_age_mut() = age;

            //Insert the children in the population
            population.individuals.push(child_1);
            population.individuals.push(child_2);
        }

        //5- Survivor selection
        survivor::factory(configuration.survivor, &mut population.individuals, initial_population_size, configuration.problem_solving);
    }

    return population;
}