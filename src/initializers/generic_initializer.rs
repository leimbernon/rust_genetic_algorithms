use rand::Rng;
use crate::traits::{ChromosomeT, GeneT};

/**
 * Function to initialize the dna of an individual
 */
pub fn generic_random_initialization<U>(alleles: &[U::Gene], genes_per_individual: i32, needs_unique_ids: bool)->Vec<U::Gene>
where
    U: ChromosomeT + Send + Sync + 'static + Clone{

    let mut rng = rand::thread_rng();
    let mut dna = Vec::new();

    //Selects the genes randomly from the vector without repeating them
    for j in 0..genes_per_individual{
        let index = rng.gen_range(0..alleles.len());
        let mut gene = alleles.get(index).cloned().unwrap();

        //If we need unique ids
        if needs_unique_ids {
            gene.set_id(j);
        }

        dna.push(gene);
    }

    dna
}


/**
 * Function to initialize the dna of an individual without repeating an array of alleles
 */
pub fn generic_random_initialization_without_repetitions<U>(alleles: &[U::Gene], genes_per_individual: i32, needs_unique_ids: bool)->Vec<U::Gene>
where
    U: ChromosomeT + Send + Sync + 'static + Clone{

    let mut rng = rand::thread_rng();
    let mut dna = Vec::new();

    let mut tmp_alleles = alleles.to_vec().clone();

    //Selects the genes randomly from the vector without repeating them
    for j in 0..genes_per_individual{
        let index = rng.gen_range(0..tmp_alleles.len());
        let mut gene = tmp_alleles.get(index).cloned().unwrap();

        //If we need unique ids
        if needs_unique_ids {
            gene.set_id(j);
        }

        tmp_alleles.remove(index);

        dna.push(gene);
    }

    dna
}