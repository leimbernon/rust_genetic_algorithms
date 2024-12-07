#[cfg(test)]
mod structures;
use genetic_algorithms::initializers::{generic_random_initialization, generic_random_initialization_without_repetitions};
use crate::structures::{Gene, Chromosome};

#[test]
fn test_initializers_generic_random_initialization(){

    let binding =  vec![Gene{id:1}, Gene{id:2}, Gene{id:3}, Gene{id:4},
                                   Gene{id:5}, Gene{id:6}, Gene{id:7}, Gene{id:8}];
    let alleles = binding.as_slice();

    let genes = generic_random_initialization::<Chromosome>(alleles, 4, false);
    assert_eq!(genes.len(), 4);
}

#[test]
fn test_initializers_generic_random_initialization_without_repetitions(){

    let binding =  vec![Gene{id:1}, Gene{id:2}, Gene{id:3}, Gene{id:4},
                                   Gene{id:5}, Gene{id:6}, Gene{id:7}, Gene{id:8}];
    let alleles = binding.as_slice();
    let genes = generic_random_initialization_without_repetitions::<Chromosome>(alleles, 6, false);

    //Checks that any allele is repeated
    let mut alleles_ids = Vec::new();
    
    for gene in genes {
        if !alleles_ids.is_empty(){
            assert!(!alleles_ids.contains(&gene.id));
        }
        alleles_ids.push(gene.id);
    }

}