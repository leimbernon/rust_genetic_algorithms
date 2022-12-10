use crate::traits::GeneT;
use crate::traits::GenotypeT;
use std::collections::HashMap;
use rand::Rng;

pub fn roulette_wheel_selection<T:GeneT, U:GenotypeT<T>>(individuals: &Vec<U>) -> HashMap<usize, usize>{

    let mut mating = HashMap::new();

    //1- Calculate the sum of all fitnesses
    let mut total_phenotype = 0.0;
    let mut rng = rand::thread_rng();

    for genotype in individuals 
    { 
        total_phenotype += genotype.get_phenotype(); 
    }

    //2- Identifies what individuals will be parents
    let mut parent_1 = None;

    for index in  0..individuals.len(){

        //We get the probability
        if &rng.gen_range(0.0..total_phenotype) >= individuals.get(index).unwrap().get_phenotype(){

            if parent_1 == None {
                //If parent 1 is not set, we set it
                parent_1 = Some(index);
            }else{
                //If parent 1 is set, we set the mating
                mating.insert(parent_1.unwrap(), index);
                parent_1 = None;
            }

        }
    }

    return mating;
}