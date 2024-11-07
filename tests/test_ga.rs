#[cfg(test)]
mod structures;
use genetic_algorithms::{operations::{Selection, Crossover, Mutation, Survivor}, population::Population, traits::{ChromosomeT, ConfigurationT}, configuration::ProblemSolving, ga};
use genetic_algorithms::ga::TerminationCause;
use crate::structures::{Gene, Chromosome};
extern crate num_cpus;

#[test]
fn test_ga_start_maximize(){

    //Creates the population
    let dna_1 = vec![Gene{id:1}, Gene{id:2}, Gene{id:3}, Gene{id:4}];
    let dna_2 = vec![Gene{id:2}, Gene{id:3}, Gene{id:4}, Gene{id:1}];
    let dna_3 = vec![Gene{id:3}, Gene{id:4}, Gene{id:1}, Gene{id:2}];
    let dna_4 = vec![Gene{id:4}, Gene{id:1}, Gene{id:2}, Gene{id:3}];
    let dna_5 = vec![Gene{id:2}, Gene{id:1}, Gene{id:3}, Gene{id:4}];
    let dna_6 = vec![Gene{id:1}, Gene{id:3}, Gene{id:4}, Gene{id:2}];
    let dna_7 = vec![Gene{id:3}, Gene{id:4}, Gene{id:2}, Gene{id:1}];
    let dna_8 = vec![Gene{id:4}, Gene{id:2}, Gene{id:1}, Gene{id:3}];
    let dna_9 = vec![Gene{id:2}, Gene{id:1}, Gene{id:4}, Gene{id:3}];
    let dna_10 = vec![Gene{id:1}, Gene{id:4}, Gene{id:3}, Gene{id:2}];

    let individuals = vec![Chromosome{dna: dna_1, fitness: 1.0, age: 0}, Chromosome{dna: dna_2, fitness: 2.0, age: 0},
    Chromosome{dna: dna_3, fitness: 3.0, age: 0}, Chromosome{dna: dna_4, fitness: 4.0, age: 0}, Chromosome{dna: dna_5, fitness: 5.0, age: 0}, 
    Chromosome{dna: dna_6, fitness: 6.0, age: 0}, Chromosome{dna: dna_7, fitness: 7.0, age: 0}, Chromosome{dna: dna_8, fitness: 8.0, age: 0},
    Chromosome{dna: dna_9, fitness: 9.0, age: 0}, Chromosome{dna: dna_10, fitness: 10.0, age: 0}];

    let mut population = Population::new(individuals);
    population = ga::Ga::new()
                        .with_problem_solving(ProblemSolving::Maximization)
                        .with_selection_method(Selection::Random)
                        .with_crossover_method(Crossover::Cycle)
                        .with_mutation_method(Mutation::Swap)
                        .with_survivor_method(Survivor::Fitness)
                        .with_population(population)
                        .run();
    
    assert_eq!(population.individuals.len(), 1);
    assert_eq!(population.individuals[0].get_fitness(), 20.0);

}

#[test]
fn test_ga_run_minimize(){

    //Creates the population
    let dna_1 = vec![Gene{id:1}, Gene{id:2}, Gene{id:3}, Gene{id:4}];
    let dna_2 = vec![Gene{id:2}, Gene{id:3}, Gene{id:4}, Gene{id:1}];
    let dna_3 = vec![Gene{id:3}, Gene{id:4}, Gene{id:1}, Gene{id:2}];
    let dna_4 = vec![Gene{id:4}, Gene{id:1}, Gene{id:2}, Gene{id:3}];
    let dna_5 = vec![Gene{id:2}, Gene{id:1}, Gene{id:3}, Gene{id:4}];
    let dna_6 = vec![Gene{id:1}, Gene{id:3}, Gene{id:4}, Gene{id:2}];
    let dna_7 = vec![Gene{id:3}, Gene{id:4}, Gene{id:2}, Gene{id:1}];
    let dna_8 = vec![Gene{id:4}, Gene{id:2}, Gene{id:1}, Gene{id:3}];
    let dna_9 = vec![Gene{id:2}, Gene{id:1}, Gene{id:4}, Gene{id:3}];
    let dna_10 = vec![Gene{id:1}, Gene{id:4}, Gene{id:3}, Gene{id:2}];

    let individuals = vec![Chromosome{dna: dna_1, fitness: 1.0, age: 0}, Chromosome{dna: dna_2, fitness: 2.0, age: 0},
    Chromosome{dna: dna_3, fitness: 3.0, age: 0}, Chromosome{dna: dna_4, fitness: 4.0, age: 0}, Chromosome{dna: dna_5, fitness: 5.0, age: 0}, 
    Chromosome{dna: dna_6, fitness: 6.0, age: 0}, Chromosome{dna: dna_7, fitness: 7.0, age: 0}, Chromosome{dna: dna_8, fitness: 8.0, age: 0},
    Chromosome{dna: dna_9, fitness: 9.0, age: 0}, Chromosome{dna: dna_10, fitness: 10.0, age: 0}];

    let mut population = Population::new(individuals);
    population = ga::Ga::new()
                    .with_problem_solving(ProblemSolving::Minimization)
                    .with_selection_method(Selection::Random)
                    .with_crossover_method(Crossover::Cycle)
                    .with_mutation_method(Mutation::Swap)
                    .with_mutation_probability_max(0.2)
                    .with_survivor_method(Survivor::Fitness)
                    .with_population(population)
                    .run();
    
    assert_eq!(population.individuals.len(), 1);
    assert_eq!(population.individuals[0].get_fitness(), 10.0);

}


#[test]
fn test_ga_run(){

    //Creates the population
    let dna_1 = vec![Gene{id:1}, Gene{id:2}, Gene{id:3}, Gene{id:4}];
    let dna_2 = vec![Gene{id:2}, Gene{id:3}, Gene{id:4}, Gene{id:1}];
    let dna_3 = vec![Gene{id:3}, Gene{id:4}, Gene{id:1}, Gene{id:2}];
    let dna_4 = vec![Gene{id:4}, Gene{id:1}, Gene{id:2}, Gene{id:3}];
    let dna_5 = vec![Gene{id:2}, Gene{id:1}, Gene{id:3}, Gene{id:4}];
    let dna_6 = vec![Gene{id:1}, Gene{id:3}, Gene{id:4}, Gene{id:2}];
    let dna_7 = vec![Gene{id:3}, Gene{id:4}, Gene{id:2}, Gene{id:1}];
    let dna_8 = vec![Gene{id:4}, Gene{id:2}, Gene{id:1}, Gene{id:3}];
    let dna_9 = vec![Gene{id:2}, Gene{id:1}, Gene{id:4}, Gene{id:3}];
    let dna_10 = vec![Gene{id:1}, Gene{id:4}, Gene{id:3}, Gene{id:2}];

    let individuals = vec![Chromosome{dna: dna_1, fitness: 1.0, age: 0}, Chromosome{dna: dna_2, fitness: 2.0, age: 0},
    Chromosome{dna: dna_3, fitness: 3.0, age: 0}, Chromosome{dna: dna_4, fitness: 4.0, age: 0}, Chromosome{dna: dna_5, fitness: 5.0, age: 0}, 
    Chromosome{dna: dna_6, fitness: 6.0, age: 0}, Chromosome{dna: dna_7, fitness: 7.0, age: 0}, Chromosome{dna: dna_8, fitness: 8.0, age: 0},
    Chromosome{dna: dna_9, fitness: 9.0, age: 0}, Chromosome{dna: dna_10, fitness: 10.0, age: 0}];

    let mut population = Population::new(individuals);
    population = ga::Ga::new()
                    .with_threads(8)
                    .with_problem_solving(ProblemSolving::Maximization)
                    .with_selection_method(Selection::Tournament)
                    .with_number_of_couples(10)
                    .with_crossover_method(Crossover::Cycle)
                    .with_mutation_method(Mutation::Swap)
                    .with_survivor_method(Survivor::Fitness)
                    .with_population(population)
                    .run();
    
    assert_eq!(population.individuals.len(), 1);
    
}

#[test]
fn test_parent_crossover_repeating_alleles(){

    //Setup the alleles and initialize the population randomly
    let binding =  vec![Gene{id:1}, Gene{id:2}, Gene{id:3}, Gene{id:4},
                                   Gene{id:5}, Gene{id:6}, Gene{id:7}, Gene{id:8}];
    let alleles = binding.as_slice();
    static GENES_PER_INDIVIDUAL: i32 = 6;
    static POPULATION_SIZE: i32 = 100;
    static NEEDS_UNIQUE_IDS: bool = false;
    static ALLELES_CAN_BE_REPEATED: bool = true;
    static NUMBER_OF_THREADS: i32 = 8;

    let population: Population<Chromosome> = ga::Ga::new()
                    .with_threads(NUMBER_OF_THREADS)
                    .with_population_size(POPULATION_SIZE)
                    .with_genes_per_individual(GENES_PER_INDIVIDUAL)
                    .with_needs_unique_ids(NEEDS_UNIQUE_IDS)
                    .with_alleles_can_be_repeated(ALLELES_CAN_BE_REPEATED)
                    .with_alleles(alleles.to_vec())
                    .random_initialization();

    //Once population has been initialized, we check for each individual in the population the number of genes in the dna
    for individual in population.individuals{
        assert!(individual.dna.len() == GENES_PER_INDIVIDUAL.try_into().unwrap());
    }
}

#[test]
fn test_parent_crossover_without_repeating_alleles(){

    //Setup the alleles and initialize the population randomly
    let binding =  vec![Gene{id:1}, Gene{id:2}, Gene{id:3}, Gene{id:4},
                                   Gene{id:5}, Gene{id:6}, Gene{id:7}, Gene{id:8}];
    let alleles = binding.as_slice();
    static GENES_PER_INDIVIDUAL: i32 = 6;
    static POPULATION_SIZE: i32 = 100;
    static NEEDS_UNIQUE_IDS: bool = false;
    static ALLELES_CAN_BE_REPEATED: bool = false;
    static NUMBER_OF_THREADS: i32 = 8;

    let population: Population<Chromosome> = ga::Ga::new()
                    .with_threads(NUMBER_OF_THREADS)
                    .with_population_size(POPULATION_SIZE)
                    .with_genes_per_individual(GENES_PER_INDIVIDUAL)
                    .with_needs_unique_ids(NEEDS_UNIQUE_IDS)
                    .with_alleles_can_be_repeated(ALLELES_CAN_BE_REPEATED)
                    .with_alleles(alleles.to_vec())
                    .random_initialization();

    //Once population has been initialized, we check for each individual we check that genes are not repeated
    for individual in population.individuals{
        let mut gene_ids = Vec::new();

        for gene in individual.dna{
            if !gene_ids.is_empty(){
                assert!(!gene_ids.contains(&gene.id));
            }
            gene_ids.push(gene.id);
        }
    }
}

fn callback_function(generation_number: &i32, population: &Population<Chromosome>, termination_cause: TerminationCause){
    assert!(*generation_number >= 7);
    assert_eq!(population.individuals.len(), 10);
    assert!(termination_cause == TerminationCause::NotTerminated ||  termination_cause == TerminationCause::GenerationLimitReached);
}

#[test]
fn test_callback_function(){
    //Creates the population
    let dna_1 = vec![Gene{id:1}, Gene{id:2}, Gene{id:3}, Gene{id:4}];
    let dna_2 = vec![Gene{id:2}, Gene{id:3}, Gene{id:4}, Gene{id:1}];
    let dna_3 = vec![Gene{id:3}, Gene{id:4}, Gene{id:1}, Gene{id:2}];
    let dna_4 = vec![Gene{id:4}, Gene{id:1}, Gene{id:2}, Gene{id:3}];
    let dna_5 = vec![Gene{id:2}, Gene{id:1}, Gene{id:3}, Gene{id:4}];
    let dna_6 = vec![Gene{id:1}, Gene{id:3}, Gene{id:4}, Gene{id:2}];
    let dna_7 = vec![Gene{id:3}, Gene{id:4}, Gene{id:2}, Gene{id:1}];
    let dna_8 = vec![Gene{id:4}, Gene{id:2}, Gene{id:1}, Gene{id:3}];
    let dna_9 = vec![Gene{id:2}, Gene{id:1}, Gene{id:4}, Gene{id:3}];
    let dna_10 = vec![Gene{id:1}, Gene{id:4}, Gene{id:3}, Gene{id:2}];

    let individuals = vec![Chromosome{dna: dna_1, fitness: 1.0, age: 0}, Chromosome{dna: dna_2, fitness: 2.0, age: 0},
                           Chromosome{dna: dna_3, fitness: 3.0, age: 0}, Chromosome{dna: dna_4, fitness: 4.0, age: 0}, Chromosome{dna: dna_5, fitness: 5.0, age: 0},
                           Chromosome{dna: dna_6, fitness: 6.0, age: 0}, Chromosome{dna: dna_7, fitness: 7.0, age: 0}, Chromosome{dna: dna_8, fitness: 8.0, age: 0},
                           Chromosome{dna: dna_9, fitness: 9.0, age: 0}, Chromosome{dna: dna_10, fitness: 10.0, age: 0}];

    let mut population = Population::new(individuals);
    population = ga::Ga::new()
        .with_threads(8)
        .with_problem_solving(ProblemSolving::Maximization)
        .with_selection_method(Selection::Tournament)
        .with_number_of_couples(10)
        .with_crossover_method(Crossover::Cycle)
        .with_mutation_method(Mutation::Swap)
        .with_survivor_method(Survivor::Fitness)
        .with_population(population)
        .with_max_generations(10)
        .run_with_callback(Some(callback_function), 8);

    assert_eq!(population.individuals.len(), 1);
}