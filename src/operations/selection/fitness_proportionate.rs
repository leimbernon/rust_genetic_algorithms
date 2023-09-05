use crate::traits::GenotypeT;
use std::collections::HashMap;
use rand::Rng;
use log::{trace, debug};

pub fn roulette_wheel_selection<U:GenotypeT>(individuals: &Vec<U>) -> HashMap<usize, usize>{

    let mut mating = HashMap::new();

    //1- Calculate the sum of all fitnesses
    debug!(target="selection_events", method="roulette_wheel_selection"; "Starting the roulette wheel selection");
    let mut total_fitness = 0.0;
    let mut rng = rand::thread_rng();

    for genotype in individuals 
    { 
        total_fitness += genotype.get_fitness(); 
    }
    trace!(target="selection_events", method="roulette_wheel_selection"; "Total fitness: {}", total_fitness);

    //2- Identifies what individuals will be parents
    let mut parent_1 = None;

    for index in  0..individuals.len(){

        //We get the probability
        if rng.gen_range(0.0..total_fitness) >= individuals.get(index).unwrap().get_fitness(){

            if parent_1.is_none() {
                //If parent 1 is not set, we set it
                parent_1 = Some(index);
            }else{
                //If parent 1 is set, we set the mating
                mating.insert(parent_1.unwrap(), index);
                parent_1 = None;
            }

        }
    }

    debug!(target="selection_events", method="roulette_wheel_selection"; "Roulette wheel selection finished");
    mating
}


pub fn stochastic_universal_sampling<U:GenotypeT>(individuals: &Vec<U>, couples: i32) -> HashMap<usize, usize>{
    
    debug!(target="selection_events", method="stochastic_universal_sampling"; "Starting the stochastic universal sampling selection");
    let mut mating = HashMap::new();
    let individual_couples = (couples*2) as usize;
    trace!(target="selection_events", method="stochastic_universal_sampling"; "Individual couples: {}", individual_couples);

    //1- Calculate the selection probabilities
    let mut total = 0.0;
    let mut last_selection_value = 0.0;
    let mut selection_probabilities = Vec::new();
    let mut rng = rand::thread_rng();

    for genotype in individuals{ 
        total += genotype.get_fitness();
    }
    trace!(target="selection_events", method="stochastic_universal_sampling"; "Total fitness: {}", total);
    for genotype in individuals{
        let selection_probability = (genotype.get_fitness() / total) + last_selection_value;
        last_selection_value = selection_probability;
        selection_probabilities.push(selection_probability);
        trace!(target="selection_events", method="stochastic_universal_sampling"; "Selection probability {}", selection_probability);
    }

    //2- Calculate the pointer distance and the starting point between 0 and the pointer distance
    let pointer_distance = 1.0 / individual_couples as f64;
    let starting_point = rng.gen_range(0.0..pointer_distance);
    trace!(target="selection_events", method="stochastic_universal_sampling"; "pointer distance {} - starting point {}", pointer_distance, starting_point);

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
        if !end_of_individuals && current_point >= selection_probabilities[i] && 
            current_point < selection_probabilities[next_individual] {

            if couple_completed {
                mating.insert(first_mate, i);
            }else{
                first_mate = i;
            }

            couple_completed = !couple_completed;

        } else if end_of_individuals && current_point >= selection_probabilities[i] {
            if couple_completed {
                mating.insert(first_mate, i);
                couple_completed = !couple_completed;
            }else{
                first_mate = i;
            }
        }

        current_point += pointer_distance;
        next_individual += 1;
    }

    debug!(target="mutation_events", method="stochastic_universal_sampling"; "Stochastic universal sampling finished");
    mating
}