pub(crate) use rand::Rng;
use crate::traits::GenotypeT;
use crate::traits::GeneT;

pub fn swap<T: GeneT, U: GenotypeT<T>>(individual: &mut U){

    //Getting two random genes from the dna of the individual
    let mut rng = rand::thread_rng();
    let index_1 = rng.gen_range(0..individual.get_dna().len());
    let index_2 = rng.gen_range(0..individual.get_dna().len());

    let gene_1 = individual.get_dna().get(index_1).copied().unwrap();
    let gene_2 = individual.get_dna().get(index_2).copied().unwrap();

    //Swapping both genes
    individual.get_dna_mut()[index_1] = gene_2;
    individual.get_dna_mut()[index_2] = gene_1;
}