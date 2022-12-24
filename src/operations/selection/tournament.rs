use crate::traits::GeneT;
use crate::traits::GenotypeT;
use std::collections::HashMap;
use rand::Rng;

pub fn tournament<T:GeneT, U:GenotypeT<T>>(individuals: &Vec<U>, couples: i32) -> HashMap<usize, usize>{

    let mut rng = rand::thread_rng();
    let mut mating = HashMap::new();
    let individual_couples = couples*2;

    let mut indexes = Vec::new();

    //Getting the list of indexes
    for i in 0..individual_couples{
        indexes.push(i);
    }

    //Getting the parents from the population
    let mut parent_1 = None;

    for _i in 0..indexes.len(){
        let index_1 = rng.gen_range(0..indexes.len());
        let index_2 = rng.gen_range(0..indexes.len());
        let final_index;

        //Fights between both parents
        if individuals[index_1].get_phenotype() >= individuals[index_2].get_phenotype() {
            final_index = index_1;
        }else{
            final_index = index_2;
        }

        //Sets the mating
        if parent_1 == None {
            parent_1 = Some(final_index);
        }else{
            mating.insert(parent_1.unwrap(), final_index);
            parent_1 = None;
        }

        indexes.remove(final_index);
    }

    return mating;
}