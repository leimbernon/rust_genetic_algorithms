use crate::traits::GenotypeT;

pub fn multipoint_crossover<U: GenotypeT>(parent_1: &U, parent_2: &U, crossover_number_of_points: &i32) -> Option<Vec<U>>{

    //Before doing the operation, we check that the dna in parent 1 has the same length of the dna in parent 2
    if parent_1.get_dna().len() != parent_2.get_dna().len() {
        panic!("parent 1 and parent 2 must have the same dna length");
    }

    let mut child_1 = U::new();
    let mut child_2 = U::new();

    let mut dna_child_1 = Vec::new();
    let mut dna_child_2 = Vec::new();

    //We check if the number of points are higher than the dna, we take the dna lenght
    let number_of_blocks = if (*crossover_number_of_points as usize) + 1 > parent_1.get_dna().len(){parent_1.get_dna().len()}else{(*crossover_number_of_points as usize) + 1};

    //We get the number of genes per block
    let number_of_genes_per_block = (parent_1.get_dna().len() / number_of_blocks) as i64;
    let mut gene_number = 0;
    let mut crossed = false;

    //Here we set the genes to the children
    for gn in 0..parent_1.get_dna().len(){
        
        //Sets the genes of the children
        if !crossed {
            dna_child_1.push(parent_1.get_dna().get(gn).cloned().unwrap());
            dna_child_2.push(parent_2.get_dna().get(gn).cloned().unwrap());
        }else{
            dna_child_1.push(parent_2.get_dna().get(gn).cloned().unwrap());
            dna_child_2.push(parent_1.get_dna().get(gn).cloned().unwrap());
        }

        //Sets the point change
        if gene_number >= number_of_genes_per_block - 1 {
            crossed = !crossed;
            gene_number = 0;
        }else{
            gene_number += 1;
        }
    }

    //Sets the dna into the children and return them
    *child_1.get_dna_mut() = dna_child_1;
    *child_2.get_dna_mut() = dna_child_2;

    Some(vec![child_1, child_2])
}