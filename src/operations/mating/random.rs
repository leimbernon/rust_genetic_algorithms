use crate::structures::gene::GeneT;
use crate::structures::genotype::Genotype;
use std::collections::HashMap;
use rand::Rng;

/**
 * Function to make the random mating between the list of individuals
 */
fn random_mating<T: GeneT>(individuals: &Vec<Genotype<T>>) -> HashMap<usize, usize>{

    let mut mating = HashMap::new();
    let mut indexes = Vec::from([..individuals.len()-1]);
    let mut rng = rand::thread_rng();

    while indexes.len() > 0 {

        //Getting the individual 1
        let random_index_1 = rng.gen_range(0..indexes.len()-1);
        indexes.remove(random_index_1);
        
        //Getting the individual 2
        let random_index_2 = rng.gen_range(0..indexes.len()-1);
        indexes.remove(random_index_2);

        //Adding the two individuals in the hashmap
        mating.insert(random_index_1, random_index_2);
    }

    mating
}