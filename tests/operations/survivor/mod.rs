use crate::structures::{Gene, Genotype};
use genetic_algorithms::operations::{survivor::fitness};

#[test]
fn test_fitness_survivor(){

    //We create 12 phenotypes for 12 individuals
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
    let individual_1 = Genotype{dna: dna_1, phenotype: 10.1};
    let individual_2 = Genotype{dna: dna_2, phenotype: 10.2};
    let individual_3 = Genotype{dna: dna_3, phenotype: 10.3};
    let individual_4 = Genotype{dna: dna_4, phenotype: 11.0};
    let individual_5 = Genotype{dna: dna_5, phenotype: 12.0};
    let individual_6 = Genotype{dna: dna_6, phenotype: 13.0};
    let individual_7 = Genotype{dna: dna_7, phenotype: 14.0};
    let individual_8 = Genotype{dna: dna_8, phenotype: 15.0};
    let individual_9 = Genotype{dna: dna_9, phenotype: 16.0};
    let individual_10 = Genotype{dna: dna_10, phenotype: 17.0};
    let individual_11 = Genotype{dna: dna_11, phenotype: 18.0};
    let individual_12 = Genotype{dna: dna_12, phenotype: 19.0};


    //We create the population and create the random mating
    let population = vec![individual_1, individual_2, individual_3, individual_4, individual_5, individual_6,
    individual_7, individual_8, individual_9, individual_10, individual_11, individual_12];

    let population = fitness::fitness_based(population, 10);

    assert_eq!(population.len(), 10);

}