use rand::Rng;
use crate::traits::GenotypeT;
use crate::traits::GeneT;

pub fn scramble<T: GeneT, U: GenotypeT<T>>(individual: &mut U){

    //Getting two random genes from the dna of the individual
    let mut rng = rand::thread_rng();
    let index_1 = rng.gen_range(0..individual.get_dna().len());
    let index_2 = rng.gen_range(index_1+1..individual.get_dna().len());

    //We scramble genes
    for i in index_1..index_2{
        let random_index = rng.gen_range(index_1..index_2);

        let current_gene = *individual.get_dna().get(i).unwrap();
        let random_gene = *individual.get_dna().get(random_index).unwrap();

        individual.get_dna_mut()[i] = random_gene;
        individual.get_dna_mut()[random_index] = current_gene;
    }
}