pub(crate) use rand::Rng;
use crate::traits::GenotypeT;

pub fn swap<U: GenotypeT>(individual: &mut U){

    //Getting two random genes from the dna of the individual
    let mut rng = rand::thread_rng();
    let index_1 = rng.gen_range(0..individual.get_dna().len());
    let index_2 = rng.gen_range(0..individual.get_dna().len());

    let gene_1 = individual.get_dna().get(index_1).copied().unwrap();
    let gene_2 = individual.get_dna().get(index_2).copied().unwrap();

    //Swapping both genes
    individual.set_gene(index_1, gene_2);
    individual.set_gene(index_2, gene_1);
}