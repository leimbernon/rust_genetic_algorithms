use genetic_algorithms::{ga::run, operations::{Selection, Crossover, Mutation, Survivor}, population::Population, traits::GenotypeT, configuration::{GaConfiguration, ProblemSolving, LimitConfiguration, SelectionConfiguration, MutationConfiguration, CrossoverConfiguration, LogLevel}};
use crate::structures::{Gene, Genotype};
extern crate num_cpus;

#[test]
fn test_ga_start_maximize(){

    //Creates the GA configuration
    let configuration = GaConfiguration{
        number_of_threads: None,
        limit_configuration: LimitConfiguration{max_generations: 100, fitness_target: None, problem_solving: ProblemSolving::Maximization, get_best_individual_by_generation: None},
        selection_configuration: SelectionConfiguration { number_of_couples: None, method: Selection::Random },
        crossover_configuration: CrossoverConfiguration{probability:Some(1.0), method: Crossover::Cycle, number_of_points: None},
        mutation_configuration: MutationConfiguration { probability: None, method: Mutation::Swap },
        survivor: Survivor::Fitness,
        log_level: Some(LogLevel::Info),
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
        number_of_threads: None,
        limit_configuration: LimitConfiguration{max_generations: 100, fitness_target: None, problem_solving: ProblemSolving::Minimization, get_best_individual_by_generation: None},
        selection_configuration: SelectionConfiguration { number_of_couples: None, method: Selection::Random },
        crossover_configuration: CrossoverConfiguration{probability:Some(1.0), method: Crossover::Cycle, number_of_points: None},
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
fn test_ga_run_multithread(){

    //Creates the GA configuration
    let configuration = GaConfiguration{
        number_of_threads: Some(8),
        limit_configuration: LimitConfiguration{max_generations: 1000, fitness_target: None, problem_solving: ProblemSolving::Maximization, get_best_individual_by_generation: None},
        selection_configuration: SelectionConfiguration{number_of_couples: Some(10), method:Selection::Tournament},
        crossover_configuration: CrossoverConfiguration{probability:Some(1.0), method: Crossover::Cycle, number_of_points: None},
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