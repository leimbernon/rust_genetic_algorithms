use crate::traits::GenotypeT;
use std::collections::HashMap;
use rand::Rng;
use log::{trace, debug};

/**
 * Function to make the random parent selection between the list of individuals
 */
pub fn random<U:GenotypeT>(individuals: &Vec<U>) -> HashMap<usize, usize>{

    let mut mating = HashMap::new();
    let mut indexes = Vec::new();
    let mut rng = rand::thread_rng();
    debug!(target="selection_events", method="random"; "Starting random selection");

    //Setting the indexes of the individuals
    let mut i = 0;
    while i < individuals.len() {
        indexes.push(i);
        i += 1;
    }

    //In this loop we create the mating map
    while !indexes.is_empty() {

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

        trace!(target="selection_events", method="random"; "Mating index 1 {} with index 2 {}", index_value_1, random_index_2);
    }

    mating
}