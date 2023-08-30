pub(crate) use rand::Rng;
use crate::traits::GenotypeT;

pub fn scramble<U: GenotypeT>(individual: &mut U){

    //Getting two random genes from the dna of the individual
    let mut rng = rand::thread_rng();
    let index_1 = rng.gen_range(0..individual.get_dna().len());
    let index_2 = rng.gen_range(index_1+1..individual.get_dna().len());

    //We scramble genes
    for i in index_1..index_2{
        let random_index = rng.gen_range(index_1..index_2);

        let current_gene = *individual.get_dna().get(i).unwrap();
        let random_gene = *individual.get_dna().get(random_index).unwrap();

        individual.set_gene(i, random_gene);
        individual.set_gene(random_index, current_gene);
    }
}