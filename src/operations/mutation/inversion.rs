pub(crate) use crate::traits::{GeneT, GenotypeT};
use rand::Rng;

pub fn inversion<T: GeneT, U: GenotypeT<T>>(individual: &mut U){

    //Getting two random genes from the dna of the individual
    let mut rng = rand::thread_rng();
    let index_1: usize = rng.gen_range(0..individual.get_dna().len());
    let index_2: usize = rng.gen_range(0..individual.get_dna().len());
    let lower_index: usize;
    let higher_index: usize;

    //We create the indexes that we want extract
    if index_1 < index_2 {
        lower_index = index_1;
        higher_index = index_2;
    }else if index_1 > index_2 {
        lower_index = index_2;
        higher_index = index_1;
    }else{
        return;
    }

    //Changes the dna
    for i in 0..(higher_index - lower_index) - 1{

        //Gets the gene
        let gene_end = individual.get_dna().get(higher_index - i).copied().unwrap();
        let gene_start = individual.get_dna().get(lower_index + i).copied().unwrap();

        //Switches both genes
        individual.get_dna_mut()[lower_index + i] = gene_end;
        individual.get_dna_mut()[higher_index - i] = gene_start;
    }
}