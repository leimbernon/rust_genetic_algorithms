use genetic_algorithms::{ga::{GaConfiguration, ProblemSolving, start}, operations::{Selection, Crossover, Mutation, Survivor}, population::Population};
use crate::{structures::{Gene, Genotype}};

#[test]
fn test_ga_start(){

    //Creates the GA configuration
    let configuration = GaConfiguration{
        problem_solving: ProblemSolving::Minimization,
        max_generations: 5,
        selection: Selection::Random,
        crossover: Crossover::Cycle,
        mutation: Mutation::Swap,
        survivor: Survivor::Fitness,
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

    let individuals = vec![Genotype{dna: dna_1, phenotype: 1.0}, Genotype{dna: dna_2, phenotype: 2.0},
    Genotype{dna: dna_3, phenotype: 3.0}, Genotype{dna: dna_4, phenotype: 4.0}, Genotype{dna: dna_5, phenotype: 5.0}, 
    Genotype{dna: dna_6, phenotype: 6.0}, Genotype{dna: dna_7, phenotype: 7.0}, Genotype{dna: dna_8, phenotype: 8.0},
    Genotype{dna: dna_9, phenotype: 9.0}, Genotype{dna: dna_10, phenotype: 10.0}];

    let population = Population::new(individuals);

    start(population, configuration);
    
    assert_eq!(true, true);

}