use std::cmp::Ordering;
pub(crate) use crate::traits::GenotypeT;
use rand::Rng;
use log::{trace, debug};

pub fn inversion<U: GenotypeT>(individual: &mut U){

    //Getting two random genes from the dna of the individual
    debug!(target="mutation_events", method="inversion"; "Starting the inversion mutation");
    let mut rng = rand::thread_rng();
    let index_1: usize = rng.gen_range(0..individual.get_dna().len());
    let index_2: usize = rng.gen_range(0..individual.get_dna().len());
    let mut lower_index: usize = 0;
    let mut higher_index: usize = 0;
    trace!(target="mutation_events", method="inversion"; "Mutation index 1: {}, mutation index 2: {}", index_1, index_2);

    //We create the indexes that we want extract
    match index_1.cmp(&index_2) {
        Ordering::Greater => {
            lower_index = index_2; 
            higher_index = index_1;
        },
        Ordering::Less => {
            lower_index = index_1;
            higher_index = index_2;
        },
        Ordering::Equal => {}
    }

    //Changes the dna
    for i in 0..(higher_index - lower_index) - 1{

        //Gets the gene
        let gene_end = individual.get_dna().get(higher_index - i).cloned().unwrap();
        let gene_start = individual.get_dna().get(lower_index + i).cloned().unwrap();

        //Switches both genes
        individual.set_gene(lower_index + i, gene_end);
        individual.set_gene(higher_index - i, gene_start);
    }

    debug!(target="mutation_events", method="inversion"; "Inversion mutation finished");
}