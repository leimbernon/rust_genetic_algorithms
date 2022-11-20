use crate::structures::GeneT;
use crate::structures::GenotypeT;
use std::collections::HashMap;
use rand::Rng;

/**
 * Function to make the random mating between the list of individuals
 */
pub fn random_mating<T:GeneT, U:GenotypeT<T>>(individuals: &Vec<U>) -> HashMap<usize, usize>{

    let mut mating = HashMap::new();
    let mut indexes = Vec::new();
    let mut rng = rand::thread_rng();

    //Setting the indexes of the individuals
    let mut i = 0;
    while i < individuals.len() {
        indexes.push(i);
        i = i + 1;
    }

    //In this loop we create the mating map
    while indexes.len() > 0 {

        //Getting the individual 1
        //We must have at least 2 remaining elements
        if indexes.len() < 2 {
            break;
        }
        let mut random_index_1 = 0;
        if indexes.len() > 1 {
            random_index_1 = rng.gen_range(0..indexes.len()-1);
        }
        let index_value_1 = indexes[random_index_1];
        mating.insert(index_value_1, 0);
        indexes.remove(random_index_1);
        
        //Getting the individual 2
        let mut random_index_2 = 0;
        if indexes.len() > 1 {
            random_index_2 = rng.gen_range(0..indexes.len()-1);
        }

        //Adding the two individuals in the hashmap
        mating.insert(index_value_1, indexes[random_index_2]);
        indexes.remove(random_index_2);
    }

    mating
}