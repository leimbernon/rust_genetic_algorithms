use genetic_algorithms::{ga::{run, self}, operations::{Selection, Crossover, Mutation, Survivor}, population::Population, traits::GenotypeT, configuration::{GaConfiguration, ProblemSolving, LimitConfiguration, SelectionConfiguration, MutationConfiguration, CrossoverConfiguration}};
use crate::structures::{Gene, Genotype};
extern crate num_cpus;

#[test]
fn test_ga_start_maximize(){

    //Creates the GA configuration
    let configuration = GaConfiguration{
        adaptive_ga: false,
        number_of_threads: None,
        limit_configuration: LimitConfiguration{max_generations: 100, fitness_target: None, problem_solving: ProblemSolving::Maximization, get_best_individual_by_generation: None},
        selection_configuration: SelectionConfiguration { number_of_couples: None, method: Selection::Random },
        crossover_configuration: CrossoverConfiguration{method: Crossover::Cycle, number_of_points: None, ..Default::default()},
        mutation_configuration: MutationConfiguration { probability: None, method: Mutation::Swap },
        survivor: Survivor::Fitness,
        log_level: None,
    };

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

    let individuals = vec![Genotype{dna: dna_1, fitness: 1.0, age: 0}, Genotype{dna: dna_2, fitness: 2.0, age: 0},
    Genotype{dna: dna_3, fitness: 3.0, age: 0}, Genotype{dna: dna_4, fitness: 4.0, age: 0}, Genotype{dna: dna_5, fitness: 5.0, age: 0}, 
    Genotype{dna: dna_6, fitness: 6.0, age: 0}, Genotype{dna: dna_7, fitness: 7.0, age: 0}, Genotype{dna: dna_8, fitness: 8.0, age: 0},
    Genotype{dna: dna_9, fitness: 9.0, age: 0}, Genotype{dna: dna_10, fitness: 10.0, age: 0}];

    let mut population = Population::new(individuals);
    population = run(population, configuration);
    
    assert_eq!(population.individuals.len(), 1);
    assert_eq!(population.individuals[0].get_fitness(), 20.0);

}

#[test]
fn test_ga_run_minimize(){

    //Creates the GA configuration
    let configuration = GaConfiguration{
        adaptive_ga: false,
        number_of_threads: None,
        limit_configuration: LimitConfiguration{max_generations: 100, fitness_target: None, problem_solving: ProblemSolving::Minimization, get_best_individual_by_generation: None},
        selection_configuration: SelectionConfiguration { number_of_couples: None, method: Selection::Random },
        crossover_configuration: CrossoverConfiguration{method: Crossover::Cycle, number_of_points: None, ..Default::default()},
        mutation_configuration: MutationConfiguration { probability: Some(0.2), method: Mutation::Swap },
        survivor: Survivor::Fitness,
        log_level: None,
    };

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

    let individuals = vec![Genotype{dna: dna_1, fitness: 1.0, age: 0}, Genotype{dna: dna_2, fitness: 2.0, age: 0},
    Genotype{dna: dna_3, fitness: 3.0, age: 0}, Genotype{dna: dna_4, fitness: 4.0, age: 0}, Genotype{dna: dna_5, fitness: 5.0, age: 0}, 
    Genotype{dna: dna_6, fitness: 6.0, age: 0}, Genotype{dna: dna_7, fitness: 7.0, age: 0}, Genotype{dna: dna_8, fitness: 8.0, age: 0},
    Genotype{dna: dna_9, fitness: 9.0, age: 0}, Genotype{dna: dna_10, fitness: 10.0, age: 0}];

    let mut population = Population::new(individuals);
    population = run(population, configuration);
    
    assert_eq!(population.individuals.len(), 1);
    assert_eq!(population.individuals[0].get_fitness(), 10.0);

}


#[test]
fn test_ga_run(){

    //Creates the GA configuration
    let configuration = GaConfiguration{
        adaptive_ga: false, 
        number_of_threads: Some(8),
        limit_configuration: LimitConfiguration{max_generations: 1000, fitness_target: None, problem_solving: ProblemSolving::Maximization, get_best_individual_by_generation: None},
        selection_configuration: SelectionConfiguration{number_of_couples: Some(10), method:Selection::Tournament},
        crossover_configuration: CrossoverConfiguration{method: Crossover::Cycle, number_of_points: None, ..Default::default()},
        mutation_configuration: MutationConfiguration{method: Mutation::Swap, probability: None},
        survivor: Survivor::Fitness,
        log_level: None,
    };

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

    let individuals = vec![Genotype{dna: dna_1, fitness: 1.0, age: 0}, Genotype{dna: dna_2, fitness: 2.0, age: 0},
    Genotype{dna: dna_3, fitness: 3.0, age: 0}, Genotype{dna: dna_4, fitness: 4.0, age: 0}, Genotype{dna: dna_5, fitness: 5.0, age: 0}, 
    Genotype{dna: dna_6, fitness: 6.0, age: 0}, Genotype{dna: dna_7, fitness: 7.0, age: 0}, Genotype{dna: dna_8, fitness: 8.0, age: 0},
    Genotype{dna: dna_9, fitness: 9.0, age: 0}, Genotype{dna: dna_10, fitness: 10.0, age: 0}];

    let mut population = Population::new(individuals);
    population = run(population, configuration);
    
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

    let population = ga::random_initialization::<Genotype>(alleles, POPULATION_SIZE, GENES_PER_INDIVIDUAL, NEEDS_UNIQUE_IDS, ALLELES_CAN_BE_REPEATED, NUMBER_OF_THREADS);

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

    let population = ga::random_initialization::<Genotype>(alleles, POPULATION_SIZE, GENES_PER_INDIVIDUAL, NEEDS_UNIQUE_IDS, ALLELES_CAN_BE_REPEATED, NUMBER_OF_THREADS);

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