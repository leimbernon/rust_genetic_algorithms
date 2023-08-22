use rand::Rng;
use crate::traits::GenotypeT;
use crate::traits::GeneT;

pub fn uniform<T: GeneT, U: GenotypeT<T>>(parent_1: &U, parent_2: &U) -> Option<Vec<U>>{
    
    //Before doing the operation, we check that the dna in the parent 1 has the same length of the dna in the parent 2
    if parent_1.get_dna().len() != parent_2.get_dna().len() {
        panic!("parent 1 and parent 2 must have the same dna length");
    }

    let mut rng = rand::thread_rng();

    //Creation of the children DNA
    let mut dna_child_1 = vec![T::new(); parent_1.get_dna().len()];
    let mut dna_child_2 = vec![T::new(); parent_2.get_dna().len()];

    let mut child_1 = U::new();
    let mut child_2 = U::new();

    for i in 0..parent_1.get_dna().len() {
        let crossover = rng.gen_range(0..2);

        //If crossover is 0, we take the genes from the corresponding parents
        if crossover == 0{
            dna_child_1[i] = parent_1.get_dna().get(i).cloned().unwrap();
            dna_child_2[i] = parent_2.get_dna().get(i).cloned().unwrap();
        }else{
            dna_child_1[i] = parent_2.get_dna().get(i).cloned().unwrap();
            dna_child_2[i] = parent_1.get_dna().get(i).cloned().unwrap();
        }
    }
    
    //Setting the DNA to the children
    *child_1.get_dna_mut() = dna_child_1;
    *child_2.get_dna_mut() = dna_child_2;

    Some(vec![child_1, child_2])
}