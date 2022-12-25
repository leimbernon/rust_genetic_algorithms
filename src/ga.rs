use crate::{population::{Population}, traits::{GenotypeT, GeneT}, operations::{selection, crossover, mutation, survivor}, configuration::ProblemSolving};
use crate::configuration::GaConfiguration;

/**
 * Function to run the genetic algorithms cycle
 */
pub fn run<T:GeneT, U:GenotypeT<T>>(mut population: Population<T,U>, configuration: GaConfiguration)->Population<T,U>
{
    //Best individual within the generations and population returned
    let mut best_individual = U::new();
    let mut initial_individual = true;
    let initial_population_size = population.size();
    let mut age = 0;

    //We first calculate the phenotype of the population, set the age of each parent and set the best individual
    for individual in &mut population.individuals{
        individual.calculate_fitness();
        *individual.get_age_mut() = age;
        
        if !initial_individual {
            best_individual = get_best_individual(&best_individual, &individual, configuration.problem_solving);
        } else{
            *best_individual.get_dna_mut() = individual.get_dna().clone();
            *best_individual.get_fitness_mut() = individual.get_fitness().clone();
            initial_individual = true;
        }
    }

    //We start the cycles
    for i in 0..configuration.limit_configuration.max_generations {

        println!("Generation number: {}", i+1);
        age += 1;

        //1- Parent selection for reproduction
        let parents = selection::factory(configuration.selection, &population.individuals, configuration.selection_configuration);

        //2- Reproduce the parents
        for j in parents.keys() {
            
            let index_parent_1 = parents.get_key_value(&j).unwrap().0;
            let index_parent_2 = parents.get_key_value(&j).unwrap().1;
            let parent_1 = population.individuals.get(*index_parent_1).unwrap().clone();
            let parent_2 = population.individuals.get(*index_parent_2).unwrap().clone();

            let mut offspring = crossover::factory(configuration.crossover, parent_1, parent_2, configuration.crossover_configuration).unwrap();
            let mut child_1 = offspring.pop().unwrap();
            let mut child_2 = offspring.pop().unwrap();

            //3- Do the mutation of the children           
            mutation::factory(configuration.mutation, &mut child_1);
            mutation::factory(configuration.mutation, &mut child_2);

            //4- Calculate the phenotype of both children and set their age
            child_1.calculate_fitness();
            child_2.calculate_fitness();

            *child_1.get_age_mut() = age;
            *child_2.get_age_mut() = age;

            //Sets the best individual
            best_individual = get_best_individual(&best_individual, &child_1, configuration.problem_solving);
            best_individual = get_best_individual(&best_individual, &child_2, configuration.problem_solving);

            //Insert the children in the population
            population.individuals.push(child_1);
            population.individuals.push(child_2);
        }

        //5- Survivor selection
        survivor::factory(configuration.survivor, &mut population.individuals, initial_population_size, configuration.problem_solving);
    }

    return Population::new(vec![best_individual]);
}

//Function to determine which of the individuals is the best individual and return the best of them
fn get_best_individual<T:GeneT, U:GenotypeT<T>>(individual_1: &U, individual_2: &U, problem_solving: ProblemSolving) -> U{

    let mut best_individual = U::new();

    if problem_solving == ProblemSolving::Maximization {

        //We check if the phenotype is the best and store it if it's the case
        if individual_1.get_fitness() >= individual_2.get_fitness(){
            *best_individual.get_dna_mut() = individual_1.get_dna().clone();
            *best_individual.get_fitness_mut() = individual_1.get_fitness().clone();
        }else{
            *best_individual.get_dna_mut() = individual_2.get_dna().clone();
            *best_individual.get_fitness_mut() = individual_2.get_fitness().clone();
        }

    } else {

        //We check if the phenotype is the best and store it if it's the case
        if individual_1.get_fitness() >= individual_2.get_fitness(){
            *best_individual.get_dna_mut() = individual_2.get_dna().clone();
            *best_individual.get_fitness_mut() = individual_2.get_fitness().clone();
        }else{
            *best_individual.get_dna_mut() = individual_1.get_dna().clone();
            *best_individual.get_fitness_mut() = individual_1.get_fitness().clone();
        }

    }

    best_individual
}