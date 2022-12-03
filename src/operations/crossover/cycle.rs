use crate::traits::GenotypeT;
use crate::traits::GeneT;


pub fn cycle<T: GeneT, U: GenotypeT<T>>(parent_1: &U, parent_2: &U) -> Option<Vec<U>>{

    //Before doing the operation, we check that the dna in the parent 1 has the same length of the dna in the parent 2
    if parent_1.get_dna().len() != parent_2.get_dna().len() {
        panic!("parent 1 and parent 2 must have the same dna length");
    }

    //We create the control variables
    let mut cycle_number = 0;
    let mut indexes: Vec<usize> = Vec::new();

    //Creation of the children DNA
    let mut dna_child_1 = vec![T::new(); parent_1.get_dna().len()];
    let mut dna_child_2 = vec![T::new(); parent_2.get_dna().len()];

    let mut child_1 = U::new();
    let mut child_2 = U::new();
    
    //We loop until having all the elements from the parent 1
    while indexes.len() <= parent_1.get_dna().len() {

        let cycle_indexes = local_cycle(&indexes, &parent_1.get_dna(), &parent_2.get_dna());
        indexes.extend(cycle_indexes.iter().copied());

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
    *child_1.get_dna_mut() = dna_child_1;
    *child_2.get_dna_mut() = dna_child_2;

    return Some(vec![child_1, child_2]);
}


fn local_cycle<T: GeneT>(indexes: &Vec<usize>, dna_parent_1: &Vec<T>, dna_parent_2: &Vec<T>) -> Vec<usize>{

    let mut index = 0;

    //We look for the starting index, that must not be repeated
    for i in 0..dna_parent_1.len() - 1{
        let mut repeated = false;

        for j in 0..indexes.len(){
            if i == indexes[j]{
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
    let starting_value = *dna_parent_1.get(index).unwrap().get_id();
    let mut value_parent_2 = -1;
    let mut cycle_indexes: Vec<usize> = Vec::new();

     //If the values are not the same
     while value_parent_2 != starting_value{

        //Adds the index in the vector
        cycle_indexes.push(index);
        value_parent_2 = *dna_parent_2.get(index).unwrap().get_id();

        //Now, we search the index in the parent 2 of the value get in the parent 1
        index = dna_parent_1.iter().position(|g| g.get_id() == &value_parent_2).unwrap();
     }

    return cycle_indexes;
}