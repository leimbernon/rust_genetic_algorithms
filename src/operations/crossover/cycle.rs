use crate::traits::GenotypeT;
use crate::traits::GeneT;

pub fn cycle<T: GeneT, U: GenotypeT<T>>(parent_1: &U, parent_2: &U) -> Vec<U> {

    //Creates the children
    let child_1 = U::create();
    let child_2 = U::create();

    //Creates the temporal vectors
    let parent_1_copy = parent_1.clone();
    let parent_2_copy = parent_2.clone();

    //Starting the cycles
    let mut cycle_number = 0;

    while parent_1_copy.get_dna().len() > 0 {

        //Getting the indexes for the current cycle
        let indexes = local_cycle(parent_1_copy, parent_2_copy);

        //Removing the dna indexes from parent 1 and parent 2
        //And setting the values of the DNA
        for i in 0..(&indexes.len()-1){

            //If the cycle number is odd
            if cycle_number & 1 == 1 {

                //Children 1 will take the indexes from parent 2 and child 2 from parent 1
                child_1.get_dna().insert(indexes[i], parent_2.get_dna().get(i).cloned().unwrap());
                child_2.get_dna().insert(indexes[i], parent_1.get_dna().get(i).cloned().unwrap());

            }else{
                //Children 1 will take the indexes from parent 1 and child 2 from parent 2
                child_1.get_dna().insert(indexes[i], parent_1.get_dna().get(i).cloned().unwrap());
                child_2.get_dna().insert(indexes[i], parent_2.get_dna().get(i).cloned().unwrap());
            }

            parent_1_copy.get_dna().remove(i);
            parent_2_copy.get_dna().remove(i);
        }
        
        cycle_number += 1;
    }

    return vec![child_1, child_2];
}

/**
 * Function to follow the indexes in the dna
 */
fn local_cycle<T: GeneT, U: GenotypeT<T>>(parent_1: &U, parent_2: &U) -> Vec<usize>{

    let mut indexes = Vec::new();
    let mut index = 0;

    //If there is some element in the starting element
    if parent_1.get_dna().get(index).is_some() {
        
        let starting_value = *parent_1.get_dna().get(index).unwrap().get_id();
        let mut value_parent_2 = -1;

        //If the values are not the same
        while value_parent_2 != starting_value{

            //Adds the index in the vector
            indexes.push(index);
            value_parent_2 = *parent_2.get_dna().get(index).unwrap().get_id();

            //Now, we search the index in the parent 2 of the value get in the parent 1
            index = search_index(parent_1, value_parent_2);
        }
    }

    return indexes;
}


/**
 * Function to search the index of a value in a vector
 */
fn search_index<T: GeneT, U: GenotypeT<T>>(parent: &U, value: i64) -> usize{

    let mut index = 0;
    while index < parent.get_dna().len() {

        if parent.get_dna().get(index).unwrap().get_id() == &value {
            break;
        }

        index += 1;
    }

    index
}