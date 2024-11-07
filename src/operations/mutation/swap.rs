pub(crate) use rand::Rng;
use crate::traits::ChromosomeT;
use log::{trace, debug};

pub fn swap<U: ChromosomeT>(individual: &mut U){

    //Getting two random genes from the dna of the individual
    debug!(target="mutation_events", method="swap"; "Starting the swap mutation");
    let mut rng = rand::thread_rng();
    let index_1 = rng.gen_range(0..individual.get_dna().len());
    let index_2 = rng.gen_range(0..individual.get_dna().len());
    trace!(target="mutation_events", method="swap"; "Mutation index 1: {}, mutation index 2: {}", index_1, index_2);

    let gene_1 = individual.get_dna().get(index_1).cloned().unwrap();
    let gene_2 = individual.get_dna().get(index_2).cloned().unwrap();

    //Swapping both genes
    individual.set_gene(index_1, gene_2);
    individual.set_gene(index_2, gene_1);
    debug!(target="mutation_events", method="swap"; "Swap mutation finished");
}