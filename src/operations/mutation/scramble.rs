pub(crate) use rand::Rng;
use crate::traits::GenotypeT;
use log::{trace, debug};

pub fn scramble<U: GenotypeT>(individual: &mut U){

    //Getting two random genes from the dna of the individual
    debug!(target="mutation_events", method="scramble"; "Starting the scramble mutation");
    let mut rng = rand::thread_rng();
    let index_1 = rng.gen_range(0..individual.get_dna().len()-1);
    let index_2 = rng.gen_range(index_1+1..individual.get_dna().len());
    trace!(target="mutation_events", method="scramble"; "Mutation index 1: {}, mutation index 2: {}", index_1, index_2);

    //We scramble genes
    for i in index_1..index_2{
        let random_index = rng.gen_range(index_1..index_2);

        let current_gene = individual.get_dna().get(i).cloned().unwrap();
        let random_gene = individual.get_dna().get(random_index).cloned().unwrap();

        individual.set_gene(i, random_gene);
        individual.set_gene(random_index, current_gene);
    }

    debug!(target="mutation_events", method="scramble"; "Scramble mutation finished");
}