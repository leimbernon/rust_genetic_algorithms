#[cfg(test)]
mod structures;
use genetic_algorithms::helpers;
use crate::structures::{Gene, Genotype};

#[test]
fn test_helpers_initialize_dna(){

    let binding =  vec![Gene{id:1}, Gene{id:2}, Gene{id:3}, Gene{id:4},
                                   Gene{id:5}, Gene{id:6}, Gene{id:7}, Gene{id:8}];
    let alleles = binding.as_slice();

    let genes = helpers::initialize_dna::<Genotype>(alleles, 4, false);
    assert_eq!(genes.len(), 4);
}

#[test]
fn test_initialize_dna_without_repeated_alleles(){

    let binding =  vec![Gene{id:1}, Gene{id:2}, Gene{id:3}, Gene{id:4},
                                   Gene{id:5}, Gene{id:6}, Gene{id:7}, Gene{id:8}];
    let alleles = binding.as_slice();
    let genes = helpers::initialize_dna_without_repeated_alleles::<Genotype>(alleles, 6, false);

    //Checks that any allele is repeated
    let mut alleles_ids = Vec::new();
    
    for gene in genes {
        if !alleles_ids.is_empty(){
            assert!(!alleles_ids.contains(&gene.id));
        }
        alleles_ids.push(gene.id);
    }

}