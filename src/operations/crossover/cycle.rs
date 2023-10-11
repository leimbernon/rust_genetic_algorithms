use crate::traits::GenotypeT;
use crate::traits::GeneT;
use log::{trace, debug};


pub fn cycle<U: GenotypeT>(parent_1: &U, parent_2: &U) -> Option<Vec<U>>{

    //Before doing the operation, we check that the dna in the parent 1 has the same length of the dna in the parent 2
    if parent_1.get_dna().len() != parent_2.get_dna().len() {
        panic!("Parent 1 and parent 2 must have the same dna length. Parent 1 has a length of {} and parent 2 has a length of {}", parent_1.get_dna().len(), parent_2.get_dna().len());
    }

    //We create the control variables
    let mut cycle_number = 0;
    let mut indexes: Vec<usize> = Vec::new();

    //Creation of the children DNA
    let mut dna_child_1 = vec![U::Gene::new(); parent_1.get_dna().len()];
    let mut dna_child_2 = vec![U::Gene::new(); parent_2.get_dna().len()];
    debug!(target="crossover_events", method="cycle"; "Starting the crossover");

    let mut child_1 = U::new();
    let mut child_2 = U::new();
    
    //We loop until having all the elements from the parent 1
    while indexes.len() <= parent_1.get_dna().len() {

        trace!(target="crossover_events", method="cycle"; "Getting the cycle indexes in the cycle number {}", cycle_number);
        let cycle_indexes = local_cycle(&indexes, parent_1.get_dna(), parent_2.get_dna());
        indexes.extend(cycle_indexes.iter().copied());
        trace!(target="crossover_events", method="cycle"; "Cycle indexes calculated in the cycle number {}", cycle_number);

        let is_odd_cycle = cycle_number & 1 == 1;

        //We insert all the values in the children dna
        for i in 0..cycle_indexes.len(){
            if is_odd_cycle {
                dna_child_1[cycle_indexes[i]] = parent_2.get_dna().get(cycle_indexes[i]).cloned().unwrap();
                dna_child_2[cycle_indexes[i]] = parent_1.get_dna().get(cycle_indexes[i]).cloned().unwrap();
            }else{
                dna_child_1[cycle_indexes[i]] = parent_1.get_dna().get(cycle_indexes[i]).cloned().unwrap();
                dna_child_2[cycle_indexes[i]] = parent_2.get_dna().get(cycle_indexes[i]).cloned().unwrap();
            }
        }

        cycle_number += 1;
    }

    //Setting the DNA to the children
    child_1.set_dna(dna_child_1.as_slice());
    child_2.set_dna(dna_child_2.as_slice());
    debug!(target="crossover_events", method="cycle"; "Crossover finished");

    Some(vec![child_1, child_2])
}


fn local_cycle<T: GeneT>(indexes: &Vec<usize>, dna_parent_1: &[T], dna_parent_2: &[T]) -> Vec<usize>{

    let mut index = 0;

    //We look for the starting index, that must not be repeated
    for i in 0..dna_parent_1.len(){
        let mut repeated = false;

        for j in indexes{
            if i == *j{
                repeated = true;
                break;
            }
        }

        //If the number is not repeated, we start from this number
        if !repeated {
            index = i;
            break;
        }
    }

    //Once we have decided the index where to start, we get the starting value
    let starting_value = dna_parent_1.get(index).unwrap().get_id();
    let mut value_parent_2 = -1;
    let mut cycle_indexes: Vec<usize> = Vec::new();

     //If the values are not the same
     while value_parent_2 != starting_value{

        //Adds the index in the vector
        cycle_indexes.push(index);
        value_parent_2 = dna_parent_2.get(index).unwrap().get_id();

        //Now, we search the index in the parent 2 of the value get in the parent 1
        let position_found = dna_parent_1.iter().position(|g| g.get_id() == value_parent_2);
        if let Some(value) = position_found {
            index = value;
        }else{
            panic!("Error finding {} of parent 2 in parent 1", value_parent_2);
        }
     }

    cycle_indexes
}