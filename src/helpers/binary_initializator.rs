use rand::Rng;
use crate::chromosomes::Binary;
use crate::genotypes::Binary as BinaryGenotype;
use crate::traits::GeneT;

pub fn binary_random_initialization(genes_per_chromosome: i32) -> Binary {
    let mut chromosome = Binary::new();
    let mut rng = rand::thread_rng();
    for i in 0..genes_per_chromosome {
        let gene = BinaryGenotype{id:i, value: rng.gen_bool(0.5)};
        chromosome.dna.push(gene);
    }
    chromosome
}