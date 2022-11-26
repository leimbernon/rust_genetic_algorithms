use crate::traits::GenotypeT;
use crate::traits::GeneT;

pub fn cycle<T: GeneT, U: GenotypeT<T>>(parent_1: &U, parent_2: &U) -> Option<Vec<Vec<T>>> {

    //Before doing the operation, we check that the dna in the parent 1 has the same length of the dna in the parent 2
    if parent_1.get_dna().len() != parent_2.get_dna().len() {
        panic!("parent 1 and parent 2 must have the same dna length");
    }

    //Creates the temporal vectors
    let mut dna_parent_1 = parent_1.get_dna().clone();
    let mut dna_parent_2 = parent_2.get_dna().clone();


    //Creates the DNA of the children
    let mut dna_child_1 = vec![T::new(); dna_parent_1.len()];
    let mut dna_child_2 = vec![T::new(); dna_parent_2.len()];

    //Starting the cycles
    let mut cycle_number = 0;

    while dna_parent_1.len() > 0 {

        //Getting the indexes for the current cycle
        let indexes = local_cycle(&dna_parent_1, &dna_parent_2);

        //Removing the dna indexes from parent 1 and parent 2
        //And setting the values of the DNA
        for i in 0..indexes.len(){

            //If the cycle number is odd
            if cycle_number & 1 == 1 {

                //Children 1 will take the indexes from parent 2 and child 2 from parent 1
                dna_child_1.insert(indexes[i], parent_2.get_dna().get(i).cloned().unwrap());
                dna_child_2.insert(indexes[i], parent_1.get_dna().get(i).cloned().unwrap());

            }else{
                //Children 1 will take the indexes from parent 1 and child 2 from parent 2
                dna_child_1.insert(indexes[i], parent_1.get_dna().get(i).cloned().unwrap());
                dna_child_2.insert(indexes[i], parent_2.get_dna().get(i).cloned().unwrap());
            }
        }

        for i in 0..indexes.len(){
            dna_parent_1.remove(indexes[i]);
            dna_parent_2.remove(indexes[i]);
        }
        
        cycle_number += 1;
    }

    return Some(vec![dna_child_1, dna_child_2]);
}

/**
 * Function to follow the indexes in the dna
 */
fn local_cycle<T: GeneT>(dna_parent_1: &Vec<T>, dna_parent_2: &Vec<T>) -> Vec<usize>{

    let mut indexes = Vec::new();
    let mut index = 0;

    //If there is some element in the starting element
    if dna_parent_1.get(index).is_some() {
        
        let starting_value = *dna_parent_1.get(index).unwrap().get_id();
        let mut value_parent_2 = -1;

        //If the values are not the same
        while value_parent_2 != starting_value{

            //Adds the index in the vector
            indexes.push(index);
            value_parent_2 = *dna_parent_2.get(index).unwrap().get_id();

            //Now, we search the index in the parent 2 of the value get in the parent 1
            index = dna_parent_1.iter().position(|g| g.get_id() == &value_parent_2).unwrap();
        }
    }

    return indexes;
}