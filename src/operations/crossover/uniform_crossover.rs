use rand::Rng;
use crate::traits::GenotypeT;

pub fn uniform<U: GenotypeT>(parent_1: &U, parent_2: &U) -> Option<Vec<U>>{

    //Before doing the operation, we check that the dna in the parent 1 has the same length of the dna in the parent 2
    if parent_1.get_dna().len() != parent_2.get_dna().len() {
        panic!("parent 1 and parent 2 must have the same dna length");
    }

    let mut rng = rand::thread_rng();

    //Creation of the children DNA
    let mut dna_child_1 = vec![U::new_gene(); parent_1.get_dna().len()];
    let mut dna_child_2 = vec![U::new_gene(); parent_2.get_dna().len()];

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
    child_1.set_dna(dna_child_1.as_slice());
    child_2.set_dna(dna_child_2.as_slice());

    Some(vec![child_1, child_2])
}