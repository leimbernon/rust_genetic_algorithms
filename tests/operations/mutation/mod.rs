use crate::structures::{Gene, Genotype};
use genetic_algorithms::operations::{mutation::swap};

#[test]
fn test_swap_mutation(){
    
    //We create 1 dna for 1 individuals
    let dna_1 = vec![Gene{id:1}, Gene{id:2}, Gene{id:3}, Gene{id:4}, Gene{id:5}, Gene{id:6}];

    //We create the individuals
    let mut individual_1 = Genotype{dna: dna_1, phenotype: 0.0};
    let individual_1_copy = individual_1.clone();

    //We mutate the dna
    swap::swap(&mut individual_1);
    assert_ne!(individual_1, individual_1_copy);
}