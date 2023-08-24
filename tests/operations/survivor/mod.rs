use crate::structures::{Gene, Genotype};
use genetic_algorithms::{operations::survivor::{fitness, age}, traits::GenotypeT, configuration::{ProblemSolving, LimitConfiguration}};

#[test]
fn test_fitness_survivor_minization(){

    //We create 12 fitnesss for 12 individuals
    let dna_1 = vec![Gene{id:1}];
    let dna_2 = vec![Gene{id:1}];
    let dna_3 = vec![Gene{id:1}];
    let dna_4 = vec![Gene{id:1}];
    let dna_5 = vec![Gene{id:1}];
    let dna_6 = vec![Gene{id:1}];
    let dna_7 = vec![Gene{id:1}];
    let dna_8 = vec![Gene{id:1}];
    let dna_9 = vec![Gene{id:1}];
    let dna_10 = vec![Gene{id:1}];
    let dna_11 = vec![Gene{id:1}];
    let dna_12 = vec![Gene{id:1}];

    //We create the individuals
    let individual_1 = Genotype{dna: dna_1, fitness: 10.1, age: 0};
    let individual_2 = Genotype{dna: dna_2, fitness: 10.2, age: 0};
    let individual_3 = Genotype{dna: dna_3, fitness: 10.3, age: 0};
    let individual_4 = Genotype{dna: dna_4, fitness: 11.0, age: 0};
    let individual_5 = Genotype{dna: dna_5, fitness: 12.0, age: 0};
    let individual_6 = Genotype{dna: dna_6, fitness: 13.0, age: 0};
    let individual_7 = Genotype{dna: dna_7, fitness: 14.0, age: 0};
    let individual_8 = Genotype{dna: dna_8, fitness: 15.0, age: 0};
    let individual_9 = Genotype{dna: dna_9, fitness: 16.0, age: 0};
    let individual_10 = Genotype{dna: dna_10, fitness: 17.0, age: 0};
    let individual_11 = Genotype{dna: dna_11, fitness: 18.0, age: 0};
    let individual_12 = Genotype{dna: dna_12, fitness: 19.0, age: 0};


    //We create the population and create the random mating
    let mut population = vec![individual_1, individual_2, individual_3, individual_4, individual_5, individual_6,
    individual_7, individual_8, individual_9, individual_10, individual_11, individual_12];

    fitness::fitness_based(&mut population, 10,  LimitConfiguration{problem_solving: ProblemSolving::Minimization, max_generations: 100, fitness_target: None, get_best_individual_by_generation: None});

    //Tests that the population has 10 individuals
    assert_eq!(population.len(), 10);
    assert_eq!(population[0].get_fitness(), 17.0);
    assert_eq!(population[9].get_fitness(), 10.1);

}


#[test]
fn test_fitness_survivor_maximization(){

    //We create 12 fitnesss for 12 individuals
    let dna_1 = vec![Gene{id:1}];
    let dna_2 = vec![Gene{id:1}];
    let dna_3 = vec![Gene{id:1}];
    let dna_4 = vec![Gene{id:1}];
    let dna_5 = vec![Gene{id:1}];
    let dna_6 = vec![Gene{id:1}];
    let dna_7 = vec![Gene{id:1}];
    let dna_8 = vec![Gene{id:1}];
    let dna_9 = vec![Gene{id:1}];
    let dna_10 = vec![Gene{id:1}];
    let dna_11 = vec![Gene{id:1}];
    let dna_12 = vec![Gene{id:1}];

    //We create the individuals
    let individual_1 = Genotype{dna: dna_1, fitness: 10.1, age: 0};
    let individual_2 = Genotype{dna: dna_2, fitness: 10.2, age: 0};
    let individual_3 = Genotype{dna: dna_3, fitness: 10.3, age: 0};
    let individual_4 = Genotype{dna: dna_4, fitness: 11.0, age: 0};
    let individual_5 = Genotype{dna: dna_5, fitness: 12.0, age: 0};
    let individual_6 = Genotype{dna: dna_6, fitness: 13.0, age: 0};
    let individual_7 = Genotype{dna: dna_7, fitness: 14.0, age: 0};
    let individual_8 = Genotype{dna: dna_8, fitness: 15.0, age: 0};
    let individual_9 = Genotype{dna: dna_9, fitness: 16.0, age: 0};
    let individual_10 = Genotype{dna: dna_10, fitness: 17.0, age: 0};
    let individual_11 = Genotype{dna: dna_11, fitness: 18.0, age: 0};
    let individual_12 = Genotype{dna: dna_12, fitness: 19.0, age: 0};


    //We create the population and create the random mating
    let mut population = vec![individual_1, individual_2, individual_3, individual_4, individual_5, individual_6,
    individual_7, individual_8, individual_9, individual_10, individual_11, individual_12];

    fitness::fitness_based(&mut population, 10, LimitConfiguration{problem_solving: ProblemSolving::Maximization, max_generations: 100, fitness_target: None, get_best_individual_by_generation: None});

    //Tests that the population has 10 individuals
    assert_eq!(population.len(), 10);
    assert_eq!(population[0].get_fitness(), 19.0);
    assert_eq!(population[9].get_fitness(), 10.3);

}


#[test]
fn test_age_based_survivor(){

    //We create 12 fitnesss for 12 individuals
    let dna_1 = vec![Gene{id:1}];
    let dna_2 = vec![Gene{id:1}];
    let dna_3 = vec![Gene{id:1}];
    let dna_4 = vec![Gene{id:1}];
    let dna_5 = vec![Gene{id:1}];
    let dna_6 = vec![Gene{id:1}];
    let dna_7 = vec![Gene{id:1}];
    let dna_8 = vec![Gene{id:1}];
    let dna_9 = vec![Gene{id:1}];
    let dna_10 = vec![Gene{id:1}];
    let dna_11 = vec![Gene{id:1}];
    let dna_12 = vec![Gene{id:1}];

    //We create the individuals
    let individual_1 = Genotype{dna: dna_1, fitness: 10.1, age: 0};
    let individual_2 = Genotype{dna: dna_2, fitness: 10.2, age: 0};
    let individual_3 = Genotype{dna: dna_3, fitness: 10.3, age: 1};
    let individual_4 = Genotype{dna: dna_4, fitness: 11.0, age: 1};
    let individual_5 = Genotype{dna: dna_5, fitness: 12.0, age: 3};
    let individual_6 = Genotype{dna: dna_6, fitness: 13.0, age: 3};
    let individual_7 = Genotype{dna: dna_7, fitness: 14.0, age: 2};
    let individual_8 = Genotype{dna: dna_8, fitness: 15.0, age: 2};
    let individual_9 = Genotype{dna: dna_9, fitness: 16.0, age: 2};
    let individual_10 = Genotype{dna: dna_10, fitness: 17.0, age: 2};
    let individual_11 = Genotype{dna: dna_11, fitness: 18.0, age: 1};
    let individual_12 = Genotype{dna: dna_12, fitness: 19.0, age: 1};


    //We create the population and create the random mating
    let mut population = vec![individual_1, individual_2, individual_3, individual_4, individual_5, individual_6,
    individual_7, individual_8, individual_9, individual_10, individual_11, individual_12];

    age::age_based(&mut population, 6);

    //Tests that the population has 6 individuals
    assert_eq!(population.len(), 6);
    assert_eq!(population[0].get_age(), 3);
    assert_eq!(population[1].get_age(), 3);
    assert_eq!(population[2].get_age(), 2);
    assert_eq!(population[3].get_age(), 2);
    assert_eq!(population[4].get_age(), 2);
    assert_eq!(population[5].get_age(), 2);

}


#[test]
fn test_survivor_fitness_fixed(){

    //We create 12 fitnesss for 12 individuals
    let dna_1 = vec![Gene{id:1}];
    let dna_2 = vec![Gene{id:1}];
    let dna_3 = vec![Gene{id:1}];
    let dna_4 = vec![Gene{id:1}];
    let dna_5 = vec![Gene{id:1}];
    let dna_6 = vec![Gene{id:1}];
    let dna_7 = vec![Gene{id:1}];
    let dna_8 = vec![Gene{id:1}];
    let dna_9 = vec![Gene{id:1}];
    let dna_10 = vec![Gene{id:1}];
    let dna_11 = vec![Gene{id:1}];
    let dna_12 = vec![Gene{id:1}];

    //We create the individuals
    let individual_1 = Genotype{dna: dna_1, fitness: 10.1, age: 0};
    let individual_2 = Genotype{dna: dna_2, fitness: 10.2, age: 0};
    let individual_3 = Genotype{dna: dna_3, fitness: 10.3, age: 0};
    let individual_4 = Genotype{dna: dna_4, fitness: 11.0, age: 0};
    let individual_5 = Genotype{dna: dna_5, fitness: 12.0, age: 0};
    let individual_6 = Genotype{dna: dna_6, fitness: 13.0, age: 0};
    let individual_7 = Genotype{dna: dna_7, fitness: 14.0, age: 0};
    let individual_8 = Genotype{dna: dna_8, fitness: 15.0, age: 0};
    let individual_9 = Genotype{dna: dna_9, fitness: 16.0, age: 0};
    let individual_10 = Genotype{dna: dna_10, fitness: 17.0, age: 0};
    let individual_11 = Genotype{dna: dna_11, fitness: 18.0, age: 0};
    let individual_12 = Genotype{dna: dna_12, fitness: 19.0, age: 0};


    //We create the population and create the random mating
    let mut population = vec![individual_1, individual_2, individual_3, individual_4, individual_5, individual_6,
    individual_7, individual_8, individual_9, individual_10, individual_11, individual_12];

    fitness::fitness_based(&mut population, 10,  LimitConfiguration{problem_solving: ProblemSolving::FixedFitness, max_generations: 100, fitness_target: Some(14.5), get_best_individual_by_generation: None});

    //Tests that the population has 10 individuals
    assert_eq!(population.len(), 10);
    assert_eq!(population[0].get_fitness(), 10.2);
    assert_eq!(population[9].get_fitness(), 15.0);

}