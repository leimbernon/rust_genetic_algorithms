pub trait GeneT: Default + Clone + Copy {
    fn new() -> Self;
    fn get_id(&self) -> &i64;
}

pub trait GenotypeT<T: GeneT>{
    fn new() -> Self;
    fn get_dna(&self) -> &Vec<T>;
    fn get_dna_mut(&mut self) -> &mut Vec<T>;
    fn get_phenotype(&self) -> &f64;
}