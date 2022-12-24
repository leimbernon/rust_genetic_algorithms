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


pub fn stochastic_universal_sampling<T:GeneT, U:GenotypeT<T>>(individuals: &Vec<U>, couples: i32) -> HashMap<usize, usize>{
    
    let mut mating = HashMap::new();
    let individual_couples = (couples*2) as usize;

    //1- Calculate the selection probabilities
    let mut total = 0.0;
    let mut last_selection_value = 0.0;
    let mut selection_probabilities = Vec::new();
    let mut rng = rand::thread_rng();

    for genotype in individuals{ 
        total += *genotype.get_phenotype();
    }
    for genotype in individuals{
        let selection_probability = (*genotype.get_phenotype() / total) + last_selection_value;
        last_selection_value = selection_probability;
        selection_probabilities.push(selection_probability);
    }

    //2- Calculate the pointer distance and the starting point between 0 and the pointer distance
    let pointer_distance = 1.0 / individual_couples as f64;
    let starting_point = rng.gen_range(0.0..pointer_distance);

    //3- Parent identification
    let mut current_point = starting_point;
    let mut next_individual = 1;

    let mut end_of_individuals = false;
    let mut couple_completed = false;
    let mut first_mate = 0;

    for i in 0..individual_couples{
       
        //We check that there are enough individuals
        if i >= individuals.len(){
            break;
        }else if next_individual >= individuals.len(){
            end_of_individuals = true;
        }

        //We check if the pointer is between the current and the next individual
        if !end_of_individuals && current_point >= selection_probabilities[i] as f64 && 
            current_point < selection_probabilities[next_individual] as f64 {

            if couple_completed {
                mating.insert(first_mate, i);
            }else{
                first_mate = i;
            }

            couple_completed = !couple_completed;

        } else if end_of_individuals && current_point >= selection_probabilities[i] as f64 {
            if couple_completed {
                mating.insert(first_mate, i);
                couple_completed = !couple_completed;
            }else{
                first_mate = i;
            }
        }

        current_point = current_point + pointer_distance;
        next_individual += 1;
    }

    return mating;
}