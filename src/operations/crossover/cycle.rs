use crate::structures::GenotypeT;
use crate::structures::GeneT;

pub fn cycle<T: GeneT, U: GenotypeT<T>>(parent_1: &U, parent_2: &U) -> Vec<U> {

    //Creates the children
    let mut child_1: U;
    let mut child_2: U;

    //Creates the temporal vectors
    let mut parent_1_copy = parent_1.clone();
    let mut parent_2_copy = parent_2.clone();

    //Starting the cycles
    let mut cycle_number = 0;

    while parent_1_copy.get_dna().len() > 0 {

        local_cycle(parent_1_copy, parent_2_copy);
        cycle_number += 1;
    }

    return vec![child_1, child_2];
}

/**
 * Function to follow the indexes in the dna
 */
fn local_cycle<T: GeneT, U: GenotypeT<T>>(parent_1: &U, parent_2: &U){

    let mut indexes = Vec::new();
    let mut index = 0;

    //If there is some element in the starting element
    if parent_1.get_dna().get(index).is_some() {
        
        let starting_value = *parent_1.get_dna().get(index).unwrap().get_id();
        let mut current_value = -1;

        //If the values are not the same
        while current_value != starting_value{

            //Adds the index in the vector
            indexes.push(index);
            current_value = *parent_2.get_dna().get(index).unwrap().get_id();

            //Now, we search the index in the parent 1 of the value get in the parent 2
            index = search_index(parent_1, current_value);

            //TODO: Finish the function
        }

    }
}


/**
 * Function to search the vaindex of a value in a vector
 */
fn search_index<T: GeneT, U: GenotypeT<T>>(parent: &U, value: i64) -> usize{

    let mut index = 0;
    while index < parent.get_dna().len() {

        if *parent.get_dna().get(index).unwrap().get_id() == value {
            break;
        }

        index += 1;
    }

    index
}